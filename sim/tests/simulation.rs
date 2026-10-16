//! Integration tests for the collision checker, damage, and death resolution.

use ace_simulator::{
    Collision, Enemy, EnemyId, EnemyType, Hit, Projectile, ProjectileId, ProjectileType, SimConfig,
    Vec2, World, check_collisions, check_collisions_bruteforce,
};

fn enemy(id: u32, kind: EnemyType, x: f32, y: f32) -> Enemy {
    Enemy::new(EnemyId(id), kind, Vec2::new(x, y))
}

fn projectile(id: u32, kind: ProjectileType, x: f32, y: f32) -> Projectile {
    // Freeze movement so these tests exercise collision and damage at fixed
    // positions.
    let mut projectile = Projectile::new(ProjectileId(id), kind, Vec2::new(x, y), 0.0);
    projectile.speed = 0.0;
    projectile
}

fn world() -> World {
    World::new(SimConfig::default())
}

// --- the pure checker -------------------------------------------------------

#[test]
fn single_circle_overlap_is_reported() {
    let enemies = [enemy(0, EnemyType::Red, 0.0, 0.0)];
    let projectiles = [projectile(0, ProjectileType::Dart, 5.0, 0.0)];

    let hits = check_collisions(&enemies, &projectiles, &SimConfig::default());

    assert_eq!(
        hits,
        vec![Hit {
            enemy: 0,
            projectile: 0
        }]
    );
}

#[test]
fn distant_objects_do_not_collide() {
    let enemies = [enemy(0, EnemyType::Red, -100.0, 0.0)];
    let projectiles = [projectile(0, ProjectileType::Dart, 100.0, 0.0)];

    assert!(check_collisions(&enemies, &projectiles, &SimConfig::default()).is_empty());
}

#[test]
fn compound_hitbox_reports_a_pair_once() {
    // A dart sitting in the overlap of the MOAB's first two circles.
    let enemies = [enemy(0, EnemyType::MOAB, 0.0, 0.0)];
    let projectiles = [projectile(0, ProjectileType::Dart, -8.0, 0.0)];

    let hits = check_collisions(&enemies, &projectiles, &SimConfig::default());

    assert_eq!(hits.len(), 1);
}

#[test]
fn compound_hitbox_rotates_with_heading() {
    let projectiles = [projectile(0, ProjectileType::Dart, 16.0, 14.0)];
    let config = SimConfig::default();

    let mut moab = enemy(0, EnemyType::MOAB, 0.0, 0.0);
    // Heading 0: the circle at local (16, 0) is 14 units away -> overlap.
    assert_eq!(check_collisions(&[moab], &projectiles, &config).len(), 1);

    // Rotated 90 degrees the circles lie on the y axis; the nearest is (0, 16),
    // which is ~16.1 units away -> no overlap.
    moab.heading = std::f32::consts::FRAC_PI_2;
    assert!(check_collisions(&[moab], &projectiles, &config).is_empty());
}

#[test]
fn out_of_bounds_enemies_are_skipped() {
    let enemies = [enemy(0, EnemyType::Red, 500.0, 0.0)];
    let projectiles = [projectile(0, ProjectileType::Dart, 500.0, 0.0)];

    assert!(check_collisions(&enemies, &projectiles, &SimConfig::default()).is_empty());
}

#[test]
fn out_of_bounds_projectiles_are_skipped() {
    let enemies = [enemy(0, EnemyType::Red, 0.0, 0.0)];
    let projectiles = [projectile(0, ProjectileType::Dart, 500.0, 0.0)];

    assert!(check_collisions(&enemies, &projectiles, &SimConfig::default()).is_empty());
}

#[test]
fn hits_are_ordered_by_projectile_then_distance() {
    // Enemy 0 is farther from projectile 0 than enemy 1, and closer to
    // projectile 1, so index order alone does not explain the result.
    let enemies = [
        enemy(0, EnemyType::Red, 8.0, 0.0),
        enemy(1, EnemyType::Red, 5.0, 0.0),
    ];
    let projectiles = [
        projectile(0, ProjectileType::Dart, 0.0, 0.0),
        projectile(1, ProjectileType::Dart, 10.0, 0.0),
    ];

    let hits = check_collisions(&enemies, &projectiles, &SimConfig::default());

    assert_eq!(
        hits,
        vec![
            Hit {
                enemy: 1,
                projectile: 0
            },
            Hit {
                enemy: 0,
                projectile: 0
            },
            Hit {
                enemy: 0,
                projectile: 1
            },
            Hit {
                enemy: 1,
                projectile: 1
            },
        ]
    );
}

