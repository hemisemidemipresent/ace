//! The mutable simulation state and its per-tick entry point.

use crate::ace::{
    Ace, DART_COUNT, DART_INTERVAL_TICKS, DART_OFFSET, MISSILE_INTERVAL_TICKS, MISSILE_OFFSETS,
};
use crate::broadphase::SimConfig;
use crate::collision::{Collision, Hit, check_collisions};
use crate::hitbox::Vec2;
use crate::movement::{move_enemies, move_projectiles};
use crate::paths::{PathId, Paths};
use crate::types::{
    Enemy, EnemyId, EnemyType, Projectile, ProjectileId, ProjectileType, Spawn, enemy_stats,
    projectile_stats,
};

/// The complete mutable state of a running simulation.
#[derive(Debug, Clone)]
pub struct World {
    pub enemies: Vec<Enemy>,
    pub projectiles: Vec<Projectile>,
    /// The Monkey Ace firing projectiles, if one is present.
    pub ace: Option<Ace>,
    /// The track geometry enemies travel along.
    pub paths: Paths,
    /// Bloons queued to enter, keyed by the tick they appear on.
    pub spawns: Vec<Spawn>,
    /// Enemies that leaked on the most recent [`World::simulate`], with the
    /// Health they had when they left. Repopulated every tick.
    pub leaks: Vec<Leak>,
    /// How many ticks have elapsed.
    pub tick: u64,
    pub config: SimConfig,
    next_enemy_id: u32,
    next_projectile_id: u32,
}

/// An Enemy that reached the end of its Path and was removed from the World.
///
/// Recorded in [`World::leaks`] so callers can account for leaks without
/// inspecting Enemies that no longer exist.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Leak {
    pub id: EnemyId,
    pub kind: EnemyType,
    /// Health remaining when the Enemy left; always positive.
    pub hp: u32,
}

impl Default for World {
    fn default() -> Self {
        Self::new(SimConfig::default())
    }
}

impl World {
    /// Create an empty world with the given configuration and the bundled paths.
    pub fn new(config: SimConfig) -> Self {
        Self {
            enemies: Vec::new(),
            projectiles: Vec::new(),
            ace: None,
            paths: Paths::bundled(),
            spawns: Vec::new(),
            leaks: Vec::new(),
            tick: 0,
            config,
            next_enemy_id: 0,
            next_projectile_id: 0,
        }
    }

    /// Queue a Bloon to enter on `tick`.
    pub fn schedule_spawn(&mut self, tick: u64, kind: EnemyType, path_id: PathId) {
        self.spawns.push(Spawn {
            tick,
            kind,
            path_id,
        });
    }

    /// Put a Bloon at the start of `path_id` right now and return its id.
    ///
    /// Panics if `path_id` is [`PathId::Anchored`].
    pub fn spawn_enemy(&mut self, kind: EnemyType, path_id: PathId) -> EnemyId {
        self.spawn_enemy_at(kind, path_id, 0.0)
    }

    /// Put a Bloon `path_distance` along `path_id` right now and return its id.
    ///
    /// Panics if `path_id` is [`PathId::Anchored`].
    pub fn spawn_enemy_at(
        &mut self,
        kind: EnemyType,
        path_id: PathId,
        path_distance: f32,
    ) -> EnemyId {
        let id = EnemyId(self.next_enemy_id);
        self.next_enemy_id += 1;
        let mut enemy = Enemy::spawn(id, kind, path_id, path_distance, &self.paths);
        enemy.hp = self.config.max_health(kind);
        self.enemies.push(enemy);
        id
    }

    /// Put a projectile into the world right now and return its id.
    pub fn spawn_projectile(
        &mut self,
        kind: ProjectileType,
        position: Vec2,
        heading: f32,
    ) -> ProjectileId {
        self.spawn_projectile_with(kind, position, heading, 0.0)
    }

    /// Advance the simulation by one tick.
    ///
    /// Order: move enemies, move projectiles, spawn the bloons due this tick,
    /// fire the ace, run the pure collision check, apply its hits (damage and
    /// pierce), detonate the missiles that connected, then resolve deaths, leaks,
    /// and child spawning. Because spawning happens after movement, a newly
    /// spawned Bloon or projectile is not moved until the next tick. Returns the
    /// hits that actually connected, by id.
    pub fn simulate(&mut self) -> Vec<Collision> {
        self.leaks.clear();
        if let Some(ace) = self.ace.as_mut() {
            ace.process_movement();
        }
        move_enemies(&mut self.enemies, &self.paths, &self.config);
        move_projectiles(&mut self.projectiles, &self.enemies);
        self.spawn_due();
        self.spawn_projectiles();

        let hits = check_collisions(&self.enemies, &self.projectiles, &self.config);
        let applied = apply_collisions(self, &hits);
        detonate_missiles(self, &applied);
        resolve_deaths(self);

        self.tick += 1;
        applied
    }

