//! Broadphase: the play area's bounds, its configuration, and a uniform grid.

use crate::hitbox::Vec2;
use crate::types::{EnemyType, enemy_stats};

/// Left edge of the play area.
pub const X_MIN: f32 = -200.0;
/// Right edge of the play area.
pub const X_MAX: f32 = 200.0;
/// Bottom edge of the play area.
pub const Y_MIN: f32 = -150.0;
/// Top edge of the play area.
pub const Y_MAX: f32 = 150.0;

/// Whether a position lies inside the play area. Edges are inclusive.
///
/// Out-of-bounds is decided by the object's centre; a hitbox spilling over the
/// edge is not clipped, but the grid never populates out-of-range cells.
pub fn in_bounds(position: Vec2) -> bool {
    position.x >= X_MIN && position.x <= X_MAX && position.y >= Y_MIN && position.y <= Y_MAX
}

/// Simulation-wide configuration.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SimConfig {
    /// Side length of a broadphase grid cell, in world units.
    pub cell_size: f32,
    /// Multiplier on the speed of non-MOAB-class Bloons.
    pub bloon_speed_multiplier: f32,
    /// Multiplier on the speed of MOAB-class Bloons.
    pub moab_speed_multiplier: f32,
    /// Multiplier on Ceramic health. Ceramics only.
    pub ceramic_health_multiplier: f32,
    /// Multiplier on MOAB-class health.
    pub moab_health_multiplier: f32,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            cell_size: 32.0,
            bloon_speed_multiplier: 1.0,
            moab_speed_multiplier: 1.0,
            ceramic_health_multiplier: 1.0,
            moab_health_multiplier: 1.0,
        }
    }
}

impl SimConfig {
    /// The speed multiplier that applies to `kind`: MOAB-class Bloons use
    /// [`Self::moab_speed_multiplier`], everything else uses
    /// [`Self::bloon_speed_multiplier`].
    pub fn speed_multiplier(&self, kind: EnemyType) -> f32 {
        if kind.is_moab_plus() {
            self.moab_speed_multiplier
        } else {
            self.bloon_speed_multiplier
        }
    }

    /// The health multiplier that applies to `kind`: MOAB-class Bloons use
    /// [`Self::moab_health_multiplier`], Ceramics use
    /// [`Self::ceramic_health_multiplier`], everything else is unscaled.
    pub fn health_multiplier(&self, kind: EnemyType) -> f32 {
        if kind.is_moab_plus() {
            self.moab_health_multiplier
        } else if kind.is_ceramic() {
            self.ceramic_health_multiplier
        } else {
            1.0
        }
    }

    /// `kind`'s starting Health under this configuration, rounded to a whole
    /// number.
    pub fn max_health(&self, kind: EnemyType) -> u32 {
        (enemy_stats(kind).max_hp as f32 * self.health_multiplier(kind)).round() as u32
    }
}

/// A uniform grid over the play area, mapping cells to indices of inserted items.
///
/// This is derived state: it is rebuilt on every collision check and never
/// stored on [`crate::World`].
#[derive(Debug, Clone)]
pub struct Grid {
    cell_size: f32,
    cols: usize,
    rows: usize,
    cells: Vec<Vec<usize>>,
}

impl Grid {
    /// Build an empty grid with the given cell size. Non-positive sizes fall back
    /// to the default of 32 units.
    pub fn new(cell_size: f32) -> Self {
        let cell_size = if cell_size > 0.0 { cell_size } else { 32.0 };
        let cols = (((X_MAX - X_MIN) / cell_size).ceil() as usize).max(1);
        let rows = (((Y_MAX - Y_MIN) / cell_size).ceil() as usize).max(1);
        Self {
            cell_size,
            cols,
            rows,
            cells: vec![Vec::new(); cols * rows],
        }
    }

    /// Inclusive cell span covering `[min, max]` along one axis, or `None` if the
    /// interval lies entirely outside the play area. Out-of-range results are
    /// clamped to the grid rather than wrapped.
    fn cell_span(&self, min: f32, max: f32, origin: f32, count: usize) -> Option<(usize, usize)> {
        let far = origin + count as f32 * self.cell_size;
        if max < origin || min > far {
            return None;
        }
        let last = (count - 1) as f32;
        let lo = (((min - origin) / self.cell_size).floor()).clamp(0.0, last) as usize;
        let hi = (((max - origin) / self.cell_size).floor()).clamp(0.0, last) as usize;
        Some((lo, hi))
    }

    /// Insert `index` into every cell overlapped by a circle's bounding box.
    ///
    /// Circles entirely outside the play area are ignored.
    pub fn insert_circle(&mut self, index: usize, center: Vec2, radius: f32) {
        let Some((x0, x1)) = self.cell_span(center.x - radius, center.x + radius, X_MIN, self.cols)
        else {
            return;
        };
        let Some((y0, y1)) = self.cell_span(center.y - radius, center.y + radius, Y_MIN, self.rows)
        else {
            return;
        };
        for x in x0..=x1 {
            for y in y0..=y1 {
                self.cells[x + y * self.cols].push(index);
            }
        }
    }

    /// Every distinct index whose circle could overlap the queried circle.
    ///
    /// Returns candidates only; callers still run the exact narrowphase test.
    pub fn query_circle(&self, center: Vec2, radius: f32) -> Vec<usize> {
        let mut found = Vec::new();
        let Some((x0, x1)) = self.cell_span(center.x - radius, center.x + radius, X_MIN, self.cols)
        else {
            return found;
        };
        let Some((y0, y1)) = self.cell_span(center.y - radius, center.y + radius, Y_MIN, self.rows)
        else {
            return found;
        };
        for x in x0..=x1 {
            for y in y0..=y1 {
                found.extend_from_slice(&self.cells[x + y * self.cols]);
            }
        }
        found.sort_unstable();
        found.dedup();
        found
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounds_edges_are_inclusive() {
        assert!(in_bounds(Vec2::new(X_MIN, Y_MIN)));
        assert!(in_bounds(Vec2::new(X_MAX, Y_MAX)));
        assert!(!in_bounds(Vec2::new(X_MAX + 0.01, 0.0)));
        assert!(!in_bounds(Vec2::new(0.0, Y_MIN - 0.01)));
    }

    #[test]
    fn querying_a_cell_returns_inserted_items() {
        let mut grid = Grid::new(32.0);
        grid.insert_circle(7, Vec2::new(0.0, 0.0), 2.0);
        assert_eq!(grid.query_circle(Vec2::new(1.0, 1.0), 2.0), vec![7]);
    }

    #[test]
    fn an_item_spanning_cells_is_returned_once() {
        let mut grid = Grid::new(8.0);
        grid.insert_circle(3, Vec2::new(0.0, 0.0), 20.0);
        assert_eq!(grid.query_circle(Vec2::new(0.0, 0.0), 20.0), vec![3]);
    }

    #[test]
    fn fully_out_of_range_circles_are_dropped() {
        let mut grid = Grid::new(32.0);
        grid.insert_circle(1, Vec2::new(X_MAX + 100.0, 0.0), 2.0);
        assert!(grid.query_circle(Vec2::new(0.0, 0.0), 1000.0).is_empty());
    }
}
