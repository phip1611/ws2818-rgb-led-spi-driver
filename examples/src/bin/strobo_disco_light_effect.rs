//! Example that definitely works on Raspberry Pi.
//! Make sure to have "SPI" on your Pi enabled and that MOSI-Pin is connected
//! with DIN-Pin.

use ws2818_rgb_led_spi_driver::encoding::encode_rgb;
use std::io::Write;
use std::time::{Instant, Duration};
use std::ops::Add;

fn main() {
    println!("Make sure to have \"SPI\" on your Pi enabled and that MOSI-Pin is connected with DIN-Pin!");
    let mut spi = ws2818_rgb_led_spi_driver::setup_spi("/dev/spidev0.0").unwrap();

    let mut white_display_bytes = vec![];
    // strobo effekt
    for _ in 0..64 {
        white_display_bytes.extend_from_slice(&encode_rgb(255, 255, 255));
    }

    let mut empty_display_bytes = vec![];
    for _ in 0..64 {
        empty_display_bytes.extend_from_slice(&encode_rgb(0, 0, 0));
    }


    // note we first aggregate all data and write then all at
    // once! otherwise timings would be impossible to reach
    loop {
        spi.write_all(&white_display_bytes).unwrap();
        sleep_busy_waiting(1);
        spi.write_all(&empty_display_bytes).unwrap();
        sleep_busy_waiting((1000 / 40) - 1);
    }
}

#[inline(always)]
pub fn sleep_busy_waiting(ms: u64) {
    let target_time = Instant::now().add(Duration::from_millis(ms));
    loop {
        if Instant::now() >= target_time { break; }
    }
}
