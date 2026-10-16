//! Python bindings for [`ace_simulator`].
//!
//! This crate is a thin, additive bridge: it owns no simulation logic. It
//! exposes a [`Session`] (one `World`), turns Rounds from `rounds.json` into
//! spawn schedules, and reads the whole `World` back out as numpy arrays. The
//! base crate is untouched apart from a public `World::leaks` list and a
//! `Path::waypoints` accessor.

use numpy::PyArray1;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

use ace_simulator as sim;

/// The hard safety cap on an episode: two minutes of game time.
const MAX_TICKS: u64 = 2 * 60 * 60;

/// The bundled, predetermined Rounds.
const ROUNDS_JSON: &str = include_str!("../rounds.json");

// Mirrors the parameter bounds agreed for the training environment.
const BLOON_SPEED_MIN: f32 = 0.05;
const BLOON_SPEED_MAX: f32 = 5.0;
const MOAB_SPEED_MIN: f32 = 0.05;
const MOAB_SPEED_MAX: f32 = 5.0;
const CERAMIC_HEALTH_MIN: f32 = 0.1;
const CERAMIC_HEALTH_MAX: f32 = 20.0;
const MOAB_HEALTH_MIN: f32 = 0.05;
const MOAB_HEALTH_MAX: f32 = 20.0;

/// A single Bloon Group from a Round.
#[derive(Debug, Clone, Copy)]
struct Group {
    kind: sim::EnemyType,
    start: f64,
    end: f64,
    count: u32,
}

/// Map a BTD6-style Bloon name onto an [`sim::EnemyType`]. Case-insensitive.
fn parse_kind(name: &str) -> Result<sim::EnemyType, String> {
    use sim::EnemyType::*;
    Ok(match name.trim().to_ascii_lowercase().as_str() {
        "red" => Red,
        "ceramic" => Ceramic,
        "moab" => MOAB,
        "bfb" => BFB,
        "zomg" => ZOMG,
        "ddt" => DDT,
        "bad" => BAD,
        "ceramicfortified" => CeramicFortified,
        "moabfortified" => MOABFortified,
        "bfbfortified" => BFBFortified,
        "zomgfortified" => ZOMGFortified,
        "ddtfortified" => DDTFortified,
        "badfortified" => BADFortified,
        _ => return Err(format!("unknown Bloon kind `{name}`")),
    })
}

/// Every Round name in `rounds.json`, sorted.
fn round_names() -> Vec<String> {
    let mut names: Vec<String> = serde_json::from_str::<serde_json::Value>(ROUNDS_JSON)
        .ok()
        .and_then(|root| root.as_object().map(|o| o.keys().cloned().collect()))
        .unwrap_or_default();
    names.sort();
    names
}

/// Parse one Round's groups out of `rounds.json`, preserving file order.
fn load_round(name: &str) -> Result<Vec<Group>, String> {
    let root: serde_json::Value =
        serde_json::from_str(ROUNDS_JSON).map_err(|error| error.to_string())?;
    let round = root
        .get(name)
        .ok_or_else(|| format!("unknown Round `{name}` (have {})", round_names().join(", ")))?;
    let groups = round
        .get("groups")
        .and_then(|groups| groups.as_array())
        .ok_or_else(|| format!("Round `{name}` has no `groups` array"))?;

    let mut out = Vec::with_capacity(groups.len());
    for group in groups {
        let bloon = group
            .get("bloon")
            .and_then(|value| value.as_str())
            .ok_or("a Bloon Group is missing `bloon`")?;
        let start = group
            .get("start")
            .and_then(|value| value.as_f64())
            .ok_or("a Bloon Group is missing `start`")?;
        let end = group
            .get("end")
            .and_then(|value| value.as_f64())
            .ok_or("a Bloon Group is missing `end`")?;
        let count = group
            .get("count")
            .and_then(|value| value.as_u64())
            .ok_or("a Bloon Group is missing `count`")? as u32;
        out.push(Group {
            kind: parse_kind(bloon)?,
            start,
            end,
            count,
        });
    }
    Ok(out)
}

/// Queue a group's Bloons. Bloon `i` spawns at
/// `floor(start + i * (end - start) / count)`, in ticks.
fn schedule_group(world: &mut sim::World, group: &Group) {
    if group.count == 0 {
        return;
    }
    let span = group.end - group.start;
    for index in 0..group.count {
        let offset = group.start + index as f64 * span / group.count as f64;
        let tick = offset.max(0.0) as u64;
        world.schedule_spawn(tick, group.kind, sim::PathId::Indexed(0));
    }
}

