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

    let mut io = StdIO::default();
    let mut lander = Lander::default();
    loop {
        lander.play_game(&mut io);

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

/// Result of a landing
enum Score {
    Perfect,
    Good,
    Poor,
    CraftDamage,
    CrashLanding,
    /// Total destruction. Associated value is depth of crater.
    NoSurvivors(f64),
}

/// An interface for input/output operations for playing the game.
trait IO {
    /// Called at the start of a new game.
    fn start_game(&mut self, lander: &Lander);

    /// Called at the start of each 10-second turn.
    ///
    /// Returns new fuel rate (K).
    fn get_fuel_rate(&mut self, l: f64, a: f64, v: f64, m: f64, n: f64) -> f64;

    /// Called if fuel runs out.
    ///
    /// `l` is the elapsed time since the start of the game.
    fn fuel_out(&mut self, l: f64);

    /// Called upon impact with the moon surface.
    fn on_the_moon(&mut self, lander: &Lander, score: Score);
}

/// Implementation of the `IO` trait using standard input and standard output.
struct StdIO {}

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

    fn get_fuel_rate(&mut self, l: f64, a: f64, v: f64, m: f64, n: f64) -> f64 {
        print!(
            "{:7.0}{:16.0}{:7.0}{:15.2}{:12.1}      ",
            l.round(),
            a.trunc(),
            (5280.0 * (a - a.trunc())).trunc(),
            3600.0 * v,
            m - n
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
        let w = 3600.0 * lander.v;
        println!("ON THE MOON AT {:8.2} SECS", lander.l);
        println!("IMPACT VELOCITY OF {:8.2} M.P.H.", w);
        println!("FUEL LEFT: {:8.2} LBS", lander.m - lander.n);

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

/// Lander simulation
///
/// The terse member names come from the names of global variables in the
/// original FOCAL code.
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

impl Default for Lander {
    fn default() -> Self {
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
}

impl Lander {
    /// Run the simulation until the lander is on the moon.
    fn play_game(&mut self, io: &mut dyn IO) {
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

    fn start_turn(&mut self, io: &mut dyn IO) {
        // 02.10 in original FOCAL code
        self.k = io.get_fuel_rate(self.l, self.a, self.v, self.m, self.n);
        self.t = 10.0;

        self.turn_loop(io);
    }

    fn turn_loop(&mut self, io: &mut dyn IO) {
        // 03.10 in original FOCAL code
        loop {
            if self.m - self.n < 0.001 {
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
        let w = 3600.0 * self.v;

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
