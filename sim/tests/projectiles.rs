//! Integration tests for projectile spawning and movement.

use ace_simulator::{ProjectileId, ProjectileType, SimConfig, Vec2, World};

#[test]
fn spawn_projectile_allocates_ids_and_records_heading() {
    let mut world = World::new(SimConfig::default());

    let first = world.spawn_projectile(ProjectileType::Dart, Vec2::new(1.0, 2.0), 0.5);
    let second = world.spawn_projectile(ProjectileType::Missile, Vec2::new(3.0, 4.0), -0.5);

    assert_eq!(first, ProjectileId(0));
    assert_eq!(second, ProjectileId(1));
    assert_eq!(world.projectiles.len(), 2);
    assert_eq!(world.projectiles[0].heading, 0.5);
    assert_eq!(world.projectiles[0].position, Vec2::new(1.0, 2.0));
    assert_eq!(world.projectiles[1].heading, -0.5);
    assert_eq!(world.projectiles[1].position, Vec2::new(3.0, 4.0));
}

#[test]
fn darts_travel_in_a_straight_line() {
    let mut world = World::new(SimConfig::default());
    world.spawn_projectile(ProjectileType::Dart, Vec2::ZERO, 0.0);

    world.simulate();

    // A Dart moves 6 units per tick and never turns.
    assert!((world.projectiles[0].position.x - 6.0).abs() < 1e-3);
    assert!(world.projectiles[0].position.y.abs() < 1e-3);
    assert_eq!(world.projectiles[0].heading, 0.0);
}

#[test]
fn darts_expire_after_their_lifespan() {
    let mut world = World::new(SimConfig::default());
    world.spawn_projectile(ProjectileType::Dart, Vec2::ZERO, 0.0);
    // Hold it still so its position does not drift while ticking down.
    world.projectiles[0].speed = 0.0;

    for _ in 0..83 {
        world.simulate();
        assert_eq!(
            world.projectiles.len(),
            1,
            "still alive before its lifespan ends"
        );
    }
    world.simulate();
    assert!(world.projectiles.is_empty(), "expires after 84 ticks");
}

#[test]
fn projectiles_leaving_the_play_area_are_kept() {
    let mut world = World::new(SimConfig::default());
    world.spawn_projectile(ProjectileType::Dart, Vec2::new(199.0, 0.0), 0.0);

    // Moves to x = 205, past the right edge, but survives until its lifespan.
    world.simulate();

    assert_eq!(world.projectiles.len(), 1);
    assert!(world.projectiles[0].position.x > 199.0);
}
