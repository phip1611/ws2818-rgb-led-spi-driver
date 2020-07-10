//! Example that definitely works on Raspberry Pi. I used a 8x8 RGB LED matrix.
//! Make sure to have "SPI" on your Pi enabled and that MOSI-Pin is connected
//! with DIN-Pin. You just need DIN pin, no clock. WS2818 uses one-wire-protocol.
//! See the specification for details

use std::io::Write;

use ws2818_examples::sleep_busy_waiting;
use ws2818_rgb_led_spi_driver::encoding::{encode_rgb, encode_rgb_vec};
use ws2818_rgb_led_spi_driver::util::clear_leds;
use std::f64::consts::PI;
use std::path::Component::RootDir;

const COLS: usize = 8;
const ROWS: usize = 8;

// This example just clears the display. Useful if you don't want to unplug power all the time.
fn main() {
    println!("Make sure to have \"SPI\" on your Pi enabled and that MOSI-Pin is connected with DIN-Pin!");
    let mut spi = ws2818_rgb_led_spi_driver::setup_spi("/dev/spidev0.0").unwrap();

    clear_leds(&mut spi, COLS * ROWS);
}
