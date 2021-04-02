use std::ops::Add;
use std::process::exit;
use std::time::{Duration, Instant};

#[inline(always)]
pub fn sleep_busy_waiting_ms(ms: u64) {
    let target_time = Instant::now().add(Duration::from_millis(ms));
    loop {
        if Instant::now() >= target_time {
            break;
        }
    }
}

/// Returns n from args or default.
pub fn get_led_num_from_args() -> usize {
    println!(
        "You can provide the number of LEDs as argument when calling from command line.\
        For example \"cargo run --bin <bin> 64\". The default is 64."
    );
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        let leds = args[1].parse::<usize>();
        if let Result::Ok(leds) = leds {
            println!("Using {} LEDs", leds);
            return leds;
        }
    }

    // Default
    println!("Using 64 LEDs");
    64
}

/// Returns n from args or default.
pub fn get_led_num_and_color_from_args() -> (usize, u8, u8, u8) {
    println!(
        "You can provide the number of LEDs and the color as arguments when calling from command line.\
        For example \"cargo run --bin <bin> 64 255 255 255\"."
    );
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() == 5 {
        let leds = args[1].parse::<usize>().unwrap();
        let r = args[2].parse::<u8>().unwrap();
        let g = args[3].parse::<u8>().unwrap();
        let b = args[4].parse::<u8>().unwrap();
        (leds, r, g, b)
    } else {
        exit(1);
    }
}

/// Returns n for n x n matrix from args or default.
pub fn get_led_square_dim_from_args() -> usize {
    println!(
        "You can provide the number of LEDs as argument when calling from command line.\
        For example \"cargo run --bin <bin> 8\" means 8x8=64 LEDs."
    );
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        let leds = args[1].parse::<usize>();
        if let Result::Ok(leds) = leds {
            println!("Using {}x{} LEDs", leds, leds);
            return leds;
        }
    }

    // Default
    println!("Using 8x8 LEDs");
    8
}

/// Returns a pixel with a random color and a minimal
/// brightness. Tries to get real colors instead of white.
pub fn get_random_pixel_val() -> (u8, u8, u8) {
    const COLORS: [(u8, u8, u8); 28] = [
        // some colors are multiple times listed to increase
        // their probability.
        (255, 255, 255), // white
        (255, 0, 0),     // red
        (255, 0, 0),     // red
        (255, 0, 0),     // red
        (0, 255, 0),     // green
        (0, 0, 255),     // blue
        (13, 255, 248),  // turquoise
        (13, 255, 248),  // turquoise
        (13, 255, 248),  // turquoise
        (255, 168, 0),   // dark orange
        (255, 168, 0),   // dark orange
        (255, 189, 0),   // bright orange
        (255, 189, 0),   // bright orange
        (255, 189, 0),   // bright orange
        (255, 255, 0),   // yellow
        (255, 255, 0),   // yellow
        (255, 255, 0),   // yellow
        (234, 10, 142),  // Telekom Magenta
        (234, 10, 142),  // Telekom Magenta
        (234, 10, 142),  // Telekom Magenta
        (175, 0, 255),   // purple
        (0, 150, 255),   // semi light blue
        (0, 198, 255),   // very light blue
        (0, 198, 255),   // very light blue
        (0, 198, 255),   // very light blue
        (255, 114, 114), // light red
        (255, 114, 114), // light red
        (255, 114, 114), // light red
    ];

    let i = rand::random::<u8>();
    let i = i % COLORS.len() as u8;

    COLORS[i as usize]
}

pub fn darken_rgb(r: u8, g: u8, b: u8, factor: f32) -> (u8, u8, u8) {
    (
        ((r as f32) * factor) as u8,
        ((g as f32) * factor) as u8,
        ((b as f32) * factor) as u8,
    )
}
