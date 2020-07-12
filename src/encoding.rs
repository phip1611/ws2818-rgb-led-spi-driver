//! Utility functions for the encoding of data. This functions respect the restrictions from `timings`-module.

use crate::timings::encoding::{SPI_BYTES_PER_DATA_BIT, WS2812_LOGICAL_ZERO_BYTES, WS2812_LOGICAL_ONE_BYTES};

const COLORS: usize = 3; // r, g, b
const BITS_PER_COLOR: usize = 8; // 0 to 255

/// The number of bytes that must be send over SPI to transfer the data of a single RGB pixel.
pub const SPI_BYTES_PER_RGB_PIXEL: usize = COLORS * BITS_PER_COLOR * SPI_BYTES_PER_DATA_BIT;

/// Encodes RGB-Values to the bytes that must be transferred via SPI MOSI.
/// These SPI bytes represent the logical zeros and ones for WS2818.
/// This counts in the constraints that come from `crate::timings`-module.
/// Due to the specification the data is send this way:
/// G7..G0,R7..R0,B7..B0
///
/// The resulting is `BYTES_PER_RGB_PIXEL` bytes long.
pub fn encode_rgb(r: u8, g: u8, b: u8) -> [u8; SPI_BYTES_PER_RGB_PIXEL] {
    let mut spi_bytes: [u8; SPI_BYTES_PER_RGB_PIXEL] = [0; SPI_BYTES_PER_RGB_PIXEL];
    let mut spi_bytes_i = 0;
    let grb = [g, r, b]; // order specified by specification
    for color in 0..COLORS {
        let mut color_bits = grb[color];
        for _ in 0..8 {
            // for each bit of our color; starting with most significant
            // we encode now one color bit in two spi bytes (for proper timings along with our frequency)
            if 0b10000000 & color_bits == 0 {
                spi_bytes[spi_bytes_i]     = WS2812_LOGICAL_ZERO_BYTES[0];
                spi_bytes[spi_bytes_i + 1] = WS2812_LOGICAL_ZERO_BYTES[1];
            } else {
                spi_bytes[spi_bytes_i]     = WS2812_LOGICAL_ONE_BYTES[0];
                spi_bytes[spi_bytes_i + 1] = WS2812_LOGICAL_ONE_BYTES[1];
            }
            color_bits = color_bits << 1;
            spi_bytes_i += 2; // update array index;
        }
    }
    debug_assert_eq!(spi_bytes_i, SPI_BYTES_PER_RGB_PIXEL);
    spi_bytes
}

/// Encodes multiple RGB values in a vector. Uses `encode_rgb()` for each value.
pub fn encode_rgb_vec(data: &Vec<(u8, u8, u8)>) -> Vec<u8> {
    encode_rgb_slice(&data[..])
}

/// Encodes multiple RGB values in a slice. Uses `encode_rgb()` for each value.
pub fn encode_rgb_slice(data: &[(u8, u8, u8)]) -> Vec<u8> {
    let mut bytes = vec![];
    data.iter().for_each(|rgb| {
        bytes.extend_from_slice(&encode_rgb(rgb.0, rgb.1, rgb.2))
    });
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_rgb_vec() {
        let e_1 = encode_rgb(5, 5, 5);
        let e_2 = encode_rgb(50, 50, 50);
        let mut e_12 = vec![];
        e_1.iter().for_each(|bits| e_12.push(*bits));
        e_2.iter().for_each(|bits| e_12.push(*bits));

        let vec = vec![(5, 5, 5), (50, 50, 50)];
        let actual = encode_rgb_vec(&vec);

        assert_eq!(e_12.len(), actual.len());
        for i in 0..e_12.len() {
         assert_eq!(e_12[i], actual[i])
        }
    }

}
