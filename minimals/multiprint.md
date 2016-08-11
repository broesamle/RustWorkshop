----
TODO: This branch needs to be restructured into the minimal examples scheme with parallel branches for each example.

TODO: Also, There is an instructive pitfall with a semicolon in an if let statement that is worth explaining in more detail (return value vs. no return value!!)

----

Minimal Instructives
====================

A collection of small instructive examples demonstrating issues related to
multithreadding in rust.

I should mention that this is not (yet) meant as a stand-alone tutorial but as a collection of small examples touching on key concepts a rust learner will encounter. The first example, despite the fact of being not even 50 lines of rust code, introduces
* Multiple threads running concurrently
* Have the main thread `join` its child threads in order to keep central ressources (here `stdout`) available for the child threads
* Shared ressources
* Locking of shared ressources
* Variable skopes and implicit unlocking via `MutexGuard` and `drop`
* Compile time / runtime
* Static Strings vs. dynamically generated strings
* Lifetime


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

Despite the fact that occasionally messages are missing from the output (here `Hello from thread number 9`) all threads are correctly started, there is always 10 elements in `vec` as we always see 10 `Good bye`s.

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

We recognise that we have to terminate the program execution hitting `Ctrl+C`.


### Print from other threads via the `printqueue`

This requires other threads to have access to `printqueue`!!!

Rust will certainly prevent us from this naive solution, for good reasons:

```
. . .
for num in 0..10 {
    threads.push(thread::spawn(move || {
        let l = printqueue.len();
        println!("Hello from thread number {}, there are {} jobs in the queue.", num, l);
    }));
. . .
```

We have moved the `printqueue` when we started the first (printer) thread. Hence, It is no longer available for others to read (or write to) it directly.


#### Concurrent read access

Before thinking about concurrent write (mutable) access to the queue I would like to focus for any form of shared access across threads to the same ressource. Reading is always easier because no changes can happen while reading data. [TODO: Link+hint to rusts rules of (im)mutable references borrowing](xxxxxxxxxx)
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

After having created the reference, the original `printqueue` is no longer available, i.e. for pushing `testpage1..7` to it.

We address this as follows:
* Replace the endless loop for an iteration over all elements in `printqueue`
* Do this without `pop` so that we do not need a mutable reference.

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

For testing that we have actually write access to the queue:
* Introduce the endless loop for printing jobs of some previous example again.
* Check for successful locking `if let Ok(guard) =`
* Have the `guard` mutable
* Change the queue by actuall `pop`ing  print jobs

```
loop {
    if let Ok(mut guard) = printqueue_thr.lock() {
        if let Some (printjob) = (*guard).pop() {
            println!("printing: {}", printjob);
        }
    }
    thread::sleep(Duration::from_millis(3));    // Modify this value to achieve timing overlap between threads.
}
```

For the other threads, we also check
* for successful locking (see above)
* whether the queue has enough elements so that we can retrieve 'our' job `(*guard)[num]`.

```
        if let Ok(guard) = printqueue_thr.lock() {
            if num < (*guard).len() {
                println!("Hello from thread number {}, job {} is there.", num, (*guard)[num]);
            }
            else {
                println!("Hello from thread number {}, could not retreive job.", num);
            } // <<-- lock implicitly released by `guard` going out of scope.
        };
```

The `;` after the `if let` block is easlily overlooked but important!

To conclude, we have a server running in an infinite loop, and printing one job from the queue, if there is any. It is important that the infinit loop locks and (implicitly) releases the queue in every round, when `guard` goes out of scope. Otherwise the 'other threads' had no chance of accessing (locking) the queue.


### Client threads feed jobs to a print server

In order to implement and demonstrate a client-server scheme we modify the 'other threads' so that each of them also
* runs in an infinite `loop`.
* from time to time `push` ing a `job` in the `queue` `(*guard).push("Some Print Job.");`.

For monitoring the dynamics between threads we
* have a look at the queue whenever the server enters the loop: `println! ("The Queue: {:?}", (*guard));`.
* adapt the sleep duration in the server and client loop so that things don't run too fast to observe.
* add a counter variable in each client thread `let mut i = 0;` increase it on every new job `i += 1` and print a message for every new job.
* delay the print clients differently so that they put their jobs in different intervals.

The overall program:

