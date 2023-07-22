use std::{collections::HashMap, io};

pub fn collect_input() -> (i32, i32) {
    println!(" *** Please input the start of desired range (an integer > 0).\n");

    let mut start = String::new();

    io::stdin()
        .read_line(&mut start)
        .expect("Failed to read line");

    // We need to explicitly convert the guess from a String to i32.
    let start: i32 = match start.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    println!(" *** Please input the end of desired range (an integer > 0 AND > start).\n");

    let mut end = String::new();

    io::stdin()
        .read_line(&mut end)
        .expect("Failed to read line");

    // We need to explicitly convert the guess from a String to i32.
    let end: i32 = match end.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    (start, end)
}

pub fn collatz(num: i32) -> i32 {
    let next_num: i32;
    if num == 1 {
        return 1;
    } else {
        next_num = if &num % 2 != 0 {
            (3 * &num) + 1
        } else {
            &num / 2
        };
    }
    next_num
}

pub fn produce_vec(num: i32, ref_hashmap: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    /*
        TODO: For the sake of safety, we'll need to make sure that the number passed in is greater than 0.
    */

    // Initialize a vector to be filled with i64 to be pushed to mem_table.
    let mut num_vec: Vec<i32> = vec![];

    let original_number: i32 = num;
    let mut current_number: i32 = original_number.clone();

    // We want to get the whole sequence, so we start by pushing the original number into the vector.
    num_vec.push(current_number);

    // Standard stuff - keep doing the thing we want until the final condition is reached (current_number == 1).
    while current_number != 1 {
        let new_number = collatz(current_number);

        // If the entry exists within the hashmap, we want to use that instead of recomputing.
        if ref_hashmap.contains_key(&new_number) {
            num_vec.extend(&ref_hashmap[&new_number]);
            // un-comment for debug info
            // println!("Extended item: {:?}", num_vec);
            current_number = 1;
        } else {
            current_number = new_number;
            num_vec.push(current_number);
            // un-comment for debug info
            // println!("Non-extended item: {:?}", num_vec);
        }
    }
    num_vec
}
