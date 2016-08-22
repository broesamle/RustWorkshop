use std::thread;

fn main() {
    let mut threads = Vec::new();
    for num in 0..10 {
        let handle = thread::spawn(move || { println!("Thread {}", num); });
        threads.push(handle);
        println!("Started thread number {:?}.", num);
    }
    println!("Vector of {} join handles.", threads.len());
}
