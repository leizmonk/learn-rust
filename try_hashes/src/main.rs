extern crate try_hashes;
extern crate rand;
use std::io;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    // generate a random vector of integer values ranging from -50 to 50 with a size between 1 and 20
    let mut values: Vec<i32> = (0..rng.gen_range(1, 20)).map(|_| rng.gen_range(-50, 50)).collect();
    values.sort();

    println!("Here's the set of integers: {:?}", values);
    println!("The average is of this set is: {}", try_hashes::avg_median_mode::average(&values));
    println!("The median is of this set is: {}", try_hashes::avg_median_mode::median(&mut values));
    println!("The mode of this set is: {}", try_hashes::avg_median_mode::mode(&values));

    // Pig Latin
    println!("What phrase should we pig latinize?");
    let mut unpigged = String::new();

    io::stdin().read_line(& mut unpigged)
        .expect("Failed to read line.");

    try_hashes::pig_latin::pig_latinize(&unpigged);
}