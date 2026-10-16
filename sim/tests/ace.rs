//! Integration tests for the Monkey Ace's attacks and targeting.

use ace_simulator::{
    Ace, Enemy, EnemyId, EnemyType, ProjectileType, SimConfig, TICK_DT, Vec2, World,
    strongest_moab_target, strongest_target, wrap_pi,
};

/// Fire the dart volley and return the ace's position and heading when it fired.
///
/// `process_movement` runs at the start of a tick, before the attack, so the
/// ace's state after the firing tick is the state the darts launched from.
fn fire_dart_volley(world: &mut World) -> (Vec2, f32) {
    for _ in 0..18 {
        world.simulate();
    }
    world.simulate();
    let ace = world.ace.unwrap();
    (ace.world_position(), ace.heading)
}

#[test]
fn ace_fires_a_ring_of_32_darts() {
    let mut world = World::new(SimConfig::default());
    world.ace = Some(Ace::new(Vec2::ZERO, 0.0));
    let (origin, heading) = fire_dart_volley(&mut world);

    let darts: Vec<_> = world
        .projectiles
        .iter()
        .filter(|projectile| projectile.kind == ProjectileType::Dart)
        .collect();

    assert_eq!(darts.len(), 32);

    // Every dart leaves the same offset, rotated by the ace's heading.
    let offset = Vec2::new(0.0, 6.5).rotate(heading);
    for dart in &darts {
        assert!((dart.position.x - (origin.x + offset.x)).abs() < 1e-3);
        assert!((dart.position.y - (origin.y + offset.y)).abs() < 1e-3);
    }

    // The ring is centred on the ace's heading.
    for (index, dart) in darts.iter().enumerate() {
        let expected = heading + index as f32 * std::f32::consts::TAU / 32.0;
        assert!(
            (dart.heading - expected).abs() < 1e-4,
            "dart {index} should point at {expected}, got {}",
            dart.heading
        );
    }
}

#[test]
fn ace_fires_two_missiles() {
    let mut world = World::new(SimConfig::default());
    world.ace = Some(Ace::new(Vec2::ZERO, 0.0));
    world.enemies.push(Enemy::new(
        EnemyId(0),
        EnemyType::MOAB,
        Vec2::new(0.0, 100.0),
    ));

    for _ in 0..89 {
        world.simulate();
    }
    world.simulate();

    let ace = world.ace.unwrap();
    let origin = ace.world_position();
    let heading = ace.heading;

    let missiles: Vec<_> = world
        .projectiles
        .iter()
        .filter(|projectile| projectile.kind == ProjectileType::Missile)
        .collect();

    assert_eq!(missiles.len(), 2);

    let left = Vec2::new(12.0, -0.24).rotate(heading);
    let right = Vec2::new(-12.0, -0.24).rotate(heading);
    assert!((missiles[0].position.x - (origin.x + left.x)).abs() < 1e-3);
    assert!((missiles[0].position.y - (origin.y + left.y)).abs() < 1e-3);
    assert!((missiles[1].position.x - (origin.x + right.x)).abs() < 1e-3);
    assert!((missiles[1].position.y - (origin.y + right.y)).abs() < 1e-3);
    assert!((missiles[0].heading - heading).abs() < 1e-3);
}

#[test]
fn ace_does_not_fire_missiles_without_a_moab() {
    let mut world = World::new(SimConfig::default());
    world.ace = Some(Ace::new(Vec2::ZERO, 0.0));
    // A non-MOAB enemy is present, but missiles require a MOAB-class target.
    world.enemies.push(Enemy::new(
        EnemyId(0),
        EnemyType::Ceramic,
        Vec2::new(1000.0, 1000.0),
    ));

    for _ in 0..200 {
        world.simulate();
    }

    let missiles = world
        .projectiles
        .iter()
        .filter(|projectile| projectile.kind == ProjectileType::Missile)
        .count();
    assert_eq!(missiles, 0);
}

