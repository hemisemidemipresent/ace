//! Geometry primitives used by the collision checker and movement.

use std::f32::consts::{PI, TAU};
use std::ops::{Add, Sub};

/// A point or offset in world space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Rotate counter-clockwise about the origin by `angle` radians.
    pub fn rotate(self, angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
        }
    }
    /// The magnitude of this vector
    pub fn mag(self) -> f32 {
        self.mag_squared().sqrt()
    }
    /// The magnitude squared of this vector
    pub fn mag_squared(self) -> f32 {
        self.x.powi(2) + self.y.powi(2)
    }
    /// The direction of this vector, in radians.
    pub fn heading(self) -> f32 {
        self.y.atan2(self.x)
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

/// Wrap an angle into `[-π, π)`.
///
/// Used wherever a signed angle error is needed, such as steering a seeking
/// projectile or the Monkey Ace's `process_movement`.
pub fn wrap_pi(angle: f32) -> f32 {
    (angle + PI).rem_euclid(TAU) - PI
}

/// A circle, positioned by an offset relative to its owner.
///
/// A projectile's single circle has an offset of [`Vec2::ZERO`]. An enemy's
/// circles are expressed relative to the enemy's position and are rotated by the
/// enemy's heading when resolved to world space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    pub offset: Vec2,
    pub radius: f32,
}

impl Circle {
    pub const fn new(offset: Vec2, radius: f32) -> Self {
        Self { offset, radius }
    }

    /// Resolve this circle's centre into world space.
    pub fn world_center(&self, origin: Vec2, heading: f32) -> Vec2 {
        origin + self.offset.rotate(heading)
    }
}

/// An enemy's collision shape: either one circle or a compound of circles.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnemyHitbox {
    Circle(Circle),
    Compound(&'static [Circle]),
}

impl EnemyHitbox {
    /// The shape's constituent circles, in local space.
    pub fn circles(&self) -> &[Circle] {
        match self {
            EnemyHitbox::Circle(circle) => std::slice::from_ref(circle),
            EnemyHitbox::Compound(circles) => circles,
        }
    }
}

/// Whether two circles overlap. Touching circles count as overlapping.
pub fn circles_overlap(a_center: Vec2, a_radius: f32, b_center: Vec2, b_radius: f32) -> bool {
    let delta = a_center - b_center;
    let reach = a_radius + b_radius;
    delta.x * delta.x + delta.y * delta.y <= reach * reach
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlapping_circles_are_detected() {
        assert!(circles_overlap(Vec2::ZERO, 3.0, Vec2::new(5.0, 0.0), 3.0));
    }

    #[test]
    fn touching_circles_count_as_overlapping() {
        assert!(circles_overlap(Vec2::ZERO, 3.0, Vec2::new(6.0, 0.0), 3.0));
    }

    #[test]
    fn separated_circles_are_not_overlapping() {
        assert!(!circles_overlap(Vec2::ZERO, 3.0, Vec2::new(6.1, 0.0), 3.0));
    }

    #[test]
    fn rotating_an_offset_moves_it_counter_clockwise() {
        let circle = Circle::new(Vec2::new(1.0, 0.0), 1.0);
        let center = circle.world_center(Vec2::ZERO, std::f32::consts::FRAC_PI_2);
        assert!(center.x.abs() < 1e-5);
        assert!((center.y - 1.0).abs() < 1e-5);
    }

    #[test]
    fn wrap_pi_normalises_angles() {
        assert!((wrap_pi(0.0) - 0.0).abs() < 1e-6);
        // `[-π, π)` means an odd multiple of π wraps to −π.
        assert!((wrap_pi(3.0 * PI) + PI).abs() < 1e-6);
        assert!((wrap_pi(-3.0 * PI) + PI).abs() < 1e-6);
        assert!((wrap_pi(TAU + 0.5) - 0.5).abs() < 1e-6);
        assert!((wrap_pi(-TAU - 0.5) + 0.5).abs() < 1e-6);
    }
}
