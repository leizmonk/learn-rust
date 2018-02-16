use std::collections::HashMap;

pub fn create() {
    println!("Make something");
}

pub fn retrieve(list: HashMap<&str, &str>) {
    for (employee, department) in list.iter() {
        println!("{} is in {}", employee, department);
    }
}