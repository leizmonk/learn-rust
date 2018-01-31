use std::io;

fn fib(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}

fn main() {
    println!("Calculate the Nth Fibonacci nubmer");
    println!("Which number in the series do you want?");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line.");

    let n: u64 = input.trim().parse::<u64>().unwrap();

    println!("Fibonacci is {}", fib(n));
}
