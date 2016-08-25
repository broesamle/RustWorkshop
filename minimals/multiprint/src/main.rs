use std::thread;
use std::time::Duration;

fn main() {
    let mut threads = Vec::new();
    let server = thread::spawn(move || {
        loop {
            println!("infinite loop alive.");
            thread::sleep(Duration::from_millis(20));
        }
    });
    for num in 0..10 {
        thread::sleep(Duration::from_millis(50)); // we spawn a new threads every 50 msec
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));  // each thread first sleeps for 100 msec
            println!("Thread {}", num);
        });
        threads.push(handle);
        println!("Started thread number {:?}.", num);
    }
    println!("Vector of {} join handles.", threads.len());
    for num in (0..10).rev() {
        let thr = threads.remove(num);
        let joinresult = thr.join();
        println!("Joined thread number {:?}, {:?}.", num, joinresult);
    }
    let joinresult = server.join();
    println!("Joined server thread: {:?}.", joinresult);
}
