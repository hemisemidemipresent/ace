//! The Monkey Ace: the moving point that fires projectiles.

use crate::hitbox::{Vec2, wrap_pi};
use std::f32::consts::{PI, TAU};

/// How the ace chooses where to fly. User-controlled; unused until
/// `process_movement` consumes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Targeting {
    #[default]
    Circle,
    FigureInfinite,
    FigureEight,
}

/// Timers that gate the ace's two attacks, in ticks.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct AceAttacks {
    pub dart_timer: f32,
    pub missile_timer: f32,
}

/// A Monkey Ace's current state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ace {
    /// Where the ace is placed on the map. Fixed.
    pub center: Vec2,
    /// The ace's movement offset relative to [`Ace::center`].
    /// Mutated by [`Ace::process_movement`].
    pub position: Vec2,
    /// Base velocity of the ace
    pub base_velocity: f32,
    /// Direction the plane faces, in radians. Rotates the attack offsets.
    /// Mutated by [`Ace::process_movement`].
    pub heading: f32,
    /// Internal movement clock, updated by [`Ace::process_movement`].
    pub phase: f32,
    /// Whether the plane is currently on path or not
    /// Mutated by [`Ace::process_movement`].
    pub on_path: bool,
    /// Targeting of the ace; user-controlled
    pub targeting: Targeting,
    /// Reverses the flight direction; user-controlled.
    pub reverse: bool,
    pub attacks: AceAttacks,
    pub target_pos: Vec2,
}

impl Default for Ace {
    fn default() -> Self {
        Self {
            center: Vec2::ZERO,
            position: Vec2::new(30.0, 0.0),
            base_velocity: 1.0,
            heading: 0.0,
            phase: 0.0,
            on_path: true,
            targeting: Targeting::Circle,
            reverse: false,
            attacks: AceAttacks::default(),
            target_pos: Vec2::new(80.0, 0.0),
        }
    }
}

impl Ace {
    /// Create an ace placed at `center`, facing `heading`, with no movement
    /// offset yet.
    pub fn new(center: Vec2, heading: f32) -> Self {
        Self {
            center,
            heading,
            ..Self::default()
        }
    }

    /// Where the ace actually is: its placement plus its movement offset.
    pub fn world_position(&self) -> Vec2 {
        self.center + self.position
    }

    /// Advance the ace's movement by one tick.
    ///
    /// Shell: the ace's flight will be implemented here by mutating
    /// [`Ace::position`] (relative to [`Ace::center`]) and [`Ace::phase`].
    /// [`crate::hitbox::wrap_pi`] is available for angle work.
    pub fn process_movement(&mut self) {
        self.target_pos = match self.targeting {
            Targeting::Circle => Vec2::new(80.0 * self.phase.cos(), 80.0 * self.phase.sin()),
            Targeting::FigureEight => {
                if self.phase <= PI {
                    let local_angle = self.phase * 2.0 + PI / 2.0;
                    Vec2::new(40.0 * local_angle.cos(), 40.0 * local_angle.sin() - 40.0)
                } else {
                    let local_angle = 1.5 * PI - (self.phase - PI) * 2.0;
                    Vec2::new(40.0 * local_angle.cos(), 40.0 * local_angle.sin() + 40.0)
                }
            }
            Targeting::FigureInfinite => {
                if self.phase <= PI {
                    let local_angle = self.phase * 2.0 + PI;
                    Vec2::new(40.0 * local_angle.cos() + 40.0, 40.0 * local_angle.sin())
                } else {
                    let local_angle = -(self.phase - PI) * 2.0;
                    Vec2::new(40.0 * local_angle.cos() - 40.0, 40.0 * local_angle.sin())
                }
            }
        };
        if self.on_path {
            self.position = self.target_pos;
            self.heading = match self.targeting {
                Targeting::Circle => self.phase + PI / 2.0,
                Targeting::FigureEight => {
                    if self.phase <= PI {
                        let local_angle = self.phase * 2.0 + PI / 2.0;
                        local_angle + PI / 2.0
                    } else {
                        let local_angle = 1.5 * PI - (self.phase - PI) * 2.0;
                        local_angle - PI / 2.0
                    }
                }
                Targeting::FigureInfinite => {
                    if self.phase <= PI {
                        let local_angle = self.phase * 2.0 + PI;
                        local_angle + PI / 2.0
                    } else {
                        let local_angle = -(self.phase - PI) * 2.0;
                        local_angle - PI / 2.0
                    }
                }
            };
            if self.reverse {
                self.heading += PI;
            }
        }
        if !self.on_path {
            let difference_vector = self.target_pos - self.position;
            let angle_change = wrap_pi(difference_vector.heading() - self.heading);
            const MAX_TURN_PER_TICK: f32 = PI / 30.0;
            self.heading += angle_change.clamp(-MAX_TURN_PER_TICK, MAX_TURN_PER_TICK);
            self.position =
                self.position + Vec2::new(1.2 * self.heading.cos(), 1.2 * self.heading.sin());

            if difference_vector.mag_squared() < 1.0 {
                self.on_path = true;
            }
        }

        // update phase
        if self.reverse {
            self.phase = (self.phase - 1.0 / 80.0 / self.base_velocity).rem_euclid(TAU);
        } else {
            self.phase = (self.phase + 1.0 / 80.0 / self.base_velocity).rem_euclid(TAU);
        }
        // normalize heading
        self.heading = wrap_pi(self.heading);
    }
}

/// Darts fired per volley.
pub const DART_COUNT: usize = 32;
/// The dart volley leaves the ace from this offset, rotated by its heading.
pub const DART_OFFSET: Vec2 = Vec2::new(0.0, 6.5);
/// Ticks between dart volleys: 0.315 s.
pub const DART_INTERVAL_TICKS: f32 = 18.9;

/// The two missile launch offsets, rotated by the ace's heading.
pub const MISSILE_OFFSETS: [Vec2; 2] = [Vec2::new(12.0, -0.24), Vec2::new(-12.0, -0.24)];
/// Ticks between missile volleys: 1.5 s.
pub const MISSILE_INTERVAL_TICKS: f32 = 90.0;
