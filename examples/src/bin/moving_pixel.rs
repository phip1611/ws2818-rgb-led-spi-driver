//! Example that definitely works on Raspberry Pi.
//! Make sure to have "SPI" on your Pi enabled and that MOSI-Pin is connected
//! with DIN-Pin.

use std::io::Write;

use ws2818_examples::sleep_busy_waiting;
use ws2818_rgb_led_spi_driver::encoding::encode_rgb;

fn main() {
    println!("Make sure to have \"SPI\" on your Pi enabled and that MOSI-Pin is connected with DIN-Pin!");
    let mut spi = ws2818_rgb_led_spi_driver::setup_spi("/dev/spidev0.0").unwrap();

    // note we first aggregate all data and write then all at
    // once! otherwise timings would be impossible to reach
    loop {
        for i in 0..64 {
            let mut data = vec![];
            for j in 0..64 {
                if i == j {
                    data.extend_from_slice(&encode_rgb(255, 255, 255));
                } else {
                    data.extend_from_slice(&encode_rgb(0, 0, 0));
                }
            }
            spi.write_all(&data).unwrap();
            sleep_busy_waiting(1000 * 1000/5); // 200ms / 5Hz
        }
    }

}
