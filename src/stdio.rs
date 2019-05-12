//! Implements game I/O using standard input and standard output.

use crate::io::{Score, IO};
use crate::lander::Lander;

use std::io;
use std::io::prelude::*;
use std::process;

/// Implementation of the `lunar::io::IO` trait using standard input and
/// standard output.
#[derive(Default)]
pub struct StdIO {
    pub echo_input: bool,
}

impl StdIO {
    /// Print instructions for playing the game.
    pub fn greeting(&self) {
        println!("CONTROL CALLING LUNAR MODULE. MANUAL CONTROL IS NECESSARY");
        println!("YOU MAY RESET FUEL RATE K EACH 10 SECS TO 0 OR ANY VALUE");
        println!("BETWEEN 8 & 200 LBS/SEC. YOU'VE 16000 LBS FUEL. ESTIMATED");
        println!("FREE FALL IMPACT TIME=120 SECS. CAPSULE WEIGHT=32500 LBS\n\n");
    }

    /// Asks whether user wants to play again.
    ///
    /// If user does not respond with something beginning with "Y" or "N", then
    /// repeats the prompt until the user provides a valid response.
    pub fn play_again(&self) -> bool {
        println!("\n\n\n\nTRY AGAIN?");
        loop {
            print!("(ANS. YES OR NO):");
            match self.accept_line() {
                Ok(line) => {
                    let line = line.trim().to_ascii_uppercase();
                    if line.starts_with('Y') {
                        return true;
                    } else if line.starts_with('N') {
                        return false;
                    }
                }
                Err(_) => return false,
            }
        }
    }

    /// Print a final message.
    pub fn farewell(&self) {
        println!("CONTROL OUT");
    }

    /// Reads a numeric value from standard input.
    ///
    /// Returns an error with kind `std::io::ErrorKind::InvalidData`
    /// if data is read that is not a valid numeric value.
    ///
    /// Calls `std::process::exit(-1)` on EOF.
    fn accept_number(&self) -> io::Result<f64> {
        let line = self.accept_line()?;
        match line.trim().parse() {
            Ok(num) => Ok(num),
            Err(err) => Err(io::Error::new(io::ErrorKind::InvalidData, err)),
        }
    }

    /// Reads a line from standard input.
    ///
    /// Calls `std::process::exit(-1)` on EOF.
    fn accept_line(&self) -> io::Result<String> {
        io::stdout().flush()?;
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        if line.is_empty() {
            process::exit(-1);
        }
        if self.echo_input {
            print!("{}", line);
        }
        Ok(line)
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

        loop {
            print!("K=:");
            match self.accept_number() {
                Ok(num) => {
                    if (num == 0.0 || num >= 8.0) && num <= 200.0 {
                        return num;
                    }
                }
                Err(err) => {
                    if err.kind() != io::ErrorKind::InvalidData {
                        panic!("unable to read input");
                    }
                }
            }
            print!("NOT POSSIBLE");
            for _ in 1..=51 {
                print!(".")
            }
        }
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
