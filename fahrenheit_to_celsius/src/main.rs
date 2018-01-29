use std::io;

fn main() {
    println!("Convert Fahrenheit to Celsius");
    println!("Temperature in Fahrenheit to convert?");

    let mut fahrenheit = String::new();

    io::stdin().read_line(&mut fahrenheit)
        .expect("Failed to read line.");

    let float_f = fahrenheit.parse::<f64>().unwrap();

    println!("{}", float_f);

    let celsius = (float_f - 32.0) * (5.0 / 9.0);

    println!("{}", celsius);

    println!("The temperature in Celsius is {}ºC", celsius);
}
