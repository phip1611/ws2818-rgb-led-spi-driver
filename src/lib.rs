//! Exports all sub modules of the WS28xx SPI driver lib.

#![no_std]

#[macro_use]
extern crate alloc;

#[cfg(feature = "adapter_spi")]
pub mod adapter_spi; // needs [std]
pub mod adapter_gen; // [no_std]
pub mod timings;
pub mod encoding;
