//! Integration tests for child Bloon spawning.

use ace_simulator::{
    Enemy, EnemyId, EnemyType, PathId, SimConfig, TICK_DT, Vec2, World, enemy_stats,
};

/// Spacing for a brood of four: `26.7 / 4`.
const MOAB_SPACING: f32 = 6.675;

#[test]
fn moab_pops_into_four_ceramics() {
    let mut world = World::new(SimConfig::default());
    world.spawn_enemy_at(EnemyType::MOAB, PathId::Indexed(0), 100.0);
    world.enemies[0].hp = 0;

    world.simulate();

    assert_eq!(world.enemies.len(), 4);
    assert!(world.enemies.iter().all(|e| e.kind == EnemyType::Ceramic));
    assert!(world.enemies.iter().all(|e| e.hp == 60));
    assert!(
        world
            .enemies
            .iter()
            .all(|e| e.path_id == PathId::Indexed(0))
    );
}

#[test]
fn children_trail_the_parent_with_capped_spacing() {
    let mut world = World::new(SimConfig::default());
    world.spawn_enemy_at(EnemyType::MOAB, PathId::Indexed(0), 100.0);
    world.enemies[0].hp = 0;

    world.simulate();

    let distances: Vec<f32> = world.enemies.iter().map(|e| e.path_distance).collect();
    // The MOAB moved one tick's worth before popping.
    let parent_distance = 100.0 + enemy_stats(EnemyType::MOAB).speed;
    assert!(
        (distances[0] - parent_distance).abs() < 1e-3,
        "first child should sit at the parent, got {}",
        distances[0]
    );
    for pair in distances.windows(2) {
        assert!(
            (pair[0] - pair[1] - MOAB_SPACING).abs() < 1e-3,
            "children should be {} apart, got {:?}",
            MOAB_SPACING,
            pair
        );
    }
}

#[test]
fn ceramic_pops_into_nothing_for_now() {
    let mut world = World::new(SimConfig::default());
    world.spawn_enemy_at(EnemyType::Ceramic, PathId::Indexed(0), 100.0);
    world.enemies[0].hp = 0;

    world.simulate();

    assert!(world.enemies.is_empty());
}

#[test]
fn anchored_parents_emit_no_children() {
    let mut world = World::new(SimConfig::default());
    world
        .enemies
        .push(Enemy::new(EnemyId(0), EnemyType::MOAB, Vec2::new(0.0, 0.0)));
    world.enemies[0].hp = 0;

    world.simulate();

    assert!(world.enemies.is_empty());
}

#[test]
fn leaked_parents_emit_no_children() {
    let mut world = World::new(SimConfig::default());
    // A distance past the end clamps to the path length, so it leaks immediately.
    world.spawn_enemy_at(EnemyType::MOAB, PathId::Indexed(0), 1.0e6);

    world.simulate();

    assert!(world.enemies.is_empty());
}

#[test]
fn popping_does_not_cascade_within_a_tick() {
    let mut world = World::new(SimConfig::default());
    world.spawn_enemy_at(EnemyType::MOAB, PathId::Indexed(0), 300.0);
    world.enemies[0].hp = 0;

    world.simulate();

    // The four Ceramics survive the tick at full health: children can only be
    // hit on a later tick.
    assert_eq!(world.enemies.len(), 4);
    assert!(world.enemies.iter().all(|e| e.hp == 60));
}

#[test]
fn bfb_pops_into_four_moabs() {
    let mut world = World::new(SimConfig::default());
    world.spawn_enemy_at(EnemyType::BFB, PathId::Indexed(0), 500.0);
    world.enemies[0].hp = 0;

    world.simulate();

    assert_eq!(world.enemies.len(), 4);
    assert!(world.enemies.iter().all(|e| e.kind == EnemyType::MOAB));
    assert!(world.enemies.iter().all(|e| e.hp == 200));
}

