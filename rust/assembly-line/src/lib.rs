pub fn production_rate_per_hour(speed: u8) -> f64 {
    if speed > 10 {
        panic!("speed must be in range [0..10]");
    }

    let base_production_rate_per_hour = 221.0;
    let success_rate = if speed <= 4 {
        1.0
    } else if 5 <= speed && speed <= 8 {
        0.9
    } else {
        0.77
    };

    speed as f64 * base_production_rate_per_hour * success_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / 60
}
