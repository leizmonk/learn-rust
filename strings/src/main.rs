fn main() {
    let hellos = [
        String::from("السلام عليكم"),
        String::from("Dobrý den"),
        String::from("Hello"),
        String::from("שָׁלוֹם"),
        String::from("नमस्ते"),
        String::from("こんにちは"),
        String::from("안녕하세요"),
        String::from("你好"),
        String::from("Olá"),
        String::from("Здравствуйте"),
        String::from("Hola"),
    ];

    for i in hellos.iter() {
        println!("{}", i);
    }

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(&s2);
    println!("s1 is {}", s1);
    println!("s2 is {}", s2);

    let s4 = String::from("Hello, ");
    let s5 = String::from("world!");
    let s6 = s4 + &s5; // Note that s4 has been moved here and can no longer be used
    println!("{}", s6);

    // len = 4 which is # of bytes storing this string
    let len = String::from("Hola").len();
    // len = 24, these are Unicode scalar characters, so take up more bytes in storage
    let len = String::from("Здравствуйте").len();

}