    /// Spawn every bloons scheduled for the current tick.
    fn spawn_due(&mut self) {
        let mut index = 0;
        while index < self.spawns.len() {
            if self.spawns[index].tick == self.tick {
                let spawn = self.spawns.swap_remove(index);
                self.spawn_enemy(spawn.kind, spawn.path_id);
            } else {
                index += 1;
            }
        }
    }

    /// Fire the ace's attacks for the current tick.
    ///
    /// Runs after Bloon spawning and before the collision check, so a freshly
    /// fired projectile can connect on the tick it appears. Each attack has its
    /// own fractional-tick timer, so the dart's 18.9-tick cadence carries its
    /// remainder.
    fn spawn_projectiles(&mut self) {
        let mut spawns: Vec<(ProjectileType, Vec2, f32, f32)> = Vec::new();
        {
            let World { ace, enemies, .. } = self;
            let Some(ace) = ace.as_mut() else {
                return;
            };

            ace.attacks.dart_timer += 1.0;
            ace.attacks.missile_timer += 1.0;

            let origin_base = ace.world_position();

            if ace.attacks.dart_timer >= DART_INTERVAL_TICKS {
                ace.attacks.dart_timer -= DART_INTERVAL_TICKS;
                let origin = origin_base + DART_OFFSET.rotate(ace.heading);
                for index in 0..DART_COUNT {
                    let heading =
                        ace.heading + index as f32 * std::f32::consts::TAU / DART_COUNT as f32;
                    spawns.push((ProjectileType::Dart, origin, heading, 0.0));
                }
            }

            if ace.attacks.missile_timer >= MISSILE_INTERVAL_TICKS {
                // Missiles only fire at MOAB-class Bloons. With no such target
                // the timer holds at the interval, so a volley is ready the tick
                // a MOAB first appears.
                if enemies.iter().any(|enemy| enemy.kind.is_moab_plus()) {
                    ace.attacks.missile_timer -= MISSILE_INTERVAL_TICKS;
                    let seeking = projectile_stats(ProjectileType::Missile)
                        .seeking
                        .expect("missiles seek");
                    for offset in MISSILE_OFFSETS {
                        let origin = origin_base + offset.rotate(ace.heading);
                        spawns.push((
                            ProjectileType::Missile,
                            origin,
                            ace.heading,
                            seeking.initial_angular_velocity,
                        ));
                    }
                } else {
                    ace.attacks.missile_timer = MISSILE_INTERVAL_TICKS;
                }
            }
        }

        for (kind, position, heading, angular_velocity) in spawns {
            self.spawn_projectile_with(kind, position, heading, angular_velocity);
        }
    }

    fn spawn_projectile_with(
        &mut self,
        kind: ProjectileType,
        position: Vec2,
        heading: f32,
        angular_velocity: f32,
    ) -> ProjectileId {
        let id = ProjectileId(self.next_projectile_id);
        self.next_projectile_id += 1;
        let mut projectile = Projectile::new(id, kind, position, heading);
        projectile.angular_velocity = angular_velocity;
        self.projectiles.push(projectile);
        id
    }
}

/// Apply damage and spend pierce for each hit, in the checker's order.
///
/// No entity is removed here: every index in `hits` must stay valid for the
/// whole pass, so removal happens later in [`resolve_deaths`].
fn apply_collisions(world: &mut World, hits: &[Hit]) -> Vec<Collision> {
    let mut applied = Vec::new();
    for hit in hits {
        let enemy_index = hit.enemy;
        let projectile_index = hit.projectile;

        if world.projectiles[projectile_index].is_spent() {
            continue;
        }
        // A dead enemy no longer blocks a projectile: the hit is skipped without spending pierce.
        if world.enemies[enemy_index].hp == 0 {
            continue;
        }

        let enemy_id = world.enemies[enemy_index].id;
        if world.projectiles[projectile_index].has_hit(enemy_id) {
            continue;
        }

        let damage = projectile_stats(world.projectiles[projectile_index].kind).damage;
        world.enemies[enemy_index].hp = world.enemies[enemy_index].hp.saturating_sub(damage);
        world.projectiles[projectile_index].pierce_remaining -= 1;
        world.projectiles[projectile_index]
            .hit_enemies
            .push(enemy_id);

        applied.push(Collision {
            enemy: enemy_id,
            projectile: world.projectiles[projectile_index].id,
        });
    }
    applied
}

