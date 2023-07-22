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

pub fn produce_vec(num: i32) -> Vec<i32> {
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
        current_number = new_number;
        num_vec.push(current_number);
    }
    num_vec
}