/// Stable integer codes for Bloon kinds, matching [`kind_names`].
fn kind_code(kind: sim::EnemyType) -> i32 {
    use sim::EnemyType::*;
    match kind {
        Red => 0,
        Ceramic => 1,
        MOAB => 2,
        BFB => 3,
        ZOMG => 4,
        DDT => 5,
        BAD => 6,
        CeramicFortified => 7,
        MOABFortified => 8,
        BFBFortified => 9,
        ZOMGFortified => 10,
        DDTFortified => 11,
        BADFortified => 12,
    }
}

/// Stable integer codes for projectile kinds.
fn projectile_code(kind: sim::ProjectileType) -> i32 {
    match kind {
        sim::ProjectileType::Dart => 0,
        sim::ProjectileType::Missile => 1,
        sim::ProjectileType::Explosion => 2,
    }
}

/// `Indexed(i)` becomes `i`; `Anchored` becomes `-1`.
fn path_code(path_id: sim::PathId) -> i32 {
    match path_id {
        sim::PathId::Anchored => -1,
        sim::PathId::Indexed(index) => index as i32,
    }
}

/// Stable integer codes for the ace's flight pattern.
fn targeting_code(targeting: sim::Targeting) -> i32 {
    match targeting {
        sim::Targeting::Circle => 0,
        sim::Targeting::FigureInfinite => 1,
        sim::Targeting::FigureEight => 2,
    }
}

fn targeting_from_code(code: u8) -> PyResult<sim::Targeting> {
    match code {
        0 => Ok(sim::Targeting::Circle),
        1 => Ok(sim::Targeting::FigureInfinite),
        2 => Ok(sim::Targeting::FigureEight),
        _ => Err(PyValueError::new_err("targeting must be 0, 1, or 2")),
    }
}

/// Total Health a Bloon of `kind` would still leave behind, recursively:
/// the base Health of the kind plus every Child, scaled by the config.
fn recursive_children_health(kind: sim::EnemyType, config: &sim::SimConfig) -> i64 {
    sim::enemy_stats(kind)
        .children
        .iter()
        .map(|&child| config.max_health(child) as i64 + recursive_children_health(child, config))
        .sum()
}

/// The default leaked penalty for one leaked Enemy: its remaining Health plus
/// the recursive Health of the Children it would have left behind.
fn leak_penalty(kind: sim::EnemyType, hp: u32, config: &sim::SimConfig) -> i64 {
    hp as i64 + recursive_children_health(kind, config)
}

fn clamp(value: f64, lo: f32, hi: f32) -> f32 {
    (value as f32).clamp(lo, hi)
}

/// Apply the optional Python `ace` dict onto an [`sim::Ace`].
fn apply_ace_overrides(ace: &mut sim::Ace, dict: &Bound<'_, PyDict>) -> PyResult<()> {
    if let Some(value) = dict.get_item("center")? {
        let (x, y): (f32, f32) = value.extract()?;
        ace.center = sim::Vec2::new(x, y);
    }
    if let Some(value) = dict.get_item("position")? {
        let (x, y): (f32, f32) = value.extract()?;
        ace.position = sim::Vec2::new(x, y);
    }
    if let Some(value) = dict.get_item("target_pos")? {
        let (x, y): (f32, f32) = value.extract()?;
        ace.target_pos = sim::Vec2::new(x, y);
    }
    if let Some(value) = dict.get_item("heading")? {
        ace.heading = value.extract()?;
    }
    if let Some(value) = dict.get_item("phase")? {
        ace.phase = value.extract()?;
    }
    if let Some(value) = dict.get_item("base_velocity")? {
        ace.base_velocity = value.extract()?;
    }
    if let Some(value) = dict.get_item("on_path")? {
        ace.on_path = value.extract()?;
    }
    if let Some(value) = dict.get_item("reverse")? {
        ace.reverse = value.extract()?;
    }
    if let Some(value) = dict.get_item("targeting")? {
        ace.targeting = if let Ok(code) = value.extract::<u8>() {
            targeting_from_code(code)?
        } else {
            let name: String = value.extract()?;
            match name
                .trim()
                .to_ascii_lowercase()
                .replace([' ', '-'], "_")
                .as_str()
            {
                "circle" => sim::Targeting::Circle,
                "figure_infinite" => sim::Targeting::FigureInfinite,
                "figure_eight" => sim::Targeting::FigureEight,
                _ => {
                    return Err(PyValueError::new_err(format!("unknown targeting `{name}`")));
                }
            }
        };
    }
    if let Some(value) = dict.get_item("dart_timer")? {
        ace.attacks.dart_timer = value.extract()?;
    }
    if let Some(value) = dict.get_item("missile_timer")? {
        ace.attacks.missile_timer = value.extract()?;
    }
    Ok(())
}

