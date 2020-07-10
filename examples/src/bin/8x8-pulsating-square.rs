//! Example that definitely works on Raspberry Pi. I used a 8x8 RGB LED matrix.
//! Make sure to have "SPI" on your Pi enabled and that MOSI-Pin is connected
//! with DIN-Pin. You just need DIN pin, no clock. WS2818 uses one-wire-protocol.
//! See the specification for details

use std::io::Write;

use ws2818_examples::sleep_busy_waiting;
use ws2818_rgb_led_spi_driver::encoding::{encode_rgb, encode_rgb_vec};
use ws2818_rgb_led_spi_driver::util::clear_leds;
use std::f64::consts::PI;
use std::path::Component::RootDir;

const DIM: usize = 8;
const BRIGHTNESS_FACTOR: f64 = 0.2;

// This example let a square flow though a quare led matrix.
fn main() {
    println!("Make sure to have \"SPI\" on your Pi enabled and that MOSI-Pin is connected with DIN-Pin!");
    let mut spi = ws2818_rgb_led_spi_driver::setup_spi("/dev/spidev0.0").unwrap();

    let mut reverse_dir = true;
    let half_dim = DIM / 2;
    let (mut r, mut g, mut b) = (255/3, 255/3 * 2, 255);
    loop {
        reverse_dir = !reverse_dir;

        // calc all active pixels for the current size factor!
        for mut size in 0..half_dim + 1 {
            r = (r + 3) % 255;
            g = (g + 5) % 255;
            b = (b + 7) % 255;
            let rgb = (
                (r as f64 * BRIGHTNESS_FACTOR) as u8,
                (g as f64 * BRIGHTNESS_FACTOR) as u8,
                (b as f64 * BRIGHTNESS_FACTOR) as u8,
            );
            if reverse_dir {
                size = half_dim + 1 - 1 -size;
            }

            let mut rgb_matrix = [[(0, 0, 0); DIM]; DIM];
            // useful for debugging
            let mut char_matrix = [[' '; DIM]; DIM];
            if size > 0 {
                let active_leds_per_active_line = 2 * size;

                let top_l_i = 0 + half_dim - size;
                let top_l_j = top_l_i;
                let bottom_r_i = DIM - 1 - half_dim + size;
                let bottom_r_j = bottom_r_i;

                for i in 0..active_leds_per_active_line {
                    // top horizontal lane
                    rgb_matrix[top_l_i][top_l_j + i] = rgb;
                    char_matrix[top_l_i][top_l_j + i] = 'X';
                }
                for i in 0..active_leds_per_active_line {
                    // bottom horizontal lane
                    rgb_matrix[bottom_r_i][top_l_j + i] = rgb;
                    char_matrix[bottom_r_i][top_l_j + i] = 'X';
                }
                for i in 0..active_leds_per_active_line {
                    // left vertical lane
                    rgb_matrix[top_l_j + i][top_l_i] = rgb;
                    char_matrix[top_l_j + i][top_l_i] = 'X';
                }
                // i don't know why tho but it works xD
                let active_leds_per_active_line = if DIM % 2 == 0 { active_leds_per_active_line } else { active_leds_per_active_line + 1 };
                for i in 0..active_leds_per_active_line {
                    // right vertical lane
                    rgb_matrix[top_l_j + i][bottom_r_i] = rgb;
                    char_matrix[top_l_j + i][bottom_r_i] = 'X';
                }
            } // else all blank / black

            let mut transfer_bits_vec: Vec<u8> = vec![];
            rgb_matrix.iter().for_each(|row| {
                row.iter().for_each(|rgb_value| {
                    transfer_bits_vec.extend_from_slice(
                        &encode_rgb(rgb_value.0, rgb_value.1, rgb_value.2 )
                    );
                });
            });
            spi.write_all(&transfer_bits_vec);
            /*char_matrix.iter().for_each(|row| {
                println!("{:?}", row);
            });*/
            sleep_busy_waiting(50);
        }
    }
}

/// Returns (x,y) (or (x,f(x)) coordinates for a specific angle and it's sinus.
fn calc_coordinates(angle_deg: usize, factor: usize) -> (f64, f64) {
    let angle_rad = PI / 180_f64 * (angle_deg as f64);
    let x_coord = angle_rad.cos();
    let y_coord = angle_rad.sin();
    // https://www.geogebra.org/resource/NngvpTKr/iYIkiJEV1NSLJEri/material-NngvpTKr.png
    let x_coord = x_coord * factor as f64;
    let y_coord = y_coord * factor as f64;
    (x_coord, y_coord)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!((10, 0), calc_coordinates(0, 10));
        assert_eq!((0, 10), calc_coordinates(90, 10));
        assert_eq!((-10, 0), calc_coordinates(180, 10));
        assert_eq!((0, -10), calc_coordinates(270, 10));
    }
}
