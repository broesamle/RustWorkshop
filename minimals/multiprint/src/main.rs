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
    let printqueue_mutex_arc = Arc::new(Mutex::new(printqueue));
    let serverqueue = printqueue_mutex_arc.clone();
    let server = thread::spawn(move || {
        let mut i = 0;
        loop {
            println!("[{}] print queue: {:?}", i, serverqueue);
            if let Ok(mut guard) = serverqueue.try_lock() {
                if let Some(printjob) = (*guard).pop() {
                    println!("printing: {}", printjob);
                }
            }
            thread::sleep(Duration::from_millis(100));
            i += 1;
        }
    });
    for num in 0..10 {
        let clientqueue = printqueue_mutex_arc.clone();
        let handle = thread::spawn(move || {
            loop {
                println!("Child {}...", num);
                if let Ok(mut guard) = clientqueue.try_lock() {
                    println!("...putting job");
                    (*guard).push("Some Print Job.");
                }
                thread::sleep(Duration::from_millis(500*(num+1)));
            }
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
