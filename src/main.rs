//! Translation of
//! <http://www.cs.brandeis.edu/~storer/LunarLander/LunarLander/LunarLanderListing.jpg>
//! by Jim Storer from FOCAL to Rust.

extern crate read_input;
use read_input::prelude::*; // imports input()

use std::env;

static mut ECHO_INPUT: bool = false;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        // If --echo is present, then write all input back to standard output.
        // (This is useful for testing with files as redirected input.)
        if args[1] == "--echo" {
            unsafe {
                ECHO_INPUT = true;
            }
        }
    }

    println!("CONTROL CALLING LUNAR MODULE. MANUAL CONTROL IS NECESSARY");
    println!("YOU MAY RESET FUEL RATE K EACH 10 SECS TO 0 OR ANY VALUE");
    println!("BETWEEN 8 & 200 LBS/SEC. YOU'VE 16000 LBS FUEL. ESTIMATED");
    println!("FREE FALL IMPACT TIME=120 SECS. CAPSULE WEIGHT=32500 LBS\n\n");

    loop {
        let mut lander = Lander::new();
        lander.play_game();

        if !play_again() {
            break;
        }
    }

    println!("CONTROL OUT\n");
}

/// Asks whether user wants to play again.
///
/// If user does not respond with something beginning with "Y" or "N", then
/// repeats the prompt until the user provides a valid response.
fn play_again() -> bool {
    let response = input::<String>()
        .msg("\n\nTRY AGAIN?\n(ANS. YES OR NO):")
        .add_err_test(
            |value| {
                let value = value.to_ascii_uppercase();
                value.starts_with('Y') || value.starts_with('N')
            },
            "(ANS. YES OR NO):",
        )
        .get();
    response.to_ascii_uppercase().starts_with('Y')
}

struct Lander {
    /// Altitude (miles)
    a: f64,
    /// Gravity
    g: f64,
    /// Intermediate altitude (miles)
    i: f64,
    /// Intermediate velocity (miles/sec)
    j: f64,
    /// Fuel rate (lbs/sec)
    k: f64,
    /// Elapsed time (sec)
    l: f64,
    /// Total weight (lbs)
    m: f64,
    /// Empty weight (lbs)
    n: f64,
    /// Time elapsed in current 10-second turn (sec)
    s: f64,
    /// Time remaining in current 10-second turn (sec)
    t: f64,
    /// Downward speed (miles/sec)
    v: f64,
    /// Thrust per pound of fuel burned
    z: f64,
}

impl Lander {
    pub fn new() -> Lander {
        Lander {
            a: 0.0,
            g: 0.0,
            i: 0.0,
            j: 0.0,
            k: 0.0,
            l: 0.0,
            m: 0.0,
            n: 0.0,
            s: 0.0,
            t: 0.0,
            v: 0.0,
            z: 0.0,
        }
    }

    /// Run the simulation until the lander is on the moon.
    fn play_game(&mut self) {
        // 01.20 in original FOCAL code
        println!("FIRST RADAR CHECK COMING UP");
        println!("\n\nCOMMENCE LANDING PROCEDURE");
        println!("TIME,SECS   ALTITUDE,MILES+FEET   VELOCITY,MPH   FUEL,LBS   FUEL RATE");

        self.a = 120.0;
        self.v = 1.0;
        self.m = 32500.0;
        self.n = 16500.0;
        self.g = 0.001;
        self.z = 1.8;
        self.l = 0.0;

        self.start_turn();
    }

    fn start_turn(&mut self) {
        print!(
            "{:7.0}{:16.0}{:7.0}{:15.2}{:12.1}      ",
            self.l.round(),
            self.a.trunc(),
            (5280.0 * (self.a - self.a.trunc())).trunc(),
            3600.0 * self.v,
            self.m - self.n
        );

        // TODO: This needs to reject values between 0 and 8,
        // and show the NOT POSSIBLE message on rejection.
        self.k = input()
            .msg("K=:")
            .inside(0.0..=200.0)
            .err("ENTER A VALUE FOR K BETWEEN 0 AND 200 LBS/SEC")
            .get();

        self.t = 10.0;

        self.turn_loop();
    }

