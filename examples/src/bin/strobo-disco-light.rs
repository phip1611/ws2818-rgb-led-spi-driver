//! Example that definitely works on Raspberry Pi.
//! Make sure you have "SPI" on your Pi enabled and that MOSI-Pin is connected
//! with DIN-Pin. You just need DIN pin, no clock. WS2818 uses one-wire-protocol.
//! See the specification for details

use ws2818_examples::{sleep_busy_waiting_ms, get_led_num_from_args};
use ws2818_rgb_led_spi_driver::encoding::encode_rgb;
use ws2818_rgb_led_spi_driver::adapter::WS28xxAdapter;

const FREQUENCY: u64 = 12; // in Hz

// experiments showed that below 3ms not all RGBs flash properly;
// this is independent from rust running in release or debug mode
const FLASH_TIME_MS: u64 = 3;

// Strobo light effect like in disco
// see https://en.wikipedia.org/wiki/Strobe_light
fn main() {
    println!("make sure you have \"SPI\" on your Pi enabled and that MOSI-Pin is connected with DIN-Pin!");
    let mut adapter = WS28xxAdapter::new("/dev/spidev0.0").unwrap();
    let num_leds = get_led_num_from_args();

    let mut white_display_bytes = vec![];
    // strobo effekt
    for _ in 0..num_leds {
        white_display_bytes.extend_from_slice(&encode_rgb(255, 255, 255));
    }

    let mut empty_display_bytes = vec![];
    for _ in 0..num_leds {
        empty_display_bytes.extend_from_slice(&encode_rgb(0, 0, 0));
    }


    // note we first aggregate all data and write then all at
    // once! otherwise timings would be impossible to reach
    loop {
        adapter.write_encoded_rgb(&white_display_bytes).unwrap();
        sleep_busy_waiting_ms(FLASH_TIME_MS);
        adapter.write_encoded_rgb(&empty_display_bytes).unwrap();
        sleep_busy_waiting_ms((1000 / FREQUENCY) - FLASH_TIME_MS);
    }
}

