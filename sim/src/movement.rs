//! Bloon and projectile movement.

use crate::broadphase::SimConfig;
use crate::hitbox::{Vec2, wrap_pi};
use crate::paths::{PathId, Paths};
use crate::types::{Enemy, Projectile, enemy_stats, projectile_stats};

/// Advance every on-path enemy by one tick.
///
/// Anchored enemies are skipped. Distance is clamped to the path length, so an
/// enemy that reaches the end waits there until the sweep removes it. Speed is
/// the kind's base speed, the path's multiplier at the enemy's position, and the
/// config's Bloon or MOAB speed multiplier.
pub fn move_enemies(enemies: &mut [Enemy], paths: &Paths, config: &SimConfig) {
    for enemy in enemies.iter_mut() {
        if let PathId::Anchored = enemy.path_id {
            continue;
        }
        let path = paths.get(enemy.path_id);
        let speed = enemy_stats(enemy.kind).speed
            * path.speed_multiplier_at(enemy.path_distance)
            * config.speed_multiplier(enemy.kind);
        let distance = (enemy.path_distance + speed).min(path.length());
        enemy.path_distance = distance;
        let (position, heading) = path.sample(distance);
        enemy.position = position;
        enemy.heading = heading;
    }
}

/// Advance every projectile by one tick.
///
/// Straight projectiles travel along their heading. Seeking projectiles steer
/// toward [`strongest_moab_target`] — the strongest Bloon they can actually hit
/// — accelerating their speed and turn rate. Every projectile's lifespan ticks
/// down by one.
pub fn move_projectiles(projectiles: &mut [Projectile], enemies: &[Enemy]) {
    let target = strongest_moab_target(enemies).map(|enemy| enemy.position);

    for projectile in projectiles.iter_mut() {
        let stats = projectile_stats(projectile.kind);
        if let Some(seeking) = stats.seeking {
            projectile.speed = (projectile.speed + seeking.acceleration).min(seeking.max_speed);
            projectile.angular_velocity = (projectile.angular_velocity
                + seeking.angular_acceleration)
                .min(seeking.max_angular_speed);
            let turn = match target {
                Some(target) => {
                    wrap_pi((target - projectile.position).heading() - projectile.heading)
                        .clamp(-projectile.angular_velocity, projectile.angular_velocity)
                }
                None => 0.0,
            };
            projectile.heading = wrap_pi(projectile.heading + turn);
        }

        projectile.position = projectile.position
            + Vec2::new(
                projectile.heading.cos() * projectile.speed,
                projectile.heading.sin() * projectile.speed,
            );
        projectile.ticks_remaining = projectile.ticks_remaining.saturating_sub(1);
    }
}

/// The strongest enemy: highest danger, then furthest along its path, then the
/// lowest id.
pub fn strongest_target(enemies: &[Enemy]) -> Option<&Enemy> {
    strongest(enemies.iter())
}

/// The strongest MOAB-class enemy, by the same ranking as [`strongest_target`].
///
/// Missiles only collide with MOAB-class Bloons, so they ignore the rest when
/// choosing what to chase.
pub fn strongest_moab_target(enemies: &[Enemy]) -> Option<&Enemy> {
    strongest(enemies.iter().filter(|enemy| enemy.kind.is_moab_plus()))
}

/// Rank enemies by danger, then path distance, then lowest id.
fn strongest<'a>(enemies: impl Iterator<Item = &'a Enemy>) -> Option<&'a Enemy> {
    enemies.max_by(|a, b| {
        enemy_stats(a.kind)
            .danger
            .cmp(&enemy_stats(b.kind).danger)
            .then(a.path_distance.total_cmp(&b.path_distance))
            .then(b.id.0.cmp(&a.id.0))
    })
}
