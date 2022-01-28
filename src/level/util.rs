use std::time::{SystemTime, UNIX_EPOCH};

use rand::XorShiftRng;

pub fn get_rng() -> XorShiftRng {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_micros();
    rand::SeedableRng::from_seed([now, now, now, now])
}