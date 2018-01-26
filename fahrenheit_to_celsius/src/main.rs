use std::io;

fn main() {
    println!("Convert Fahrenheit to Celsius");
    println!("Temperature in Fahrenheit to convert?");

    let mut fahrenheit = f64::new();

    io::stdin().read_line(&mut fahrenheit)
        .expect("Failed to read line.");

    let celsius = fahrenheit - 32 * (5 / 9);

    println!("The temperature in Celsius is {}ºC", celsius);
}
