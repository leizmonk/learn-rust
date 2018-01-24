fn main() {
    let _x = 2.0; // f64, 64-bit float, default float type
    let _y: f32 = 3.0; // f32, 32-bit float, must be specified

    // Binary operators
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    println!("Sum is {}", sum);
    println!("Difference is {}", difference);
    println!("Product is {}", product);
    println!("Quotient is {}", quotient);
    println!("Remainder is {}", remainder);

    // Tuples
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tuple; // One method of destructuring
    
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    // Another way to destructure a tuple
    let five_hundred = tuple.0;
    let six_point_four = tuple.1;
    let one = tuple.2;

    println!("First element of the tuple is: {}", five_hundred);
    println!("Second element of the tuple is: {}", six_point_four);
    println!("Third element of the tuple is: {}", one);

    // Arrays - immutable
    let arr = [1, 2, 3, 4, 5];
    let first = arr[0];
    let last = arr[4];

    println!("First element of the array is: {}", first);
    println!("First element of the array is: {}", last);    
}
