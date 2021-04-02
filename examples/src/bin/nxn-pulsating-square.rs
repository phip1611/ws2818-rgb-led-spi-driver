//! Example that definitely works on Raspberry Pi.
//! Make sure you have "SPI" on your Pi enabled and that MOSI-Pin is connected
//! with DIN-Pin. You just need DIN pin, no clock. WS2818 uses one-wire-protocol.
//! See the specification for details

use ws2818_examples::{get_led_square_dim_from_args, sleep_busy_waiting_ms};
use ws2818_rgb_led_spi_driver::adapter_gen::WS28xxAdapter;
use ws2818_rgb_led_spi_driver::adapter_spi::WS28xxSpiAdapter;
use ws2818_rgb_led_spi_driver::encoding::encode_rgb;

const BRIGHTNESS_FACTOR: f64 = 0.2;

// This example let a square flow though a quare led matrix.
fn main() {
    println!("make sure you have \"SPI\" on your Pi enabled and that MOSI-Pin is connected with DIN-Pin!");
    let mut adapter = WS28xxSpiAdapter::new("/dev/spidev0.0").unwrap();
    let dim = get_led_square_dim_from_args();
    let _num_leds = dim * dim;

    let mut reverse_dir = true;
    let half_dim = dim / 2;
    let (mut r, mut g, mut b) = (255 / 3, 255 / 3 * 2, 255);
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
                size = half_dim + 1 - 1 - size;
            }

            let mut rgb_matrix = vec![];
            // useful for debugging
            let mut char_matrix = vec![];
            for _ in 0..dim {
                let mut rgb_row_vec = vec![];
                let mut char_row_vec = vec![];
                for _ in 0..dim {
                    rgb_row_vec.push((0, 0, 0));
                    // useful for debugging
                    char_row_vec.push(' ');
                }
                rgb_matrix.push(rgb_row_vec);
                char_matrix.push(char_row_vec);
            }

            if size > 0 {
                let active_leds_per_active_line = 2 * size;

                let top_l_i = 0 + half_dim - size;
                let top_l_j = top_l_i;
                let bottom_r_i = dim - 1 - half_dim + size;
                let _bottom_r_j = bottom_r_i;

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
                let active_leds_per_active_line = if dim % 2 == 0 {
                    active_leds_per_active_line
                } else {
                    active_leds_per_active_line + 1
                };
                for i in 0..active_leds_per_active_line {
                    // right vertical lane
                    rgb_matrix[top_l_j + i][bottom_r_i] = rgb;
                    char_matrix[top_l_j + i][bottom_r_i] = 'X';
                }
            } // else all blank / black

            let mut transfer_bits_vec: Vec<u8> = vec![];
            rgb_matrix.iter().for_each(|row| {
                row.iter().for_each(|rgb_value| {
                    transfer_bits_vec.extend_from_slice(&encode_rgb(
                        rgb_value.0,
                        rgb_value.1,
                        rgb_value.2,
                    ));
                });
            });
            adapter.write_encoded_rgb(&transfer_bits_vec).unwrap();
            /*char_matrix.iter().for_each(|row| {
                println!("{:?}", row);
            });*/
            sleep_busy_waiting_ms(50);
        }
    }
}
