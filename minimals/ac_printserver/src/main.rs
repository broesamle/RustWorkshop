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
    let printqueue_thr = printqueue_arc.clone();
    threads.push(thread::spawn(move || {
        for job in printqueue_thr.iter() {
            thread::sleep(Duration::from_millis(1));
            println!("print queue: {}", job);
        }
    }));
    for num in 0..7 {
        let printqueue_thr = printqueue_arc.clone();
        threads.push(thread::spawn(move || {
            println!("Hello from thread number {}, I am interested in {}.", num, printqueue_thr[num]);
        }));
        println!("Started thread number {:?}.", num);
    }
    while let Some(thr) = threads.pop() {
        let _ = thr.join();
        println!("Good bye.");
    }
}