```
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
                    println!("Hello from thread number {}, I will put job number {}.", num, i);
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
```
This is the console output of a typical run:
```
Hello from thread number 0, I will put job number 1.
Hello from thread number 2, I will put job number 1.
The Queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7", "Some Print Job.", "Some Print Job."]
printing: Some Print Job.
Hello from thread number 3, I will put job number 1.
Hello from thread number 1, I will put job number 1.
Hello from thread number 4, I will put job number 1.
Hello from thread number 6, I will put job number 1.
Hello from thread number 5, I will put job number 1.
The Queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7", "Some Print Job.", "Some Print Job.", "Some Print Job.", "Some Print Job.", "Some Print Job.", "Some Print Job."]
printing: Some Print Job.
The Queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7", "Some Print Job.", "Some Print Job.", "Some Print Job.", "Some Print Job.", "Some Print Job."]
printing: Some Print Job.
The Queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7", "Some Print Job.", "Some Print Job.", "Some Print Job.", "Some Print Job."]
printing: Some Print Job.
Hello from thread number 0, I will put job number 2.
The Queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7", "Some Print Job.", "Some Print Job.", "Some Print Job.", "Some Print Job."]
printing: Some Print Job.
The Queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7", "Some Print Job.", "Some Print Job.", "Some Print Job."]
printing: Some Print Job.
The Queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7", "Some Print Job.", "Some Print Job."]
printing: Some Print Job.
Hello from thread number 0, I will put job number 3.
Hello from thread number 1, I will put job number 2.
The Queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7", "Some Print Job.", "Some Print Job.", "Some Print Job."]
printing: Some Print Job.
The Queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7", "Some Print Job.", "Some Print Job."]
printing: Some Print Job.
The Queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7", "Some Print Job."]
printing: Some Print Job.
Hello from thread number 2, I will put job number 2.
Hello from thread number 0, I will put job number 4.
The Queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7", "Some Print Job.", "Some Print Job."]
printing: Some Print Job.
The Queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7", "Some Print Job."]
printing: Some Print Job.
The Queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
printing: testpage7
The Queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6"]
printing: testpage6
Hello from thread number 0, I will put job number 5.
Hello from thread number 3, I will put job number 2.
Hello from thread number 1, I will put job number 3.
The Queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "Some Print Job.", "Some Print Job.", "Some Print Job."]
printing: Some Print Job.
The Queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "Some Print Job.", "Some Print Job."]
printing: Some Print Job.
The Queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "Some Print Job."]
printing: Some Print Job.
Hello from thread number 0, I will put job number 6.
Hello from thread number 4, I will put job number 2.
The Queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "Some Print Job.", "Some Print Job."]
printing: Some Print Job.
The Queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "Some Print Job."]
printing: Some Print Job.
The Queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5"]
printing: testpage5
Hello from thread number 2, I will put job number 3.
Hello from thread number 0, I will put job number 7.
The Queue: ["testpage1", "testpage2", "testpage3", "testpage4", "Some Print Job.", "Some Print Job."]
printing: Some Print Job.
Hello from thread number 1, I will put job number 4.
Hello from thread number 5, I will put job number 2.
The Queue: ["testpage1", "testpage2", "testpage3", "testpage4", "Some Print Job.", "Some Print Job.", "Some Print Job."]
printing: Some Print Job.
The Queue: ["testpage1", "testpage2", "testpage3", "testpage4", "Some Print Job.", "Some Print Job."]
printing: Some Print Job.
The Queue: ["testpage1", "testpage2", "testpage3", "testpage4", "Some Print Job."]
printing: Some Print Job.
Hello from thread number 0, I will put job number 8.
Hello from thread number 6, I will put job number 2.
The Queue: ["testpage1", "testpage2", "testpage3", "testpage4", "Some Print Job.", "Some Print Job."]
printing: Some Print Job.
The Queue: ["testpage1", "testpage2", "testpage3", "testpage4", "Some Print Job."]
printing: Some Print Job.
The Queue: ["testpage1", "testpage2", "testpage3", "testpage4"]
printing: testpage4
Hello from thread number 0, I will put job number 9.
Hello from thread number 3, I will put job number 3.
Hello from thread number 1, I will put job number 5.
The Queue: ["testpage1", "testpage2", "testpage3", "Some Print Job.", "Some Print Job.", "Some Print Job."]
printing: Some Print Job.
The Queue: ["testpage1", "testpage2", "testpage3", "Some Print Job.", "Some Print Job."]
printing: Some Print Job.
The Queue: ["testpage1", "testpage2", "testpage3", "Some Print Job."]
printing: Some Print Job.
Hello from thread number 2, I will put job number 4.
Hello from thread number 0, I will put job number 10.
The Queue: ["testpage1", "testpage2", "testpage3", "Some Print Job.", "Some Print Job."]
printing: Some Print Job.
The Queue: ["testpage1", "testpage2", "testpage3", "Some Print Job."]
printing: Some Print Job.
The Queue: ["testpage1", "testpage2", "testpage3"]
printing: testpage3
The Queue: ["testpage1", "testpage2"]
printing: testpage2
Hello from thread number 0, I will put job number 11.
Hello from thread number 1, I will put job number 6.
Hello from thread number 4, I will put job number 3.
The Queue: ["testpage1", "Some Print Job.", "Some Print Job.", "Some Print Job."]
printing: Some Print Job.
The Queue: ["testpage1", "Some Print Job.", "Some Print Job."]
printing: Some Print Job.
The Queue: ["testpage1", "Some Print Job."]
printing: Some Print Job.
Hello from thread number 0, I will put job number 12.
The Queue: ["testpage1", "Some Print Job."]
printing: Some Print Job.
The Queue: ["testpage1"]
printing: testpage1
The Queue: []
Hello from thread number 2, I will put job number 5.
Hello from thread number 0, I will put job number 13.
Hello from thread number 3, I will put job number 4.
Hello from thread number 1, I will put job number 7.
The Queue: ["Some Print Job.", "Some Print Job.", "Some Print Job.", "Some Print Job."]
printing: Some Print Job.
Hello from thread number 5, I will put job number 3.
The Queue: ["Some Print Job.", "Some Print Job.", "Some Print Job.", "Some Print Job."]
printing: Some Print Job.
The Queue: ["Some Print Job.", "Some Print Job.", "Some Print Job."]
printing: Some Print Job.
^C
```

