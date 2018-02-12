use std::collections::HashMap;

pub fn pig_latinize(input: &str) {
    let vowels = ["a", "e", "i", "o", "u", "y"];
    let mut pigged = HashMap::new();

    for w in input.split_whitespace() { 
        pigged.insert(w, w.chars().next().unwrap().to_string());
    }

    println!("{:?}", pigged);
}