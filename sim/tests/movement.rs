//! Integration tests for path sampling, Bloon movement, spawning, and leaks.

use ace_simulator::{
    Enemy, EnemyId, EnemyType, Path, PathId, Paths, SimConfig, Vec2, World, enemy_stats,
};

fn approx(a: Vec2, b: Vec2) -> bool {
    (a.x - b.x).abs() < 1e-3 && (a.y - b.y).abs() < 1e-3
}

fn approx_angle(a: f32, b: f32) -> bool {
    let delta =
        (a - b + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU) - std::f32::consts::PI;
    delta.abs() < 1e-3
}

fn short_paths() -> Paths {
    Paths::new(vec![Path::new(
        "short",
        &[Vec2::new(0.0, 0.0), Vec2::new(5.0, 0.0)],
    )])
}

#[test]
fn path_sampling_walks_straight_segments() {
    let path = Path::new(
        "test",
        &[
            Vec2::new(0.0, 0.0),
            Vec2::new(10.0, 0.0),
            Vec2::new(10.0, 10.0),
        ],
    );

    assert!((path.length() - 20.0).abs() < 1e-4);

    let (position, heading) = path.sample(0.0);
    assert!(approx(position, Vec2::new(0.0, 0.0)));
    assert!(approx_angle(heading, 0.0));

    let (position, heading) = path.sample(5.0);
    assert!(approx(position, Vec2::new(5.0, 0.0)));
    assert!(approx_angle(heading, 0.0));

    let (position, heading) = path.sample(15.0);
    assert!(approx(position, Vec2::new(10.0, 5.0)));
    assert!(approx_angle(heading, std::f32::consts::FRAC_PI_2));
}

#[test]
fn path_length_is_recomputed_from_waypoints() {
    // The bundled path's first segment is 108.7 units of straight line, but the
    // CSV `distance` column claims 212.0 for it. Recomputation must win.
    let paths = Paths::bundled();
    let path = paths.get(PathId::Indexed(0));

    let (position, _) = path.sample(108.7);
    assert!(approx(position, Vec2::new(-91.3, -69.9)));
}

#[test]
fn enemies_move_along_their_path() {
    let mut world = World::new(SimConfig::default());
    let id = world.spawn_enemy(EnemyType::Red, PathId::Indexed(0));
    assert_eq!(world.enemies[0].id, id);

    let (start, _) = world.paths.get(PathId::Indexed(0)).sample(0.0);
    assert!(approx(world.enemies[0].position, start));
    assert_eq!(world.enemies[0].path_distance, 0.0);

    world.simulate();

    // Red speed is 1.0 unit per tick.
    assert!((world.enemies[0].path_distance - 1.0).abs() < 1e-4);
    let (expected, expected_heading) = world.paths.get(PathId::Indexed(0)).sample(1.0);
    assert!(approx(world.enemies[0].position, expected));
    assert!(approx_angle(world.enemies[0].heading, expected_heading));
}

#[test]
fn speed_depends_on_kind() {
    let mut world = World::new(SimConfig::default());
    world.spawn_enemy(EnemyType::Red, PathId::Indexed(0));
    world.spawn_enemy(EnemyType::MOAB, PathId::Indexed(0));

    world.simulate();

    assert!((world.enemies[0].path_distance - 1.0).abs() < 1e-4);
    assert!((world.enemies[1].path_distance - enemy_stats(EnemyType::MOAB).speed).abs() < 1e-4);
}

#[test]
fn spawns_happen_after_movement_on_their_tick() {
    let mut world = World::new(SimConfig::default());
    world.schedule_spawn(0, EnemyType::Red, PathId::Indexed(0));
    world.schedule_spawn(1, EnemyType::Red, PathId::Indexed(0));

    world.simulate(); // tick 0
    assert_eq!(world.enemies.len(), 1);
    assert_eq!(
        world.enemies[0].path_distance, 0.0,
        "a Bloon is not moved on the tick it spawns"
    );

    world.simulate(); // tick 1
    assert_eq!(world.enemies.len(), 2);
    assert!(
        (world.enemies[0].path_distance - 1.0).abs() < 1e-4,
        "the earlier Bloon moved"
    );
    assert_eq!(
        world.enemies[1].path_distance, 0.0,
        "the new Bloon did not move"
    );
}

#[test]
fn anchored_enemies_never_move() {
    let mut world = World::new(SimConfig::default());
    world
        .enemies
        .push(Enemy::new(EnemyId(0), EnemyType::Red, Vec2::new(0.0, 0.0)));

    world.simulate();

    assert_eq!(world.enemies[0].path_id, PathId::Anchored);
    assert_eq!(world.enemies[0].position, Vec2::new(0.0, 0.0));
}

#[test]
fn reaching_the_end_of_the_path_leaks_the_enemy() {
    let mut world = World::new(SimConfig::default());
    world.paths = short_paths();
    world.schedule_spawn(0, EnemyType::Red, PathId::Indexed(0));

    // Tick 0 spawns at distance 0; each later tick advances 1 unit. The path is
    // 5 units long, so the Bloon leaks on tick 5.
    for _ in 0..6 {
        world.simulate();
    }

    assert!(
        world.enemies.is_empty(),
        "the leaked Bloon should be removed"
    );
}

#[test]
#[should_panic(expected = "Anchored")]
fn resolving_a_path_for_an_anchored_enemy_is_an_error() {
    let paths = Paths::bundled();
    let _ = paths.get(PathId::Anchored);
}

#[test]
#[should_panic(expected = "Anchored")]
fn spawning_an_anchored_enemy_is_an_error() {
    let mut world = World::new(SimConfig::default());
    world.spawn_enemy(EnemyType::Red, PathId::Anchored);
}
