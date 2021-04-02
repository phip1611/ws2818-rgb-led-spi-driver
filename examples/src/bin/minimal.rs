//! Example that definitely works on Raspberry Pi.
//! Make sure you have "SPI" on your Pi enabled and that MOSI-Pin is connected
//! with DIN-Pin. You just need DIN pin, no clock. WS2818 uses one-wire-protocol.
//! See the specification for details

use ws2818_rgb_led_spi_driver::adapter_gen::WS28xxAdapter;
use ws2818_rgb_led_spi_driver::adapter_spi::WS28xxSpiAdapter;
use ws2818_rgb_led_spi_driver::encoding::encode_rgb;

fn main() {
    println!("make sure you have \"SPI\" on your Pi enabled and that MOSI-Pin is connected with DIN-Pin!");
    let mut adapter = WS28xxSpiAdapter::new("/dev/spidev0.0").unwrap();

    // Method 1: encode first and write in two step (prefered way; better performance)
    {
        let mut spi_encoded_rgb_bits = vec![];
        // set first three pixels to bright red, bright green and bright blue
        spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(255, 0, 0));
        spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(0, 255, 0));
        spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(0, 0, 255));
        adapter.write_encoded_rgb(&spi_encoded_rgb_bits).unwrap();
    }

    // Method 2: encode and write in one step
    {
        let mut rgb_values = vec![];
        // set first three pixels to bright red, bright green and bright blue
        rgb_values.push((255, 0, 0));
        rgb_values.push((0, 255, 0));
        rgb_values.push((0, 0, 255));
        adapter.write_rgb(&rgb_values).unwrap();
    }
}
