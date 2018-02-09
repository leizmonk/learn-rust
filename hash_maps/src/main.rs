use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // using insert to modify a hash map
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // overwrites the score for "Blue", which was 10
    scores.entry(String::from("Blue")).or_insert(50); // won't overwrite ("Blue", 25) since it exists
    scores.entry(String::from("Yellow")).or_insert(50);

    // using iter, zip, and collect to create a hash map from two vectors 
    let teams = vec![String::from("Red"), String::from("Green")];
    let original_scores = vec![12, 22];
    let new_scores: HashMap<_, _> = teams.iter().zip(original_scores.iter()).collect();

    // hash maps and ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value); 
    // println!("{}", field_name) <- would produce a value moved error
    
    // iterate over a hash map using for
    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }

    // iterate over a string and count occurences of a word
    let text = "hello world wonderful world hello";
    let mut used = HashMap::new();

    for word in text.split_whitespace() {
        // will only insert an updated to count if the word
        let count = used.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", used);

    println!("{:?}", scores);
    println!("{:?}", new_scores);
    println!("{:?}", map);
}

