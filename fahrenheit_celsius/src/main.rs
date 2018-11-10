fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}

fn main() {
    let fahrenheit = 75;
    let celsius = fahrenheit_to_celsius(fahrenheit as f64);
    println!("Hello, did you know that {}F is {:.2}C", fahrenheit, celsius);
}
