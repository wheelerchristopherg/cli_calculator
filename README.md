# cli_calculator
A CLI calculator that parses and solves arithmetic expressions while respecting order of operations.

This is mostly a project initially intended for learning how to do basic arithmetic expression parsing. Once I finished implementing the parser in python, I wanted to use this as an exercise for learning other languages. I'm currently working on a rust port. 

Possible future implementations may be in C, C++, Scala, maybe Go.

# Usage
## Python
* run cli_calculator in python/src
  * This should be completely self contained and does not require any dependencies.
  * Tested with python 3.10.

## Rust
* run `cargo run` in the rust directory

## All
Once you start the program, an interactive prompt will be shown that looks like this:

`> `

From here, you can enter any arithmetic expression such as:
* 10 + 2 * 4 - 3
* (10 + 2) * (4 - 3)
* (10*(1-3))/2

The result of each of these expressions will be given in the form: `x0 = 15.0`

You can also use previous results in subsequent expressions. For example if you enter `10 + 3` you will receive the result `x0 = 13.0`, then if you enter `x0 / 2` you will receive the result `x1 = 6.5`.

To gracefully quit, just enter an empty expression.