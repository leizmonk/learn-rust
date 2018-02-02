fn main() {
    let mut s1 = String::from("hello");
    let mut s2 = String::from("what up");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);

    if len > 3 {
        change(&mut s1)
    } else {
        change(&mut s2);
    }

    let nil_ref = no_dangle();

    println!("{}", nil_ref);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

// Will error out unless all references are mutable
fn change(some_str: &mut String) {
    some_str.push_str(", world!");

    println!("{}", some_str);
}

// If String s were referenced here then this would error because
// There's nothing to reference once s goes out of scope
fn no_dangle() -> String {
    let s = String::from("meow");

    s
}