#[derive(Debug)]
enum IpAddr {
    // Couldn't do this with a struct
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(182, 16, 131, 27);

    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:#?}", home);
    println!("{:#?}", loopback);
}
