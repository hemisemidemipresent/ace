//! Hardcoded track geometry and arc-length sampling.
//!
//! The waypoints are transcribed from `bloon_paths.csv` (kept in the repository
//! as the source data), rounded to three decimals to strip float32 noise.
//! Cumulative distances are recomputed here from the waypoint coordinates rather
//! than read from that file's `distance` column: the column carries the game
//! track's true arc length, which is longer than the straight polyline on two
//! curved stretches, and movement follows the polyline.

use crate::hitbox::Vec2;

/// Identity of a path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PathId {
    /// Not on any path. An anchored Enemy never moves.
    Anchored,
    /// Index into a [`Paths`] set.
    Indexed(u32),
}

/// The main path, as hardcoded from `bloon_paths.csv`.
const MAIN_PATH: [Vec2; 37] = [
    Vec2::new(-200.0, -69.9),
    Vec2::new(-91.3, -69.9),
    Vec2::new(-84.3, -67.93),
    Vec2::new(-80.3, -61.73),
    Vec2::new(-80.3, 70.17),
    Vec2::new(-79.05, 73.86),
    Vec2::new(-70.9, 77.09),
    Vec2::new(-30.8, 77.09),
    Vec2::new(-27.1, 72.67),
    Vec2::new(-26.9, -65.7),
    Vec2::new(-21.7, -70.8),
    Vec2::new(19.2, -70.8),
    Vec2::new(21.8, -65.1),
    Vec2::new(21.5, 72.87),
    Vec2::new(25.5, 78.87),
    Vec2::new(69.0, 78.87),
    Vec2::new(75.2, 76.37),
    Vec2::new(77.3, 71.27),
    Vec2::new(77.3, -65.2),
    Vec2::new(79.4, -69.9),
    Vec2::new(86.5, -72.0),
    Vec2::new(102.8, -72.0),
    Vec2::new(106.2, -68.4),
    Vec2::new(106.2, -29.7),
    Vec2::new(103.99, -26.1),
    Vec2::new(46.2, -25.03),
    Vec2::new(-88.1, -29.87),
    Vec2::new(-100.1, -29.5),
    Vec2::new(-109.7, -29.17),
    Vec2::new(-109.7, 18.2),
    Vec2::new(-103.3, 21.2),
    Vec2::new(-35.0, 21.4),
    Vec2::new(99.6, 17.7),
    Vec2::new(106.4, 18.1),
    Vec2::new(106.4, 53.4),
    Vec2::new(96.8, 53.7),
    Vec2::new(-160.0, 53.0),
];

/// A point on a path, with its cumulative distance from the start.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Waypoint {
    pub position: Vec2,
    /// Distance from the path start to this waypoint.
    pub distance: f32,
    /// Speed multiplier applied on the segment that starts at this waypoint.
    pub speed_multiplier: f32,
}

/// A polyline an enemy travels along, addressed by cumulative distance.
#[derive(Debug, Clone)]
pub struct Path {
    name: &'static str,
    waypoints: Vec<Waypoint>,
    length: f32,
}

impl Path {
    /// Build a path from waypoints, computing cumulative distances.
    pub fn new(name: &'static str, points: &[Vec2]) -> Self {
        let mut waypoints = Vec::with_capacity(points.len());
        let mut distance = 0.0;
        for (index, &position) in points.iter().enumerate() {
            if index > 0 {
                let delta = position - points[index - 1];
                distance += (delta.x * delta.x + delta.y * delta.y).sqrt();
            }
            waypoints.push(Waypoint {
                position,
                distance,
                speed_multiplier: 1.0,
            });
        }
        Self {
            name,
            waypoints,
            length: distance,
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Total arc length of the path.
    pub fn length(&self) -> f32 {
        self.length
    }

    /// The path's waypoints, in order.
    pub fn waypoints(&self) -> &[Waypoint] {
        &self.waypoints
    }

    /// The path's start position and initial heading.
    pub fn start(&self) -> (Vec2, f32) {
        self.sample(0.0)
    }

    /// Position and heading at `distance` along the path, clamped to the ends.
    pub fn sample(&self, distance: f32) -> (Vec2, f32) {
        let count = self.waypoints.len();
        if count == 0 {
            return (Vec2::ZERO, 0.0);
        }
        if count == 1 {
            return (self.waypoints[0].position, 0.0);
        }

        let distance = distance.clamp(0.0, self.length);
        let end = self
            .waypoints
            .partition_point(|waypoint| waypoint.distance < distance)
            .clamp(1, count - 1);
        let from = self.waypoints[end - 1];
        let to = self.waypoints[end];

        let span = to.distance - from.distance;
        let t = if span > 0.0 {
            (distance - from.distance) / span
        } else {
            0.0
        };
        let position = Vec2::new(
            from.position.x + (to.position.x - from.position.x) * t,
            from.position.y + (to.position.y - from.position.y) * t,
        );
        let delta = to.position - from.position;
        let heading = if delta.x == 0.0 && delta.y == 0.0 {
            0.0
        } else {
            delta.y.atan2(delta.x)
        };
        (position, heading)
    }

    /// The speed multiplier of the segment that contains `distance`.
    pub fn speed_multiplier_at(&self, distance: f32) -> f32 {
        let count = self.waypoints.len();
        if count < 2 {
            return 1.0;
        }
        let distance = distance.clamp(0.0, self.length);
        let end = self
            .waypoints
            .partition_point(|waypoint| waypoint.distance < distance)
            .clamp(1, count - 1);
        self.waypoints[end - 1].speed_multiplier
    }
}

/// A set of paths, addressed by [`PathId`].
#[derive(Debug, Clone, Default)]
pub struct Paths {
    paths: Vec<Path>,
}

impl Paths {
    /// Wrap a set of paths.
    pub fn new(paths: Vec<Path>) -> Self {
        Self { paths }
    }

    /// The paths that ship with the simulator.
    pub fn bundled() -> Self {
        Self {
            paths: vec![Path::new("Path", &MAIN_PATH)],
        }
    }

    pub fn len(&self) -> usize {
        self.paths.len()
    }

    pub fn is_empty(&self) -> bool {
        self.paths.is_empty()
    }

    /// The path for `id`, or `None` for [`PathId::Anchored`].
    pub fn try_get(&self, id: PathId) -> Option<&Path> {
        match id {
            PathId::Anchored => None,
            PathId::Indexed(index) => self.paths.get(index as usize),
        }
    }

    /// The path for `id`.
    ///
    /// Resolving a path for [`PathId::Anchored`] is a programming error and
    /// panics, as is an out-of-range index.
    pub fn get(&self, id: PathId) -> &Path {
        match id {
            PathId::Anchored => panic!("PathId::Anchored has no path"),
            PathId::Indexed(index) => &self.paths[index as usize],
        }
    }
}
