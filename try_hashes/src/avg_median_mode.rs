use std::collections::HashMap;

pub fn average(values: &Vec<i32>) -> f32 {
    // iterate through the values and sum them
    let sum: i32 = values.iter().sum();
    // cast the length of the values vector to an integer
    let count = values.len() as i32;    

    println!("The sum of the set is: {:?}", sum);
    println!("There are {:?} members in this set", count);

    // cast both sum and count as floats to get accurate average
    sum as f32 / count as f32
}

pub fn median(values: &mut Vec<i32>) -> f32 {
    let count = values.len() as i32;
    // cast count as usize and divide by 2, this gives the middle index when rounded down    
    let middle_index = count as usize / 2;

    // accounts for even or odd sized vectors
    if count % 2 == 0 {
        // if even sized vector, average the middle 2 values    
        (values[middle_index] as f32 + values[middle_index - 1] as f32) / 2.0
    } else  {
        // if odd sized vector, return middle index
        values[middle_index] as f32
    }
}

pub fn mode(values: &Vec<i32>) -> i32 {
    // new hash map for occurences of each int in the vector
    let mut occurrences = HashMap::new();

    // for each int in the vector, insert into the hashmap with a val of 0 or
    // add one to the value of the existing key (an int)
    for &int in values {
        *occurrences.entry(int).or_insert(0) += 1;
    }

    // convert occurrences to an iterator, this is copypasta
    // TODO: need to handle the case where more than one integer is the mode
    // and also when every integer occurs exactly once
    occurrences.into_iter()
        .max_by_key(|&(_, count)| count) // get the key w the largest int value
        .map(|(val, _)| val)
        .expect("Cannot compute mode of zero numbers")
}