extern crate rand;

use std::cmp::Ordering;
use std::collections::HashMap;
use rand::Rng;

const MAX_LENGTH: u32 = 100;

fn main() {
    let mut num_list = Vec::new();
    let mut sum = 0;
    let mut max = 0;
    let mut num_counter_table = HashMap::new();

    for _ in (1..MAX_LENGTH).rev() {
        num_list.push(rand::thread_rng().gen_range(1, 10));
    }

    num_list.sort_by(|a, b| a.partial_cmp(b).unwrap());

    for num in &num_list {
        let count = num_counter_table.entry(num.to_string()).or_insert(0);
        *count += 1;
        sum = sum + num;
        if let Ordering::Greater = num.cmp(&max) {
            max = *num;
        }
    }

    let mut counter = 0;
    let mut mod_num = String::new();
    for (num, count) in &num_counter_table {
        if let Ordering::Greater = count.cmp(&counter) {
            counter = *count;
            mod_num = num.clone();
        }
    }
    let median = ((MAX_LENGTH as f64) / 2.0) as usize;

    println!("number list: {:?}", num_list);
    println!("means: {}", (sum as f64) / (MAX_LENGTH as f64));
    println!("max: {}", max);
    println!("mod: {}", mod_num);
    println!("median: {}", &num_list[median]);
}
