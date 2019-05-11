//! Generic interface for I/O operations used by the simulation.

use crate::lander::Lander;

/// Result of a landing
pub enum Score {
    Perfect,
    Good,
    Poor,
    CraftDamage,
    CrashLanding,
    /// Associated value is depth of crater.
    NoSurvivors(f64),
}

/// An interface for input/output operations for playing the game.
///
/// Each method is an "event handler" that is called by a `Lander` method when a
/// significant event occurs or when input is needed.  These methods may inspect
/// the public members of `Lander`.
pub trait IO {
    /// Called at the start of a new game.
    fn start_game(&mut self, lander: &Lander);

    /// Called at the start of each 10-second turn.
    ///
    /// Must return new fuel rate (K).
    fn get_fuel_rate(&mut self, lander: &Lander) -> f64;

    /// Called if fuel runs out.
    ///
    /// `l` is the elapsed time since the start of the game.
    fn fuel_out(&mut self, l: f64);

    /// Called upon impact with the moon surface.
    fn on_the_moon(&mut self, lander: &Lander, score: Score);
}

/// Minimal implementation of `IO`.
///
/// Does nothing.  Returns 0.0 from `get_fuel_rate`.  Useful for testing only.
///
/// ```
/// # use lunar::io::NullIO;
/// # use lunar::lander::Lander;
/// let mut io = NullIO {};
/// let mut lander = Lander::default();
/// lander.play_game(&mut io);
/// ```
pub struct NullIO {}

impl IO for NullIO {
    fn start_game(&mut self, _lander: &Lander) {}

    fn get_fuel_rate(&mut self, _lander: &Lander) -> f64 {
        0.0
    }

    fn fuel_out(&mut self, _l: f64) {}

    fn on_the_moon(&mut self, _lander: &Lander, _score: Score) {}
}
