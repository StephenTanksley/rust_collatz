# rust_collatz
A quick and dirty implementation of a Collatz Conjecture sequence visualizer in Rust.

### The Collatz Conjecture...what is it?

![The Collatz Conjecture](https://github.com/StephenTanksley/rust_collatz/assets/33350617/be7b8f90-74d0-447c-88e9-235d99de8125)


To quote the first paragraph from the [Wikipedia](https://en.wikipedia.org/wiki/Collatz_conjecture) article on the topic: 

> The Collatz conjecture is one of the most famous unsolved problems in mathematics. The conjecture asks whether repeating two simple arithmetic operations will eventually transform every positive integer into 1. It concerns sequences of integers in which each term is obtained from the previous term as follows: if the previous term is even, the next term is one half of the previous term. If the previous term is odd, the next term is 3 times the previous term plus 1. The conjecture is that these sequences always reach 1, no matter which positive integer is chosen to start the sequence. 

### Great...so why are you doing this?

Great question! Long story short - I wanna learn Rust. I find that I learn how to write programs best by writing programs. I've written an implementation of a Collatz Conjecture visualizer before in Python, but since Rust is the language I want to learn, I'm taking what I've learned from Python and translating it to Rust.

My Python implementation can be found [here](https://colab.research.google.com/drive/1euTOJU-cLdRkHHLKwct-Nrq3QRIML-ha?usp=sharing).

### How do I run it?

This project assumes that you've already gotten so excited about trying Rust that you've already installed it on your system. If you haven't done that yet, I'd suggest you start [here](https://www.rust-lang.org/tools/install).

Once you've done that, do the following steps:

1) Open your terminal, move to your desired destination directory and run `git clone https://github.com/StephenTanksley/rust_collatz.git`.
2) `cd` into the project folder.
3) To run the project, enter `cargo run`.
4) To run the (admittedly currently limited) unit test suite, enter `cargo test`.

### Questions/comments/suggestions?

I'd love to hear from more experienced Rustaceans about how I can write more idiomatic Rust. I'm always accessible by email at <stephen.tanksley@gmail.com> or on [Linkedin](https://www.linkedin.com/in/stephentanksley)
