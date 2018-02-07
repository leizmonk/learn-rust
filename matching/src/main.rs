#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
  Alabama,
  Alaska,
  Arizona,
  Arkansas,
  California,
  Colorado,
  Connecticut,
  Delaware,
  Florida,
  Georgia,
  Hawaii,
  Idaho,
  Illinois,
  Indiana,
  Iowa,
  Kansas,
  Kentucky,
  Louisiana,
  Maine,
  Maryland,
  Massachusetts,
  Michigan,
  Minnesota,
  Mississippi,
  Missouri,
  Montana,
  Nebraska,
  Nevada,
  NewHampshire,
  NewJersey,
  NewMexico,
  NewYork,
  NorthCarolina,
  NorthDakota,
  Ohio,
  Oklahoma,
  Oregon,
  Pennsylvania,
  RhodeIsland,
  SouthCarolina,
  SouthDakota,
  Tennessee,
  Texas,
  Utah,
  Vermont,
  Virginia,
  Washington,
  WestVirginia,
  Wisconsin,
  Wyoming
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("The value of six is {:?}, the value of none is {:?}", six, none);

    // Result: "It's a quarter from Hawaii!""
    value_in_cents(Coin::Quarter(UsState::Hawaii));
}

// Matching for an Option
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => {
            println!("Can't add 1 to {:?}", x);
            None
        }, 
        Some(i) => {
            println!("Adding 1 to {:?}", x);
            Some(i + 1)
        },
    }
}

// Matching based on the type of coin
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("It's a penny");
            1
        },
        Coin::Nickel => {
            println!("It's a nickel");
            5
        },
        Coin::Dime => {
            println!("It's a dime");
            10
        },
        // Bind some data about the state of the quarter to its value
        Coin::Quarter(state) => {
            println!("It's a quarter from {:?}!", state);
            25
        },
    }
}
