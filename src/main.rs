use rust_collatz::{collatz, collect_input, produce_vec};
use std::collections::HashMap;
use std::process::exit;

struct RangeBounds {
    start: i32,
    end: i32,
}

fn main() {
    /*
        OPPORTUNITIES FOR IMPROVEMENT:

        1) Need a mechanism to make a call to the HashMap so we're not recreating work.
            - Vectors are created within the produce_vec function.
            - produce_vec is unaware of the hashmap.
            - would need to be made aware of the hashmap in a read-only fashion.
        2) Output from mem_table is unsorted and it would be nice to return it in sorted order.
    */

    // initialize a HashMap to collect the results
    let mut mem_table: HashMap<i32, Vec<i32>> = HashMap::new();

    let (start, end) = collect_input();

    let boundaries = RangeBounds { start, end };

    for num in boundaries.start..=boundaries.end {
        if num <= 0 {
            println!("Please use a positive integer greater than zero!");
            exit(1);
        }
        let num_vector: Vec<i32> = produce_vec(num, &mem_table);
        // This line will check to see if our key exists in the HashMap. If it does not, .or_insert() will insert a value of our choice into it at the provided key.
        mem_table.entry(num).or_insert(num_vector);
    }

    for (key, value) in mem_table.iter() {
        println!("{key}: {:?}\n", value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collatz_with_one() {
        assert_eq!(collatz(1), 1);
    }

    #[test]
    fn test_collatz_with_two() {
        assert_eq!(collatz(2), 1);
    }

    #[test]
    fn test_collatz_with_three() {
        assert_eq!(collatz(3), 10);
    }

    #[test]
    fn test_produce_vec_with_one() {
        let mut mem_table: HashMap<i32, Vec<i32>> = HashMap::new();
        let expected: Vec<i32> = vec![1];
        assert_eq!(produce_vec(1, &mem_table), expected);
    }

    #[test]
    fn test_produce_vec_with_eighteen() {
        let mut mem_table: HashMap<i32, Vec<i32>> = HashMap::new();
        let expected: Vec<i32> = vec![
            18, 9, 28, 14, 7, 22, 11, 34, 17, 52, 26, 13, 40, 20, 10, 5, 16, 8, 4, 2, 1,
        ];
        assert_eq!(produce_vec(18, &mem_table), expected);
    }

    // TODO: Add tests for collecting user input - 0, negative numbers, floats, strings, etc.

    /*
        TODO: Currently lacking the knowledge of how to set up and test an early exit
        for a user-input of 0, or a negative number, or a float.
    */
}
