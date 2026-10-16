//! Integration tests for leaking: a Bloon that reaches the end of its Path
//! with Health remaining is reported in `World::leaks`.

use ace_simulator::{EnemyType, PathId, SimConfig, World};

#[test]
fn a_bloon_reaching_the_end_is_reported_as_a_leak() {
    let mut world = World::new(SimConfig::default());
    let length = world.paths.get(PathId::Indexed(0)).length();
    let id = world.spawn_enemy_at(EnemyType::MOAB, PathId::Indexed(0), length - 0.1);

    world.simulate();

    assert_eq!(world.leaks.len(), 1);
    assert_eq!(world.leaks[0].id, id);
    assert_eq!(world.leaks[0].kind, EnemyType::MOAB);
    assert_eq!(world.leaks[0].hp, 200);
    assert!(world.enemies.is_empty());
}

#[test]
fn a_popped_bloon_that_also_reaches_the_end_is_not_a_leak() {
    let mut world = World::new(SimConfig::default());
    let length = world.paths.get(PathId::Indexed(0)).length();
    world.spawn_enemy_at(EnemyType::Red, PathId::Indexed(0), length - 1.0);
    world.enemies[0].hp = 0;

    world.simulate();

    assert!(world.leaks.is_empty());
    assert!(world.enemies.is_empty());
}

#[test]
fn leaks_are_cleared_between_ticks() {
    let mut world = World::new(SimConfig::default());
    let length = world.paths.get(PathId::Indexed(0)).length();
    world.spawn_enemy_at(EnemyType::Red, PathId::Indexed(0), length - 1.0);

    world.simulate();
    assert_eq!(world.leaks.len(), 1);

    world.simulate();
    assert!(world.leaks.is_empty());
}
