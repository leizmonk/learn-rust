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

// Can take in slice of a whole String, as well as string literal (&str type)
fn alt_first_word(s: &str) -> &str { 
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // Return a slice of the string up to the first index position where
            // found a byte representing the string literal for a space
            println!("1 - {}", &s[0..i]);
            return &s[0..i];
        }
    }

    // If there were no spaces, take a full slice of the entire String 
    println!("2 - {}", &s[..]);
    &s[..]
}