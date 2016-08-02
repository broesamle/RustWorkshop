use std::thread;
use std::sync::{Arc, Mutex};
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

    let printqueue_shared = Arc::new(Mutex::new(printqueue));
    let printqueue_thr = printqueue_shared.clone();
    threads.push(thread::spawn(move || {
        let guard = printqueue_thr.lock().unwrap();
        for job in (*guard).iter() {
            thread::sleep(Duration::from_millis(1));
            println!("print queue: {}", job);
        }
    }));
    for num in 0..7 {
        let printqueue_thr = printqueue_shared.clone();
        threads.push(thread::spawn(move || {
            let guard = printqueue_thr.lock().unwrap();
            println!("Hello from thread number {}, I am interested in {}.", num, (*guard)[num]);
        }));
        println!("Started thread number {:?}.", num);
    }
    while let Some(thr) = threads.pop() {
        let _ = thr.join();
    }
}
