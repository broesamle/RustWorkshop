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
        loop {
            if let Ok(mut guard) = printqueue_thr.lock() {
                println! ("The Queue: {:?}", (*guard));
                if let Some (printjob) = (*guard).pop() {
                    println!("printing: {}", printjob);
                }
            }
            thread::sleep(Duration::from_millis(30));
        }
    }));
    for num in 0..7 {
        let printqueue_thr = printqueue_shared.clone();
        threads.push(thread::spawn(move || {
            let mut i = 0;
            loop {
                if let Ok(mut guard) = printqueue_thr.lock() {
                    i += 1;
                    let job = format! ("Printjob number {} from thread {}.", i, num);
                    println!("I will put a job in the queue: {}", job);
                    (*guard).push("Some Print Job.");
                };
                thread::sleep(Duration::from_millis(100*(num+1)));
            }
        }));
    }
    while let Some(thr) = threads.pop() {
        let _ = thr.join();
    }
}
