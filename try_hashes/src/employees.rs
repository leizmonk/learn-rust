use std::collections::HashMap;

pub fn create(input: &str) {
    // Read the input string
    // Assume first word after "add" is the name to add
    // Assume first word after "to" is the department
    // Error if string isn't exactly 4 words
    let mut total_words = Vec::new();

    for word in input.split_whitespace() {
        total_words.push(word);
        println!("{:?}", word);
    }

    if total_words.len() < 4 {
        println!("FUCK OFF");
    } else {
        println!("you cool");
    }

    // or_insert the key: value pair of employee: department to employee_list
}

pub fn retrieve(list: HashMap<&str, &str>) {
    for (employee, department) in list.iter() {
        println!("{} is in {}", employee, department);
    }
}