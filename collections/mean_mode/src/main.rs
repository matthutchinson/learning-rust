// Given a list of integers, use a vector and return the mean (the average value), median (when
// sorted, the value in the middle position), and mode (the value that occurs most often; a hash
// map will be helpful here) of the list.

use std::collections::HashMap;

fn calculate_mean(vec: &Vec<i32>) -> f32 {
    let sum: i32 = vec.iter().sum();
    sum as f32 / vec.len() as f32
}

fn calculate_median(vec: &Vec<i32>) -> i32 {
    let mut sorted:Vec<i32> = vec.to_vec();
    sorted.sort();

    let midpoint = sorted.len() / 2;
    sorted[midpoint]
}

fn calculate_mode(vec: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    let mut high_count = 0;
    let mut mode = vec[0];

    for num in vec {
        let count = map.entry(num).or_insert(0);
        *count += 1;
        // or above two lines could be swapped with this instead
        // map.entry(num).and_modify(|count| {*count += 1}).or_insert(1);

        if count > &mut high_count {
            high_count = *count;
            mode = *num;
        }
    }

    mode
}

fn main() {
    let v = vec![1, 13, 75, 123, 14, 12, 12, 75, 99, 75, 1, 1];

    let mean:f32 = calculate_mean(&v);
    let median:i32 = calculate_median(&v);
    let mode:i32 = calculate_mode(&v);

    println!("Mean: {}", mean);
    println!("Median: {}", median);
    println!("Mode: {}", mode);
}
