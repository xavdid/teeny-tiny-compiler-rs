# teeny-tiny compiler (in Rust)

A novice's Rust implementation of Austin Z. Henley's [Let's make a Teeny Tiny compiler](https://austinhenley.com/blog/teenytinycompiler1.html).

## Running the Code

This project uses [just](https://github.com/casey/just) for task running.

To see the `c` code generated from a `.teeny` file (without executing that `c` code), run `just build`, optionally passing the path to a `.teeny` file. To build and execute a file, use `just run` instead (again, passing the path to a `.teeny` file).
