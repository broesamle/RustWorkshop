

Multiprint
==========

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


### Step 1: How it does not work

#### [Snapshot] Step 1a:
TODO: Label for
https://github.com/broesamle/RustWorkshop/commit/d08ef2617c35913cce417e1e3048f4598a0e11a9

#### [Testing] Step 1a:
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

. . .

```

Occasionally we get:

```
thread '<unnamed>' panicked at 'cannot access stdout during shutdown', src/libcore/option.rs:700
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```
indicating that something (the standard output component) was (already) 'broken' while some (the child) process (still) wanted to use it.


To figure out what is going on here, we follow a common strategy in debugging/understanding information processing systems: Get more information about the actual processing steps -- make our app more 'talkative'.

Let us put some console output after starting each new thread: `println!("Started thread number {:?}.", num);`

#### [Snapshot] Step 1b
TODO: Label for
https://github.com/broesamle/RustWorkshop/commit/99eeef5e603b5e8cf5c6f415dfb6972cd46fcdd5

#### [Testing] Step 1b
This gives us the following output:

```
[~/projets/RustWorkshop/minimals/multiprint]$ cargo run
   Compiling multiprint v0.1.0 (file:///home/broe/projets/RustWorkshop/minimals/multiprint)
    Finished debug [unoptimized + debuginfo] target(s) in 0.49 secs
     Running `target/debug/multiprint`
Started thread number 0.
Started thread number 1.
Started thread number 2.
Thread 0
Started thread number 3.
Thread 2
Thread 3
Thread 1
Started thread number 4.
Started thread number 5.
Thread 4
Started thread number 6.
Started thread number 7.
Started thread number 8.
Thread 6
Thread 8
Thread 5
Started thread number 9.
Thread 9
Thread 7
[~/projets/RustWorkshop/minimals/multiprint]$ cargo run
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/multiprint`
Started thread number 0.
Started thread number 1.
Started thread number 2.
Thread 0
Thread 1
Thread 2
Started thread number 3.
Started thread number 4.
Started thread number 5.
Started thread number 6.
Thread 3
Thread 6
Thread 4
Thread 5
Started thread number 7.
Started thread number 8.
Started thread number 9.
Thread 8
Thread 7
[~/projets/RustWorkshop/minimals/multiprint]$
```

We see that all threads are started (initiated, `spawn`ed) in both runs but we see some console output missing (here: `Thread 9`). So, number nine is spawned but its effort to print a message to the console is not successful -- presumably because the the 'something' is already shot down when the threads want to use it.

What has to happen here is to prevent the main program to be already shod down while there is still children doing their businesses, using the (console printing) infrastructure.

### Step 2: How it does work -- _stay in touch_

When starting a new thread, a handle is returned in order to _stay in touch_ with the child thread. As a first step towards understanding this mechanism, we collect these _handles_ in a vector.


What will be achieved in _Step 2_ is to _join_ the child threads -- that is, execution can only continue after the child has finished its business. We will see in a minute how this improves things in our case.

To be able to do so we collect the handles in a vector.

#### [Snapshot] Step 2a
TODO: Label for
https://github.com/broesamle/RustWorkshop/commit/342c98a32508069ee8d3a28bb277c0b0e39f6531

#### [Testing] Step 2a
Output:
```
Started thread number 0.
Started thread number 1.
Started thread number 2.
Started thread number 3.
Thread 0
Thread 1
Thread 2
Started thread number 4.
Thread 4
Thread 3
Started thread number 5.
Thread 5
Started thread number 6.
Thread 6
Started thread number 7.
Started thread number 8.
Thread 7
Thread 8
Started thread number 9.
Vector of 10 join handles.
Thread 9
```

Despite the fact we already know (and can print) the length of that vector (after spawning all threads) the threads still take their time to get done their individual console outputs. Again, the output of the late birds being dropped, occasionally.


So we run a second loop through all the handles. As an additional control, we print a message for each thread.

#### [Snapshot] Step 2b
TODO Label for
https://github.com/broesamle/RustWorkshop/commit/a4df5c004a949c03aec3bce6272d6bcff137f464

#### [Testing] Step 2b

Building gives us:
```
[~/projets/RustWorkshop/minimals/multiprint]$ cargo build
   Compiling multiprint v0.1.0 (file:///home/broe/projets/RustWorkshop/minimals/multiprint)
src/main.rs:12:13: 12:16 warning: unused variable: `thr`, #[warn(unused_variables)] on by default
src/main.rs:12         let thr = threads.remove(num);
                           ^~~
    Finished debug [unoptimized + debuginfo] target(s) in 0.56 secs
```

hmhmhm 'just a warning' so let's test ;-)

```
[~/projets/RustWorkshop/minimals/multiprint]$ cargo run
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/multiprint`
Started thread number 0.
Started thread number 1.
Started thread number 2.
Started thread number 3.
Thread 1
Thread 2
Thread 0
Started thread number 4.
Thread 3
Thread 4
Started thread number 5.
Thread 5
Started thread number 6.
Thread 6
Started thread number 7.
Started thread number 8.
Thread 8
Started thread number 9.
Vector of 10 join handles.
Joined thread number 9.
Thread 7
Joined thread number 8.
Joined thread number 7.
Joined thread number 6.
Joined thread number 5.
Joined thread number 4.
Joined thread number 3.
Joined thread number 2.
Joined thread number 1.
Joined thread number 0.
Thread 9
```
Weird. After joining thread `9..0` there is still a message coming in from thread 9.

Reading the warning carefully -- which I strongly recommend -- tells us exactly what is missing here. `thr` is not used anywhere, i.e. not for joining the threads. In a hurry we forgot the essential part, the `join`.



#### [Snapshot] Step 2c
TODO Label for
https://github.com/broesamle/RustWorkshop/commit/56fcd406414e5eb646291930cda735ee7ce60593

#### [Testing] Step 2c
It builds:
```
[~/projets/RustWorkshop/minimals/multiprint]$ cargo build
   Compiling multiprint v0.1.0 (file:///home/broe/projets/RustWorkshop/minimals/multiprint)
src/main.rs:13:9: 13:20 warning: unused result which must be used, #[warn(unused_must_use)] on by default
src/main.rs:13         thr.join();
                       ^~~~~~~~~~~
    Finished debug [unoptimized + debuginfo] target(s) in 0.55 secs

```
Running it shows that it now works as expected. All outputs from the subthreads are present before the second loop finalises the execution of the main thread by joining the children.
```
Started thread number 0.
Started thread number 1.
Started thread number 2.
Started thread number 3.
Started thread number 4.
Thread 3
Thread 2
Thread 1
Thread 0
Thread 4
Started thread number 5.
Thread 5
Started thread number 6.
Thread 6
Started thread number 7.
Thread 7
Started thread number 8.
Thread 8
Started thread number 9.
Vector of 10 join handles.
Thread 9
Joined thread number 9.
Joined thread number 8.
Joined thread number 7.
Joined thread number 6.
Joined thread number 5.
Joined thread number 4.
Joined thread number 3.
Joined thread number 2.
Joined thread number 1.
Joined thread number 0.
```
Again there was a warning: Rust wants us to use the result of the joining -- presumably because this result can be quite important, from an overall system's functionality POV.

Here we go:

#### TODO: Snapshot 2d
https://github.com/broesamle/RustWorkshop/commit/7a9fcaf4b1db87160caf5cb80f919d8f53bdc7c2

#### TODO: Testing 2d

```
Started thread number 0.
Started thread number 1.
Thread 0
Thread 1
Started thread number 2.
Started thread number 3.
Started thread number 4.
Thread 3
Started thread number 5.
Thread 2
Started thread number 6.
Started thread number 7.
Thread 5
Thread 6
Thread 4
Started thread number 8.
Thread 7
Thread 8
Started thread number 9.
Vector of 10 join handles.
Thread 9
Joined thread number 9, Ok(()).
Joined thread number 8, Ok(()).
Joined thread number 7, Ok(()).
Joined thread number 6, Ok(()).
Joined thread number 5, Ok(()).
Joined thread number 4, Ok(()).
Joined thread number 3, Ok(()).
Joined thread number 2, Ok(()).
Joined thread number 1, Ok(()).
Joined thread number 0, Ok(()).
```
Looks nice ... all threds reply `Ok(())` :-)

What we have done here almost exactly matches the example on  
[threads at rustbyexample](http://rustbyexample.com/std_misc/threads.html).


Minimal 'print server'
----------------------

Add a _server_ thread running in an infinite loop (while the others are exiting after printing their message on the console).

#### [Snapshot] Step 3a
TODO Label for commit 3e759605dab677c7e98c563ea6a52a3caeb8aca4


#### [Testing] Step 3a

After running this version, all we see is:
```
infinite loop alive.
infinite loop alive.
infinite loop alive.
infinite loop alive.
infinite loop alive.
infinite loop alive.
infinite loop alive.
infinite loop alive.
infinite loop alive.
infinite loop alive.
infinite loop alive.
infinite loop alive.
infinite loop alive.
infinite loop alive.
infinite loop alive.
infinite loop alive.
infinite loop alive.
infinite loop alive.
infinite loop alive.
infinite loop alive.
infinite loop alive.
infinite loop alive.
infinite loop alive.
infinite loop alive.
infinite loop alive.
infinite loop alive.
infinite loop alive.
^C
```
And we have to stop it by pressing `Ctrl+C`. Even scrolling up the console window after stopping it really quickly does not show the messages of the other threads. (Did we mention that computers can do calculations _fast_?!

`cargo run > output.txt` helps us here. What we would otherwise see (or not see because it runs too fast) in the console now goes into a text file `output.txt`. It will be located in the `src/` subdirectory.

*Exercise:* It is quite instructive to have a look at the file which, on my machine after ~1 second of execution time produced more than 120000 lines of console output!

*Exercise:* Also, try to remove or comment out the second last two lines, which are responsible for joining the server thread. What output do you get? Explain, what happens.
```
//let joinresult = server.join();
//println!("Joined server thread: {:?}.", joinresult);
```

Next, let us slow down things a bit by adding some waiting times. Strictly speaking this is not necessary but it helps us using the console for output and seeing the temporal dynamics of multithreaded execution.


#### [Snapshot] Step 3b
TODO Label for commit 72a3f0b6ba87033942037ee450205880b6335c8c


#### [Testing] Step 3b
The exact numbers in the delays are more or less arbitrary -- the example 3b gives something like that as a typical output.

```
infinite loop alive.
infinite loop alive.
infinite loop alive.
Started thread number 0.
infinite loop alive.
infinite loop alive.
Started thread number 1.
infinite loop alive.
infinite loop alive.
infinite loop alive.
Thread 0
Started thread number 2.
infinite loop alive.
infinite loop alive.
Thread 1
infinite loop alive.
Started thread number 3.
infinite loop alive.
infinite loop alive.
Thread 2
Started thread number 4.
infinite loop alive.
infinite loop alive.
Thread 3
infinite loop alive.
Started thread number 5.
infinite loop alive.
infinite loop alive.
Thread 4
Started thread number 6.
infinite loop alive.
infinite loop alive.
Thread 5
infinite loop alive.
Started thread number 7.
infinite loop alive.
infinite loop alive.
Thread 6
Started thread number 8.
infinite loop alive.
infinite loop alive.
Thread 7
infinite loop alive.
Started thread number 9.
Vector of 10 join handles.
infinite loop alive.
infinite loop alive.
Thread 8
infinite loop alive.
infinite loop alive.
Thread 9
Joined thread number 9, Ok(()).
Joined thread number 8, Ok(()).
Joined thread number 7, Ok(()).
Joined thread number 6, Ok(()).
Joined thread number 5, Ok(()).
Joined thread number 4, Ok(()).
Joined thread number 3, Ok(()).
Joined thread number 2, Ok(()).
infinite loop alive.
Joined thread number 1, Ok(()).
Joined thread number 0, Ok(()).
infinite loop alive.
infinite loop alive.
infinite loop alive.
infinite loop alive.
infinite loop alive.
infinite loop alive.
```

### Step 4: Print jobs from a queue

After establishing one thread that runs for ever (which is the indended behaviour for a _server_ we can now think about something useful for it to do. As an example we will simulate a print server.


The next step establishes a printer queue.
For testing purposes we add a number of print jobs and then let the server run.

#### [Snapshot] Step 4a
TODO Label for commit aed3218f7f78acf7e44b38c22bd23ab4f85dc3c3


#### [Testing] Step 4a
Output proves that the server can read (and print to the console) the print queue.
```
print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
Started thread number 0.
print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
Started thread number 1.
print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
Thread 0
Started thread number 2.
print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
Thread 1
print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
Started thread number 3.
print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
```
Now, let us have it `pop` one job from the queue in every iteration of the loop and _print it_.

#### [Snapshot] Step 4b
TODO Label for commit


#### [Testing] Step 4b

```
print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
printing: testpage7
print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6"]
printing: testpage6
print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5"]
printing: testpage5
Started thread number 0.
print queue: ["testpage1", "testpage2", "testpage3", "testpage4"]
printing: testpage4
print queue: ["testpage1", "testpage2", "testpage3"]
printing: testpage3
print queue: ["testpage1", "testpage2"]
printing: testpage2
Started thread number 1.
print queue: ["testpage1"]
printing: testpage1
print queue: []
Thread 0
Started thread number 2.
print queue: []
print queue: []
print queue: []

. . .
```

Nice, the print queue is diminished job by job and the `testpage1..7` are printed (on the console). There is  a lot of code in this example which we currently do not need:

------------

TODO: Remove the following code block and put an *Exercise:* to remove the un-necessary parts.
```
for num in 0..10 {
    thread::sleep(Duration::from_millis(50)); // we spawn a new threads every 50 msec
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));  // each thread first sleeps for 100 msec
        println!("Thread {}", num);
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
```
We will keep them for the next steps, when the other threads will be _clients_ of the print _server_ thread.

------------


Concurrent access
-----------------

### Step 5: How it does not work


This requires other threads to have access to `printqueue`!!!

Rust will prevent us from this naive solution, (for good reasons):

```
. . .
for num in 0..10 {
    threads.push(thread::spawn(move || {
        let l = printqueue.len();
        println!("Hello from thread number {}, there are {} jobs in the queue.", num, l);
    }));
. . .
```

We have _moved_ the `printqueue` when we started the first (printer) thread. Hence, It is no longer available for others to read (or write to) it directly.

line 14 without the `move`: `let server = thread::spawn( || {` gives the following error:

```
Compiling multiprint v0.1.0 (file:///home/broe/projets/RustWorkshop/minimals/multiprint)
src/main.rs:14:33: 14:35 error: closure may outlive the current function, but it borrows `printqueue`, which is owned by the current function [E0373]
src/main.rs:14     let server = thread::spawn( || {
                                            ^~
src/main.rs:14:33: 14:35 help: run `rustc --explain E0373` to see a detailed explanation
src/main.rs:16:43: 16:53 note: `printqueue` is borrowed here
src/main.rs:16             println!("print queue: {:?}", printqueue);
                                                      ^~~~~~~~~~
<std macros>:2:27: 2:58 note: in this expansion of format_args!
<std macros>:3:1: 3:54 note: in this expansion of print! (defined in <std macros>)
src/main.rs:16:13: 16:55 note: in this expansion of println! (defined in <std macros>)
src/main.rs:14:33: 14:35 help: to force the closure to take ownership of `printqueue` (and any other referenced variables), use the `move` keyword, as shown:
src/main.rs        let server = thread::spawn( move || {
src/main.rs:27:13: 27:23 error: capture of moved value: `printqueue` [E0382]
src/main.rs:27         let handle = thread::spawn(move || {
                                               ^~~~~~~
src/main.rs:27:13: 27:23 help: run `rustc --explain E0382` to see a detailed explanation
src/main.rs:27:13: 27:23 note: move occurs because `printqueue` has type `std::vec::Vec<&'static str>`, which does not implement the `Copy` trait
error: aborting due to 2 previous errors
error: Could not compile `multiprint`.

To learn more, run the command again with --verbose.
```

For the moment we accept that things to be used in a spawned child thread need to be 'somehow moved' into that child thread.

So we put back the move and deal with less scary looking error message we get...

#### [Snapshot] Step 5a
TODO Label for commit 1b4da8ec9dafc1273cc3120a32e3c504b63f9093


#### [Testing] Step 5a
...this one:

```
src/main.rs:27:13: 27:23 error: capture of moved value: `printqueue` [E0382]
src/main.rs:27     let server = thread::spawn( move || {
                                               ^~~~~~~
src/main.rs:27:13: 27:23 help: run `rustc --explain E0382` to see a detailed explanation
src/main.rs:27:13: 27:23 note: move occurs because `printqueue` has type `std::vec::Vec<&'static str>`, which does not implement the `Copy` trait
```

The error message directs us to line number 27: `printqueue.push("testpage from client.");`
Temporarily removing this line makes the error disappear. So it is 'caused' at that line. Yet, the additional hint points to something else: `let server = thread::spawn(move || {` This is where the move occurs. As we already realised above: The move is somehow crucial.


### Step 6: Concurrent read access

Before thinking about concurrent write access to the (mutable) queue I would like to focus on _any_ form of shared access across threads to the queue. Reading is always easier because no changes can happen while reading data.

----------

[TODO: Link+hint to rusts rules of (im)mutable references borrowing](xxxxxxxxxx)

----------

So the first thing to do is to get read access to the queue from all threads, server and clients.

To do so, we remove the write access to the queue both, in the server and the children.
```
if let Some(printjob) = printqueue.pop() {
    println!("printing: {}", printjob);
}
```

```
printqueue.push("testpage from client.");
println!("Thread {} pushed a job into the queue.", num);
```

And add a read access in the clients:
```
println!("Thread {} can read the print queue: {:?}", num, printqueue);
```

The moving still gets in the way:
```
src/main.rs:24:71: 24:81 error: capture of moved value: `printqueue` [E0382]
src/main.rs:24     let server = thread::spawn(move || {
                                              ^~~~~~~
<std macros>:2:27: 2:58 note: in this expansion of format_args!
<std macros>:3:1: 3:54 note: in this expansion of print! (defined in <std macros>)
src/main.rs:24:13: 24:83 note: in this expansion of println! (defined in <std macros>)
src/main.rs:24:71: 24:81 help: run `rustc --explain E0382` to see a detailed explanation
src/main.rs:24:71: 24:81 note: move occurs because `printqueue` has type `std::vec::Vec<&'static str>`, which does not implement the `Copy` trait
error: aborting due to previous error
error: Could not compile `multiprint`.
```

------

TODO: Somewhere there was a section on how to read an error message. It would fit here well.

------

Line 24 is the read access in the clients.

```
println!("Thread {} can read the print queue: {:?}", num, printqueue);
```

---------

TODO: We could make an excursion into lifetimes here: Borrow via `&printqueue` and only read the thing but then we get
```
closure may outlive the current function, but it borrows `printqueue`, which is owned by the current function [E0373]
```
Which directly leads into the somewhat unclear situation of shared stack frames

---------

We could make copies of the queue and then move it (but this does not make sense because we plan to have one queue for all threads to use for to do their printing.

The trick will be: Instead of moving the whole queue we can just move (multiple copies) of references to one (shared) queue. The device intended to do the job is called an `Arc`.

```
fn main() {
    let mut threads = Vec::new();
    let mut printqueue: Vec<&str> = Vec::new();
    let printqueue_arc = Arc::new(printqueue);
    printqueue.push("testpage1");
    printqueue.push("testpage2");
. . .

```
Fine, but the compiler now complains (seven times!!) about
```
use of moved value: `printqueue`
```
in lines 9..15 which is where we add the test pages:
```
printqueue.push("testpage1");
printqueue.push("testpage2");
printqueue.push("testpage3");
printqueue.push("testpage4");
printqueue.push("testpage5");
printqueue.push("testpage6");
printqueue.push("testpage7");
```
In addition we have the same complaint in lines 18 and 26.

After having created the reference, the original `printqueue` has moved into the `Arc` and is no longer available, i.e. for pushing `testpage1..7` to it.

What we encounter here is the security features of rust which tend to be quite annoying when getting started, before one actually understands what is going on.


For the seven test pages we sort things out by making the Arc after adding the test pages.

------------------
TODO: snapshot/testing 6a for commit d8df61c63bd6ef9a16e1db557b07e7baf30790e5

-----------------

Building this gives us two remaining errors:
```
error: capture of moved value: `printqueue`
```
in lines 18 and 26.

This is where the threads want to access the queue. This is where the crucial part happens, the **access to the shared ressource from multiple threads**.

So, we have our Arc in place, the queue has been 'moved into it' somehow and I promised that it would allow us to have the same one queue accessible from each thread.



-----------------

TODO: Detailed description for the cloning part!

TODO: snapshot/testing 6b for commit 38b47609601e0a7566f8952b24547f16fae76928

-----------------


```
[~/projets/RustWorkshop/minimals/multiprint]$ cargo build
   Compiling multiprint v0.1.0 (file:///home/broe/projets/RustWorkshop/minimals/multiprint)
    Finished debug [unoptimized + debuginfo] target(s) in 0.67 secs
[~/projets/RustWorkshop/minimals/multiprint]$ cargo run
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/multiprint`
print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
Started thread number 0.
Thread 0 can read the print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
Started thread number 1.
Thread 1 can read the print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
Started thread number 2.
Thread 2 can read the print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
Started thread number 3.

. . .

Started thread number 9.
Vector of 10 join handles.
Thread 9 can read the print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
Joined thread number 9, Ok(()).
Joined thread number 8, Ok(()).
Joined thread number 7, Ok(()).
Joined thread number 6, Ok(()).
Joined thread number 5, Ok(()).
Joined thread number 4, Ok(()).
Joined thread number 3, Ok(()).
Joined thread number 2, Ok(()).
Joined thread number 1, Ok(()).
Joined thread number 0, Ok(()).
print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]
print queue: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]

. . .
```


### Step 7: Concurrent write access

Why all the ado about write access from multiple threads? Imagine the server is in the process of fetching the last job from the queue while _at the very same moment_ a client pushes another job into the queue. How many elements does the queue have *'during'* these two overlaping operations?

For the moment it is enough to know: Such operations on the same data strucutre should just not overlap. Never. They have to happen one after the other.

-------------
TODO:
This is something that, at first sight should not be possible anyway... at least not without careful [locking discipline](http://stackoverflow.com/questions/23350954/why-does-rust-have-mutexes-and-other-sychronization-primitives-if-sharing-of-mu).

-------------

This is what `Mutex` gives us. It ensures that there can only be one thread at a time accessing the queue (or any (most) other datastructures we need to share).

The construction that does the job goes as follows:
* An `Arc` will give all threads a concurrent reference to one `Mutex`
* The `Mutex`, in turn, _grants_ or _denies_ permission to the queue.

```
thread 1,
|  thread 2,
|  |  ...
|  |  |
|  |  | have reference to
|  |  |
v  v  v

Arc
|
| holds
|
v

Mutex
|
| holds
|
v

queue

```

Instead of an `Arc` with the `printqueue` 'in it' directly we are now using an `Arc` holding a `Mutex` which in turn holds the `printqueue`.

Again, the arc will distribute references to the threads via `clone`.


-------------
TODO: Snapshot for commit 582aa07fde6cd23b88689a071df23a28d42bcd34

-------------

To the rust newcomer (almost) surprisingly, this builds and executes:
[~/projets/RustWorkshop/minimals/multiprint]$ cargo build
   Compiling multiprint v0.1.0 (file:///home/broe/projets/RustWorkshop/minimals/multiprint)
    Finished debug [unoptimized + debuginfo] target(s) in 0.72 secs
[~/projets/RustWorkshop/minimals/multiprint]$ cargo run
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/multiprint`
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
Started thread number 0.
Thread 0 can read the print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
Started thread number 1.
Thread 1 can read the print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
Started thread number 2.

After replacing the queue with an `arc->queue` and now with an `arc->mutex->queue` datastructure it is a bit surprising that we can still just print 'the queue'.
Inspecting console output carefulle we can see _some_ differences to the output:

`["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"]`

vs.

`Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }`


What if we want to print the number of items in the queue? Replacing
```
println!("Thread {} can read the print queue: {:?}", num, clientqueue);
```
by
```
println!("Thread {} can read the print queue: {:?}", num, clientqueue.len());
```
hints at the next challenge:
```
[~/projets/RustWorkshop/minimals/multiprint]$ cargo build
   Compiling multiprint v0.1.0 (file:///home/broe/projets/RustWorkshop/minimals/multiprint)
src/main.rs:28:83: 28:86 error: no method named `len` found for type `std::sync::Arc<std::sync::Mutex<std::vec::Vec<&str>>>` in the current scope
src/main.rs:28             println!("Thread {} can read the print queue: {:?}", num, clientqueue.len());
                                                                                                 ^~~
<std macros>:2:27: 2:58 note: in this expansion of format_args!
<std macros>:3:1: 3:54 note: in this expansion of print! (defined in <std macros>)
src/main.rs:28:13: 28:90 note: in this expansion of println! (defined in <std macros>)
```
What we have done before is a debug output of the whole structure `arc->mutex->queue`. Accessing the length of the queue is something different:

A similar thing will happen, if we just want to access the first element of the queue. We need:
* `let guard = printqueue_thr.lock().unwrap();`
* `*guard` instead of `clientqueue`/`serverqueue`

Lets test with read access to the length of the queue, first.
This is how the client thread has to be modified:

```rust
for num in 0..10 {
    let clientqueue = printqueue_mutex_arc.clone();
    thread::sleep(Duration::from_millis(200)); // we spawn a new threads every 50 msec
    let handle = thread::spawn(move || {
        let guard = clientqueue.lock().unwrap();

        println!("Thread {} can read the print queue with {:?} elements.", num, (*guard).len());
        thread::sleep(Duration::from_millis(100));  // each thread first sleeps for 100 msec
    });
    threads.push(handle);
    println!("Started thread number {:?}.", num);
}
```


------
TODO:
Snapshot e0436669134f7e33bc7493ce4d06e75c5d3f2776

------

The output clearly shows the locking and unlocking behavior:
```
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
Started thread number 0.
print queue: Mutex { <locked> }
Thread 0 can read the print queue: 7
print queue: Mutex { <locked> }
print queue: Mutex { <locked> }
print queue: Mutex { <locked> }
print queue: Mutex { <locked> }
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
Started thread number 1.
Thread 1 can read the print queue: 7
print queue: Mutex { <locked> }
print queue: Mutex { <locked> }
print queue: Mutex { <locked> }
print queue: Mutex { <locked> }
print queue: Mutex { <locked> }
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
```
Wheneve a new client thread is started (which happens every 200 msec in this configuration) it locks the queue and accesses the number of elements via `len()`.
Then the thread waits for 100 msec. The server loop operates at a higher rate, waiting 20 msec in each iteration, only. We can see during the access of a child thread the server loop finds the mutex locked.


Let's see how it will behave if the server tries to access (not just debug output) the queue simultaneously with the clients:

-----
TODO:
Snapshot for commit bf24373

-----

```
Compiling multiprint v0.1.0 (file:///home/broe/projets/RustWorkshop/minimals/multiprint)
 Finished debug [unoptimized + debuginfo] target(s) in 0.77 secs
  Running `target/debug/multiprint`
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
Server thread can read the print queue with 7 elements.
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
Server thread can read the print queue with 7 elements.
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
Server thread can read the print queue with 7 elements.
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
Server thread can read the print queue with 7 elements.
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
Server thread can read the print queue with 7 elements.
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
Server thread can read the print queue with 7 elements.
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
Server thread can read the print queue with 7 elements.
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
Server thread can read the print queue with 7 elements.
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
Server thread can read the print queue with 7 elements.
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
Server thread can read the print queue with 7 elements.
Started thread number 0.
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
Server thread can read the print queue with 7 elements.
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
Thread 0 can read the print queue with 7 elements.
Server thread can read the print queue with 7 elements.
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
Server thread can read the print queue with 7 elements.
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
Server thread can read the print queue with 7 elements.
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
Server thread can read the print queue with 7 elements.
Started thread number 1.
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
Thread 1 can read the print queue with 7 elements.
Server thread can read the print queue with 7 elements.
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
Server thread can read the print queue with 7 elements.
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
Server thread can read the print queue with 7 elements.
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
Server thread can read the print queue with 7 elements.
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
Server thread can read the print queue with 7 elements.
Started thread number 2.
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
Thread 2 can read the print queue with 7 elements.
Server thread can read the print queue with 7 elements.
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
Server thread can read the print queue with 7 elements.
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
Server thread can read the print queue with 7 elements.
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
Server thread can read the print queue with 7 elements.
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
Server thread can read the print queue with 7 elements.
Started thread number 3.

. . .
```
From the output alone, the locking phases 'magically' disappeared. Watching the actual behaviour we can observe a delay in the server messages whenever a child thread locks the mutex.
According to the [standard library documentation](https://doc.rust-lang.org/std/sync/struct.Mutex.html#method.lock), `lock`  

> ... will block the local thread until it is available to acquire the mutex. Upon returning, the thread is the only thread with the mutex held.

This is exactly what we observe.


```
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
printing: testpage7
Started thread number 0.
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6"] }
printing: Some Print Job.
Started thread number 1.
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6"] }
printing: Some Print Job.
Started thread number 2.
Started thread number 3.
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6"] }
printing: Some Print Job.
Started thread number 4.
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6"] }
printing: Some Print Job.
Started thread number 5.
Started thread number 6.
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "Some Print Job."] }
printing: Some Print Job.
Started thread number 7.
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "Some Print Job."] }
printing: Some Print Job.
Started thread number 8.
Started thread number 9.
Vector of 10 join handles.
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "Some Print Job.", "Some Print Job."] }
printing: Some Print Job.
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "Some Print Job.", "Some Print Job."] }
printing: Some Print Job.
Joined thread number 9, Ok(()).
Joined thread number 8, Ok(()).
Joined thread number 7, Ok(()).
Joined thread number 6, Ok(()).
Joined thread number 5, Ok(()).
Joined thread number 4, Ok(()).
Joined thread number 3, Ok(()).
Joined thread number 2, Ok(()).
Joined thread number 1, Ok(()).
Joined thread number 0, Ok(()).
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "Some Print Job.", "Some Print Job."] }
printing: Some Print Job.
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "Some Print Job."] }
printing: Some Print Job.
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6"] }
printing: testpage6
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5"] }
printing: testpage5
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4"] }
printing: testpage4
print queue: Mutex { data: ["testpage1", "testpage2", "testpage3"] }
printing: testpage3
print queue: Mutex { data: ["testpage1", "testpage2"] }
printing: testpage2
print queue: Mutex { data: ["testpage1"] }
printing: testpage1
print queue: Mutex { data: [] }
print queue: Mutex { data: [] }
print queue: Mutex { data: [] }
print queue: Mutex { data: [] }
print queue: Mutex { data: [] }
```


### Client threads feed jobs to a print server

In order to demonstrate a more realistic client-server dynamics behaviour we modify the clients so that each of them
* runs in an infinite loop just as the server does
* run at fixed speed by not waiting for the lock to become availale (but taking it only if it is available) `if let Ok(mut guard) = serverqueue.try_lock() { . . . }`
* from time to time `push`es a `job` in the `queue`.
* add a counter variable in the server loop `let mut i = 0;`, increase it on every new loop iteration `i += 1` and print a message for every new job.
* adapt the sleep durations in the server and client loop so that an interesting dynamics occurs
* delay the print clients differently so that they put their jobs in different intervals.

First, we adapt the server loop to
* count the iterations

We also adapt the sleep durations.

------
TODO:
Snapshot for commit
905865723c240300961b6beb677cdbb39c6d3ff5
------

```
Started thread number 0.
Started thread number 1.
Started thread number 2.
Child 0...
...putting job
Child 1...
...putting job
[0] print queue: Mutex { <locked> }
Child 2...
Started thread number 3.
Started thread number 4.
Started thread number 5.
Child 4...
...putting job
Child 3...
...putting job
Child 5...
...putting job
Started thread number 6.
Started thread number 7.
Child 6...
...putting job
Started thread number 8.
Child 7...
...putting job
Started thread number 9.
Vector of 10 join handles.
Child 9...
...putting job
Child 8...
...putting job
[1] print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7", "Some Print Job.", "Some Print Job.", "Some Print Job.", "Some Print Job.", "Some Print Job.", "Some Print Job.", "Some Print Job.", "Some Print Job.", "Some Print Job."] }
printing: Some Print Job.
[2] print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7", "Some Print Job.", "Some Print Job.", "Some Print Job.", "Some Print Job.", "Some Print Job.", "Some Print Job.", "Some Print Job.", "Some Print Job."] }
printing: Some Print Job.
[3] print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7", "Some Print Job.", "Some Print Job.", "Some Print Job.", "Some Print Job.", "Some Print Job.", "Some Print Job.", "Some Print Job."] }
printing: Some Print Job.
[4] print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7", "Some Print Job.", "Some Print Job.", "Some Print Job.", "Some Print Job.", "Some Print Job.", "Some Print Job."] }
printing: Some Print Job.
Child 0...
...putting job
[5] print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7", "Some Print Job.", "Some Print Job.", "Some Print Job.", "Some Print Job.", "Some Print Job.", "Some Print Job."] }
printing: Some Print Job.
[6] print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7", "Some Print Job.", "Some Print Job.", "Some Print Job.", "Some Print Job.", "Some Print Job."] }
printing: Some Print Job.
[7] print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7", "Some Print Job.", "Some Print Job.", "Some Print Job.", "Some Print Job."] }
printing: Some Print Job.
[8] print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7", "Some Print Job.", "Some Print Job.", "Some Print Job."] }
printing: Some Print Job.
[9] print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7", "Some Print Job.", "Some Print Job."] }
printing: Some Print Job.
Child 0...
...putting job
Child 1...
...putting job
[10] print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7", "Some Print Job.", "Some Print Job.", "Some Print Job."] }
printing: Some Print Job.
[11] print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7", "Some Print Job.", "Some Print Job."] }
printing: Some Print Job.
[12] print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7", "Some Print Job."] }
printing: Some Print Job.
[13] print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6", "testpage7"] }
printing: testpage7
[14] print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "testpage6"] }
printing: testpage6
Child 0...
...putting job
Child 2...
...putting job
[15] print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "Some Print Job.", "Some Print Job."] }
printing: Some Print Job.
[16] print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5", "Some Print Job."] }
printing: Some Print Job.
[17] print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4", "testpage5"] }
printing: testpage5
[18] print queue: Mutex { data: ["testpage1", "testpage2", "testpage3", "testpage4"] }
printing: testpage4
[19] print queue: Mutex { data: ["testpage1", "testpage2", "testpage3"] }
printing: testpage3
Child 0...
...putting job
Child 1...
...putting job
Child 3...
...putting job
[20] print queue: Mutex { data: ["testpage1", "testpage2", "Some Print Job.", "Some Print Job.", "Some Print Job."] }
printing: Some Print Job.
[21] print queue: Mutex { data: ["testpage1", "testpage2", "Some Print Job.", "Some Print Job."] }
printing: Some Print Job.
[22] print queue: Mutex { data: ["testpage1", "testpage2", "Some Print Job."] }
printing: Some Print Job.
[23] print queue: Mutex { data: ["testpage1", "testpage2"] }
printing: testpage2
[24] print queue: Mutex { data: ["testpage1"] }
printing: testpage1
Child 0...
...putting job

. . .
```

Feel free to experiment with the delays and observe the effect on the queue.
