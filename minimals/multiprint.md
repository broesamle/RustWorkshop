
#### TODO: Snapshot ____
Snapshot for 99eeef5e603b5e8cf5c6f415dfb6972cd46fcdd5

#### TODO: Testing ____
Output:

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

We see that all threads are initiated (spawned) in both runs but we see some console output missing from time to time (here: `Thread 9`)

#### TODO: Snapshot ____
Snapshot for 342c98a32508069ee8d3a28bb277c0b0e39f6531

#### TODO: Testing ____
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

We collect the handles in a vector. Despite the fact we already know (and can print) the length of that vector (after spawning all threads) the threads still take their time to get done their individual console outputs. Again, the output of the late birds being dropped, occasionally.

#### TODO: Snapshot ____
Snapshot for a4df5c004a949c03aec3bce6272d6bcff137f464

Building gives us:
```
[~/projets/RustWorkshop/minimals/multiprint]$ cargo build
   Compiling multiprint v0.1.0 (file:///home/broe/projets/RustWorkshop/minimals/multiprint)
src/main.rs:12:13: 12:16 warning: unused variable: `thr`, #[warn(unused_variables)] on by default
src/main.rs:12         let thr = threads.remove(num);
                           ^~~
    Finished debug [unoptimized + debuginfo] target(s) in 0.56 secs
```

hmhmhm just a warning; lets test ;-)

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

Reading the warning carefully tells us exactly what is missing here. `thr` is not used anywhere, i.e. not for joining the threads. In fact, we did not join any of the threads in this version.


#### TODO: Snapshot ____
Snapshot for 56fcd406414e5eb646291930cda735ee7ce60593

```
[~/projets/RustWorkshop/minimals/multiprint]$ cargo build
   Compiling multiprint v0.1.0 (file:///home/broe/projets/RustWorkshop/minimals/multiprint)
src/main.rs:13:9: 13:20 warning: unused result which must be used, #[warn(unused_must_use)] on by default
src/main.rs:13         thr.join();
                       ^~~~~~~~~~~
    Finished debug [unoptimized + debuginfo] target(s) in 0.55 secs

```

#### TODO: Testing ____
It works as expected. All outputs from the subthreads are present before the second loop finalises the execution of the main thread by joining the children.
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
Rust wants us to use the result of the joining -- presumably because this result can be quite important, from an overall system's functionality POV. Here we go:

#### TODO: Snapshot ____
Snapshot for 7a9fcaf4b1db87160caf5cb80f919d8f53bdc7c2

#### TODO: Testing ____

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
