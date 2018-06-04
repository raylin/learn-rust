use std::thread;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("#{} from spawned thread", i);
        }
    });

    for i in 1..10 {
        println!("#{} from main thread", i);
    }
}
