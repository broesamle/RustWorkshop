

Minimal Instructives
====================

This is work in progress!! Most of the sections are still missing !!!

A collection of small but dense examples will dare the learner to get in touch with the complexities of systems programming in rust.

Nevertheless it strives for beeing an intro from scratch, with no prior programming skills required. Happy to receive your feedback.



Intro: Rust for absolute beginners
----------------------------------

Computers are complex machinery.
* Making a microchip out of a piece of silicium to do calculations is not trivial.
* Even an outdated smartphone is by (several) orders of magnitude more complex
    than the computer used in the [Apollo Guidance Computer (Wikipedia)](https://en.wikipedia.org/wiki/Apollo_Guidance_Computer).

Programming languages are tools to handle part of the complexities of the machinery.

* Programming languages differ regarding the amount of complexity they show or hide from their users. How much you want/need to see depends on what you want to learn/do.

* If you are interested in how things really work, this course may be _the right choice_ for you.

* If you prefer quick results over a deeper understanding of the machines you are working on, I do not recommend this course to you. I.e. if you are striving for web development there is no need for the level of control rust is offering you. In that case you can save a lot of time in mastering the (beautiful) complexities that come in tow with the core design of rust as a language.

    * For instance, [Python](https://www.python.org/), [Ruby](https://www.ruby-lang.org/), [Java][1java], [Java Script](https://en.wikipedia.org/wiki/JavaScript) offer much more background functionality to make contemporary information processing machinery convenient to use.
    * You see less of the complexity . . .
    * . . . and you have to learn less about it.

[1java]: https://en.wikipedia.org/wiki/Java_(programming_language)

* Rust, was designed as a systems programming language.
    * Its purpose is not to hide the compexity away from its users but to expose it to them.
    * It will bring you in contact with the priciples of computing machinery: Your PC, tablet,  mobile phone, more specific microcontrolers . . .


### Information Processing Technology

<img src="../images/fingers-1_smaller.jpg" width="20%"/>
<img src="../images/fingers-2_smaller.jpg" width="20%"/>
<img src="../images/fingers-3_smaller.jpg" width="20%"/>

<img src="../images/peanuts-1_smaller.jpg" width="20%"/>
<img src="../images/peanuts-2_smaller.jpg" width="20%"/>
<img src="../images/peanuts-3_smaller.jpg" width="20%"/>

Whether it is fingers, peanuts or strokes on paper -- or electrical signals in a wire or microchip. The underlying idea is the same.
one, two, three.

Why is calculating with hands or with calculator so much simpler than making a modern computer calculate? A person who uses hands, or paper to calculate is, in fact not only using hands or paper, but also a brain that knows how to calculate.

Programming means to transform the knowledge of person into a sequence of instructions suitable for a stupid piece of silicium to execute them. That is why calculating is relatively simple, but making a calculator is not.

[]()


### Memory

How much did we have of apples, peanuts, fingers,  . . ?

<img src="../images/fingers+peanuts+paper.jpg" width="30%" />

Today's computers have a memory that is organised in cells with addresses. Technically, there is no such thing as names for rows in a table as in the picture above. Just memory _cells_ with _addresses_. You can think of it as numbered locations of equal size.

<img src="../images/memorycellsA.jpg" width ="30%" />

For the machine there is only addresses, numbers of cells, in consecutive order. In each cell, there is a value. There is no such thin as an empty cell. There is always a number in each cell so we must be careful, which cells to use.

> Ok, `fingers` at address 5, `apples` at address 7, `peanuts` at address 6, . . .

To make it easier (for humans) to keep track of which value is kept at what address, we use variables. At first it looks more complicated:

<img src="../images/variables_names+addressesA.jpg" width="60%" />

As a human, you can relax and focus on two parts only: variables' names and their values.

<img src="../images/variables_names+addressesB.jpg" width="60%" />



### Variables in Rust

As I mentioned above, programming languages were designed to mediate between the technical complexities and the way humans prefer to think. Variables are the first feature we will learn in this respect.

In rust you can express the situation like this
```
let apples = 8;
let fingers = 3;
let peanuts = 2;
```

The follogwing _Counting Hands_ project will guide you into rust so that you can try it yourself.


First Project: Counting Hands
-----------------------------

### Step1: Your first project

First, you have to open a console window. The console works like a chat window, except that you chat with your _operatin system_ and the things you type in are _commands_ which it would (try to) execute.

TODO: add/link to instructions how to open a console and walk to the directory where the project will be created (also in mac and windows); What is a working directory . . . etc.

Now, you should have a console window open and your working directory/folder should be the place where you want to place the project.

Typically, the console window reminds you of the directory you are in at the moment, here `~/projets/RustWorkshop/minimals` and the `$` indicates that it is waiting for a command to be typed.  Your console should show something like

```
[~/projets/RustWorkshop/minimals]$

```

Make a new project by typing `cargo new --bin countinghands` into the console. Instead of `countinghands` you can choose whatever name you like.

Not much seems to have happened, except for a new line is shown, again with the working directory reminder and the `$`.

```
[~/projets/RustWorkshop/minimals]$ cargo new --bin countinghands
[~/projets/RustWorkshop/minimals]$

```
Don't worry, not news is good news in this case, cargo does not reply anything if everything goes as expected. You can check about the new project by typing `ls`:
```
[~/projets/RustWorkshop/minimals]$ ls
countinghands  graphout  README.md
```
What you will see exactly, will depend on the contents of your directory but you should find the name of your new project among the reply of `ls`.

By typing `cd countinghands` (or whatever name you gave it) you change your working directory so that it is now the project directory.

The following snapshot shows you what cargo will create for you in a subdirectory named `countinghands`.

#### [Testing] Step 1
Type `cargo build` into the console which should now look something like this:
```
[~/projets/RustWorkshop/minimals/countinghands]$ cargo build
   Compiling countinghands v0.1.0 (file:///home/broe/projets/RustWorkshop/minimals/countinghands)
    Finished debug [unoptimized + debuginfo] target(s) in 0.55 secs
[~/projets/RustWorkshop/minimals/countinghands]$
```
Type `cargo run` and you should see
```
[~/projets/RustWorkshop/minimals/countinghands]$ cargo run
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/countinghands`
Hello, world!
[~/projets/RustWorkshop/minimals/countinghands]$
```

Cargo has _built_ the first project into something that your _operating system_ can lounch as an application or 'app'. Traditionally, the minimal app to begin with is one that just sais `Hello world!`. In our case, cargo has already created that for us and you should see `Hello world!` in your console now.


#### [Snapshot] Step 1
Whenever a useful intermediate state is achieved, a _snapshot_ will offer the project (as it should be at this point) for download. Furthermore, you can investigate the changes that were made from the last project.


[download files](https://github.com/broesamle/RustWorkshop/releases/tag/countinghands01_firstproject)
|
[see changes](https://github.com/broesamle/RustWorkshop/commit/f46e703d85ea21bf90d1d59c58fa511d5daa7ee8)

### Step 2: Where is 'the program'?
Now that we have already used the `cargo` _build system_ for to create your first app the next obvious question is: Where is 'the program'. There is three important components involved:
* The _source code_ is what humans can read and write. Here, this is written in _Rust_.
* The _build system_ translates the source code into something that . . .
* . . . your operating system can start as an _application_ or _executable_.

The sources are located in the subdirectory `src` in your project folder. Lets have a look: `cd src` and ls should show you that the file `main.rs` is there.

```
[~/projets/RustWorkshop/minimals/countinghands]$ cd src
[~/projets/RustWorkshop/minimals/countinghands/src]$ ls
main.rs
[~/projets/RustWorkshop/minimals/countinghands/src]
```

`cat main.rs` finally shows you 'the program', the source of the program, to be precise:
```
fn main() {
    println!("Hello, world!");
}
```

----
** ! ! ! TO BE CONTINUED ! ! !  **

----

Project: Graphics in a Window
-----------------------------

* Operating System
* compiler
* Library
* Standard Library

TODO: Is it responsible not to introduce closures up to this point?


### Step 1: New Project

TODO: we need a link to the intro/fundament example here.


#### [Snapshot] Step 1
[download files](https://github.com/broesamle/RustWorkshop/releases/tag/graphout01_new-project)
|
[see changes](https://github.com/broesamle/RustWorkshop/commit/c7dd93813e05d4142ad64fa903013cfa22331f30)

#### [Testing] Step 1
TODO: we need a link to the intro/fundament example here.


### Step 2: Access the Windowmanagement (and Graphics Output)

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


### Step 3: Event loop

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

It does the job, the window remains visible, containing weird mixture of background and titlebar... well, after all, we have created and 'registered' a window at the window management system of the _operating system_ but our application does not care about anything. It just loops infinitely.


Next, we need access to the events generated by the operating system's window management. In particular, it will provide a set of informations the _graphics library_ would need to 'draw things correctly'; window position, size etc.

One more dependency: `piston2d-opengl_graphics = "0.31.0"`.

Providing us an `extern crate opengl_graphics;` so that we can `use opengl_graphics::{GlGraphics, OpenGL};`.
Finally, we initialise the graphics system which we can now use.

```
let opengl = OpenGL::V3_2;
let gl = GlGraphics::new(opengl);
```

#### [Snapshot] Step 3b
[download files](https://github.com/broesamle/RustWorkshop/releases/tag/graphout03b_opengl_graphics)
|
[see changes](https://github.com/broesamle/RustWorkshop/commit/a74eabff42f439c633323ea22f4f1f82a00c07e2)

#### [Testing] Step 3b

`cargo build` should give:

```
   Compiling graphout v0.1.0 (file:///home/broe/projets/RustWorkshop/minimals/graphout)
src/main.rs:10:9: 10:15 warning: unused variable: `window`, #[warn(unused_variables)] on by default
src/main.rs:10     let window: Window =
                       ^~~~~~
src/main.rs:15:9: 15:11 warning: unused variable: `gl`, #[warn(unused_variables)] on by default
src/main.rs:15     let gl = GlGraphics::new(opengl);
                       ^~
    Finished debug [unoptimized + debuginfo] target(s) in 1.60 secs
```
No missing dependencies, all extern crates are there.
Complaints about unused variables are fine, we will use them in a minute.

Now, we replace the 'non-reactive' infinite loop with something that has the potential to react to events happening to the window, the keyboard and so on.

We will need:
```
use piston::event_loop::Events;
use piston::input::RenderEvent;
```

`window` and `gl` need to be mutable. We also will need to declare

```
let mut events = window.events();
```

The great change is to replace `loop {}` with an (admittedly not exactly trivial) construct that will also loop infinitely:

```
for e in events.next(&mut window) {
    if let Some(r) = e.render_args() {
        gl.draw(r.viewport(), |c, gl| {
            // all  drawing actions will happen here soon
        });
    }
}
```

* In every loop cycle it provides an event `e`.

* If the `e` has some render arguments attached to it `if let Some(r) = e.render_args()` . . .
* . . . we activate the graphics system via `gl.draw(...)`
* which gets additional information about how and where to draw based on `r.viewport()`

#### [Snapshot] Step 3c
[download files](https://github.com/broesamle/RustWorkshop/releases/tag/graphout03c_eventloop%2Bwindow)
|
[see changes](https://github.com/broesamle/RustWorkshop/commit/0f66156843c0732821572b34ff1bf453f4d303ed)

#### [Testing] Step 3c
The main difference is that now, we can close the window by pressing the close button `(x)` in the title bar. Again, this is not spectacular in itself but it indicates progress towards a smooth integration with the window system.
