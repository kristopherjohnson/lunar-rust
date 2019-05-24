use lunar::io::{Score, IO};
use lunar::lander::Lander;

macro_rules! assert_close {
    ($left:expr, $right:expr) => {
        assert_eq!(
            ($left * 100.0).round() / 100.0,
            ($right * 100.0).round() / 100.0
        );
    };
}

/// A line of output with current lander status, and the fuel-rate input
struct Line(
    i32, // secs
    i32, // miles
    i32, // feet
    f64, // mph
    f64, // lbs
    i32, // rate
);

#[rustfmt::skip]
impl Line {
    fn elapsed_secs(&self)   -> i32 { self.0 }
    fn miles(&self)          -> i32 { self.1 }
    fn feet(&self)           -> i32 { self.2 }
    fn v_mph(&self)          -> f64 { self.3 }
    fn fuel_remaining(&self) -> f64 { self.4 }
    fn k(&self)              -> i32 { self.5 }

    fn altitude(&self) -> f64 {
        self.miles() as f64 + (self.feet() as f64 / 5280.0)
    }
}

/// Implementation of `lunar::io::IO` used for testing.
struct TestIO<'a> {
    expected_lines: &'a [Line],
    expected_on_moon_secs: f64,
    expected_impact_mph: f64,
    expected_fuel_left: f64,
    line_index: usize,
}

impl<'a> TestIO<'a> {
    /// Creates a new instance of `TestIO`.
    fn new(
        expected_lines: &'a [Line],
        expected_on_moon_secs: f64,
        expected_impact_mph: f64,
        expected_fuel_left: f64,
    ) -> TestIO {
        TestIO {
            expected_lines: expected_lines,
            expected_on_moon_secs: expected_on_moon_secs,
            expected_impact_mph: expected_impact_mph,
            expected_fuel_left: expected_fuel_left,
            line_index: 0,
        }
    }

    /// Returns true if all expected lines have been processed.
    fn is_complete(&self) -> bool {
        assert_eq!(self.line_index, self.expected_lines.len());
        true
    }
}

impl<'a> IO for TestIO<'a> {
    fn start_game(&mut self, _lander: &Lander) {}

    fn get_fuel_rate(&mut self, lander: &Lander) -> f64 {
        let expected = &self.expected_lines[self.line_index];

        assert_eq!(
            expected.elapsed_secs(),
            lander.elapsed_secs().round() as i32
        );
        assert_close!(expected.altitude(), lander.altitude());
        assert_close!(expected.v_mph(), lander.v_mph());
        assert_close!(expected.fuel_remaining(), lander.fuel_remaining());

        self.line_index += 1;

        expected.k() as f64
    }

    fn fuel_out(&mut self, _l: f64) {}

    fn on_the_moon(&mut self, lander: &Lander, _score: Score) {
        assert_close!(self.expected_on_moon_secs, lander.elapsed_secs());
        assert_close!(self.expected_impact_mph, lander.v_mph());
        assert_close!(self.expected_fuel_left, lander.fuel_remaining());
    }
}

#[test]
fn good_landing() {
    #[rustfmt::skip]
    let expected_lines: [Line; 20] = [
        //     TIME,SECS   ALTITUDE,MILES+FEET   VELOCITY,MPH   FUEL,LBS   FUEL RATE
        Line (        0,           120,     0,       3600.00,   16000.0,      0   ),
        Line (       10,           109,  5016,       3636.00,   16000.0,      0   ),
        Line (       20,            99,  4224,       3672.00,   16000.0,      0   ),
        Line (       30,            89,  2904,       3708.00,   16000.0,      0   ),
        Line (       40,            79,  1056,       3744.00,   16000.0,      0   ),
        Line (       50,            68,  3960,       3780.00,   16000.0,      0   ),
        Line (       60,            58,  1056,       3816.00,   16000.0,      0   ),
        Line (       70,            47,  2904,       3852.00,   16000.0,      180 ),
        Line (       80,            37,  1626,       3518.79,   14200.0,      200 ),
        Line (       90,            28,   438,       3118.26,   12200.0,      200 ),
        Line (      100,            20,    71,       2686.18,   10200.0,      200 ),
        Line (      110,            13,  1022,       2217.65,    8200.0,      200 ),
        Line (      120,             7,  3868,       1706.49,    6200.0,      200 ),
        Line (      130,             3,  4011,       1144.83,    4200.0,      200 ),
        Line (      140,             1,  2263,        522.40,    2200.0,      130 ),
        Line (      150,             0,  3000,         91.49,     900.0,      15  ),
        Line (      160,             0,  1805,         71.39,     750.0,      15  ),
        Line (      170,             0,   908,         50.80,     600.0,      15  ),
        Line (      180,             0,   317,         29.70,     450.0,      14  ),
        Line (      190,             0,    11,         11.96,     310.0,      30  ),
    ];
    let expected_on_moon_secs = 190.93;
    let expected_impact_mph = 4.53;
    let expected_fuel_left = 282.04;
    let mut io = TestIO::new(
        &expected_lines,
        expected_on_moon_secs,
        expected_impact_mph,
        expected_fuel_left,
    );

    let mut lander = Lander::default();
    lander.play_game(&mut io);
    assert!(io.is_complete());
}
