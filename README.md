This is a simple and tiny implementation of a concrete number parser written in Rust.

## What's a concrete number?

A concrete number is a number associated with the thing it's counting. Here I'm using it to mean a combination of a numerical quantity and an SI physical unit—for example, "3 metres", or "12 Newtons".

## Goals:

- Add, subtract, multiply, and divide concrete numbers
- Perform unit conversion 

## Todo:

- [x] Model concrete numbers
- [x] Write lexer to convert string inputs into concrete numbers
- [x] Write parser to evaluate math expressions and output result, including unit conversion
- [x] Turn into CLI tool for command-line unit parsing
- [ ] Process SI prefixes
- [ ] Implement unit conversion