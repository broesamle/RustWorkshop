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