/// A tiny deterministic generator so the equivalence test is reproducible.
fn sample_world() -> (Vec<Enemy>, Vec<Projectile>) {
    let mut seed: u64 = 0x1234_5678_9abc_def0;
    let mut next = move || {
        seed = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((seed >> 33) as f64 / (1u64 << 31) as f64) as f32
    };

    let mut enemies = Vec::new();
    for i in 0..60 {
        let kind = if i % 5 == 0 {
            EnemyType::MOAB
        } else {
            EnemyType::Red
        };
        let mut e = enemy(i, kind, -260.0 + next() * 520.0, -200.0 + next() * 400.0);
        e.heading = next() * std::f32::consts::TAU;
        enemies.push(e);
    }

    let mut projectiles = Vec::new();
    for i in 0..60 {
        let kind = if i % 3 == 0 {
            ProjectileType::Missile
        } else {
            ProjectileType::Dart
        };
        projectiles.push(projectile(
            i,
            kind,
            -260.0 + next() * 520.0,
            -200.0 + next() * 400.0,
        ));
    }

    (enemies, projectiles)
}

#[test]
fn grid_agrees_with_bruteforce() {
    let (enemies, projectiles) = sample_world();
    let config = SimConfig::default();

    let gridded = check_collisions(&enemies, &projectiles, &config);
    let reference = check_collisions_bruteforce(&enemies, &projectiles, &config);

    assert_eq!(gridded, reference);
    assert!(!gridded.is_empty(), "sample world should produce hits");
}

#[test]
fn cell_size_does_not_change_results() {
    let (enemies, projectiles) = sample_world();
    let reference = check_collisions(
        &enemies,
        &projectiles,
        &SimConfig {
            cell_size: 32.0,
            ..Default::default()
        },
    );

    for cell_size in [2.0, 8.0, 32.0, 64.0, 200.0, 1000.0] {
        let got = check_collisions(
            &enemies,
            &projectiles,
            &SimConfig {
                cell_size,
                ..Default::default()
            },
        );
        assert_eq!(got, reference, "cell size {cell_size} changed the result");
    }
}

// --- damage, pierce, and death ---------------------------------------------

#[test]
fn simulate_reports_applied_hits() {
    let mut world = world();
    world.enemies.push(enemy(0, EnemyType::Red, 0.0, 0.0));
    world
        .projectiles
        .push(projectile(0, ProjectileType::Dart, 0.0, 0.0));

    let hits = world.simulate();

    assert_eq!(
        hits,
        vec![Collision {
            enemy: EnemyId(0),
            projectile: ProjectileId(0)
        }]
    );
}

#[test]
fn missiles_do_not_hit_non_moab_bloons() {
    let mut world = world();
    world.enemies.push(enemy(0, EnemyType::Red, 0.0, 0.0));
    world
        .projectiles
        .push(projectile(0, ProjectileType::Missile, 0.0, 0.0));

    let hits = world.simulate();

    // The overlap is ignored: no event, no damage, no pierce spent, no blast.
    assert!(hits.is_empty());
    assert_eq!(world.enemies[0].hp, 1);
    assert_eq!(world.projectiles[0].pierce_remaining, 1);
    assert!(
        world
            .projectiles
            .iter()
            .all(|p| p.kind != ProjectileType::Explosion)
    );
}

#[test]
fn missiles_detonate_into_an_explosion() {
    let mut world = world();
    world.enemies.push(enemy(0, EnemyType::MOAB, 0.0, 0.0));
    // A real spawn, so the explosion gets a distinct id (1).
    world.spawn_projectile(ProjectileType::Missile, Vec2::ZERO, 0.0);

    let hits = world.simulate();

    // The contact is reported even though the missile dealt no damage.
    assert_eq!(
        hits,
        vec![Collision {
            enemy: EnemyId(0),
            projectile: ProjectileId(0)
        }]
    );
    assert_eq!(world.enemies[0].hp, 200);

    // A one-tick explosion now sits at (approximately) the impact point.
    let explosion = world
        .projectiles
        .iter()
        .find(|p| p.kind == ProjectileType::Explosion)
        .expect("explosion spawned");
    assert_eq!(explosion.id, ProjectileId(1));
    assert!(explosion.position.x.abs() < 1.0 && explosion.position.y.abs() < 1.0);
    assert_eq!(explosion.pierce_remaining, 5);
}

