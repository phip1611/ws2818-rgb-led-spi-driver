use std::time::{Instant, Duration};
use std::ops::Add;

#[inline(always)]
pub fn sleep_busy_waiting(ms: u64) {
    let target_time = Instant::now().add(Duration::from_millis(ms));
    loop {
        if Instant::now() >= target_time { break; }
    }
}
