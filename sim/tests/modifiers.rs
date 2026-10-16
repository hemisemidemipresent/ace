//! Integration tests for simulation-wide multiplicative modifiers.

use ace_simulator::{EnemyType, PathId, SimConfig, World, enemy_stats};

#[test]
fn defaults_leave_stats_unchanged() {
    let mut world = World::new(SimConfig::default());
    world.spawn_enemy(EnemyType::Ceramic, PathId::Indexed(0));
    world.spawn_enemy(EnemyType::MOAB, PathId::Indexed(0));

    world.simulate();

    assert_eq!(world.enemies[0].hp, enemy_stats(EnemyType::Ceramic).max_hp);
    assert_eq!(world.enemies[1].hp, enemy_stats(EnemyType::MOAB).max_hp);
    assert!((world.enemies[0].path_distance - enemy_stats(EnemyType::Ceramic).speed).abs() < 1e-3);
    assert!((world.enemies[1].path_distance - enemy_stats(EnemyType::MOAB).speed).abs() < 1e-3);
}

#[test]
fn category_selection() {
    let config = SimConfig {
        bloon_speed_multiplier: 1.1,
        moab_speed_multiplier: 1.2,
        ceramic_health_multiplier: 1.3,
        moab_health_multiplier: 1.4,
        ..Default::default()
    };

    // Bloon speed covers everything below MOAB-class, Ceramics included.
    assert_eq!(config.speed_multiplier(EnemyType::Red), 1.1);
    assert_eq!(config.speed_multiplier(EnemyType::Ceramic), 1.1);
    assert_eq!(config.speed_multiplier(EnemyType::MOAB), 1.2);
    assert_eq!(config.speed_multiplier(EnemyType::BADFortified), 1.2);

    // Health: MOAB-class first, Ceramics next, everything else unscaled.
    assert_eq!(config.health_multiplier(EnemyType::Red), 1.0);
    assert_eq!(config.health_multiplier(EnemyType::Ceramic), 1.3);
    assert_eq!(config.health_multiplier(EnemyType::CeramicFortified), 1.3);
    assert_eq!(config.health_multiplier(EnemyType::MOAB), 1.4);
    assert_eq!(config.health_multiplier(EnemyType::MOABFortified), 1.4);
}

#[test]
fn bloon_speed_multiplier_scales_non_moab_bloons() {
    let config = SimConfig {
        bloon_speed_multiplier: 2.0,
        ..Default::default()
    };
    let mut world = World::new(config);
    world.spawn_enemy(EnemyType::Ceramic, PathId::Indexed(0));
    world.spawn_enemy(EnemyType::MOAB, PathId::Indexed(0));

    world.simulate();

    assert!(
        (world.enemies[0].path_distance - 2.0 * enemy_stats(EnemyType::Ceramic).speed).abs() < 1e-3
    );
    assert!((world.enemies[1].path_distance - enemy_stats(EnemyType::MOAB).speed).abs() < 1e-3);
}

#[test]
fn moab_speed_multiplier_scales_moab_class_only() {
    let config = SimConfig {
        moab_speed_multiplier: 0.5,
        ..Default::default()
    };
    let mut world = World::new(config);
    world.spawn_enemy(EnemyType::Red, PathId::Indexed(0));
    world.spawn_enemy(EnemyType::MOAB, PathId::Indexed(0));
    world.spawn_enemy(EnemyType::DDT, PathId::Indexed(0));

    world.simulate();

    assert!((world.enemies[0].path_distance - 1.0).abs() < 1e-3);
    assert!(
        (world.enemies[1].path_distance - 0.5 * enemy_stats(EnemyType::MOAB).speed).abs() < 1e-3
    );
    assert!(
        (world.enemies[2].path_distance - 0.5 * enemy_stats(EnemyType::DDT).speed).abs() < 1e-3
    );
}

#[test]
fn ceramic_health_multiplier_scales_ceramics_only() {
    let config = SimConfig {
        ceramic_health_multiplier: 3.0,
        ..Default::default()
    };
    let mut world = World::new(config);
    world.spawn_enemy(EnemyType::Ceramic, PathId::Indexed(0));
    world.spawn_enemy(EnemyType::CeramicFortified, PathId::Indexed(0));
    world.spawn_enemy(EnemyType::MOAB, PathId::Indexed(0));
    world.spawn_enemy(EnemyType::Red, PathId::Indexed(0));

    assert_eq!(world.enemies[0].hp, 180);
    assert_eq!(world.enemies[1].hp, 360);
    assert_eq!(world.enemies[2].hp, enemy_stats(EnemyType::MOAB).max_hp);
    assert_eq!(world.enemies[3].hp, 1);
}

#[test]
fn moab_health_multiplier_scales_moab_class_only() {
    let config = SimConfig {
        moab_health_multiplier: 2.0,
        ..Default::default()
    };
    let mut world = World::new(config);
    world.spawn_enemy(EnemyType::MOAB, PathId::Indexed(0));
    world.spawn_enemy(EnemyType::DDT, PathId::Indexed(0));
    world.spawn_enemy(EnemyType::BADFortified, PathId::Indexed(0));
    world.spawn_enemy(EnemyType::Ceramic, PathId::Indexed(0));

    assert_eq!(world.enemies[0].hp, 400);
    assert_eq!(world.enemies[1].hp, 800);
    assert_eq!(world.enemies[2].hp, 80_000);
    assert_eq!(world.enemies[3].hp, 60);
}

#[test]
fn health_modifiers_apply_to_children() {
    let config = SimConfig {
        ceramic_health_multiplier: 2.0,
        moab_health_multiplier: 2.0,
        ..Default::default()
    };
    let mut world = World::new(config);
    world.spawn_enemy_at(EnemyType::MOAB, PathId::Indexed(0), 100.0);
    world.enemies[0].hp = 0;

    world.simulate();

    // The MOAB's Ceramic children use the Ceramic multiplier, not the MOAB one.
    assert_eq!(world.enemies.len(), 4);
    assert!(world.enemies.iter().all(|e| e.kind == EnemyType::Ceramic));
    assert!(world.enemies.iter().all(|e| e.hp == 120));
}

#[test]
fn health_is_rounded_to_a_whole_number() {
    let config = SimConfig {
        ceramic_health_multiplier: 1.5,
        ..Default::default()
    };
    assert_eq!(config.max_health(EnemyType::Ceramic), 90);

    let config = SimConfig {
        ceramic_health_multiplier: 0.333,
        ..Default::default()
    };
    assert_eq!(config.max_health(EnemyType::Ceramic), 20);
}
