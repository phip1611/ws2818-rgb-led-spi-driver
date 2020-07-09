//! Utility functions for the encoding of data. This functions respect the restrictions from `timings`-module.

use crate::timings::encoding::{SPI_BYTES_PER_DATA_BIT, WS2812_LOGICAL_ZERO_BYTES, WS2812_LOGICAL_ONE_BYTES};

const COLORS: usize = 3; // r, g, b
const BITS_PER_COLOR: usize = 8; // 0 to 255

/// The number of bytes that must be send over SPI to transfer the data of a single RGB pixel.
pub const SPI_BYTES_PER_RGB_PIXEL: usize = COLORS * BITS_PER_COLOR * SPI_BYTES_PER_DATA_BIT;

/// Encodes RGB-Values to the correct bit representation for the WS2818.
/// This counts in the constraints that come from `crate::timings`-module.
/// Due to the specification the data is send this way:
/// G7..G0,R7..R0,B7..B0
///
/// The resulting is `BYTES_PER_RGB_PIXEL` bytes long.
pub fn encode_rgb(mut r: u8, mut g: u8, mut b: u8) -> [u8; SPI_BYTES_PER_RGB_PIXEL] {
    let mut spi_bytes: [u8; SPI_BYTES_PER_RGB_PIXEL] = [0; SPI_BYTES_PER_RGB_PIXEL];
    let mut spi_bytes_i = 0;
    let grb = [g, r, b]; // order specified by specification
    for color in 0..COLORS {
        let mut color_bits = grb[color];
        for _ in 0..8 {
            // for each bit of our color; starting with most significant
            // we encode now one color bit in two spi bytes (for proper timings along with our frequency)
            if 0b10000000 & color_bits == 0 {
                spi_bytes[i]     = WS2812_LOGICAL_ZERO_BYTES[0];
                spi_bytes[i + 1] = WS2812_LOGICAL_ZERO_BYTES[1];
            } else {
                spi_bytes[i]     = WS2812_LOGICAL_ONE_BYTES[0];
                spi_bytes[i + 1] = WS2812_LOGICAL_ONE_BYTES[1];
            }
            color_bits = color_bits << 1;
            spi_bytes_i += 2; // update array index;
        }
    }
    debug_assert!(spi_bytes_i + 1, SPI_BYTES_PER_RGB_PIXEL);
    spi_bytes
}