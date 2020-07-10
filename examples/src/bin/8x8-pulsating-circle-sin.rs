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

const COLS: usize = 8;
const ROWS: usize = 8;

// This example uses sinus and cosines to let a growing circle flow through your LED matrix.
// It looks best on 64x64 displays and more. It calculates all coordinates using sin and cos
// and tries to map the coordinates to the LED matrix.
fn main() {
    println!("Make sure to have \"SPI\" on your Pi enabled and that MOSI-Pin is connected with DIN-Pin!");
    let mut spi = ws2818_rgb_led_spi_driver::setup_spi("/dev/spidev0.0").unwrap();

    let half_cols = (COLS as f64/2_f64).floor();
    let half_rows = (ROWS as f64/2_f64).floor();

    let mut reverse_dir = true;
    loop {
        reverse_dir = !reverse_dir;
        for mut factor in 0..ROWS/2 {

            let mut rgb_matrix = [[(0, 0, 0); COLS]; ROWS];
            let mut char_matrix = [[' '; COLS]; ROWS];
            for angle_deg in 0..360 {
                let (x_coord, y_coord) = calc_coordinates(angle_deg, factor);

                // now normalize coord like coord origin is in middle of matrix
                // coordination system coordinates are like (x,f(x)) respectively
                // (x,y), while matrix coordinates are (i, j)
                // => x implies j (matrix col)
                // => y implies i (matrix row)

                let matrix_i = (-y_coord + half_rows) as usize;
                let matrix_j = (x_coord + half_cols) as usize;

                if matrix_i < ROWS && matrix_j < COLS {
                    char_matrix[matrix_i][matrix_j] = 'X';
                    if factor % 3 == 0 {
                        rgb_matrix[matrix_i][matrix_j] = (60, 0, 0);
                    } else if factor % 3 == 1 {
                        rgb_matrix[matrix_i][matrix_j] = (0, 60, 0);
                    } else {
                        rgb_matrix[matrix_i][matrix_j] = (0, 0, 60);
                    }
                }
            }
            /*
            this is useful to simulate larger displays and test the math functions!
            char_matrix.iter().for_each(|row| {
               println!("{:?}", row);
            });
            println!();
            println!();*/

            let mut transfer_bits_vec: Vec<u8> = vec![];
            rgb_matrix.iter().for_each(|row| {
                row.iter().for_each(|rgb_value| {
                    transfer_bits_vec.extend_from_slice(
                        &encode_rgb(rgb_value.0, rgb_value.1, rgb_value.2 )
                    );
                });
            });

            spi.write_all(&transfer_bits_vec);
            sleep_busy_waiting(100);
        }
    }
}

/// Returns (x,y) (or (x,f(x)) coordinates for a specific angle and it's sinus.
fn calc_coordinates(angle_deg: usize, factor: usize) -> (f64, f64) {
    let angle_rad = PI/180_f64 * (angle_deg as f64);
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
