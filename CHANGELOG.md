# 1.0.0
Don't expose SpiDev rate anymore. There is now a nice wrapper struct called `WS28xxAdapter`.
New minimal code:

```
//! Example that definitely works on Raspberry Pi.
//! Make sure you have "SPI" on your Pi enabled and that MOSI-Pin is connected
//! with DIN-Pin. You just need DIN pin, no clock. WS2818 uses one-wire-protocol.
//! See the specification for details

use ws2818_rgb_led_spi_driver::encoding::{encode_rgb};
use ws2818_rgb_led_spi_driver::adapter::WS28xxAdapter;

fn main() {
    println!("make sure you have \"SPI\" on your Pi enabled and that MOSI-Pin is connected with DIN-Pin!");
    let mut adapter = WS28xxAdapter::new("/dev/spidev0.0").unwrap();

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
```

# 0.1.5
Updated README.

# 0.1.4
- Crate re-exports `spidev::Spidev` via `ws2818_rgb_led_spi_driver::Spidev`.
- added `crate::encoding::encode_rgb_slice(data: &[(u8, u8, u8)]) -> Vec<u8>`
