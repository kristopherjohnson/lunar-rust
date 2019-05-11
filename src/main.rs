//! Translation of
//! <http://www.cs.brandeis.edu/~storer/LunarLander/LunarLander/LunarLanderListing.jpg>
//! by Jim Storer from FOCAL to Rust.

extern crate read_input;
use read_input::prelude::*; // imports input()

use std::env;

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
        unsafe {
            play_game();
        }

        if !play_again() {
            break;
        }
    }

    println!("CONTROL OUT\n");
}

static mut ECHO_INPUT: bool = false;

// TODO: wrap these global variables in a struct
// and remove the "unsafe" modifiers
static mut A: f64 = 0.0;
static mut G: f64 = 0.0;
static mut I: f64 = 0.0;
static mut J: f64 = 0.0;
static mut K: f64 = 0.0;
static mut L: f64 = 0.0;
static mut M: f64 = 0.0;
static mut N: f64 = 0.0;
static mut S: f64 = 0.0;
static mut T: f64 = 0.0;
static mut V: f64 = 0.0;
static mut Z: f64 = 0.0;

/// Run the simulation until the lander is on the moon.
unsafe fn play_game() {
    // 01.20 in original FOCAL code
    println!("FIRST RADAR CHECK COMING UP");
    println!("\n\nCOMMENCE LANDING PROCEDURE");
    println!("TIME,SECS   ALTITUDE,MILES+FEET   VELOCITY,MPH   FUEL,LBS   FUEL RATE");

    A = 120.0;
    V = 1.0;
    M = 32500.0;
    N = 16500.0;
    G = 0.001;
    Z = 1.8;
    L = 0.0;

    start_turn();
}

unsafe fn start_turn() {
    print!(
        "{:7.0}{:16.0}{:7.0}{:15.2}{:12.1}      ",
        L.round(),
        A.trunc(),
        (5280.0 * (A - A.trunc())).trunc(),
        3600.0 * V,
        M - N
    );

    // TODO: This needs to reject values between 0 and 8,
    // and show the NOT POSSIBLE message on rejection.
    K = input()
        .msg("K=:")
        .inside(0.0..=200.0)
        .err("ENTER A VALUE FOR K BETWEEN 0 AND 200 LBS/SEC")
        .get();

    T = 10.0;

    turn_loop();
}

unsafe fn turn_loop() {
    loop {
        // 03.10 in original FOCAL code
        if M - N < 0.001 {
            fuel_out();
            return;
        }

        if T < 0.001 {
            start_turn();
            return;
        }

        S = T;

        if N + S * K - M > 0.0 {
            S = (M - N) / K;
        }

        apply_thrust();

        if I <= 0.0 {
            loop_until_on_the_moon();
            return;
        }

        if (V > 0.0) && (J < 0.0) {
            // 08.10 in original FOCAL code
            loop {
                // FOCAL-to-C gotcha: In FOCAL, multiplication has a higher
                // precedence than division.  In C, they have the same
                // precedence and are evaluated left-to-right.  So the
                // original FOCAL subexpression `M * G / Z * K` can't be
                // copied as-is into C: `Z * K` has to be parenthesized to
                // get the same result.
                let W = (1.0 - M * G / (Z * K)) / 2.0;
                S = M * V / (Z * K * (W + (W * W + V / Z).sqrt())) + 0.5;
                apply_thrust();
                if I <= 0.0 {
                    loop_until_on_the_moon();
                    return;
                }
                update_lander_state();
                if -J < 0.0 || V <= 0.0 {
                    turn_loop();
                    return;
                }
            }
        }

        update_lander_state();
    }
}

// 07.10 in original FOCAL code
unsafe fn loop_until_on_the_moon() {
    while S >= 0.005 {
        let d = V + (V * V + 2.0 * A * (G - Z * K / M)).sqrt();
        S = 2.0 * A / d;
        apply_thrust();
        update_lander_state();
    }
    on_the_moon();
}

// 04.10 in original FOCAL code
unsafe fn fuel_out() {
    println!("FUEL OUT AT {:8.2} SECS", L);
    S = ((V * V + 2.0 * A * G).sqrt() - V) / G;
    V += G * S;
    L += S;

    on_the_moon();
    return;
}

// 05.10 in original FOCAL code
unsafe fn on_the_moon() {
    // 260
    let w = 3600.0 * V;
    println!("ON THE MOON AT {:.2} SECS", L);
    println!("IMPACT VELOCITY OF {:.2} M.P.H.", w);
    println!("FUEL LEFT: {:.2} LBS", M - N);

    // 270
    if w <= 1.2 {
        println!("PERFECT LANDING!");
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
            "IN FACT YOU BLASTED A NEW LUNAR CRATER {:.2} FT. DEEP",
            w * 0.277_777
        );
    }
    // fall out to unwind and exit play_game()
}

// Subroutine at line 06.10 in original FOCAL code
unsafe fn update_lander_state() {
    L += S;
    T -= S;
    M -= S * K;
    A = I;
    V = J;
}

// Subroutine at line 09.10 in original FOCAL code
unsafe fn apply_thrust() {
    let q = S * K / M;

    let q_2 = q.powi(2);
    let q_3 = q.powi(3);
    let q_4 = q.powi(4);
    let q_5 = q.powi(5);

    J = V + G * S + Z * (-q - q * q / 2.0 - q_3 / 3.0 - q_4 / 4.0 - q_5 / 5.0);
    I = A - G * S * S / 2.0 - V * S
        + Z * S * (q / 2.0 + q_2 / 6.0 + q_3 / 12.0 + q_4 / 20.0 + q_5 / 30.0);
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
