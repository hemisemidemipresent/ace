//! Enemy and projectile kinds, their base stats, and their instances.

use crate::hitbox::{Circle, EnemyHitbox, Vec2};
use crate::paths::{PathId, Paths};
use crate::{TICK_DT, TICKS_PER_SECOND};

/// The kind of a Bloon. (A Bloon is the umbrella class; see `GLOSSARY.md`.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnemyType {
    /// Basic single-hit Bloon.
    Red,
    /// Fast, high-health Bloon. Pops into nothing yet.
    Ceramic,
    /// Large, high-health Bloon; pops into four Ceramics.
    MOAB,
    /// Very large, very high-health Bloon; pops into four MOABs.
    BFB,
    /// The largest, highest-health Bloon; pops into four BFBs.
    ZOMG,
    /// A fast, high-health Bloon; pops into four Ceramics.
    DDT,
    /// The strongest Bloon; pops into two ZOMGs and three DDTs.
    BAD,
    /// A [`EnemyType::Ceramic`] with double health.
    CeramicFortified,
    /// A [`EnemyType::MOAB`] with double health; leaves fortified Ceramics.
    MOABFortified,
    /// A [`EnemyType::BFB`] with double health; leaves fortified MOABs.
    BFBFortified,
    /// A [`EnemyType::ZOMG`] with double health; leaves fortified BFBs.
    ZOMGFortified,
    /// A [`EnemyType::DDT`] with double health; leaves fortified Ceramics.
    DDTFortified,
    /// A [`EnemyType::BAD`] with double health; leaves fortified ZOMGs and DDTs.
    BADFortified,
}

impl EnemyType {
    /// Whether this kind is a Ceramic, fortified or not.
    pub fn is_ceramic(self) -> bool {
        matches!(self, EnemyType::Ceramic | EnemyType::CeramicFortified)
    }

    /// Whether this kind is a MOAB or stronger (MOAB, BFB, ZOMG, DDT, BAD and
    /// their fortified variants).
    pub fn is_moab_plus(self) -> bool {
        matches!(
            self,
            EnemyType::MOAB
                | EnemyType::BFB
                | EnemyType::ZOMG
                | EnemyType::DDT
                | EnemyType::BAD
                | EnemyType::MOABFortified
                | EnemyType::BFBFortified
                | EnemyType::ZOMGFortified
                | EnemyType::DDTFortified
                | EnemyType::BADFortified
        )
    }
}

/// The kind of a projectile fired by a tower.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProjectileType {
    /// A dart fired radially by the Monkey Ace.
    Dart,
    /// A seeking missile fired by the Monkey Ace.
    Missile,
    /// The one-tick blast left behind when a missile detonates.
    Explosion,
}

impl ProjectileType {
    /// Whether this projectile may damage `enemy`.
    ///
    /// Missiles only damage MOAB-class Bloons; darts and explosions damage any
    /// kind.
    pub fn can_hit(self, enemy: EnemyType) -> bool {
        match self {
            ProjectileType::Dart | ProjectileType::Explosion => true,
            ProjectileType::Missile => enemy.is_moab_plus(),
        }
    }
}

/// Identity of an [`Enemy`] within a [`crate::World`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EnemyId(pub u32);

/// Identity of a [`Projectile`] within a [`crate::World`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProjectileId(pub u32);

/// Base stats shared by every enemy of a given kind.
///
/// Speed is world units per tick. `danger` ranks a kind for targeting: higher is
/// stronger, all else equal. Hitboxes are stored in local space and rotate with
/// the enemy's heading. `children` are the Bloons spawned when this kind pops,
/// in spawn order; it is empty when the kind pops into nothing.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EnemyStats {
    pub max_hp: u32,
    pub speed: f32,
    pub danger: u32,
    pub hitbox: EnemyHitbox,
    pub children: &'static [EnemyType],
}

/// The steering parameters of a seeking projectile, all per tick.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SeekingStats {
    pub acceleration: f32,
    pub max_speed: f32,
    pub initial_angular_velocity: f32,
    pub angular_acceleration: f32,
    pub max_angular_speed: f32,
}

