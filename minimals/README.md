Minimal Instructives
====================
A collection of small instructive examples demonstrating issues related to
multithreadding in rust.

Console output from multiple threads
------------------------------------

### How it does not work

```
use std::thread;

fn main() {
    for num in 0..10 {
        thread::spawn(move || { println!("Thread {}", num); });
    }
}
```

We get a different output on every run:
```
[~/projets/RustWorkshop/minimals/aa_multiprint]$ cargo run
    Finished debug [unoptimized + debuginfo] target(s) in 0.1 secs
     Running `target/debug/aa_multiprint`
Thread 0
Thread 2
Thread 3
[~/projets/RustWorkshop/minimals/aa_multiprint]$ cargo run
    Finished debug [unoptimized + debuginfo] target(s) in 0.1 secs
     Running `target/debug/aa_multiprint`
Thread 0
Thread 2
Thread 1
Thread 4
Thread 3
Thread 5
thread '<unnamed>thread '<unnamed>thread '' panicked at 'cannot access stdout during shutdown', src/libcore/option.rs:<unnamed>700[~/projets/RustWorkshop/minimals/aa_multiprint]$ cargo run
    Finished debug [unoptimized + debuginfo] target(s) in 0.1 secs
     Running `target/debug/aa_multiprint`
Thread 0
Thread 2
Thread 1
Thread 5
Thread 4
[~/projets/RustWorkshop/minimals/aa_multiprint]$ cargo run
    Finished debug [unoptimized + debuginfo] target(s) in 0.1 secs
     Running `target/debug/aa_multiprint`
Thread 0
Thread 1
Thread 2
Thread 3
Thread 4
[~/projets/RustWorkshop/minimals/aa_multiprint]$ cargo run
    Finished debug [unoptimized + debuginfo] target(s) in 0.1 secs
     Running `target/debug/aa_multiprint`
Thread 0
Thread 1
Thread 2
Thread 5
Thread 4
Thread 6
[~/projets/RustWorkshop/minimals/aa_multiprint]$
```

Occasionally we get:
```
thread '<unnamed>' panicked at 'cannot access stdout during shutdown', src/libcore/option.rs:700
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```
indicating that the standard output was already 'broken' while the child process still wanted to use it.

To understand more of what is going on, we collect the result of the spawning in a vector and we do more verbose output:

```
use std::thread;

fn main() {
    let mut threads = Vec::new();
    for num in 0..10 {
        threads.push(thread::spawn(move || {
            println!("Hello from thread number {}", num);
        }));
        println!("Started thread number {:?}.", num);
    }
    while let Some(thr) = threads.pop() {
        println!("Good bye {:?}.", thr.thread());
    }
}

```
Despite the fact that occasionally messages are missing from the output (here `Hello from thread number 9`) all threads are correctly started and there is always 10 elements in `vec`.
```
Started thread number 0.
Started thread number 1.
Started thread number 2.
Hello from thread number 1
Hello from thread number 0
Started thread number 3.
Hello from thread number 2
Hello from thread number 3
Started thread number 4.
Hello from thread number 4
Started thread number 5.
Hello from thread number 5
Started thread number 6.
Hello from thread number 6
Started thread number 7.
Hello from thread number 7
Started thread number 8.
Hello from thread number 8
Started thread number 9.
Good bye None.
Good bye None.
Good bye None.
Good bye None.
Good bye None.
Good bye None.
Good bye None.
Good bye None.
Good bye None.
Good bye None.
```

### How it works: explicitly joining the threads.

When explicitly joining all the threads (forcing the main thread with stdout to wait for the children to finish) the messages from the threads are all properly printed (in varying order, though).

```
while let Some(thr) = threads.pop() {
    thr.join();
    println!("Good bye.");
}
```