#[test]
fn zomg_pops_into_four_bfbs() {
    let mut world = World::new(SimConfig::default());
    world.spawn_enemy_at(EnemyType::ZOMG, PathId::Indexed(0), 500.0);
    world.enemies[0].hp = 0;

    world.simulate();

    assert_eq!(world.enemies.len(), 4);
    assert!(world.enemies.iter().all(|e| e.kind == EnemyType::BFB));
    assert!(world.enemies.iter().all(|e| e.hp == 700));
}

#[test]
fn ddt_pops_into_four_ceramics() {
    let mut world = World::new(SimConfig::default());
    world.spawn_enemy_at(EnemyType::DDT, PathId::Indexed(0), 500.0);
    world.enemies[0].hp = 0;

    world.simulate();

    assert_eq!(world.enemies.len(), 4);
    assert!(world.enemies.iter().all(|e| e.kind == EnemyType::Ceramic));
    assert!(world.enemies.iter().all(|e| e.hp == 60));
}

#[test]
fn bad_pops_into_two_zomgs_then_three_ddts() {
    let mut world = World::new(SimConfig::default());
    world.spawn_enemy_at(EnemyType::BAD, PathId::Indexed(0), 500.0);
    world.enemies[0].hp = 0;

    world.simulate();

    let kinds: Vec<EnemyType> = world.enemies.iter().map(|e| e.kind).collect();
    assert_eq!(
        kinds,
        vec![
            EnemyType::ZOMG,
            EnemyType::ZOMG,
            EnemyType::DDT,
            EnemyType::DDT,
            EnemyType::DDT,
        ]
    );
}

#[test]
fn fortified_moab_pops_into_four_fortified_ceramics() {
    let mut world = World::new(SimConfig::default());
    world.spawn_enemy_at(EnemyType::MOABFortified, PathId::Indexed(0), 100.0);
    world.enemies[0].hp = 0;

    world.simulate();

    assert_eq!(world.enemies.len(), 4);
    assert!(
        world
            .enemies
            .iter()
            .all(|e| e.kind == EnemyType::CeramicFortified)
    );
    assert!(world.enemies.iter().all(|e| e.hp == 120));
}

#[test]
fn fortified_bad_pops_into_fortified_zomgs_and_ddts() {
    let mut world = World::new(SimConfig::default());
    world.spawn_enemy_at(EnemyType::BADFortified, PathId::Indexed(0), 500.0);
    world.enemies[0].hp = 0;

    world.simulate();

    let kinds: Vec<EnemyType> = world.enemies.iter().map(|e| e.kind).collect();
    assert_eq!(
        kinds,
        vec![
            EnemyType::ZOMGFortified,
            EnemyType::ZOMGFortified,
            EnemyType::DDTFortified,
            EnemyType::DDTFortified,
            EnemyType::DDTFortified,
        ]
    );
}