/// Base stats shared by every projectile of a given kind.
///
/// `speed` is the straight-line speed, and also the launch speed of a seeking
/// projectile. `pierce` is the number of distinct Bloons the projectile may
/// damage before it is spent. `lifespan_ticks` is how long it lives. `seeking`
/// is present for missiles.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProjectileStats {
    pub speed: f32,
    pub radius: f32,
    pub pierce: u32,
    pub damage: u32,
    pub lifespan_ticks: u32,
    pub seeking: Option<SeekingStats>,
}

/// A Bloon queued to enter the simulation on a given tick.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Spawn {
    pub tick: u64,
    pub kind: EnemyType,
    pub path_id: PathId,
}

/// The shared hitbox of the standard single-circle Bloons.
const BLOON_CIRCLE: Circle = Circle::new(Vec2::ZERO, 8.0);

static MOAB_CIRCLES: [Circle; 2] = [
    Circle::new(Vec2::new(6.3, 0.0), 14.7),
    Circle::new(Vec2::new(-11.0, 0.0), 13.18),
];

static BFB_CIRCLES: [Circle; 2] = [
    Circle::new(Vec2::new(12.3, 0.0), 18.01),
    Circle::new(Vec2::new(-11.6, 0.0), 20.04),
];

static ZOMG_CIRCLES: [Circle; 3] = [
    Circle::new(Vec2::new(17.2, 0.0), 15.23),
    Circle::new(Vec2::new(5.1, 0.0), 19.64),
    Circle::new(Vec2::new(-17.2, 0.0), 24.02),
];

static DDT_CIRCLES: [Circle; 2] = [
    Circle::new(Vec2::new(10.4, 0.0), 19.85),
    Circle::new(Vec2::new(-9.6, 0.0), 17.44),
];

static BAD_CIRCLES: [Circle; 2] = [
    Circle::new(Vec2::new(20.2, 0.0), 23.6),
    Circle::new(Vec2::new(-14.2, 0.0), 28.45),
];

/// A MOAB leaves four Ceramics.
static MOAB_CHILDREN: [EnemyType; 4] = [EnemyType::Ceramic; 4];

/// A BFB leaves four MOABs.
static BFB_CHILDREN: [EnemyType; 4] = [EnemyType::MOAB; 4];

/// A ZOMG leaves four BFBs.
static ZOMG_CHILDREN: [EnemyType; 4] = [EnemyType::BFB; 4];

/// A DDT leaves four Ceramics.
static DDT_CHILDREN: [EnemyType; 4] = [EnemyType::Ceramic; 4];

/// A BAD leaves two ZOMGs followed by three DDTs.
static BAD_CHILDREN: [EnemyType; 5] = [
    EnemyType::ZOMG,
    EnemyType::ZOMG,
    EnemyType::DDT,
    EnemyType::DDT,
    EnemyType::DDT,
];

/// A fortified MOAB leaves four fortified Ceramics.
static MOAB_FORTIFIED_CHILDREN: [EnemyType; 4] = [EnemyType::CeramicFortified; 4];

/// A fortified BFB leaves four fortified MOABs.
static BFB_FORTIFIED_CHILDREN: [EnemyType; 4] = [EnemyType::MOABFortified; 4];

/// A fortified ZOMG leaves four fortified BFBs.
static ZOMG_FORTIFIED_CHILDREN: [EnemyType; 4] = [EnemyType::BFBFortified; 4];

/// A fortified DDT leaves four fortified Ceramics.
static DDT_FORTIFIED_CHILDREN: [EnemyType; 4] = [EnemyType::CeramicFortified; 4];

/// A fortified BAD leaves two fortified ZOMGs followed by three fortified DDTs.
static BAD_FORTIFIED_CHILDREN: [EnemyType; 5] = [
    EnemyType::ZOMGFortified,
    EnemyType::ZOMGFortified,
    EnemyType::DDTFortified,
    EnemyType::DDTFortified,
    EnemyType::DDTFortified,
];

