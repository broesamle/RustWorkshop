

Minimal Instructives
====================

A collection of small instructive examples for learning rust.
Typically the examples start from scratch with an empty or trivial 'initial commit' in order to then use git diffs for each step towards a more interesting version, until the example is finalised.


Graphics in a Window
--------------------

* Operating System
* compiler
* Library
* Standard Library

### Step1: New Project

Make a new project by typing `cargo new --bin graphout` in the console. Instead of `graphout` you can choose whatever name you like.

The following snapshot shows you what cargo will create for you in a subdirectory named `graphout`.


#### [Snapshot] Step 1
[download files](https://github.com/broesamle/RustWorkshop/releases/tag/graphout01_new-project)
|
[see changes](https://github.com/broesamle/RustWorkshop/commit/c7dd93813e05d4142ad64fa903013cfa22331f30)

#### [Testing] Step 1
TODO

### Step2: Access the Windowmanagement (and Graphics Output)

The _operating system_ offers functionality for opening a window. In order to use this, we need to `use` a number of _libraries_

```
use glutin_window::GlutinWindow as Window;
use piston::window::WindowSettings;
```

In other examples it was enough to `use std::sync` and then the program could be compiled. In this case the situation is a little different. We get an *ERROR*:

```
error: unresolved import `glutin_window::GlutinWindow`. Maybe a missing `extern crate glutin_window`? [E0432]
src/main.rs:1 use glutin_window::GlutinWindow as Window;
                  ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
```

The reason why the build system is not happy this time is because `glutin_window` it is not part of the rust _standard library. Refer to the extern crates like this:
```
extern crate piston;
extern crate glutin_window;
```

There is another error we have to address:

```
error: can't find crate for `piston` [E0463]
```

We refer to an _extern_ crate, integrate it into our program by its name `piston` and `glutin_window`. Since the crates in question are not part of the standard library, cargo will have to download them. What it wants to know now is which version it should get (and from which source etc).

Add the following to the `[dependencies]` section in `Cargo.toml`:

```
piston = "0.24.0"
pistoncore-glutin_window = "0.28.0"
```

Instead of `println!("Hello, world!");` we can now create a new window like this:

```
fn main() {
    let window: Window =
        WindowSettings::new("Hello World!", [512; 2])
            .build().unwrap();
}
```

#### [Snapshot] Step 2
[download files](https://github.com/broesamle/RustWorkshop/releases/tag/graphout02_open-window)
|
[see changes](https://github.com/broesamle/RustWorkshop/commit/3495e498198651826cb58e27c0fcdeb7d9f15064)

#### [Testing] Step 2

`cargo build` should output something like

```
Compiling graphout v0.1.0 (file:///home/broe/projets/RustWorkshop/minimals/graphout)
src/main.rs:8:9: 8:15 warning: unused variable: `window`, #[warn(unused_variables)] on by default
src/main.rs:8     let window: Window =
                   ^~~~~~
 Finished debug [unoptimized + debuginfo] target(s) in 1.50 secs
```

Don't worry about the `unused variable` warning for now.

Running the program `cargo run` gives the console output

```
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/graphout`
```

Otherwise not much is happening. The careful observer (using a not too fast machine) may recognise a quick flash of a window or icon in the application louncher bar. So there is some interaction going on between our program and the operating system.

Despite not useful in itself, having those two components 'talk to each other' at all is a good result for now.


### Step 3: Infinite loop

The window 'just flashes' because our application quits immediately after lounching it. It follows the Scheme
* start
* run (from first to last instruction)
* terminate.

What we want is something like
* start
* run
* terminate, when the user 'closes the application'.

That is, the application reacts to event(s) which are generated outside the application: The user closes the window, presses `escape`, etc. First progress  towards reacting to events would be not to terminate (at all).

After creating the window we just loop infinitely (just add this line before the final `}`:

```
loop { }
```

#### [Snapshot] Step 3a
[download files](https://github.com/broesamle/RustWorkshop/releases/tag/graphout03a_infinite-loop)
|
[see changes](https://github.com/broesamle/RustWorkshop/commit/5cc87606651ac43ad0aeb7c00e30ed48fba96be7)

#### [Testing] Step 3a

Well, it does the job, the window remains visible, containing weird mixture of background and titlebar... Well, after all, we have created and 'registered' a window at the window management system of the _operating system_ but our application does not care about anything. It just loops infinitely. 
