//! Translation of
//! <http://www.cs.brandeis.edu/~storer/LunarLander/LunarLander/LunarLanderListing.jpg>
//! by Jim Storer from FOCAL to Rust.

extern crate read_input;
use read_input::prelude::*; // imports input()

use lunar::lander::Lander;
use lunar::stdio::StdIO;

use std::env;

static mut ECHO_INPUT: bool = false;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        // If --echo is present, then write all input back to standard output.
        // (This is useful for testing with files as redirected input.)
        // TODO: implement this in StdIO.
        if args[1] == "--echo" {
            unsafe {
                ECHO_INPUT = true;
            }
        }
    }

    let mut io = StdIO::default();

    io.greeting();

    let mut lander = Lander::default();
    loop {
        lander.play_game(&mut io);

        if !play_again() {
            break;
        }
    }

    io.farewell();
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
