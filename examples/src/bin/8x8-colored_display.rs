//! Example that definitely works on Raspberry Pi. I used a 8x8 RGB LED matrix.
//! Make sure to have "SPI" on your Pi enabled and that MOSI-Pin is connected
//! with DIN-Pin. You just need DIN pin, no clock. WS2818 uses one-wire-protocol.
//! See the specification for details

use std::io::Write;

use ws2818_examples::sleep_busy_waiting;
use ws2818_rgb_led_spi_driver::encoding::{encode_rgb, encode_rgb_vec};
use ws2818_rgb_led_spi_driver::util::clear_leds;
use std::f64::consts::PI;

const COLS: usize = 8;
const ROWS: usize = 8;

const LEDS: usize = COLS * ROWS;
const LEDS_F64: f64 = LEDS as f64;

// Some colored animation on a 8x8 led matrix.
fn main() {
    println!("Make sure to have \"SPI\" on your Pi enabled and that MOSI-Pin is connected with DIN-Pin!");
    let mut spi = ws2818_rgb_led_spi_driver::setup_spi("/dev/spidev0.0").unwrap();

    let mut rgb_data = vec![];
    for row in 0..ROWS {
        for col in 0..COLS {
            // red is on the top right
            // green is on the bottom right
            // blue is on the bottom left

            // calc red
            let (r_row, r_col) = ((ROWS - row - 1) as f64, col as f64);
            let r = 255_f64 * ((r_row + 1_f64) * (r_col + 1_f64))/(LEDS_F64);
            let r = r.round() as u8;

            // calc green
            let (g_row, g_col) = (row as f64, col as f64);
            let g = 255_f64 * ((g_row + 1_f64) * (g_col + 1_f64))/(LEDS_F64);
            let g = g.round() as u8;

            // calc blue
            let (b_row, b_col) = (row as f64, (COLS - col - 1) as f64);
            let b = 255_f64 * ((b_row + 1_f64) * (b_col + 1_f64))/(LEDS_F64);
            let b = b.round() as u8;

            //rgb_data.push((r, g, b));
            rgb_data.push((r, g, b));
        }
    }

    let spi_bytes = encode_rgb_vec(&rgb_data);
    spi.write_all(&spi_bytes);

    sleep_busy_waiting(1000);

    // now let's rotate the bits

    let mut rgb_data_current = Vec::new();
    rgb_data_current.extend_from_slice(&rgb_data);
    loop {
        for row in 0..ROWS {
            for col in 0..COLS {
                let led_i = row * COLS + col;
                if led_i == 0 {
                    // in very first iteration
                    let curr = (&rgb_data_current[led_i]).clone();
                    let next = (&rgb_data_current[led_i + 1]).clone();
                    std::mem::replace(&mut rgb_data_current[led_i], next);
                    std::mem::replace(&mut rgb_data_current[LEDS - 1], curr);
                }
                else if led_i + 1 < LEDS {
                    let next = (&rgb_data_current[led_i + 1]).clone();
                    std::mem::replace(&mut rgb_data_current[led_i], next);
                }
            }
        }
        let mut rgb_data_current_encoded =  encode_rgb_vec(&rgb_data_current);
        spi.write_all(&rgb_data_current_encoded);
        sleep_busy_waiting(50);
    }

}
