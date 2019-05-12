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
            io.echo_input = true;
        }
    }

    io.greeting();

    let mut lander = Lander::default();
    loop {
        lander.play_game(&mut io);

        if !io.play_again() {
            break;
        }
    }

    io.farewell();
}
