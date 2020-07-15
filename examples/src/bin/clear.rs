//! Example that definitely works on Raspberry Pi. I used a 8x8 RGB LED matrix.
//! make sure you have "SPI" on your Pi enabled and that MOSI-Pin is connected
//! with DIN-Pin. You just need DIN pin, no clock. WS2818 uses one-wire-protocol.
//! See the specification for details

use ws2818_examples::{get_led_num_from_args};
use ws2818_rgb_led_spi_driver::adapter::WS28xxAdapter;

// This example just clears the display. Useful if you don't want to unplug power all the time.
fn main() {
    println!("make sure you have \"SPI\" on your Pi enabled and that MOSI-Pin is connected with DIN-Pin!");
    let mut adapter = WS28xxAdapter::new("/dev/spidev0.0").unwrap();

    let num_leds = get_led_num_from_args();
    adapter.clear(num_leds);
}
