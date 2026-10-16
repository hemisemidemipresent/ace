//! `ace_simulator` — a headless, frame-by-frame simulation of BTD6 Monkey Ace
//! combat.
//!
//! Bloons spawn on a schedule, travel a hardcoded path, and pop into children.
//! A Monkey Ace fires radial darts and seeking missiles, and the collision
//! checker turns those into damage. See `README.md` for the overview and
//! `docs/` for the contracts.

pub mod ace;
pub mod broadphase;
pub mod collision;
pub mod hitbox;
pub mod movement;
pub mod paths;
pub mod types;
pub mod world;

/// Simulation ticks per second.
pub const TICKS_PER_SECOND: f32 = 60.0;
/// Seconds per tick.
pub const TICK_DT: f32 = 1.0 / TICKS_PER_SECOND;

pub use ace::{Ace, AceAttacks, Targeting};
pub use broadphase::{SimConfig, X_MAX, X_MIN, Y_MAX, Y_MIN, in_bounds};
pub use collision::{Collision, Hit, check_collisions, check_collisions_bruteforce};
pub use hitbox::{Circle, EnemyHitbox, Vec2, circles_overlap, wrap_pi};
pub use movement::{move_enemies, move_projectiles, strongest_moab_target, strongest_target};
pub use paths::{Path, PathId, Paths, Waypoint};
pub use types::{
    Enemy, EnemyId, EnemyStats, EnemyType, Projectile, ProjectileId, ProjectileStats,
    ProjectileType, SeekingStats, Spawn, enemy_stats, projectile_stats,
};
pub use world::{Leak, World};
