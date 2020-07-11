//! Example that definitely works on Raspberry Pi. I used a 8x8 RGB LED matrix.
//! Make sure to have "SPI" on your Pi enabled and that MOSI-Pin is connected
//! with DIN-Pin. You just need DIN pin, no clock. WS2818 uses one-wire-protocol.
//! See the specification for details

use std::io::Write;
use ws2818_rgb_led_spi_driver::encoding::{encode_rgb};

fn main() {
    println!("Make sure to have \"SPI\" on your Pi enabled and that MOSI-Pin is connected with DIN-Pin!");
    let mut spi = ws2818_rgb_led_spi_driver::setup_spi("/dev/spidev0.0").unwrap();

    let mut spi_bits = vec![];
    // set first three pixels to bright red, bright green and bright blue
    spi_bits.extend_from_slice(&encode_rgb(255, 0, 0));
    spi_bits.extend_from_slice(&encode_rgb(0, 255, 0));
    spi_bits.extend_from_slice(&encode_rgb(0, 0, 255));
    spi.write_all(&spi_bits).unwrap();
}
