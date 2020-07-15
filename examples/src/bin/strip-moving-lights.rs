//! Example that definitely works on Raspberry Pi.
//! Make sure you have "SPI" on your Pi enabled and that MOSI-Pin is connected
//! with DIN-Pin. You just need DIN pin, no clock. WS2818 uses one-wire-protocol.
//! See the specification for details

use ws2818_examples::{get_led_num_from_args, darken_rgb, get_random_pixel_val, sleep_busy_waiting_ms};
use ws2818_rgb_led_spi_driver::adapter::WS28xxAdapter;
use std::time::{Instant, Duration};
use std::ops::Add;

pub const FREQUENCY: u64 = 20; // 30 Hz
pub const FREQUENCY_MS: u64 = 1000 / FREQUENCY;

// This animation sends moving light impulses via the LED strip
fn main() {
    println!("make sure you have \"SPI\" on your Pi enabled and that MOSI-Pin is connected with DIN-Pin!");
    let mut adapter = WS28xxAdapter::new("/dev/spidev0.0").unwrap();
    let num_leds = get_led_num_from_args();
    let mut anim = MovingLightStripsAnimation::new(num_leds);

    let mut next_light_time = Instant::now();
    loop {
        let now = Instant::now();
        if now >= next_light_time {
            anim.add_next_light_impulse();
            next_light_time = now.add(Duration::from_secs(1))
        }
        anim.shift_all_pixels();
        adapter.write_rgb(&anim.rgb_data[MOVING_LIGHT_IMPULSE_LEN..]).unwrap();
        sleep_busy_waiting_ms(FREQUENCY_MS);
    }
}

const MOVING_LIGHT_IMPULSE_LEN: usize = 15;

pub struct MovingLightStripsAnimation {
    rgb_data: Vec<(u8, u8, u8)>,
}

impl MovingLightStripsAnimation {
    pub fn new(led_count: usize) -> Self {
        MovingLightStripsAnimation {
            rgb_data: vec![(0, 0, 0); led_count + MOVING_LIGHT_IMPULSE_LEN],
        }
    }
}

impl MovingLightStripsAnimation {

    /// Shifts all pixel to the next position. Beginning is filled
    /// with black (0, 0, 0).
    fn shift_all_pixels(&mut self) {
        let upper_border = self.rgb_data.len();
        for i in 0..upper_border {
            // loop backwards
            let i = upper_border - 1 - i;

            if i == 0 {
                std::mem::replace(
                    &mut self.rgb_data[i],
                    (0, 0, 0)
                );
            } else {
                let prev = self.rgb_data[i - 1].clone();
                std::mem::replace(
                    &mut self.rgb_data[i],
                    prev
                );
            }
        }
    }
    fn add_next_light_impulse(&mut self) {
        let (r, g, b) = get_random_pixel_val();
        std::mem::replace(&mut self.rgb_data[00], darken_rgb(r, g, b, 0.1));
        std::mem::replace(&mut self.rgb_data[01], darken_rgb(r, g, b, 0.2));
        std::mem::replace(&mut self.rgb_data[02], darken_rgb(r, g, b, 0.4));
        std::mem::replace(&mut self.rgb_data[03], darken_rgb(r, g, b, 0.6));
        std::mem::replace(&mut self.rgb_data[04], darken_rgb(r, g, b, 0.7));
        std::mem::replace(&mut self.rgb_data[05], darken_rgb(r, g, b, 0.8));
        std::mem::replace(&mut self.rgb_data[06], darken_rgb(r, g, b, 0.9));
        std::mem::replace(&mut self.rgb_data[07], (r, g, b));
        std::mem::replace(&mut self.rgb_data[08], darken_rgb(r, g, b, 0.9));
        std::mem::replace(&mut self.rgb_data[09], darken_rgb(r, g, b, 0.8));
        std::mem::replace(&mut self.rgb_data[10], darken_rgb(r, g, b, 0.7));
        std::mem::replace(&mut self.rgb_data[11], darken_rgb(r, g, b, 0.6));
        std::mem::replace(&mut self.rgb_data[12], darken_rgb(r, g, b, 0.4));
        std::mem::replace(&mut self.rgb_data[13], darken_rgb(r, g, b, 0.2));
        std::mem::replace(&mut self.rgb_data[14], darken_rgb(r, g, b, 0.1));
    }
}

