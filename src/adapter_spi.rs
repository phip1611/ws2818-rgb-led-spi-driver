//! Adapter

use std::io;
use spidev::{SpidevOptions, SpiModeFlags, Spidev};
use crate::timings::PI_SPI_HZ;
use crate::encoding::encode_rgb_slice;
use std::io::Write;
use alloc::string::{String, ToString};
use alloc::fmt::format;
use crate::adapter_gen::{HardwareDev, WS28xxGenAdapter, WS28xxAdapter};
use alloc::boxed::Box;

struct SpiHwAdapterDev(Spidev);

impl HardwareDev for SpiHwAdapterDev {
    fn write_all(&mut self, encoded_data: &[u8]) -> Result<(), String> {
        self.0.write_all(&encoded_data)
            .map_err(|_| {
                format!(
                    "Failed to send {} bytes via SPI. Perhaps your SPI buffer is too small!\
                     Check https://www.raspberrypi.org/forums/viewtopic.php?p=309582#p309582 for example.",
                    encoded_data.len()
                )
            })
    }
}

impl SpiHwAdapterDev {
    /// Connects your application with the SPI-device of your device.
    /// This uses the `spidev`-crate. Returns a new adapter object
    /// for the WS28xx LEDs.
    ///
    /// * `dev` - Device name. Probably "/dev/spidev0.0" if available.
    ///
    /// Fails if connection to SPI can't be established.
    pub fn new(dev: &str) -> io::Result<Self> {
        let mut spi = Spidev::open(dev)?;
        let options = SpidevOptions::new()
            .bits_per_word(8)
            // According to https://www.raspberrypi.org/documentation/hardware/raspberrypi/spi/README.md
            .max_speed_hz(PI_SPI_HZ)
            .mode(SpiModeFlags::SPI_MODE_0)
            .build();
        spi.configure(&options)?;
        spi.map(|spi| Self(spi))
    }
}

/// Adapter that connects your application via SPI to your WS28xx LEDs.
/// This requires an SPI device on your machine. This doesn't work
/// with `#[no-std]`.
pub struct WS28xxSpiAdapter {
    gen: WS28xxGenAdapter,
}

impl WS28xxSpiAdapter {
    /// Connects your application with the SPI-device of your device.
    /// This uses the `spidev`-crate. Returns a new adapter object
    /// for the WS28xx LEDs.
    ///
    /// * `dev` - Device name. Probably "/dev/spidev0.0" if available.
    ///
    /// Fails if connection to SPI can't be established.
    pub fn new(dev: &str) -> Result<Self, String> {
        let mut spi = SpiHwAdapterDev::new(dev)
            .map_err(|err| err.to_string())?;
        let spi = Box::from(spi);
        let gen = WS28xxGenAdapter::new(spi);
        Ok(Self {
            gen
        })
    }
}

impl WS28xxAdapter for WS28xxSpiAdapter {
    fn get_hw_dev(&mut self) -> &mut Box<dyn HardwareDev> {
        // forward to generic adapter
        // todo this is not the best code design because this requires
        //  each sub adapter (like a sub class in OOP) to implement
        //  this manually..
        self.gen.get_hw_dev()
    }
}
