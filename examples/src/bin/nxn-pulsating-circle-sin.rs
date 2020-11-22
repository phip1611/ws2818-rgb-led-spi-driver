//! Example that definitely works on Raspberry Pi.
//! Make sure you have "SPI" on your Pi enabled and that MOSI-Pin is connected
//! with DIN-Pin. You just need DIN pin, no clock. WS2818 uses one-wire-protocol.
//! See the specification for details

use ws2818_examples::{sleep_busy_waiting_ms, get_led_square_dim_from_args};
use ws2818_rgb_led_spi_driver::encoding::{encode_rgb};
use std::f64::consts::PI;
use ws2818_rgb_led_spi_driver::adapter_spi::WS28xxSpiAdapter;
use ws2818_rgb_led_spi_driver::adapter_gen::WS28xxAdapter;

// This example uses sinus and cosines to let a growing circle flow through your LED matrix.
// It looks best on 64x64 displays and more. It calculates all coordinates using sin and cos
// and tries to map the coordinates to the LED matrix.

// edit: there is still a bug somewhere and because of this it looks weird :/
fn main() {
    println!("make sure you have \"SPI\" on your Pi enabled and that MOSI-Pin is connected with DIN-Pin!");
    let mut adapter = WS28xxSpiAdapter::new("/dev/spidev0.0").unwrap();

    let dim = get_led_square_dim_from_args();
    let rows = dim;
    let cols = dim;

    let half_cols = (cols as f64/2_f64).floor();
    let half_rows = (rows as f64/2_f64).floor();

    let mut reverse_dir = true;
    loop {
        reverse_dir = !reverse_dir;
        for factor in 0..rows /2 {

            let mut rgb_matrix = vec![];
            // useful for debugging
            let mut char_matrix = vec![];
            for _ in 0..rows {
                let mut rgb_row_vec = vec![];
                let mut char_row_vec = vec![];
                for _ in 0..cols {
                    rgb_row_vec.push((0, 0, 0));
                    // useful for debugging
                    char_row_vec.push(' ');
                }
                rgb_matrix.push(rgb_row_vec);
                char_matrix.push(char_row_vec);
            }

            for angle_deg in 0..360 {
                let (x_coord, y_coord) = calc_coordinates(angle_deg, factor);

                // now normalize coord like coord origin is in middle of matrix
                // coordination system coordinates are like (x,f(x)) respectively
                // (x,y), while matrix coordinates are (i, j)
                // => x implies j (matrix col)
                // => y implies i (matrix row)

                let matrix_i = (-y_coord + half_rows) as usize;
                let matrix_j = (x_coord + half_cols) as usize;

                if matrix_i < rows && matrix_j < cols {
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

            adapter.write_encoded_rgb(&transfer_bits_vec).unwrap();
            sleep_busy_waiting_ms(100);
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