/// Read the whole `World` into a Python dict of scalars and numpy arrays.
fn observe(
    py: Python<'_>,
    world: &sim::World,
    leaked_step: i64,
    leaked_total: i64,
) -> PyResult<Py<PyDict>> {
    let dict = PyDict::new(py);

    dict.set_item("tick", world.tick)?;
    dict.set_item("n_enemies", world.enemies.len())?;
    dict.set_item("n_projectiles", world.projectiles.len())?;
    dict.set_item("n_pending_spawns", world.spawns.len())?;
    dict.set_item("leaked_penalty_step", leaked_step)?;
    dict.set_item("leaked_penalty_total", leaked_total)?;
    dict.set_item("cell_size", world.config.cell_size)?;
    dict.set_item(
        "bloon_speed_multiplier",
        world.config.bloon_speed_multiplier,
    )?;
    dict.set_item("moab_speed_multiplier", world.config.moab_speed_multiplier)?;
    dict.set_item(
        "ceramic_health_multiplier",
        world.config.ceramic_health_multiplier,
    )?;
    dict.set_item(
        "moab_health_multiplier",
        world.config.moab_health_multiplier,
    )?;

    match world.ace.as_ref() {
        Some(ace) => {
            dict.set_item("ace_present", true)?;
            dict.set_item("ace_center_x", ace.center.x)?;
            dict.set_item("ace_center_y", ace.center.y)?;
            dict.set_item("ace_position_x", ace.position.x)?;
            dict.set_item("ace_position_y", ace.position.y)?;
            dict.set_item("ace_heading", ace.heading)?;
            dict.set_item("ace_phase", ace.phase)?;
            dict.set_item("ace_base_velocity", ace.base_velocity)?;
            dict.set_item("ace_on_path", ace.on_path)?;
            dict.set_item("ace_targeting", targeting_code(ace.targeting))?;
            dict.set_item("ace_reverse", ace.reverse)?;
            dict.set_item("ace_dart_timer", ace.attacks.dart_timer)?;
            dict.set_item("ace_missile_timer", ace.attacks.missile_timer)?;
            dict.set_item("ace_target_pos_x", ace.target_pos.x)?;
            dict.set_item("ace_target_pos_y", ace.target_pos.y)?;
        }
        None => {
            dict.set_item("ace_present", false)?;
        }
    }

    let enemy_count = world.enemies.len();
    let mut enemy_id = Vec::with_capacity(enemy_count);
    let mut enemy_kind = Vec::with_capacity(enemy_count);
    let mut enemy_path = Vec::with_capacity(enemy_count);
    let mut enemy_distance = Vec::with_capacity(enemy_count);
    let mut enemy_x = Vec::with_capacity(enemy_count);
    let mut enemy_y = Vec::with_capacity(enemy_count);
    let mut enemy_heading = Vec::with_capacity(enemy_count);
    let mut enemy_hp = Vec::with_capacity(enemy_count);
    for enemy in &world.enemies {
        enemy_id.push(enemy.id.0);
        enemy_kind.push(kind_code(enemy.kind));
        enemy_path.push(path_code(enemy.path_id));
        enemy_distance.push(enemy.path_distance);
        enemy_x.push(enemy.position.x);
        enemy_y.push(enemy.position.y);
        enemy_heading.push(enemy.heading);
        enemy_hp.push(enemy.hp);
    }
    dict.set_item("enemy_id", PyArray1::from_vec(py, enemy_id))?;
    dict.set_item("enemy_kind", PyArray1::from_vec(py, enemy_kind))?;
    dict.set_item("enemy_path_id", PyArray1::from_vec(py, enemy_path))?;
    dict.set_item(
        "enemy_path_distance",
        PyArray1::from_vec(py, enemy_distance),
    )?;
    dict.set_item("enemy_x", PyArray1::from_vec(py, enemy_x))?;
    dict.set_item("enemy_y", PyArray1::from_vec(py, enemy_y))?;
    dict.set_item("enemy_heading", PyArray1::from_vec(py, enemy_heading))?;
    dict.set_item("enemy_hp", PyArray1::from_vec(py, enemy_hp))?;

    let projectile_count = world.projectiles.len();
    let mut projectile_id = Vec::with_capacity(projectile_count);
    let mut projectile_kind = Vec::with_capacity(projectile_count);
    let mut projectile_x = Vec::with_capacity(projectile_count);
    let mut projectile_y = Vec::with_capacity(projectile_count);
    let mut projectile_heading = Vec::with_capacity(projectile_count);
    let mut projectile_speed = Vec::with_capacity(projectile_count);
    let mut projectile_angular = Vec::with_capacity(projectile_count);
    let mut projectile_pierce = Vec::with_capacity(projectile_count);
    let mut projectile_ticks = Vec::with_capacity(projectile_count);
    let mut hit_projectile = Vec::new();
    let mut hit_enemy = Vec::new();
    for projectile in &world.projectiles {
        projectile_id.push(projectile.id.0);
        projectile_kind.push(projectile_code(projectile.kind));
        projectile_x.push(projectile.position.x);
        projectile_y.push(projectile.position.y);
        projectile_heading.push(projectile.heading);
        projectile_speed.push(projectile.speed);
        projectile_angular.push(projectile.angular_velocity);
        projectile_pierce.push(projectile.pierce_remaining);
        projectile_ticks.push(projectile.ticks_remaining);
        for enemy_id in &projectile.hit_enemies {
            hit_projectile.push(projectile.id.0);
            hit_enemy.push(enemy_id.0);
        }
    }
    dict.set_item("projectile_id", PyArray1::from_vec(py, projectile_id))?;
    dict.set_item("projectile_kind", PyArray1::from_vec(py, projectile_kind))?;
    dict.set_item("projectile_x", PyArray1::from_vec(py, projectile_x))?;
    dict.set_item("projectile_y", PyArray1::from_vec(py, projectile_y))?;
    dict.set_item(
        "projectile_heading",
        PyArray1::from_vec(py, projectile_heading),
    )?;
    dict.set_item("projectile_speed", PyArray1::from_vec(py, projectile_speed))?;
    dict.set_item(
        "projectile_angular_velocity",
        PyArray1::from_vec(py, projectile_angular),
    )?;
    dict.set_item(
        "projectile_pierce_remaining",
        PyArray1::from_vec(py, projectile_pierce),
    )?;
    dict.set_item(
        "projectile_ticks_remaining",
        PyArray1::from_vec(py, projectile_ticks),
    )?;
    dict.set_item("hit_projectile_id", PyArray1::from_vec(py, hit_projectile))?;
    dict.set_item("hit_enemy_id", PyArray1::from_vec(py, hit_enemy))?;

    let spawn_count = world.spawns.len();
    let mut spawn_tick = Vec::with_capacity(spawn_count);
    let mut spawn_kind = Vec::with_capacity(spawn_count);
    let mut spawn_path = Vec::with_capacity(spawn_count);
    for spawn in &world.spawns {
        spawn_tick.push(spawn.tick as i64);
        spawn_kind.push(kind_code(spawn.kind));
        spawn_path.push(path_code(spawn.path_id));
    }
    dict.set_item("spawn_tick", PyArray1::from_vec(py, spawn_tick))?;
    dict.set_item("spawn_kind", PyArray1::from_vec(py, spawn_kind))?;
    dict.set_item("spawn_path_id", PyArray1::from_vec(py, spawn_path))?;

    Ok(dict.unbind())
}