#[test]
fn high_tier_stats_and_danger() {
    assert_eq!(enemy_stats(EnemyType::BFB).max_hp, 700);
    assert!((enemy_stats(EnemyType::BFB).speed - 6.25 * TICK_DT).abs() < 1e-6);
    assert_eq!(enemy_stats(EnemyType::ZOMG).max_hp, 4000);
    assert!((enemy_stats(EnemyType::ZOMG).speed - 4.5 * TICK_DT).abs() < 1e-6);
    assert_eq!(enemy_stats(EnemyType::DDT).max_hp, 400);
    assert!((enemy_stats(EnemyType::DDT).speed - 66.0 * TICK_DT).abs() < 1e-6);
    assert_eq!(enemy_stats(EnemyType::BAD).max_hp, 20_000);
    assert!((enemy_stats(EnemyType::BAD).speed - 4.5 * TICK_DT).abs() < 1e-6);

    // Strongest-first ordering.
    assert!(enemy_stats(EnemyType::BAD).danger > enemy_stats(EnemyType::DDT).danger);
    assert!(enemy_stats(EnemyType::DDT).danger > enemy_stats(EnemyType::ZOMG).danger);
    assert!(enemy_stats(EnemyType::ZOMG).danger > enemy_stats(EnemyType::BFB).danger);
    assert!(enemy_stats(EnemyType::BFB).danger > enemy_stats(EnemyType::MOAB).danger);

    // Fortified variants double the base health and inherit everything else.
    assert_eq!(enemy_stats(EnemyType::CeramicFortified).max_hp, 120);
    assert_eq!(enemy_stats(EnemyType::MOABFortified).max_hp, 400);
    assert_eq!(enemy_stats(EnemyType::BFBFortified).max_hp, 1400);
    assert_eq!(enemy_stats(EnemyType::ZOMGFortified).max_hp, 8000);
    assert_eq!(enemy_stats(EnemyType::DDTFortified).max_hp, 800);
    assert_eq!(enemy_stats(EnemyType::BADFortified).max_hp, 40_000);
    for (base, fortified) in [
        (EnemyType::Ceramic, EnemyType::CeramicFortified),
        (EnemyType::MOAB, EnemyType::MOABFortified),
        (EnemyType::BFB, EnemyType::BFBFortified),
        (EnemyType::ZOMG, EnemyType::ZOMGFortified),
        (EnemyType::DDT, EnemyType::DDTFortified),
        (EnemyType::BAD, EnemyType::BADFortified),
    ] {
        assert_eq!(enemy_stats(fortified).danger, enemy_stats(base).danger);
        assert_eq!(enemy_stats(fortified).speed, enemy_stats(base).speed);
    }

    // DDT and BAD count as MOAB-class for missile targeting; so do their
    // fortified variants. A fortified Ceramic is not MOAB-class.
    assert!(EnemyType::DDT.is_moab_plus());
    assert!(EnemyType::BAD.is_moab_plus());
    assert!(EnemyType::MOABFortified.is_moab_plus());
    assert!(EnemyType::BADFortified.is_moab_plus());
    assert!(!EnemyType::Ceramic.is_moab_plus());
    assert!(!EnemyType::CeramicFortified.is_moab_plus());
}

#[test]
fn moab_class_hitboxes_have_the_expected_circles() {
    let moab = enemy_stats(EnemyType::MOAB).hitbox;
    assert_eq!(moab.circles().len(), 2);
    assert!((moab.circles()[0].radius - 14.7).abs() < 1e-6);
    assert!((moab.circles()[0].offset.x - 6.3).abs() < 1e-6);
    assert!((moab.circles()[1].radius - 13.18).abs() < 1e-6);
    assert!((moab.circles()[1].offset.x + 11.0).abs() < 1e-6);

    let bfb = enemy_stats(EnemyType::BFB).hitbox;
    assert_eq!(bfb.circles().len(), 2);
    assert!((bfb.circles()[0].radius - 18.01).abs() < 1e-6);
    assert!((bfb.circles()[1].radius - 20.04).abs() < 1e-6);

    let zomg = enemy_stats(EnemyType::ZOMG).hitbox;
    assert_eq!(zomg.circles().len(), 3);
    assert!((zomg.circles()[0].radius - 15.23).abs() < 1e-6);
    assert!((zomg.circles()[1].radius - 19.64).abs() < 1e-6);
    assert!((zomg.circles()[2].radius - 24.02).abs() < 1e-6);
    assert!((zomg.circles()[2].offset.x + 17.2).abs() < 1e-6);

    let ddt = enemy_stats(EnemyType::DDT).hitbox;
    assert_eq!(ddt.circles().len(), 2);
    assert!((ddt.circles()[0].radius - 19.85).abs() < 1e-6);
    assert!((ddt.circles()[0].offset.x - 10.4).abs() < 1e-6);
    assert!((ddt.circles()[1].radius - 17.44).abs() < 1e-6);
    assert!((ddt.circles()[1].offset.x + 9.6).abs() < 1e-6);

    let bad = enemy_stats(EnemyType::BAD).hitbox;
    assert_eq!(bad.circles().len(), 2);
    assert!((bad.circles()[0].radius - 23.6).abs() < 1e-6);
    assert!((bad.circles()[0].offset.x - 20.2).abs() < 1e-6);
    assert!((bad.circles()[1].radius - 28.45).abs() < 1e-6);
    assert!((bad.circles()[1].offset.x + 14.2).abs() < 1e-6);
}