#[test]
fn explosions_damage_every_bloon_in_radius() {
    let mut world = world();
    world.enemies.push(enemy(0, EnemyType::MOAB, 0.0, 0.0));
    world.enemies.push(enemy(1, EnemyType::Red, 25.0, 0.0));
    world.enemies.push(enemy(2, EnemyType::Red, 100.0, 0.0));
    // A real spawn, so the explosion gets a distinct id (1).
    world.spawn_projectile(ProjectileType::Missile, Vec2::ZERO, 0.0);

    // Tick 0: the missile detonates. Tick 1: the blast lands.
    world.simulate();
    let hits = world.simulate();

    assert_eq!(world.enemies.len(), 2);
    let moab = world.enemies.iter().find(|e| e.id == EnemyId(0)).unwrap();
    let far = world.enemies.iter().find(|e| e.id == EnemyId(2)).unwrap();
    assert_eq!(moab.hp, 50, "the triggering MOAB is in the blast");
    assert_eq!(far.hp, 1, "a Bloon outside 30 units is untouched");
    // Both in-radius enemies are reported as blast hits, by the explosion's id.
    assert_eq!(hits.len(), 2);
    assert!(hits.iter().all(|h| h.projectile == ProjectileId(1)));
}

#[test]
fn damage_reduces_health() {
    let mut world = world();
    world.enemies.push(enemy(0, EnemyType::MOAB, 0.0, 0.0));
    world
        .projectiles
        .push(projectile(0, ProjectileType::Dart, 0.0, 0.0));

    world.simulate();

    assert_eq!(world.enemies[0].hp, 197);
}

#[test]
fn popped_enemies_are_removed() {
    let mut world = world();
    world.enemies.push(enemy(0, EnemyType::Red, 0.0, 0.0));
    world
        .projectiles
        .push(projectile(0, ProjectileType::Dart, 0.0, 0.0));

    world.simulate();

    assert!(world.enemies.is_empty());
}

#[test]
fn a_projectile_damages_an_enemy_only_once() {
    let mut world = world();
    world.enemies.push(enemy(0, EnemyType::MOAB, 0.0, 0.0));
    world
        .projectiles
        .push(projectile(0, ProjectileType::Dart, 0.0, 0.0));

    world.simulate();
    assert_eq!(world.enemies[0].hp, 197);
    assert_eq!(world.projectiles[0].pierce_remaining, 7);

    // Still overlapping on the next tick, but already hit: nothing happens.
    let hits = world.simulate();
    assert!(hits.is_empty());
    assert_eq!(world.enemies[0].hp, 197);
    assert_eq!(world.projectiles[0].pierce_remaining, 7);
}

#[test]
fn pierce_limits_distinct_enemies_hit() {
    let mut world = world();
    // Nine Reds at increasing distance; a Dart has eight pierce.
    for index in 0..9 {
        world
            .enemies
            .push(enemy(index, EnemyType::Red, 1.0 + index as f32, 0.0));
    }
    world
        .projectiles
        .push(projectile(0, ProjectileType::Dart, 0.0, 0.0));

    let hits = world.simulate();

    assert_eq!(hits.len(), 8);
    assert_eq!(world.enemies.len(), 1);
    assert_eq!(world.enemies[0].id, EnemyId(8));
    // Pierce spent, so the projectile was swept.
    assert!(world.projectiles.is_empty());
}

#[test]
fn dead_enemies_do_not_consume_pierce() {
    let mut world = world();
    // A MOAB softened to three health, so one Dart pops it.
    let mut moab = enemy(0, EnemyType::MOAB, 0.0, 0.0);
    moab.hp = 3;
    world.enemies.push(moab);
    // Two Darts, each damage 3. The first pops the MOAB; the second finds it dead
    // and is skipped without spending pierce.
    world
        .projectiles
        .push(projectile(0, ProjectileType::Dart, 0.0, 0.0));
    world
        .projectiles
        .push(projectile(1, ProjectileType::Dart, 0.0, 0.0));

    let hits = world.simulate();

    assert_eq!(
        hits,
        vec![Collision {
            enemy: EnemyId(0),
            projectile: ProjectileId(0)
        }]
    );
    assert!(world.enemies.is_empty());
    assert_eq!(world.projectiles.len(), 2);
    assert_eq!(world.projectiles[1].pierce_remaining, 8);
}
