pub fn production_rate_per_hour(speed: u8) -> f64 {
    let raw_rate = speed as f64 * 221.0;

    if speed >= 9 {
        raw_rate * 0.77
    } else if speed >= 5 {
        raw_rate * 0.9
    } else {
        raw_rate
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / 60
}

pub fn main() {
    println!("{}", production_rate_per_hour(6))
}