As we can see, the server takes some time until the print queue is empty. If the loop of the server was 'too slow' the print queue would tend to grow 'infinitely'.


### Transfer non-trivial information


#### Lifetime, consumption and references

It would be nice if not all print jobs were the same `Some Print Job.`. How to transfer an interesting text to the server to print. It sounds trivial but it isn't:

Our job should look like this: `Printjob number 5 from thread 1`.

First modification lets the clients create an according text and -- for now -- output it by themselves via `println!`:

```
if let Ok(mut guard) = printqueue_thr.lock() {
    i += 1;
    let job = format! ("Printjob number {} from thread {}.", i, num);
    println!("I will put a job in the queue: {}", job);
    (*guard).push("Some Print Job.");
};
```

Next, we want to push it to the print queue instead of the trivial text `Some Print Job.`:

This `(*guard).push(job);` does not succeed because push wants a reference to a string instead of a string. The reason why it cannot take the string as it is lies in the fact that vectors need things of the same size (which is the case for references but not for strings.

```
src/main.rs:39:35: 39:38 error: mismatched types [E0308]
src/main.rs:39                     (*guard).push(job);
                                                 ^~~
src/main.rs:39:35: 39:38 help: run `rustc --explain E0308` to see a detailed explanation
src/main.rs:39:35: 39:38 note: expected type `&str`
src/main.rs:39:35: 39:38 note:    found type `std::string::String`
```
So we give it a reference to the job `(*guard).push(&job);` which, fortunately, is still not making the compiler happy:
```
src/main.rs:39:36: 39:39 error: `job` does not live long enough
src/main.rs:39                     (*guard).push(&job);
                                                  ^~~
src/main.rs:39:36: 39:39 note: reference must be valid for the static lifetime...
src/main.rs:37:86: 40:18 note: ...but borrowed value is only valid for the block suffix following statement 1 at 37:85
src/main.rs:37                     let job = format! ("Printjob number {} from thread {}.", i, num);
```

Why could we use the trivial message before but we cannot use the non-trivial one now? The trivial one was already known at compile time so that it _lives long enough_, namely for the whole program execution. We call this _lifetime_ `'static`.

For the non-trivial message, rust cannot be sure and, hence, stops us from carelessly using the reference.  `push`, in turn, is defined so as to _consume_ the reference for good reasons -- it needs the reference to point at something useful, something that _lives as long as the reference_, i.e. presumably longer as the client thread, potentially as long as the print queue.

Finally, we can use `String` instead of `&str` which is not bound to the _lifetime_ or _stackframe_ of the child thread:
* Define the printqueue as `let printqueue: Vec<String> = Vec::new();`.
* Push `String` rather than `&str` into the queue:
    `printqueue.push(String::from("testpage1");`
* Correspondingly, `job` receives a `String` from format so that `(*guard).push(job);` just works.