/// One headless simulation World, driven tick-by-tick from Python.
#[pyclass]
struct Session {
    world: sim::World,
    leaked_penalty_total: i64,
}

#[pymethods]
impl Session {
    #[new]
    fn new() -> Self {
        Self {
            world: sim::World::default(),
            leaked_penalty_total: 0,
        }
    }

    /// Start a new episode and return the first observation.
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (
        round,
        ceramic_health_multiplier=1.0,
        moab_health_multiplier=1.0,
        bloon_speed_multiplier=1.0,
        moab_speed_multiplier=1.0,
        ace=None
    ))]
    fn reset(
        &mut self,
        py: Python<'_>,
        round: &str,
        ceramic_health_multiplier: f64,
        moab_health_multiplier: f64,
        bloon_speed_multiplier: f64,
        moab_speed_multiplier: f64,
        ace: Option<Bound<'_, PyDict>>,
    ) -> PyResult<Py<PyDict>> {
        let groups = load_round(round).map_err(PyValueError::new_err)?;
        let config = sim::SimConfig {
            ceramic_health_multiplier: clamp(
                ceramic_health_multiplier,
                CERAMIC_HEALTH_MIN,
                CERAMIC_HEALTH_MAX,
            ),
            moab_health_multiplier: clamp(moab_health_multiplier, MOAB_HEALTH_MIN, MOAB_HEALTH_MAX),
            bloon_speed_multiplier: clamp(bloon_speed_multiplier, BLOON_SPEED_MIN, BLOON_SPEED_MAX),
            moab_speed_multiplier: clamp(moab_speed_multiplier, MOAB_SPEED_MIN, MOAB_SPEED_MAX),
            ..Default::default()
        };

        let mut world = sim::World::new(config);
        let mut ace_state = sim::Ace::default();
        if let Some(dict) = ace.as_ref() {
            apply_ace_overrides(&mut ace_state, dict)?;
        }
        world.ace = Some(ace_state);
        for group in groups {
            schedule_group(&mut world, &group);
        }

        self.world = world;
        self.leaked_penalty_total = 0;
        observe(py, &self.world, 0, 0)
    }

    /// Hold `targeting`/`reverse` for `ticks` ticks and return
    /// `(observation, terminated, truncated)`.
    #[pyo3(signature = (targeting, reverse, ticks=6))]
    fn step(
        &mut self,
        py: Python<'_>,
        targeting: u8,
        reverse: bool,
        ticks: u32,
    ) -> PyResult<(Py<PyDict>, bool, bool)> {
        if let Some(ace) = self.world.ace.as_mut() {
            ace.targeting = targeting_from_code(targeting)?;
            ace.reverse = reverse;
        }

        let remaining = MAX_TICKS.saturating_sub(self.world.tick);
        let run = (ticks as u64).min(remaining);
        let mut leaked_step = 0i64;
        for _ in 0..run {
            self.world.simulate();
            for leak in &self.world.leaks {
                leaked_step += leak_penalty(leak.kind, leak.hp, &self.world.config);
            }
        }
        self.leaked_penalty_total += leaked_step;

        let terminated = self.world.spawns.is_empty() && self.world.enemies.is_empty();
        let truncated = self.world.tick >= MAX_TICKS;
        let observation = observe(py, &self.world, leaked_step, self.leaked_penalty_total)?;
        Ok((observation, terminated, truncated))
    }

    /// The current observation without advancing the World.
    fn state(&self, py: Python<'_>) -> PyResult<Py<PyDict>> {
        observe(py, &self.world, 0, self.leaked_penalty_total)
    }

    /// The track geometry: one dict per Path, with flat waypoint arrays.
    fn paths(&self, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let dict = PyDict::new(py);
        let paths = PyList::empty(py);
        for index in 0..self.world.paths.len() {
            let path = self.world.paths.get(sim::PathId::Indexed(index as u32));
            let entry = PyDict::new(py);
            entry.set_item("name", path.name())?;
            entry.set_item("length", path.length())?;
            let mut x = Vec::new();
            let mut y = Vec::new();
            let mut distance = Vec::new();
            let mut speed_multiplier = Vec::new();
            for waypoint in path.waypoints() {
                x.push(waypoint.position.x);
                y.push(waypoint.position.y);
                distance.push(waypoint.distance);
                speed_multiplier.push(waypoint.speed_multiplier);
            }
            entry.set_item("waypoint_x", PyArray1::from_vec(py, x))?;
            entry.set_item("waypoint_y", PyArray1::from_vec(py, y))?;
            entry.set_item("waypoint_distance", PyArray1::from_vec(py, distance))?;
            entry.set_item(
                "waypoint_speed_multiplier",
                PyArray1::from_vec(py, speed_multiplier),
            )?;
            paths.append(entry)?;
        }
        dict.set_item("paths", paths)?;
        Ok(dict.unbind())
    }

    /// A fixed-capacity flat float32 view, padded with zeros.
    ///
    /// Returns `global` (1-D), `enemies` and `projectiles` (flat, reshape by
    /// the advertised strides), plus the strides themselves.
    #[pyo3(signature = (max_bloons=512, max_projectiles=4096))]
    fn observe_flat(
        &self,
        py: Python<'_>,
        max_bloons: usize,
        max_projectiles: usize,
    ) -> PyResult<Py<PyDict>> {
        const ENEMY_STRIDE: usize = 8;
        const PROJECTILE_STRIDE: usize = 9;

        let mut global = vec![
            self.world.tick as f32,
            self.world.enemies.len() as f32,
            self.world.projectiles.len() as f32,
            self.world.spawns.len() as f32,
            self.leaked_penalty_total as f32,
            self.world.config.bloon_speed_multiplier,
            self.world.config.moab_speed_multiplier,
            self.world.config.ceramic_health_multiplier,
            self.world.config.moab_health_multiplier,
        ];
        match self.world.ace.as_ref() {
            Some(ace) => global.extend_from_slice(&[
                ace.center.x,
                ace.center.y,
                ace.position.x,
                ace.position.y,
                ace.heading,
                ace.phase,
                ace.base_velocity,
                if ace.on_path { 1.0 } else { 0.0 },
                targeting_code(ace.targeting) as f32,
                if ace.reverse { 1.0 } else { 0.0 },
                ace.attacks.dart_timer,
                ace.attacks.missile_timer,
                ace.target_pos.x,
                ace.target_pos.y,
            ]),
            None => global.extend(std::iter::repeat_n(0.0, 14)),
        }

        let mut enemies = vec![0.0f32; max_bloons * ENEMY_STRIDE];
        for (index, enemy) in self.world.enemies.iter().take(max_bloons).enumerate() {
            let base = index * ENEMY_STRIDE;
            enemies[base] = enemy.position.x;
            enemies[base + 1] = enemy.position.y;
            enemies[base + 2] = enemy.heading;
            enemies[base + 3] = enemy.hp as f32;
            enemies[base + 4] = enemy.path_distance;
            enemies[base + 5] = kind_code(enemy.kind) as f32;
            enemies[base + 6] = path_code(enemy.path_id) as f32;
            enemies[base + 7] = 1.0;
        }

        let mut projectiles = vec![0.0f32; max_projectiles * PROJECTILE_STRIDE];
        for (index, projectile) in self
            .world
            .projectiles
            .iter()
            .take(max_projectiles)
            .enumerate()
        {
            let base = index * PROJECTILE_STRIDE;
            projectiles[base] = projectile.position.x;
            projectiles[base + 1] = projectile.position.y;
            projectiles[base + 2] = projectile.heading;
            projectiles[base + 3] = projectile.speed;
            projectiles[base + 4] = projectile.angular_velocity;
            projectiles[base + 5] = projectile.pierce_remaining as f32;
            projectiles[base + 6] = projectile.ticks_remaining as f32;
            projectiles[base + 7] = projectile_code(projectile.kind) as f32;
            projectiles[base + 8] = 1.0;
        }

        let dict = PyDict::new(py);
        dict.set_item("global", PyArray1::from_vec(py, global))?;
        dict.set_item("enemies", PyArray1::from_vec(py, enemies))?;
        dict.set_item("projectiles", PyArray1::from_vec(py, projectiles))?;
        dict.set_item("enemy_stride", ENEMY_STRIDE)?;
        dict.set_item("projectile_stride", PROJECTILE_STRIDE)?;
        Ok(dict.unbind())
    }

    #[getter]
    fn leaked_penalty_total(&self) -> i64 {
        self.leaked_penalty_total
    }
}