#[test]
fn missiles_turn_toward_their_target() {
    let mut world = World::new(SimConfig::default());
    // A MOAB directly behind the missile's initial heading, far enough that the
    // missile steers but does not reach it within the test window.
    world.enemies.push(Enemy::new(
        EnemyId(0),
        EnemyType::MOAB,
        Vec2::new(1000.0, 0.0),
    ));
    world.spawn_projectile(ProjectileType::Missile, Vec2::ZERO, std::f32::consts::PI);

    for _ in 0..40 {
        world.simulate();
    }

    let missile = world
        .projectiles
        .iter()
        .find(|projectile| projectile.kind == ProjectileType::Missile)
        .expect("missile still alive");

    assert!(
        (missile.heading - std::f32::consts::PI).abs() > 0.1,
        "missile should have started turning, heading is {}",
        missile.heading
    );
    assert!(
        missile.speed > 25.0 * TICK_DT,
        "missile should be accelerating"
    );
    assert!(missile.angular_velocity != 0.0);
}

#[test]
fn missiles_ignore_bloons_they_cannot_hit() {
    let mut world = World::new(SimConfig::default());
    // A Ceramic sits directly behind the missile: it must not attract it.
    world.enemies.push(Enemy::new(
        EnemyId(0),
        EnemyType::Ceramic,
        Vec2::new(100.0, 0.0),
    ));
    world.spawn_projectile(ProjectileType::Missile, Vec2::ZERO, std::f32::consts::PI);

    for _ in 0..40 {
        world.simulate();
    }

    let missile = world
        .projectiles
        .iter()
        .find(|projectile| projectile.kind == ProjectileType::Missile)
        .expect("missile still alive");
    assert!(
        wrap_pi(missile.heading - std::f32::consts::PI).abs() < 1e-6,
        "missile should fly straight, heading is {}",
        missile.heading
    );
}

#[test]
fn strongest_moab_target_ignores_non_moab_bloons() {
    let red = Enemy::new(EnemyId(0), EnemyType::Red, Vec2::ZERO);
    let ceramic = Enemy::new(EnemyId(1), EnemyType::Ceramic, Vec2::ZERO);
    let enemies = [red, ceramic];

    assert!(strongest_moab_target(&enemies).is_none());

    let moab = Enemy::new(EnemyId(2), EnemyType::MOAB, Vec2::ZERO);
    let enemies = [red, ceramic, moab];
    assert_eq!(strongest_moab_target(&enemies).unwrap().id, EnemyId(2));
}

#[test]
fn strongest_target_prefers_danger_over_distance() {
    let mut far_red = Enemy::new(EnemyId(0), EnemyType::Red, Vec2::ZERO);
    far_red.path_distance = 500.0;
    let moab = Enemy::new(EnemyId(1), EnemyType::MOAB, Vec2::ZERO);

    let enemies = [far_red, moab];

    assert_eq!(strongest_target(&enemies).unwrap().id, EnemyId(1));
}

#[test]
fn strongest_target_breaks_ties_by_distance() {
    let near = Enemy::new(EnemyId(0), EnemyType::MOAB, Vec2::ZERO);
    let mut far = Enemy::new(EnemyId(1), EnemyType::MOAB, Vec2::ZERO);
    far.path_distance = 100.0;

    let enemies = [near, far];

    assert_eq!(strongest_target(&enemies).unwrap().id, EnemyId(1));
}

#[test]
fn strongest_target_breaks_remaining_ties_by_lowest_id() {
    let high = Enemy::new(EnemyId(5), EnemyType::Red, Vec2::ZERO);
    let low = Enemy::new(EnemyId(2), EnemyType::Red, Vec2::ZERO);

    let enemies = [high, low];

    assert_eq!(strongest_target(&enemies).unwrap().id, EnemyId(2));
}

#[test]
fn world_position_is_center_plus_offset() {
    let mut ace = Ace::new(Vec2::new(100.0, 0.0), 0.0);
    ace.position = Vec2::new(10.0, 0.0);
    assert_eq!(ace.world_position(), Vec2::new(110.0, 0.0));
}
