fn main() {
    let mut s = String::from("hello"); // variable is in scope from here

    s.push_str(", world!"); // appends a literal to String

    println!("{}", s);

    str_move();

    int_copy();

    pass_val_fn();
} // scope is over, variable s is no longer valid, drop() gets called here on s

fn str_move() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    // println!("{}, world!", s1); <- errors if s1 isn't cloned because it would to s2 
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn int_copy() {
    let x = 5; // ints have a known size at compile time and can be copied w/o clone()
    let y = x;

    println!("x = {}, y = {}", x, y);
}

fn pass_val_fn() {
    let s = String::from("hello");  // s comes into scope.

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here.

    let x = 5;                      // x comes into scope.

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward.

} // Here, x goes out of scope, then s. But since s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope.
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope.
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
