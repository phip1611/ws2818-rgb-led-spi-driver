//! Example that definitely works on Raspberry Pi. I used a 8x8 RGB LED matrix.
//! Make sure to have "SPI" on your Pi enabled and that MOSI-Pin is connected
//! with DIN-Pin. You just need DIN pin, no clock. WS2818 uses one-wire-protocol.
//! See the specification for details

use std::io::Write;

use ws2818_examples::sleep_busy_waiting;
use ws2818_rgb_led_spi_driver::encoding::encode_rgb;

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

