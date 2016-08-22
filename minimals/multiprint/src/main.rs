use std::thread;

fn main() {
    for num in 0..10 {
        thread::spawn(move || { println!("Thread {}", num); });
        println!("Started thread number {:?}.", num);
    }
}
