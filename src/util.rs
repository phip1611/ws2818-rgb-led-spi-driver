//! Utility functions for writing stuff to the LED chain. This uses functions from the
//! `crate::encoding` modulel

use spidev::Spidev;
use std::io::Write;
use crate::encoding::{encode_rgb, encode_rgb_vec};

pub fn clear_leds(spi: &mut Spidev, num_leds: usize) {
    let data = vec![(0, 0, 0); num_leds];
    let data = encode_rgb_vec(&data);
    spi.write_all(&data);
}
