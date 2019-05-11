//! Implements game I/O using standard input and standard output.

extern crate read_input;
use read_input::prelude::*; // imports input()

use crate::io::{Score, IO};
use crate::lander::Lander;

/// Implementation of the `lunar::io::IO` trait using standard input and
/// standard output.
pub struct StdIO {}

impl StdIO {
    /// Print instructions for playing the game.
    pub fn greeting(&self) {
        println!("CONTROL CALLING LUNAR MODULE. MANUAL CONTROL IS NECESSARY");
        println!("YOU MAY RESET FUEL RATE K EACH 10 SECS TO 0 OR ANY VALUE");
        println!("BETWEEN 8 & 200 LBS/SEC. YOU'VE 16000 LBS FUEL. ESTIMATED");
        println!("FREE FALL IMPACT TIME=120 SECS. CAPSULE WEIGHT=32500 LBS\n\n");
    }

    /// Print a final message.
    pub fn farewell(&self) {
        println!("CONTROL OUT");
    }
}

impl Default for StdIO {
    fn default() -> Self {
        StdIO {}
    }
}

impl IO for StdIO {
    fn start_game(&mut self, _lander: &Lander) {
        println!("FIRST RADAR CHECK COMING UP");
        println!("\n\nCOMMENCE LANDING PROCEDURE");
        println!("TIME,SECS   ALTITUDE,MILES+FEET   VELOCITY,MPH   FUEL,LBS   FUEL RATE");
    }

    fn get_fuel_rate(&mut self, lander: &Lander) -> f64 {
        print!(
            "{:7.0}{:16.0}{:7.0}{:15.2}{:12.1}      ",
            lander.l.round(),
            lander.a.trunc(),
            (5280.0 * (lander.a - lander.a.trunc())).trunc(),
            lander.v_mph(),
            lander.m - lander.n
        );

        // TODO: This needs to reject values between 0 and 8,
        // and show the NOT POSSIBLE message on rejection.
        input()
            .msg("K=:")
            .inside(0.0..=200.0)
            .err("ENTER A VALUE FOR K BETWEEN 0 AND 200 LBS/SEC")
            .get()
    }

    fn fuel_out(&mut self, l: f64) {
        println!("FUEL OUT AT {:8.2} SECS", l);
    }

    fn on_the_moon(&mut self, lander: &Lander, score: Score) {
        println!("ON THE MOON AT {:8.2} SECS", lander.l);
        println!("IMPACT VELOCITY OF {:8.2} M.P.H.", lander.v_mph());
        println!("FUEL LEFT: {:8.2} LBS", lander.fuel_remaining());

        match score {
            Score::Perfect => println!("PERFECT LANDING !-(LUCKY"),
            Score::Good => println!("GOOD LANDING-(COULD BE BETTER)"),
            Score::Poor => println!("CONGRATULATIONS ON A POOR LANDING"),
            Score::CraftDamage => println!("CRAFT DAMAGE. GOOD LUCK"),
            Score::CrashLanding => println!("CRASH LANDING-YOU'VE 5 HRS OXYGEN"),
            Score::NoSurvivors(crater_depth) => {
                println!("SORRY,BUT THERE WERE NO SURVIVORS-YOU BLEW IT!");
                println!(
                    "IN FACT YOU BLASTED A NEW LUNAR CRATER {:8.2} FT. DEEP",
                    crater_depth
                );
            }
        }
    }
}
