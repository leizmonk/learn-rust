#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let _v = vec![1, 2, 3];
    let mut woot = Vec::new();

    woot.push(5);
    woot.pop();

    let mut d = vec![9, 10, 11, 12, 13, 14, 15];

    // mutable reference
    // let third_d: &i32 = &d[2];

    // get creates an option
    // let third_d: Option<&i32> = d.get(2);

    // must use * to dereference i before modifying it
    for i in &mut d {
        *i += 3;
    }

    for i in &d {
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("violet")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &row {
        println!("{:?}", i);
    }
}
