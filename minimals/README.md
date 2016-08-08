

Minimal Instructives
====================

A collection of small instructive examples for learning rust.
Typically the examples start from scratch with an empty or trivial 'initial commit' in order to then use git diffs for each step towards a more interesting version, until the example is finalised.


Rust for absolute beginners
---------------------------

Computers are complex machinery.
* Making a microchip out of a piece of silicium to do calculations is not trivial.
* Even an outdated smartphone is by (several) orders of magnitude more complex
    than the computer used in the [Apollo Guidance Computer (Wikipedia)](https://en.wikipedia.org/wiki/Apollo_Guidance_Computer).

Programming languages are tools to handle part of the complexities of the machinery.

* Programming languages differ regarding the amount of complexity they show or hide from their users. How much you want/need to see depends on what you want to learn/do.

* If you are interested in how things really work, this course may be _the right choice_ for you.

* If you prefer quick results over a deeper understanding of the machines you are working on, I do not recommend this course to you. I.e. if you are striving for web development there is no need for the level of control rust is offering you. In that case you can save a lot of time in mastering the (beautiful) complexities that come in tow with the core design of rust as a language.

    * For instance, [Python](xxx), [Ruby](xxx), [Java](xxx), [Java Script](xxx) offer much more background functionality to make contemporary information processing machinery convenient to use.
    * You see less of the complexity . . .
    * . . . and you have to learn less about it.

* Rust, was designed as a systems programming language.
    * Its purpose is not to hide the compexity away from its users but to expose it to them.
    * It will bring you in contact with the priciples of computing machinery: Your PC, tablet,  mobile phone, more specific microcontrolers . . .


Information Processing Technology
---------------------------------

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
