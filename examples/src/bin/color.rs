//! Example that definitely works on Raspberry Pi.
//! Make sure you have "SPI" on your Pi enabled and that MOSI-Pin is connected
//! with DIN-Pin. You just need DIN pin, no clock. WS2818 uses one-wire-protocol.
//! See the specification for details

use ws2818_examples::{get_led_num_and_color_from_args};
use ws2818_rgb_led_spi_driver::adapter_spi::WS28xxSpiAdapter;
use ws2818_rgb_led_spi_driver::adapter_gen::WS28xxAdapter;

pub const FREQUENCY: u64 = 20; // 30 Hz
pub const FREQUENCY_MS: u64 = 1000 / FREQUENCY;

// Sets first n pixels to color x. "cargo run 64 255 255 255" (num-leds, red, green, blue)
fn main() {
    println!("Sets first n pixels to color x");
    println!("make sure you have \"SPI\" on your Pi enabled and that MOSI-Pin is connected with DIN-Pin!");
    let mut adapter = WS28xxSpiAdapter::new("/dev/spidev0.0").unwrap();
    let (num_leds, r, g, b) = get_led_num_and_color_from_args();
    let data = vec![(r, g, b); num_leds];
    adapter.write_rgb(&data).unwrap();
}

