//! Simple, stripped down, educational, no_std-compatible driver for WS28XX (WS2811/12) RGB LEDs.
//! Uses SPI device for timing/clock, and works definitely on Linux/Raspberry Pi.
//!
//! The main goal of this crate was to work on Raspberry Pi. As the title says, this is bound
//! to `spi`-device for sending data. But you can use it also on other systems, i.e. embedded systems
//! (no_std-environments) but in these cases you must provide an own `encoding.rs` file if the
//! refresh rate doesn't match the value in [`timings::PI_SPI_HZ`].

#![no_std]

#[cfg(feature = "adapter_spidev")]
extern crate std;

#[macro_use]
extern crate alloc;

pub mod adapter_gen; // generic [no_std] hardware abstraction
#[cfg(feature = "adapter_spidev")]
pub mod adapter_spi; // specific [std]-implementation

// bound to Raspberry Pi SPI device but you can easily provide your own
// timings.
pub mod timings;
// bound to Raspberry Pi SPI device but you can easily provide your own
// encoding functions.
pub mod encoding;
