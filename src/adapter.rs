//! Adapter

use std::io;
use spidev::{SpidevOptions, SpiModeFlags, Spidev};
use crate::timings::PI_SPI_HZ;
use crate::encoding::encode_rgb_slice;
use std::io::Write;

/// Adapter that connects your application via SPI to your WS28xx LEDs.
pub struct WS28xxAdapter {
    spi: Spidev,
}

impl WS28xxAdapter {

    /// Connects your application with the SPI-device of your device.
    /// This uses the `spidev`-crate. Returns a new adapter object
    /// for the WS28xx LEDs.
    ///
    /// * `dev` - Device name. Probably "/dev/spidev0.0" if available.
    ///
    /// Fails if connection to SPI can't be established.
    pub fn new(dev: &str) -> io::Result<Self> {
        let spi = WS28xxAdapter::setup_spi(dev);
        spi.map(|spi| Self {
            spi
        })
    }

    /// Encodes RGB values and write them via SPI to the LEDs. The length of the vector
    /// is the number of LEDs you want to write to. *Note* that if you have performance critical
    /// applications (like you need a signal on the LEDS on a given time) it's a better idea
    /// to encode the data earlier by yourself using `crate::encoding`-module and calling
    /// `WS28xxAdapter::write_encoded_rgb`. Otherwise and if your device is slow the encoding
    /// could cost a few milliseconds - depending on your amount of data.
    pub fn write_rgb(&mut self, rgb_data: &[(u8, u8, u8)]) -> Result<(), String> {
        let encoded_data = encode_rgb_slice(rgb_data);
        self.write_encoded_rgb(&encoded_data)
    }

    /// Clears all LEDs. Sets each to (0, 0, 0).
    pub fn clear(&mut self, num_leds: usize) {
        let data = vec![(0, 0, 0); num_leds];
        self.write_rgb(&data).unwrap();
    }

    /// Directly writes encoded RGB values via SPI to the LEDs. This method and the encoded data
    /// must fulfill the restrictions given by `crate::timings` and `crate::encoding`.
    pub fn write_encoded_rgb(&mut self, encoded_data: &[u8]) -> Result<(), String> {
        self.spi.write_all(&encoded_data)
            .map_err(|_| {
                format!(
                    "Failed to send {} bytes via SPI. Perhaps your SPI buffer is too small!\
                     Check https://www.raspberrypi.org/forums/viewtopic.php?p=309582#p309582 for example.",
                    encoded_data.len()
                )}
            )
    }

    /// Sets up SPI.
    fn setup_spi(dev: &str) -> io::Result<Spidev> {
        let mut spi = Spidev::open(dev)?;
        let options = SpidevOptions::new()
            .bits_per_word(8)
            // According to https://www.raspberrypi.org/documentation/hardware/raspberrypi/spi/README.md
            .max_speed_hz(PI_SPI_HZ)
            .mode(SpiModeFlags::SPI_MODE_0)
            .build();
        spi.configure(&options)?;
        Ok(spi)
    }
}
