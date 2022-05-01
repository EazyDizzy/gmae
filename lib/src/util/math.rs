pub fn round_based(n: f32, base: u32) -> f32 {
    let modifier = 10_i32.pow(base) as f32;

    (n * modifier).round() / modifier
}