/// The Bundled Round names.
#[pyfunction]
fn list_rounds() -> Vec<String> {
    round_names()
}

/// Bloon kind names indexed by the integer code used in observations.
#[pyfunction]
fn kind_names() -> Vec<&'static str> {
    vec![
        "Red",
        "Ceramic",
        "MOAB",
        "BFB",
        "ZOMG",
        "DDT",
        "BAD",
        "CeramicFortified",
        "MOABFortified",
        "BFBFortified",
        "ZOMGFortified",
        "DDTFortified",
        "BADFortified",
    ]
}

/// Projectile kind names indexed by the integer code used in observations.
#[pyfunction]
fn projectile_names() -> Vec<&'static str> {
    vec!["Dart", "Missile", "Explosion"]
}

/// Targeting names indexed by the integer code used in observations.
#[pyfunction]
fn targeting_names() -> Vec<&'static str> {
    vec!["circle", "figure_infinite", "figure_eight"]
}

#[pymodule]
fn _ace_sim(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<Session>()?;
    module.add_function(wrap_pyfunction!(list_rounds, module)?)?;
    module.add_function(wrap_pyfunction!(kind_names, module)?)?;
    module.add_function(wrap_pyfunction!(projectile_names, module)?)?;
    module.add_function(wrap_pyfunction!(targeting_names, module)?)?;
    Ok(())
}
