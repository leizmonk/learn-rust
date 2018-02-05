#[derive(Debug)] // derive needed to be able to use {:?} syntax in printing a debug msg
struct Rectangle {
    width: u32, 
    height: u32,
}

impl Rectangle {
    // method that only takes in instances of the Rectangle struct
    // without any other parameters
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // compares an instance of the Rectangle struct with a second
    // instance as a parameter to assess fit
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated function that doesn't take self as an argument
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    let square1 = Rectangle::square(15); // created a square calling the associated fn here

    println!("rect1 {:#?}", rect1);
    println!("rect2 {:#?}", rect2);
    println!("rect3 {:#?}", rect3);
    println!("square1 {:#?}", square1);

    println!(
        "The area of the rectangle is {} square pixels.", 
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold square1? {}", rect1.can_hold(&square1));
}
