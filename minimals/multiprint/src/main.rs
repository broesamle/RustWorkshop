use std::thread;
use std::sync::Arc;
use std::time::Duration;

fn main() {
    let mut threads = Vec::new();
    let mut printqueue: Vec<&str> = Vec::new();
    printqueue.push("testpage1");
    printqueue.push("testpage2");
    printqueue.push("testpage3");
    printqueue.push("testpage4");
    printqueue.push("testpage5");
    printqueue.push("testpage6");
    printqueue.push("testpage7");
    let printqueue_arc = Arc::new(printqueue);
    let serverqueue = printqueue_arc.clone();
    let server = thread::spawn(move || {
        loop {
            println!("print queue: {:?}", serverqueue);
            thread::sleep(Duration::from_millis(20));
        }
    });
    for num in 0..10 {
        let clientqueue = printqueue_arc.clone();
        thread::sleep(Duration::from_millis(50)); // we spawn a new threads every 50 msec

        let handle = thread::spawn(move || {
            println!("Thread {} can read the print queue: {:?}", num, clientqueue);
            thread::sleep(Duration::from_millis(100));  // each thread first sleeps for 100 msec
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
