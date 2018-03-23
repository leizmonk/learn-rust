extern crate try_hashes;
// extern crate rand;
use std::io;
use std::collections::HashMap;
// use rand::Rng;


fn main() {
    // // Average, Mean, Mode
    // let mut rng = rand::thread_rng();
    // // generate a random vector of integer values ranging from -50 to 50 with a size between 1 and 20
    // let mut values: Vec<i32> = (0..rng.gen_range(1, 20)).map(|_| rng.gen_range(-50, 50)).collect();
    // values.sort();

    // println!("Here's the set of integers: {:?}", values);
    // println!("The average is of this set is: {}", try_hashes::avg_median_mode::average(&values));
    // println!("The median is of this set is: {}", try_hashes::avg_median_mode::median(&mut values));
    // println!("The mode of this set is: {}", try_hashes::avg_median_mode::mode(&values));

    // // Pig Latin
    // println!("What phrase should we pig latinize?");
    // let mut unpigged = String::new();

    // io::stdin().read_line(&mut unpigged)
    //     .expect("Failed to read line.");
    // try_hashes::pig_latin::pig_latinize(&unpigged);

    // Employee Names
    let mut employee_list = HashMap::new();
    // Seed with some data

    employee_list.insert("Jill", "Accounting");
    employee_list.insert("Mordecai", "Sales");
    employee_list.insert("Freda", "Operations");

    let departments = ["Accounting", "Sales", "Marketing", "Operations", "IT"];
    
    println!("Create employee, or retrieve list?");
    println!("If adding employee type: 'add [name] to [department]'");
    println!("Available departments: {:?}", departments);
    println!("Otherwise, type 'retrieve' to get a list of employees.");
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)
        .expect("Failed to read line.");
    let input: &str = &*choice;

    if choice.trim() == "retrieve" {
        try_hashes::employees::retrieve(employee_list);
    } else if choice.trim().contains("add") {
        try_hashes::employees::create(input);
    } else {
        println!("Fucked.");
    }
}