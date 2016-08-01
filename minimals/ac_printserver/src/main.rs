use std::thread;

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

    threads.push(thread::spawn(move || {
        loop {
            while let Some(printjob) = printqueue.pop() {
                println!("printing: {}", printjob);
            }
        }
    }));
    for num in 0..10 {
        threads.push(thread::spawn(move || {
            println!("Hello from thread number {}", num);
        }));
        println!("Started thread number {:?}.", num);
    }
    while let Some(thr) = threads.pop() {
        let _ = thr.join();
        println!("Good bye.");
    }
}
