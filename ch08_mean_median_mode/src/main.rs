use std::collections::HashMap;

fn main() {
    let v: Vec<i32> = vec![2, 1, 3, 3];
    // mean
    let sum: i32 = v.iter().sum();
    let mean = sum / v.len() as i32;
    println!("mean: {}", mean);
    // median
    let mut v2 = v.to_vec();
    v2.sort();
    let median = v2[v2.len() / 2];
    println!("median: {:?}", median);
    // mode
    let mut map = HashMap::new();
    for val in v.iter() {
        let count = map.entry(val).or_insert(0);
        *count += 1;
    }
    // TODO implement Option to handle case when no values are found
    let mut highest_count = 0;
    let mut mode = 0;
    for (key, value) in &map {
        if value > &highest_count {
            highest_count = *value;
            mode = **key;
        }
    }
    println!("mode is {:?} with a count of {:?}", mode, highest_count);
}
