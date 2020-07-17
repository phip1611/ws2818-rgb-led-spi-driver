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

// This animations sends animated colored light strips from the middle to the strip to the edges.
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
        adapter.write_rgb(&anim.rgb_strip_vec_data[MOVING_LIGHT_IMPULSE_LEN..]).unwrap();
        sleep_busy_waiting_ms(FREQUENCY_MS);
    }
}

const MOVING_LIGHT_IMPULSE_LEN: usize = 15;

pub struct MovingLightStripsAnimation {
    led_count: usize,
    rgb_strip_vec_data: Vec<(u8, u8, u8)>,
    new_rgb_data_vec: Vec<(u8, u8, u8)>,
}

impl MovingLightStripsAnimation {
    pub fn new(mut led_count: usize) -> Self {
        if led_count % 2 != 1 {
            led_count = (led_count / 2) + 1;
        }
        MovingLightStripsAnimation {
            led_count,
            rgb_strip_vec_data: vec![(0, 0, 0); led_count],
            new_rgb_data_vec: vec![(0, 0, 0); MOVING_LIGHT_IMPULSE_LEN],
        }
    }

    fn add_next_light_impulse(&mut self) {
        let (r, g, b) = get_random_pixel_val();
        std::mem::replace(&mut self.new_rgb_data_vec[00], darken_rgb(r, g, b, 0.1));
        std::mem::replace(&mut self.new_rgb_data_vec[01], darken_rgb(r, g, b, 0.2));
        std::mem::replace(&mut self.new_rgb_data_vec[02], darken_rgb(r, g, b, 0.4));
        std::mem::replace(&mut self.new_rgb_data_vec[03], darken_rgb(r, g, b, 0.6));
        std::mem::replace(&mut self.new_rgb_data_vec[04], darken_rgb(r, g, b, 0.7));
        std::mem::replace(&mut self.new_rgb_data_vec[05], darken_rgb(r, g, b, 0.8));
        std::mem::replace(&mut self.new_rgb_data_vec[06], darken_rgb(r, g, b, 0.9));
        std::mem::replace(&mut self.new_rgb_data_vec[07], (r, g, b));
        std::mem::replace(&mut self.new_rgb_data_vec[08], darken_rgb(r, g, b, 0.9));
        std::mem::replace(&mut self.new_rgb_data_vec[09], darken_rgb(r, g, b, 0.8));
        std::mem::replace(&mut self.new_rgb_data_vec[10], darken_rgb(r, g, b, 0.7));
        std::mem::replace(&mut self.new_rgb_data_vec[11], darken_rgb(r, g, b, 0.6));
        std::mem::replace(&mut self.new_rgb_data_vec[12], darken_rgb(r, g, b, 0.4));
        std::mem::replace(&mut self.new_rgb_data_vec[13], darken_rgb(r, g, b, 0.2));
        std::mem::replace(&mut self.new_rgb_data_vec[14], darken_rgb(r, g, b, 0.1));
    }

    /// Shifts all pixel to the next position.
    /// Iterates backwards through  `self.rgb_strip_vec_data` from both sides!
    /// Because our strip looks like this:
    ///
    /// ```
    /// [LED 0]   [LED 1]       ... [LED 5]  [LED 6]  ... [LED N]
    /// [RGB N/2] [RGB N/2 - 1] ... [RGB 0]  [RGB 1]  ... [RGB N/2]  // RGB value; animated motion to the edges
    /// [Vec[0]]  [Vec[1]]      ... [Vec[x]] [Vec[y]] ... [Vec[N]]
    /// ```
    fn shift_all_pixels(&mut self) {
        for i in 0..self.led_count / 2 {
            let i_left = i;
            let i_right = self.led_count - i;

            if i_left - i_right == 1 {
                // genau dann in der mitte
                let new = self.new_rgb_data_vec.last().unwrap().clone();
                std::mem::replace(&mut self.rgb_strip_vec_data[i_left], new);
                std::mem::replace(&mut self.rgb_strip_vec_data[i_right], new);
            } else {
                let prev_left = self.rgb_strip_vec_data[i_left + 1].clone();
                std::mem::replace(&mut self.rgb_strip_vec_data[i_left], prev_left);
                let prev_right = self.rgb_strip_vec_data[i_right - 1].clone();
                std::mem::replace(&mut self.rgb_strip_vec_data[i_right], prev_right);
            }
        }

        for i in 0..MOVING_LIGHT_IMPULSE_LEN {
            let i = MOVING_LIGHT_IMPULSE_LEN - 1 - i;

            if i == 0 {
                std::mem::replace(&mut self.new_rgb_data_vec[i], (0, 0, 0));
            } else {
                let prev = self.new_rgb_data_vec[i - 1].clone();

                std::mem::replace(&mut self.new_rgb_data_vec[i], prev);
            }
        }
    }
}

