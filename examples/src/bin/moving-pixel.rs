//! Example that definitely works on Raspberry Pi. I used a 8x8 RGB LED matrix.
//! make sure you have "SPI" on your Pi enabled and that MOSI-Pin is connected
//! with DIN-Pin. You just need DIN pin, no clock. WS2818 uses one-wire-protocol.
//! See the specification for details

use ws2818_examples::{sleep_busy_waiting, get_led_num_from_args};
use ws2818_rgb_led_spi_driver::encoding::encode_rgb;
use ws2818_rgb_led_spi_driver::adapter::WS28xxAdapter;

// Example that shows a single moving pixel though the 8x8 led matrix.
fn main() {
    println!("make sure you have \"SPI\" on your Pi enabled and that MOSI-Pin is connected with DIN-Pin!");
    let mut adapter = WS28xxAdapter::new("/dev/spidev0.0").unwrap();
    let num_leds = get_led_num_from_args();

    // note we first aggregate all data and write then all at
    // once! otherwise timings would be impossible to reach


    let mut i = 0;
    loop {
        let mut data = vec![];
        for j in 0..num_leds {
            // fill num_leds-1 pixels with black; one with white
            if i == j {
                data.extend_from_slice(&encode_rgb(50, 50, 50));
            } else {
                data.extend_from_slice(&encode_rgb(0, 0, 0));
            }
        }
        adapter.write_encoded_rgb(&data).unwrap();

        i = (i + 1) % num_leds;
        sleep_busy_waiting(1000/10); // 100ms / 10Hz
    }

}
