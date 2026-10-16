//! Demo: a Monkey Ace fires darts and missiles while bloons travel the path.

use ace_simulator::{Ace, Enemy, EnemyId, EnemyType, PathId, SimConfig, Vec2, World};

fn main() {
    let mut world = World::new(SimConfig::default());
    world.ace = Some(Ace::new(Vec2::ZERO, 0.0));
    world.schedule_spawn(0, EnemyType::Red, PathId::Indexed(0));
    world.schedule_spawn(0, EnemyType::MOAB, PathId::Indexed(0));

    // A stationary target above the ace so the demo shows a dart connecting.
    world
        .enemies
        .push(Enemy::new(EnemyId(0), EnemyType::Red, Vec2::new(0.0, 20.0)));

    for tick in 0..20 {
        let hits = world.simulate();
        println!(
            "tick {tick}: {} hit(s), {} enemy/enemies, {} projectile(s)",
            hits.len(),
            world.enemies.len(),
            world.projectiles.len()
        );
    }
}
