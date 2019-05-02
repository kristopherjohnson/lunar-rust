extern crate read_input;
use read_input::prelude::*; // input()

fn main() {
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

unsafe fn trace(_location: &str) {
    /*
    println!(
        "<{}: A={} G={} I={} J={} K={} L={} M={} N={} S={} T={} V={} Z={}>",
        location, A, G, I, J, K, L, M, N, S, T, V, Z
    );
    */
}

/// Run the simulation until the lander is on the moon.
unsafe fn play_game() {
    trace("play_game");

    // 30 - 110
    println!("CONTROL CALLING LUNAR MODULE. MANUAL CONTROL IS NECESSARY");
    println!("YOU MAY RESET FUEL RATE K EACH 10 SECS TO 0 OR ANY VALUE");
    println!("BETWEEN 8 & 200 LBS/SEC. YOU'VE 16000 LBS FUEL. ESTIMATED");
    println!("FREE FALL IMPACT TIME=120 SECS. CAPSULE WEIGHT=32500 LBS");
    println!("FIRST RADAR CHECK COMING UP");
    println!("\n\nCOMMENCE LANDING PROCEDURE");

    // 120
    L = 0.0;

    // 130
    println!("TIME,SECS   ALTITUDE,MILES+FEET   VELOCITY,MPH   FUEL,LBS   FUEL RATE");

    // 140
    A = 120.0;
    V = 1.0;
    M = 32500.0;
    N = 16500.0;
    G = 0.001;
    Z = 1.8;
    S = 0.0;

    goto_150();
}

unsafe fn goto_150() {
    trace("goto_150");

    // 150
    print!(
        "{:8}{:15}{:7}{:15.2}{:12.1}      K=",
        L.round(),
        A.trunc(),
        (5280.0 * (A - A.trunc())).trunc(),
        3600.0 * V,
        M - N
    );

    K = input()
        .msg(":")
        .inside(0.0..=200.0)
        .err("ENTER A VALUE FOR K BETWEEN 0 AND 200 LBS/SEC")
        .get();

    T = 10.0;

    goto_160();
}

unsafe fn goto_160() {
    trace("goto_160");

    // 160
    if M - N < 0.001 {
        // 240
        println!("FUEL OUT AT {:.2} SECS", L);
        S = (-V + (V * V + 2.0 * A * G).sqrt()) / G;

        // 250
        V = V + G * S;
        L = L + S;

        goto_260();
        return;
    }

    // 170
    if T < 0.001 {
        goto_150();
        return;
    }

    // 180
    S = T;
    if M >= N + S * K {
        goto_200();
        return;
    }

    // 190
    S = (M - N) / K;
    goto_200();
}

unsafe fn goto_200() {
    trace("goto_200");

    // 200
    gosub_420();
    if I <= 0.0 {
        goto_340();
    }
    // 210
    else if V <= 0.0 {
        goto_230();
    }
    // 220
    else if J <= 0.0 {
        loop {
            // 370
            let w = (1.0 - M * G / (Z * K)) / 2.0;
            S = M * V / (Z * K * (w + (w * w + V / Z).sqrt())) + 0.05;
            gosub_420();

            // 380
            if I <= 0.0 {
                goto_340();
                return;
            }

            // 390
            gosub_330();
            if J > 0.0 {
                goto_160();
                return;
            }
            // 400
            else if V > 0.0 {
                continue;
            }
            break;
        }
        // 410
        goto_160();
    } else {
        goto_230();
    }
}

unsafe fn goto_230() {
    trace("goto_230");

    // 230
    gosub_330();
    goto_160();
}

unsafe fn goto_260() {
    trace("goto_260");

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
            w * 0.277777
        );
    }

    // 440
    println!("\n\n");

    // fall out to unwind and exit play_game()
}

unsafe fn gosub_330() {
    trace("gosub_330");

    // 330
    L = L + S;
    T = T - S;
    M = M - S * K;
    A = I;
    V = J;
}

unsafe fn goto_340() {
    trace("goto_340");

    loop {
        // 340
        if S < 0.005 {
            goto_260();
            return;
        }

        // 350
        let d = V + (V * V + 2.0 * A * (G - Z * K / M)).sqrt();
        S = 2.0 * A / d;

        // 360
        gosub_420();
        gosub_330();
    }
}

unsafe fn gosub_420() {
    trace("gosub_420");

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
        .msg("TRY AGAIN?\n(ANS. YES OR NO):")
        .add_err_test(
            |value| {
                let value = value.to_ascii_uppercase();
                value.starts_with("Y") || value.starts_with("N")
            },
            "(ANS. YES OR NO):",
        )
        .get();
    response.to_ascii_uppercase().starts_with("Y")
}
