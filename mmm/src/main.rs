extern crate rand;

use rand::Rng;

fn main() {
    let mut num_list = Vec::new();

    for _ in (1..10).rev() {
        num_list.push(rand::thread_rng().gen_range(1, 100));
    }

    println!("{:?}", num_list);
}
