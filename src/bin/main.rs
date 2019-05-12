//! Translation of
//! <http://www.cs.brandeis.edu/~storer/LunarLander/LunarLander/LunarLanderListing.jpg>
//! by Jim Storer from FOCAL to Rust.

use lunar::lander::Lander;
use lunar::stdio::StdIO;

use std::env;

fn main() {
    let mut io = StdIO::default();

    let args = env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        // If --echo is present, then write all input back to standard output.
        // (This is useful for testing with files as redirected input.)
        if args[1] == "--echo" {
            eprintln!("ECHO");
            io.echo_input = true;
        } else {
            eprintln!("NO ECHO");
        }
    }

    io.greeting();

    let mut lander = Lander::default();
    loop {
        lander.play_game(&mut io);

        if !play_again(&io) {
            break;
        }
    }

    io.farewell();
}

/// Asks whether user wants to play again.
///
/// If user does not respond with something beginning with "Y" or "N", then
/// repeats the prompt until the user provides a valid response.
fn play_again(io: &StdIO) -> bool {
    println!("\n\n\n\nTRY AGAIN?");
    loop {
        print!("(ANS. YES OR NO):");
        match io.accept_line() {
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