/// Base stats for an enemy kind.
pub fn enemy_stats(kind: EnemyType) -> EnemyStats {
    match kind {
        EnemyType::Red => EnemyStats {
            max_hp: 1,
            speed: 1.0,
            danger: 1,
            hitbox: EnemyHitbox::Circle(BLOON_CIRCLE),
            children: &[],
        },
        EnemyType::Ceramic => EnemyStats {
            max_hp: 60,
            speed: 2.5,
            danger: 2,
            hitbox: EnemyHitbox::Circle(BLOON_CIRCLE),
            children: &[],
        },
        EnemyType::MOAB => EnemyStats {
            max_hp: 200,
            speed: 25.0 * TICK_DT,
            danger: 3,
            hitbox: EnemyHitbox::Compound(&MOAB_CIRCLES),
            children: &MOAB_CHILDREN,
        },
        EnemyType::BFB => EnemyStats {
            max_hp: 700,
            speed: 6.25 * TICK_DT,
            danger: 4,
            hitbox: EnemyHitbox::Compound(&BFB_CIRCLES),
            children: &BFB_CHILDREN,
        },
        EnemyType::ZOMG => EnemyStats {
            max_hp: 4000,
            speed: 4.5 * TICK_DT,
            danger: 5,
            hitbox: EnemyHitbox::Compound(&ZOMG_CIRCLES),
            children: &ZOMG_CHILDREN,
        },
        EnemyType::DDT => EnemyStats {
            max_hp: 400,
            speed: 66.0 * TICK_DT,
            danger: 6,
            hitbox: EnemyHitbox::Compound(&DDT_CIRCLES),
            children: &DDT_CHILDREN,
        },
        EnemyType::BAD => EnemyStats {
            max_hp: 20_000,
            speed: 4.5 * TICK_DT,
            danger: 7,
            hitbox: EnemyHitbox::Compound(&BAD_CIRCLES),
            children: &BAD_CHILDREN,
        },
        // Fortified variants inherit speed, danger, and hitbox from their base
        // and change only health and children.
        EnemyType::CeramicFortified => EnemyStats {
            max_hp: 120,
            ..enemy_stats(EnemyType::Ceramic)
        },
        EnemyType::MOABFortified => EnemyStats {
            max_hp: 400,
            children: &MOAB_FORTIFIED_CHILDREN,
            ..enemy_stats(EnemyType::MOAB)
        },
        EnemyType::BFBFortified => EnemyStats {
            max_hp: 1400,
            children: &BFB_FORTIFIED_CHILDREN,
            ..enemy_stats(EnemyType::BFB)
        },
        EnemyType::ZOMGFortified => EnemyStats {
            max_hp: 8000,
            children: &ZOMG_FORTIFIED_CHILDREN,
            ..enemy_stats(EnemyType::ZOMG)
        },
        EnemyType::DDTFortified => EnemyStats {
            max_hp: 800,
            children: &DDT_FORTIFIED_CHILDREN,
            ..enemy_stats(EnemyType::DDT)
        },
        EnemyType::BADFortified => EnemyStats {
            max_hp: 40_000,
            children: &BAD_FORTIFIED_CHILDREN,
            ..enemy_stats(EnemyType::BAD)
        },
    }
}

/// Base stats for a projectile kind.
pub fn projectile_stats(kind: ProjectileType) -> ProjectileStats {
    match kind {
        ProjectileType::Dart => ProjectileStats {
            speed: 6.0,
            radius: 3.0,
            pierce: 8,
            damage: 3,
            lifespan_ticks: 84,
            seeking: None,
        },
        ProjectileType::Missile => ProjectileStats {
            speed: 25.0 * TICK_DT,
            radius: 1.5,
            pierce: 1,
            // Missiles deal no damage; they detonate into an explosion.
            damage: 0,
            lifespan_ticks: (16.8 * TICKS_PER_SECOND) as u32,
            seeking: Some(SeekingStats {
                acceleration: 160.0 * TICK_DT * TICK_DT,
                max_speed: 400.0 * TICK_DT,
                initial_angular_velocity: 25.0_f32.to_radians() * TICK_DT,
                angular_acceleration: 150.0_f32.to_radians() * TICK_DT * TICK_DT,
                max_angular_speed: 500.0_f32.to_radians() * TICK_DT,
            }),
        },
        ProjectileType::Explosion => ProjectileStats {
            // A stationary one-tick blast. Its radius is the blast radius.
            speed: 0.0,
            radius: 30.0,
            pierce: 5,
            damage: 150,
            lifespan_ticks: 1,
            seeking: None,
        },
    }
}

