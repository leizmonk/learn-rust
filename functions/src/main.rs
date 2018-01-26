fn main() {
    another_function(3, 1);
    let res = try_this(5);
    println!("The value of try_this(5) is: {}", res);    
}

fn another_function(x: i32, y: i64) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn try_this(a: i32) -> i32 {
    a + 1
}