cf. [threads at rustbyexample](http://rustbyexample.com/std_misc/threads.html)


Minimal 'print server'
----------------------

### A non-terminating thread looking for print jobs

The following example introduces one thread which `loop`s infinitely, looking for 'print jobs' (strings) in the `printqueue`.
```
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
```
This is a typical output:

```
Started thread number 0.
Started thread number 1.
Started thread number 2.
Hello from thread number 1
Hello from thread number 2
printing: testpage7
printing: testpage6
printing: testpage5
printing: testpage4
printing: testpage3
printing: testpage2
printing: testpage1
Started thread number 3.
Hello from thread number 0
Hello from thread number 3
Started thread number 4.
Hello from thread number 4
Started thread number 5.
Hello from thread number 5
Started thread number 6.
Started thread number 7.
Started thread number 8.
Hello from thread number 8
Started thread number 9.
Hello from thread number 9
Hello from thread number 7
Good bye.
Hello from thread number 6
Good bye.
Good bye.
Good bye.
Good bye.
Good bye.
Good bye.
Good bye.
Good bye.
Good bye.
^C
[~/projets/RustWorkshop/minimals/ac_printserve]$
```
We recognise that we have to terminate the program execution hitting `Ctrl+C` (last two lines).


### Print from other threads via the `printqueue`

This requires other threads to have (write) access to `printqueue`!!!

Rust will certainly prevent us from this solution for good reasons:
```
. . .
for num in 0..10 {
    threads.push(thread::spawn(move || {
        let l = printqueue.len();
        println!("Hello from thread number {}, there are {} jobs in the queue.", num, l);
    }));
. . .
```
We have moved the `printqueue` when we started the first (printer) thread. It is, hence, no longer available for others to read (or write to) it directly.


#### Concurrent read access

Instead of moving the whole queue we can just move (multiple copies) of references to one (shared) queue.
```
fn main() {
    let mut threads = Vec::new();
    let mut printqueue: Vec<&str> = Vec::new();
    let printqueue_arc = Arc::new(printqueue);
    printqueue.push("testpage1");
    printqueue.push("testpage2");
. . .
```
Fine, but the compiler now complains (seven times) about
```
use of moved value: `printqueue`
```
After having created the reference, the original `printqueue` is no longer available, i.e. for pushing `testpage1..7` to it. We address this as follows:
* Replace the endless loop for an iteration over all elements in `printqueue`
* Do this without `pop` so that we do not need a mutable reference

```
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
            // Fiddling around with the timing allowes to see
            // how the for loop runs in parallel with the other threads started below
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
```

The output now looks like this:
```
Started thread number 0.
Started thread number 1.
Hello from thread number 0, I am interested in testpage1.
print queue: testpage1
print queue: testpage2
print queue: testpage3
Started thread number 2.
print queue: testpage4
print queue: testpage5
print queue: testpage6
print queue: testpage7
Hello from thread number 1, I am interested in testpage2.
Hello from thread number 2, I am interested in testpage3.
Started thread number 3.
Started thread number 4.
Hello from thread number 3, I am interested in testpage4.
Hello from thread number 4, I am interested in testpage5.
Started thread number 5.
Hello from thread number 5, I am interested in testpage6.
Started thread number 6.
Hello from thread number 6, I am interested in testpage7.
Good bye.
Good bye.
Good bye.
Good bye.
Good bye.
Good bye.
Good bye.
Good bye.
```

#### Concurrent write access

This is something that, at first sight should not be possible anyway... at least not without careful [locking discipline](http://stackoverflow.com/questions/23350954/why-does-rust-have-mutexes-and-other-sychronization-primitives-if-sharing-of-mu). This is what `Mutex` gives us.

Instead of an `Arc` with the `printqueue` in it directly we are now using an `Arc` with a `Mutex` which in turn 'holds' the `printqueue`.

```
use std::sync::{Arc, Mutex};

. . .

    let printqueue_shared = Arc::new(Mutex::new(printqueue));
    let printqueue_thr = printqueue_shared.clone();
```
Please note the renamed variable `printqueue_shared`.

When using the queue we need:
* `let guard = printqueue_thr.lock().unwrap();`
* `*guard` instead of `printqueue_thr`

```
threads.push(thread::spawn(move || {
    let guard = printqueue_thr.lock().unwrap();
    for job in (*guard).iter() {
        thread::sleep(Duration::from_millis(1));
```
. . . and similarly
```
let guard = printqueue_thr.lock().unwrap();
println!("Hello from thread number {}, I am interested in {}.", num, (*guard)[num]);
```


The overall program after introducing Mutex:
```
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
```
