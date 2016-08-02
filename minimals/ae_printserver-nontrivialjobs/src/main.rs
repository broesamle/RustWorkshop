use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let mut threads = Vec::new();
    let mut printqueue: Vec<String> = Vec::new();

    printqueue.push(String::from("testpage1"));
    printqueue.push(String::from("testpage2"));
    printqueue.push(String::from("testpage3"));
    printqueue.push(String::from("testpage4"));
    printqueue.push(String::from("testpage5"));
    printqueue.push(String::from("testpage6"));
    printqueue.push(String::from("testpage7"));

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
                    (*guard).push(job);
                };
                thread::sleep(Duration::from_millis(100*(num+1)));
            }
        }));
    }
    while let Some(thr) = threads.pop() {
        let _ = thr.join();
    }
}
