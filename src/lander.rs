//! Implementation of the simulation's calculations and logic.

use super::io::{Score, IO};

/// Lander simulation
///
/// The terse member names come from the names of global variables in the
/// original FOCAL code.
#[derive(Copy, Clone, Default)]
pub struct Lander {
    /// Altitude (miles)
    pub a: f64,
    /// Gravity
    pub g: f64,
    /// Intermediate altitude (miles)
    i: f64,
    /// Intermediate velocity (miles/sec)
    j: f64,
    /// Fuel rate (lbs/sec)
    pub k: f64,
    /// Elapsed time (sec)
    pub l: f64,
    /// Total weight (lbs)
    pub m: f64,
    /// Empty weight (lbs)
    pub n: f64,
    /// Time elapsed in current 10-second turn (sec)
    s: f64,
    /// Time remaining in current 10-second turn (sec)
    t: f64,
    /// Downward speed (miles/sec)
    pub v: f64,
    /// Thrust per pound of fuel burned
    pub z: f64,
}

impl Lander {
    /// Run the simulation until the lander is on the moon.
    ///
    /// Methods of the `IO` object will be called to get input and report
    /// results.
    pub fn play_game(&mut self, io: &mut dyn IO) {
        self.a = 120.0;
        self.v = 1.0;
        self.m = 32500.0;
        self.n = 16500.0;
        self.g = 0.001;
        self.z = 1.8;
        self.l = 0.0;

        io.start_game(self);

        self.start_turn(io);
    }

    /// Return downward speed as miles/hour
    pub fn v_mph(&self) -> f64 {
        3600.0 * self.v
    }

    /// Return pounds of fuel remaining
    pub fn fuel_remaining(&self) -> f64 {
        self.m - self.n
    }

    fn start_turn(&mut self, io: &mut dyn IO) {
        // 02.10 in original FOCAL code
        self.k = io.get_fuel_rate(self);
        self.t = 10.0;

        self.turn_loop(io);
    }

    fn turn_loop(&mut self, io: &mut dyn IO) {
        // 03.10 in original FOCAL code
        loop {
            if self.fuel_remaining() < 0.001 {
                self.fuel_out(io);
                return;
            }

            if self.t < 0.001 {
                self.start_turn(io);
                return;
            }

            self.s = self.t;

            if self.n + self.s * self.k - self.m > 0.0 {
                self.s = (self.m - self.n) / self.k;
            }

            self.apply_thrust();

            if self.i <= 0.0 {
                self.loop_until_on_the_moon(io);
                return;
            }

            if (self.v > 0.0) && (self.j < 0.0) {
                // 08.10 in original FOCAL code
                loop {
                    // FOCAL-to-Rust gotcha: In FOCAL, multiplication has a
                    // higher precedence than division.  In Rust, they have the
                    // same precedence and are evaluated left-to-right.  So the
                    // original FOCAL subexpression `M * G / Z * K` can't be
                    // copied as-is into Rust: `Z * K` has to be parenthesized
                    // to get the same result.
                    let w = (1.0 - self.m * self.g / (self.z * self.k)) / 2.0;
                    self.s = self.m * self.v
                        / (self.z * self.k * (w + (w * w + self.v / self.z).sqrt()))
                        + 0.5;
                    self.apply_thrust();
                    if self.i <= 0.0 {
                        self.loop_until_on_the_moon(io);
                        return;
                    }
                    self.update_state();
                    if self.j >= 0.0 || self.v <= 0.0 {
                        self.turn_loop(io);
                        return;
                    }
                }
            }

            self.update_state();
        }
    }

    fn loop_until_on_the_moon(&mut self, io: &mut dyn IO) {
        // 07.10 in original FOCAL code
        while self.s >= 0.005 {
            let d = self.v
                + (self.v * self.v + 2.0 * self.a * (self.g - self.z * self.k / self.m)).sqrt();
            self.s = 2.0 * self.a / d;
            self.apply_thrust();
            self.update_state();
        }
        self.on_the_moon(io);
    }

    fn fuel_out(&mut self, io: &mut dyn IO) {
        // 04.10 in original FOCAL code
        io.fuel_out(self.l);
        self.s = ((self.v * self.v + 2.0 * self.a * self.g).sqrt() - self.v) / self.g;
        self.v += self.g * self.s;
        self.l += self.s;

        self.on_the_moon(io);
        return;
    }

    fn on_the_moon(&mut self, io: &mut dyn IO) {
        // 05.10 in original FOCAL code
        let w = self.v_mph();

        let score = if w <= 1.0 {
            Score::Perfect
        } else if w <= 10.0 {
            Score::Good
        } else if w <= 22.0 {
            Score::Poor
        } else if w <= 40.0 {
            Score::CraftDamage
        } else if w <= 60.0 {
            Score::CrashLanding
        } else {
            Score::NoSurvivors(w * 0.277_277)
        };

        io.on_the_moon(self, score)

        // fall out to unwind and exit play_game()
    }

    fn update_state(&mut self) {
        // Subroutine at line 06.10 in original FOCAL code
        self.l += self.s;
        self.t -= self.s;
        self.m -= self.s * self.k;
        self.a = self.i;
        self.v = self.j;
    }

    fn apply_thrust(&mut self) {
        // Subroutine at line 09.10 in original FOCAL code
        let q = self.s * self.k / self.m;

        let q_2 = q.powi(2);
        let q_3 = q.powi(3);
        let q_4 = q.powi(4);
        let q_5 = q.powi(5);

        self.j = self.v
            + self.g * self.s
            + self.z * (-q - q * q / 2.0 - q_3 / 3.0 - q_4 / 4.0 - q_5 / 5.0);
        self.i = self.a - self.g * self.s * self.s / 2.0 - self.v * self.s
            + self.z * self.s * (q / 2.0 + q_2 / 6.0 + q_3 / 12.0 + q_4 / 20.0 + q_5 / 30.0);
    }
}
