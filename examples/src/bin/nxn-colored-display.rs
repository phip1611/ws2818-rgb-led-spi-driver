//! Example that definitely works on Raspberry Pi. I used a 8x8 RGB LED matrix.
//! make sure you have "SPI" on your Pi enabled and that MOSI-Pin is connected
//! with DIN-Pin. You just need DIN pin, no clock. WS2818 uses one-wire-protocol.
//! See the specification for details

use ws2818_examples::{sleep_busy_waiting, get_led_square_dim_from_args};
use ws2818_rgb_led_spi_driver::encoding::{encode_rgb_slice};
use ws2818_rgb_led_spi_driver::adapter::WS28xxAdapter;

// Some colored animation on a 8x8 led matrix.
fn main() {
    println!("make sure you have \"SPI\" on your Pi enabled and that MOSI-Pin is connected with DIN-Pin!");
    let mut adapter = WS28xxAdapter::new("/dev/spidev0.0").unwrap();
    let dim = get_led_square_dim_from_args();
    let rows = dim * dim;
    let cols = dim * dim;
    let leds = cols * rows;
    let leds_f64 = leds as f64;

    let mut rgb_data = vec![];
    for row in 0..rows {
        for col in 0..cols {
            // red is on the top right
            // green is on the bottom right
            // blue is on the bottom left

            // calc red
            let (r_row, r_col) = ((rows - row - 1) as f64, col as f64);
            let r = 255_f64 * ((r_row + 1_f64) * (r_col + 1_f64))/(leds_f64);
            let r = r.round() as u8;

            // calc green
            let (g_row, g_col) = (row as f64, col as f64);
            let g = 255_f64 * ((g_row + 1_f64) * (g_col + 1_f64))/(leds_f64);
            let g = g.round() as u8;

            // calc blue
            let (b_row, b_col) = (row as f64, (cols - col - 1) as f64);
            let b = 255_f64 * ((b_row + 1_f64) * (b_col + 1_f64))/(leds_f64);
            let b = b.round() as u8;

            //rgb_data.push((r, g, b));
            rgb_data.push((r, g, b));
        }
    }

    let spi_bytes = encode_rgb_slice(&rgb_data);
    adapter.write_encoded_rgb(&spi_bytes).unwrap();

    sleep_busy_waiting(1000);

    // now let's rotate the bits

    let mut rgb_data_current = Vec::new();
    rgb_data_current.extend_from_slice(&rgb_data);
    loop {
        for row in 0..rows {
            for col in 0..cols {
                let led_i = row * cols + col;
                if led_i == 0 {
                    // in very first iteration
                    let curr = (&rgb_data_current[led_i]).clone();
                    let next = (&rgb_data_current[led_i + 1]).clone();
                    std::mem::replace(&mut rgb_data_current[led_i], next);
                    std::mem::replace(&mut rgb_data_current[leds - 1], curr);
                }
                else if led_i + 1 < leds {
                    let next = (&rgb_data_current[led_i + 1]).clone();
                    std::mem::replace(&mut rgb_data_current[led_i], next);
                }
            }
        }
        let rgb_data_current_encoded =  encode_rgb_slice(&rgb_data_current);
        adapter.write_encoded_rgb(&rgb_data_current_encoded).unwrap();
        sleep_busy_waiting(50);
    }

}
