use std::time::{Instant, Duration};
use std::ops::Add;

#[inline(always)]
pub fn sleep_busy_waiting(ms: u64) {
    let target_time = Instant::now().add(Duration::from_millis(ms));
    loop {
        if Instant::now() >= target_time { break; }
    }
}

/// Returns n from args or default.
pub fn get_led_num_from_args() -> usize {
    println!(
        "You can provide the number of LEDs as argument when calling from command line.\
        For example \"cargo run --bin moving_pixel 64\". The default is 64."
    );
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        let leds = args[0].parse::<usize>();
        if let Result::Ok(leds) = leds {
            println!("Using {} LEDs", leds);
            return leds;
        }
    }

    // Default
    println!("Using 64 LEDs");
    64
}

/// Returns n for n x n matrix from args or default.
pub fn get_led_square_dim_from_args() -> usize {
    println!(
        "You can provide the number of LEDs as argument when calling from command line.\
        For example \"cargo run --bin nxn-colored-display 8\" means 8x8=64 LEDs."
    );
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        let leds = args[0].parse::<usize>();
        if let Result::Ok(leds) = leds {
            println!("Using {}x{} LEDs", leds, leds);
            return leds;
        }
    }

    // Default
    println!("Using 8x8 LEDs");
    8
}
