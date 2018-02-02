fn main() {
    let s = String::from("The quick brown fox jumped over the fence.");
    first_word(&s);
    alt_first_word(&s);
}

// Not preferred, brittle since the source string could be cleared after this is called
// which would render this fn invalid
fn first_word(s: &String) -> usize {
    // converts String to arr of bytes for iterating through
    let bytes = s.as_bytes(); 

    // iter() returns each el in a collection, enumerate() wraps this and returns a tuple
    // using &item here instead of item because .iter().enumerate() returns a reference
    for (i, &item) in bytes.iter().enumerate() {
        // byte literal syntax
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn alt_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}