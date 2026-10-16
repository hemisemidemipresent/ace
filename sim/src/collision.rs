//! The pure collision checker.
//!
//! This module reports overlaps between enemies and projectiles. It never
//! mutates: spending pierce, applying damage, and despawning belong to
//! [`crate::World::simulate`].

use crate::broadphase::{Grid, SimConfig, in_bounds};
use crate::hitbox::Vec2;
use crate::types::{Enemy, EnemyId, Projectile, ProjectileId, enemy_stats, projectile_stats};

/// A location: an overlap between one enemy and one projectile during a tick.
///
/// Indices point into the `enemies` and `projectiles` slices passed to
/// [`check_collisions`]. They are only valid until something is removed from
/// those slices, which is why [`crate::World::simulate`] applies them before
/// sweeping.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hit {
    pub enemy: usize,
    pub projectile: usize,
}

/// An overlap between an enemy and a projectile, named by id.
///
/// [`crate::World::simulate`] returns the hits that actually applied damage, so
/// a `Collision` is a connection rather than a bare geometric overlap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Collision {
    pub enemy: EnemyId,
    pub projectile: ProjectileId,
}

/// Report every enemy/projectile pair that overlaps.
///
/// Pairs involving an out-of-bounds object are skipped, as are pairs a
/// projectile cannot damage (missiles only hit MOAB-class Bloons). Each pair is
/// reported at most once, even when several circles of a compound hitbox overlap the same
/// projectile. Hits are ordered by projectile index, then by the distance from
/// the projectile's centre to the nearest overlapping hitbox circle, then by
/// enemy index. This is the order in which pierce must be resolved.
pub fn check_collisions(
    enemies: &[Enemy],
    projectiles: &[Projectile],
    config: &SimConfig,
) -> Vec<Hit> {
    let mut grid = Grid::new(config.cell_size);
    for (index, enemy) in enemies.iter().enumerate() {
        if !in_bounds(enemy.position) {
            continue;
        }
        let stats = enemy_stats(enemy.kind);
        for circle in stats.hitbox.circles() {
            let center = circle.world_center(enemy.position, enemy.heading);
            grid.insert_circle(index, center, circle.radius);
        }
    }

    let mut found = Vec::new();
    for (projectile_index, projectile) in projectiles.iter().enumerate() {
        if !in_bounds(projectile.position) {
            continue;
        }
        let pstats = projectile_stats(projectile.kind);
        for &enemy_index in &grid.query_circle(projectile.position, pstats.radius) {
            let enemy = &enemies[enemy_index];
            if !projectile.kind.can_hit(enemy.kind) {
                continue;
            }
            if let Some(distance_sq) =
                nearest_overlap_distance_sq(enemy, projectile.position, pstats.radius)
            {
                found.push((projectile_index, enemy_index, distance_sq));
            }
        }
    }

    finish(found)
}

/// The same result as [`check_collisions`], without the grid.
///
/// Kept as the reference implementation: it is obviously correct and the tests
/// assert the grid agrees with it.
pub fn check_collisions_bruteforce(
    enemies: &[Enemy],
    projectiles: &[Projectile],
    _config: &SimConfig,
) -> Vec<Hit> {
    let mut found = Vec::new();
    for (projectile_index, projectile) in projectiles.iter().enumerate() {
        if !in_bounds(projectile.position) {
            continue;
        }
        let pstats = projectile_stats(projectile.kind);
        for (enemy_index, enemy) in enemies.iter().enumerate() {
            if !in_bounds(enemy.position) {
                continue;
            }
            if !projectile.kind.can_hit(enemy.kind) {
                continue;
            }
            if let Some(distance_sq) =
                nearest_overlap_distance_sq(enemy, projectile.position, pstats.radius)
            {
                found.push((projectile_index, enemy_index, distance_sq));
            }
        }
    }

    finish(found)
}

/// Sort `(projectile, enemy, distance_sq)` candidates into the canonical hit
/// order and drop duplicates.
fn finish(mut found: Vec<(usize, usize, f32)>) -> Vec<Hit> {
    found.sort_unstable_by(|a, b| {
        a.0.cmp(&b.0)
            .then_with(|| a.2.total_cmp(&b.2))
            .then_with(|| a.1.cmp(&b.1))
    });
    found.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);
    found
        .into_iter()
        .map(|(projectile, enemy, _)| Hit { enemy, projectile })
        .collect()
}

/// The squared distance from `projectile_center` to the nearest overlapping
/// hitbox circle of `enemy`, or `None` if no circle overlaps.
fn nearest_overlap_distance_sq(
    enemy: &Enemy,
    projectile_center: Vec2,
    projectile_radius: f32,
) -> Option<f32> {
    let stats = enemy_stats(enemy.kind);
    let mut nearest: Option<f32> = None;
    for circle in stats.hitbox.circles() {
        let center = circle.world_center(enemy.position, enemy.heading);
        let delta = projectile_center - center;
        let distance_sq = delta.x * delta.x + delta.y * delta.y;
        let reach = projectile_radius + circle.radius;
        if distance_sq <= reach * reach {
            nearest = Some(match nearest {
                Some(current) if current <= distance_sq => current,
                _ => distance_sq,
            });
        }
    }
    nearest
}