/// A Bloon instance in the simulation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Enemy {
    pub id: EnemyId,
    pub kind: EnemyType,
    /// The path this enemy travels, or [`PathId::Anchored`] to stay put.
    pub path_id: PathId,
    /// Distance travelled along `path_id`. Zero for anchored enemies.
    pub path_distance: f32,
    pub position: Vec2,
    /// Direction of travel in radians; `0` is `+x`, counter-clockwise positive.
    pub heading: f32,
    /// Current health, seeded from the kind's `max_hp`.
    pub hp: u32,
}

impl Enemy {
    /// Create an anchored enemy at `position` with full health and a zero
    /// heading. It never moves.
    pub fn new(id: EnemyId, kind: EnemyType, position: Vec2) -> Self {
        Self {
            id,
            kind,
            path_id: PathId::Anchored,
            path_distance: 0.0,
            position,
            heading: 0.0,
            hp: enemy_stats(kind).max_hp,
        }
    }

    /// Create an enemy `path_distance` along `path_id`, clamped to the path.
    ///
    /// Panics if `path_id` is [`PathId::Anchored`]; use [`Enemy::new`] for an
    /// anchored enemy.
    pub fn spawn(
        id: EnemyId,
        kind: EnemyType,
        path_id: PathId,
        path_distance: f32,
        paths: &Paths,
    ) -> Self {
        let path = paths.get(path_id);
        let path_distance = path_distance.clamp(0.0, path.length());
        let (position, heading) = path.sample(path_distance);
        Self {
            id,
            kind,
            path_id,
            path_distance,
            position,
            heading,
            hp: enemy_stats(kind).max_hp,
        }
    }
}

/// A projectile instance in the simulation.
///
/// `speed`, `angular_velocity`, and `ticks_remaining` are runtime state. A
/// projectile damages each distinct enemy at most once; `hit_enemies` records
/// which ones.
#[derive(Debug, Clone, PartialEq)]
pub struct Projectile {
    pub id: ProjectileId,
    pub kind: ProjectileType,
    pub position: Vec2,
    /// Direction of travel in radians; `0` is `+x`, counter-clockwise positive.
    pub heading: f32,
    /// Current speed in world units per tick.
    pub speed: f32,
    /// Current turn rate in radians per tick; zero for straight projectiles.
    pub angular_velocity: f32,
    pub pierce_remaining: u32,
    pub hit_enemies: Vec<EnemyId>,
    /// Ticks left before the projectile expires.
    pub ticks_remaining: u32,
}

impl Projectile {
    /// Create a projectile at `position` travelling at `heading`, with full
    /// pierce, no hits, and a full lifespan.
    pub fn new(id: ProjectileId, kind: ProjectileType, position: Vec2, heading: f32) -> Self {
        let stats = projectile_stats(kind);
        Self {
            id,
            kind,
            position,
            heading,
            speed: stats.speed,
            angular_velocity: 0.0,
            pierce_remaining: stats.pierce,
            hit_enemies: Vec::new(),
            ticks_remaining: stats.lifespan_ticks,
        }
    }

    /// Whether this projectile has already damaged `enemy`.
    pub fn has_hit(&self, enemy: EnemyId) -> bool {
        self.hit_enemies.contains(&enemy)
    }

    /// Whether this projectile has spent all of its pierce.
    pub fn is_spent(&self) -> bool {
        self.pierce_remaining == 0
    }
}