/// Detonate every missile that connected this tick.
///
/// A missile deals no damage; each of its contacts instead spawns a one-tick
/// [`ProjectileType::Explosion`] at the missile's position. This runs after
/// [`apply_collisions`] so the contacts are known, and before [`resolve_deaths`]
/// so the spent missiles are still present to be read. Neither the pure checker
/// nor [`apply_collisions`] constructs a projectile.
fn detonate_missiles(world: &mut World, applied: &[Collision]) {
    let mut blasts = Vec::new();
    for collision in applied {
        let Some(projectile) = world
            .projectiles
            .iter()
            .find(|projectile| projectile.id == collision.projectile)
        else {
            continue;
        };
        if projectile.kind == ProjectileType::Missile {
            blasts.push(projectile.position);
        }
    }

    for position in blasts {
        world.spawn_projectile(ProjectileType::Explosion, position, 0.0);
    }
}

/// Remove popped and leaked enemies and dead projectiles, then emit children.
///
/// Children are emitted only after the whole pass, so they cannot be moved or
/// hit until a later tick. Anchored parents have no path and emit nothing;
/// leaked parents were not popped and emit nothing. A projectile is dead when it
/// has spent its pierce or run out its lifespan.
fn resolve_deaths(world: &mut World) {
    let popped: Vec<(EnemyType, PathId, f32)> = world
        .enemies
        .iter()
        .filter(|enemy| enemy.hp == 0)
        .map(|enemy| (enemy.kind, enemy.path_id, enemy.path_distance))
        .collect();

    // Capture leaked Enemies before they are removed. A Bloon at zero Health
    // has popped and counts as a pop, not a leak.
    let leaks: Vec<Leak> = world
        .enemies
        .iter()
        .filter(|enemy| enemy.hp > 0 && is_leaked(enemy, &world.paths))
        .map(|enemy| Leak {
            id: enemy.id,
            kind: enemy.kind,
            hp: enemy.hp,
        })
        .collect();
    world.leaks.extend(leaks);

    {
        let World {
            enemies,
            paths,
            projectiles,
            ..
        } = world;
        enemies.retain(|enemy| enemy.hp > 0 && !is_leaked(enemy, paths));
        projectiles.retain(|projectile| !projectile.is_spent() && projectile.ticks_remaining > 0);
    }

    for (kind, path_id, parent_distance) in popped {
        spawn_children(world, kind, path_id, parent_distance);
    }
}

/// Whether an enemy has travelled past the end of its path.
fn is_leaked(enemy: &Enemy, paths: &Paths) -> bool {
    match enemy.path_id {
        PathId::Anchored => false,
        PathId::Indexed(_) => enemy.path_distance >= paths.get(enemy.path_id).length(),
    }
}

/// Emit the children of a popped Bloon.
fn spawn_children(world: &mut World, kind: EnemyType, path_id: PathId, parent_distance: f32) {
    if matches!(path_id, PathId::Anchored) {
        return;
    }
    let children = enemy_stats(kind).children;
    for (child_kind, distance) in children
        .iter()
        .zip(child_distances(parent_distance, children.len()))
    {
        world.spawn_enemy_at(*child_kind, path_id, distance);
    }
}

/// Distances for a brood of `count` children trailing a parent at
/// `parent_distance`.
///
/// Spacing is `min(26.7 / count, 8.6)`: the first child sits at the parent, and
/// each later one trails the previous by that spacing. Distances are clamped to
/// zero so children cannot precede the path start.
fn child_distances(parent_distance: f32, count: usize) -> Vec<f32> {
    if count == 0 {
        return Vec::new();
    }
    let spacing = (26.7 / count as f32).min(8.6);
    (0..count)
        .map(|index| (parent_distance - index as f32 * spacing).max(0.0))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::child_distances;

    fn assert_close(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < 1e-3,
            "expected {expected}, got {actual}"
        );
    }

    #[test]
    fn spacing_is_capped_for_small_broods() {
        // 26.7 / 3 = 8.9, capped to 8.6.
        let distances = child_distances(100.0, 3);
        assert_close(distances[0], 100.0);
        assert_close(distances[1], 91.4);
        assert_close(distances[2], 82.8);
    }

    #[test]
    fn spacing_shrinks_for_large_broods() {
        // 26.7 / 4 = 6.675, below the cap.
        let distances = child_distances(100.0, 4);
        assert_close(distances[1], 93.325);
        assert_close(distances[2], 86.65);
        assert_close(distances[3], 79.975);
    }

    #[test]
    fn children_before_the_start_clamp_to_zero() {
        assert_eq!(child_distances(2.0, 4), vec![2.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn no_children_means_no_distances() {
        assert!(child_distances(100.0, 0).is_empty());
    }
}
