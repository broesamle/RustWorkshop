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
                if let Some (printjob) = (*guard).pop() {
                    println!("printing: {}", printjob);
                }
            }
            thread::sleep(Duration::from_millis(3));
        }
    }));
    for num in 0..7 {
        let printqueue_thr = printqueue_shared.clone();
        threads.push(thread::spawn(move || {
            if let Ok(guard) = printqueue_thr.lock() {
                if num < (*guard).len() {
                    println!("Hello from thread number {}, job {} is there.", num, (*guard)[num]);
                }
                else {
                    println!("Hello from thread number {}, could not retreive job.", num);
                }
            };
        }));
    }
    while let Some(thr) = threads.pop() {
        let _ = thr.join();
    }
}
