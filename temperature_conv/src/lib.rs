pub fn fahrenheit_to_celsius(f : f64) -> f64 {
    return (f - 32.0) * (5/9) as f64
}

pub fn celsius_to_fahrenheit(c : f64) -> f64 {
    return (c * (5/9) as f64) + 32.0
}