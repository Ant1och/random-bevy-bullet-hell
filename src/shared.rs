pub fn move_toward_f32(from: f32, to: f32, delta: f64) -> f32 {
    match (to - from).abs() as f64 >= delta {
        true => from + (to - from).signum() * delta as f32,
        false => to,
    }
}
