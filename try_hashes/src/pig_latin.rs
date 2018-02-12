use std::collections::HashMap;

// Sourced from: https//codereview.stackexchange.com/questions/172866/pig-latin-exercise-in-rust
pub fn pig_latinize(input: &str) {
    // .chars() converts input to an iterator, peekable allows this iterator to 
    // optionally return a reference to the next element  
    let mut chars = input.chars().peekable();
    // will contain the pig latinized string
    let mut oinked = String::new();

    // outer while loop walks through the input string
    while let Some(c) = chars.next() {
        // first match constructs the suffix
        let suffix = match c {
            // if the first character was a vowel, the suffix will be "-hay" 
            'a' | 'e' | 'i' | 'o' | 'u' => {
                // push the vowel starting the word into latinized result
                oinked.push(c);
                String::from("-hay")
            }
            // else if the first character was a non-vowel, alphabetic consonant
            // construct the suffix "-{}ay". this works despite the range here including
            // vowels because words that started with vowels were already handled
            // in the first branch of this match
            'a'...'z' | 'A'...'Z' => {
                // don't push anything into oinked here because otherwise
                // you'd end up with "world-way"
                format!("-{}ay", c)
            }
            // in all other cases, push the character as is, don't attempt to construct a suffix
            _ => {
                oinked.push(c);
                continue;
            }
        };

        // inner while loop peeks down the iterator
        // until it hits the end, this re-constructs the base word 
        while let Some(&c) = chars.peek() {
            println!("{:?}", &c);
            // if the character is alphabetic, push it to oinked
            match c {
                'a'...'z' | 'A'...'Z' => {
                    chars.next();
                    oinked.push(c);
                }
                _ => break,
            }
        }

        // // append the appropriate suffix
        // oinked += &suffix;
    }

    println!("{}", oinked);
}

