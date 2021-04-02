//! Module with all code + comments related to timing. The WS2818 has specific restrictions
//! how long high and low signal must be send on DIN-wire in nanoseconds. All logic and constants
//! needed to cope with this are placed here.
//!
//! Please note that we have to cope with high frequencies which can be really tricky.
//! Perhaps you need other timings on your device. This was tested on a Raspberry Pi with
//! its SPI device.
//!
//! See device specification for further details.

/// The frequency for the SPI device that should be used. While this was developed I focused
/// on Raspberry Pi. Works on other Linux systems with SPI device probably too if they have
/// a similar frequency. Otherwise you may need to change the values in `encoding.rs`.
pub const PI_SPI_HZ: u32 = 15_600_000;
// 15.6 Mhz, see https://www.raspberrypi.org/documentation/hardware/raspberrypi/spi/README.md

// this means 1 / 15_600_000 * 1E9 ns/cycle => 64ns / cycle => 15.6 MBit/s
//
// See data sheet: https://cdn-shop.adafruit.com/datasheets/WS2812.pdf
//
// Timings of WS2818:
//
// pub const _T0H_NS: u64 = 350; // ±150ns tolerance
// pub const _T0L_NS: u64 = 800; // ±150ns tolerance
// pub const _T1H_NS: u64 = 700; // ±150ns tolerance
// pub const _T1L_NS: u64 = 600; // ±150ns tolerance
// pub const _TRESET: u64 = 50_000; // >50 µs
//
// One Wire Protocol on WS2812 requires the
// - "logical 0 Bit" to be:
//   - T0H_NS ±150ns to be high
//   - T0L_NS ±150ns to be low     (most of the time; at the end)
// - "logical 1 Bit" to be:
//   - T1H_NS ±150ns to be high    (most of the time; at the beginning)
//   - T1L_NS ±150ns to be low
//
// T0H_NS = 350ns ± 150ns => 1_1111          ( 5 bits * 64ns per bit ~ 320ns)
// T0L_NS = 800ns ± 150ns => 000_0000_0000   (11 bits * 64ns per bit ~ 704ns)
//
// T1H_NS = 700ns ± 150ns => 1_1111_1111    (9 bits * 64ns per bit ~ 576ns)
// T1L_NS = 600ns ± 150ns => 000_0000        (7 bits * 64ns per bit ~ 448ns)
//
// => !! we encode one data bit in two SPI byte for the proper timings !!

/// Timing-encoding specific constants. Actual encoding functions should be
/// inside `crate::encoding`!
pub mod encoding {
    /// How many SPI bytes must be send for a single data bit.
    /// This number of bytes result in one logical zero or one
    /// on WS2818 LED.
    pub const SPI_BYTES_PER_DATA_BIT: usize = 2;

    /// See code comments above where this value comes from!
    /// These are the bits to send via SPI MOSI that represent a logical 0
    /// on WS2812 RGB LED interface. Frequency + length results in the proper timings.
    pub const WS2812_LOGICAL_ZERO_BYTES: [u8; SPI_BYTES_PER_DATA_BIT] = [0b1111_1000, 0b0000_0000];

    /// See code comments above where this value comes from!
    /// These are the bits to send via SPI MOSI that represent a logical 1
    /// on WS2812 RGB LED interface. Frequency + length results in the proper timings.
    pub const WS2812_LOGICAL_ONE_BYTES: [u8; SPI_BYTES_PER_DATA_BIT] = [0b1111_1111, 0b1000_0000];
}
