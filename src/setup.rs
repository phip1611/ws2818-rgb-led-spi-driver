use std::io;
use spidev::{Spidev, SpidevOptions, SpiModeFlags};
use crate::timings::PI_SPI_HZ;

/// Connects your application with the SPI-device of your device.
/// This uses the `spidev`-crate.
///
/// * `dev` - Device name. Probably "/dev/spidev0.0" if available.
pub fn setup_spi(dev: &str) -> io::Result<Spidev> {
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
