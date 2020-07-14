# 0.1.5
Updated README.

# 0.1.4
- Crate re-exports `spidev::Spidev` via `ws2818_rgb_led_spi_driver::Spidev`.
- added `crate::encoding::encode_rgb_slice(data: &[(u8, u8, u8)]) -> Vec<u8>`
