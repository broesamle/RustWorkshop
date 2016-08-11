**IMPORTANT: ** Do not work on the content in this file directly. Work on the respective branches and then merge into master, creating this compilation of all examples.

This is work in progress!! Most of the sections are still missing !!!


Rust for absolute beginners
==================================


Units
-----

[Counting Hands Project](countinghands.md)

[Graphics in a Window](graphout.md)

[Multithreaded Console Output](multiprint.md)


Introduction
------------

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



#### Variables in Rust

As I mentioned above, programming languages were designed to mediate between the technical complexities and the way humans prefer to think. Variables are the first feature we will learn in this respect.

In rust you can express the situation like this
```
let apples = 8;
let fingers = 3;
let peanuts = 2;
```

The _Counting Hands_ project in the next larger section will guide you into rust so that you can try it yourself.


### Operations


Memory stores the data we want to process. Operations are the steps that actually process the data.

**Count one up:**

<img src="../images/fingers-1_smaller.jpg" width="15%"/> &rarr;
<img src="../images/fingers-2_smaller.jpg" width="15%"/>

<img src="../images/fingers-2_smaller.jpg" width="15%"/> &rarr;
<img src="../images/fingers-3_smaller.jpg" width="15%"/>

**Count one down:**

<img src="../images/fingers-3_smaller.jpg" width="15%"/> &rarr;
<img src="../images/fingers-2_smaller.jpg" width="15%"/>

<img src="../images/fingers-2_smaller.jpg" width="15%"/> &rarr;
<img src="../images/fingers-1_smaller.jpg" width="15%"/>

If you combine them carefully you can do proper calculations:

**Add hands:**

Yes, I should say add the value of one hand to the value in another.

<img src="../images/fingers-1_smaller.jpg" width="15%"/>
+
<img src="../images/fingers-2_smaller.jpg" width="15%"/>

How do we do it? Here is a simple _program_ for adding two numbers with hands and fingers:

As long as we see a finger in the right hand we:
* `count one down` on the right hand and
* `count one up` on the left hand

And this is how it executes for 1 + 2:

1. **Left:** <img src="../images/fingers-1_smaller.jpg" width="15%"/>  **Right:** <img src="../images/fingers-2_smaller.jpg" width="15%"/>

2. **Left:** <img src="../images/fingers-2_smaller.jpg" width="15%"/>  **Right:** <img src="../images/fingers-1_smaller.jpg" width="15%"/>

3. **Left:** <img src="../images/fingers-3_smaller.jpg" width="15%"/>  **Right:** Yes, I should have taken a picture of a hand showing zero fingers.

4. You are getting the point. Now that the right hand does not show any fingers we do not continue and in the right hand have our
**Result!!**

#### Operations in Rust
In rust, you would express this as follows

```
let mut left_hand = 1;
let mut right_hand = 2;

while right_hand > 0 {
    left_hand = count_one_up(left_hand);
    right_hand = count_one_down(right_hand);
}
```

At this point, we can not yet test this example in rust. But we will come back to it after finishing some essential preparations.

[NEXT: Counting Hands Project](countinghands.md)