    fn turn_loop(&mut self) {
        loop {
            // 03.10 in original FOCAL code
            if self.m - self.n < 0.001 {
                self.fuel_out();
                return;
            }

            if self.t < 0.001 {
                self.start_turn();
                return;
            }

            self.s = self.t;

            if self.n + self.s * self.k - self.m > 0.0 {
                self.s = (self.m - self.n) / self.k;
            }

            self.apply_thrust();

            if self.i <= 0.0 {
                self.loop_until_on_the_moon();
                return;
            }

            if (self.v > 0.0) && (self.j < 0.0) {
                // 08.10 in original FOCAL code
                loop {
                    // FOCAL-to-C gotcha: In FOCAL, multiplication has a higher
                    // precedence than division.  In C, they have the same
                    // precedence and are evaluated left-to-right.  So the
                    // original FOCAL subexpression `M * G / Z * K` can't be
                    // copied as-is into C: `Z * K` has to be parenthesized to
                    // get the same result.
                    let w = (1.0 - self.m * self.g / (self.z * self.k)) / 2.0;
                    self.s = self.m * self.v
                        / (self.z * self.k * (w + (w * w + self.v / self.z).sqrt()))
                        + 0.5;
                    self.apply_thrust();
                    if self.i <= 0.0 {
                        self.loop_until_on_the_moon();
                        return;
                    }
                    self.update_lander_state();
                    if self.j >= 0.0 || self.v <= 0.0 {
                        self.turn_loop();
                        return;
                    }
                }
            }

            self.update_lander_state();
        }
    }

    // 07.10 in original FOCAL code
    fn loop_until_on_the_moon(&mut self) {
        while self.s >= 0.005 {
            let d = self.v
                + (self.v * self.v + 2.0 * self.a * (self.g - self.z * self.k / self.m)).sqrt();
            self.s = 2.0 * self.a / d;
            self.apply_thrust();
            self.update_lander_state();
        }
        self.on_the_moon();
    }

    // 04.10 in original FOCAL code
    fn fuel_out(&mut self) {
        println!("FUEL OUT AT {:8.2} SECS", self.l);
        self.s = ((self.v * self.v + 2.0 * self.a * self.g).sqrt() - self.v) / self.g;
        self.v += self.g * self.s;
        self.l += self.s;

        self.on_the_moon();
        return;
    }

    // 05.10 in original FOCAL code
    fn on_the_moon(&mut self) {
        let w = 3600.0 * self.v;
        println!("ON THE MOON AT {:8.2} SECS", self.l);
        println!("IMPACT VELOCITY OF {:8.2} M.P.H.", w);
        println!("FUEL LEFT: {:8.2} LBS", self.m - self.n);

        if w <= 1.0 {
            println!("PERFECT LANDING !-(LUCKY");
        } else if w <= 10.0 {
            println!("GOOD LANDING-(COULD BE BETTER)");
        } else if w <= 22.0 {
            println!("CONGRATULATIONS ON A POOR LANDING");
        } else if w <= 40.0 {
            println!("CRAFT DAMAGE. GOOD LUCK");
        } else if w <= 60.0 {
            println!("CRASH LANDING-YOU'VE 5 HRS OXYGEN");
        } else {
            println!("SORRY,BUT THERE WERE NO SURVIVORS-YOU BLEW IT!");
            println!(
                "IN FACT YOU BLASTED A NEW LUNAR CRATER {:8.2} FT. DEEP",
                w * 0.277_777
            );
        }
        // fall out to unwind and exit play_game()
    }

    // Subroutine at line 06.10 in original FOCAL code
    fn update_lander_state(&mut self) {
        self.l += self.s;
        self.t -= self.s;
        self.m -= self.s * self.k;
        self.a = self.i;
        self.v = self.j;
    }

    // Subroutine at line 09.10 in original FOCAL code
    fn apply_thrust(&mut self) {
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

