fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        
        number -= 1;
    }

    println!("LIFTOFF!!!");

    // arr_while();
    arr_for();
    reverse();
}

fn arr_while() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn arr_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }    
}

fn reverse() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("TIME TO DIE.");
}