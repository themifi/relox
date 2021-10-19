# Relox
[![Check and Lint](https://github.com/themifi/relox/actions/workflows/check-and-lint.yaml/badge.svg)](https://github.com/themifi/relox/actions/workflows/check-and-lint.yaml)
[![Test with Code Coverage](https://github.com/themifi/relox/actions/workflows/test.yaml/badge.svg)](https://github.com/themifi/relox/actions/workflows/test.yaml)
[![Line Coverage](https://codecov.io/gh/themifi/relox/branch/main/graph/badge.svg?token=F6ZU01G0EW)](https://codecov.io/gh/themifi/relox)

**Re**implementation of the **Lox** programming language.

[Crafting interpreters book](https://www.craftinginterpreters.com/) by Robert Nystrom explains how to implement the Lox programming language from scratch. This is my own implementation here written in [the Rust programming lanugage](https://www.rust-lang.org/).

The Lox programming language:

- is a scripting language
- shares C-like syntax
- is dynamically typed
- is garbage collected

More description [in the book](https://www.craftinginterpreters.com/the-lox-language.html).

## Components

- [x] Tree-walk interpreter
  - [x] Scanning
  - [x] Parsing
  - [x] Evaluating
- [ ] Intermediate representation
- [ ] Optimization
- [ ] Code generation
- [ ] Virtual machine
- [ ] Runtime

## Path 

![A map of the territory](https://www.craftinginterpreters.com/image/a-map-of-the-territory/mountain.png)

- [ ] [A Tree-Walk Interpreter](https://www.craftinginterpreters.com/a-tree-walk-interpreter.html)
  - [x] [Scanning](https://www.craftinginterpreters.com/scanning.html) text source code into lexems
  - [x] [Representing Code](https://www.craftinginterpreters.com/representing-code.html) in syntax tree
  - [x] [Parsing expression](https://www.craftinginterpreters.com/parsing-expressions.html) with hand-written recursive descent parser
  - [x] [Evaluating expression](https://www.craftinginterpreters.com/evaluating-expressions.html) with bare-bones intepreter
  - [ ] Statements and State
  - [ ] Control Flow
  - [ ] Functions
  - [ ] Resolving and Binding
  - [ ] Classes
  - [ ] Inheritance
- [ ] A Bytecode Virtual Machine
  - [ ] Chunks of Bytecode
  - [ ] A Virtual Machine
  - [ ] Scanning on Demand
  - [ ] Compiling Expressions
  - [ ] Types of Values
  - [ ] Strings
  - [ ] Hash Tables
  - [ ] Global Variables
  - [ ] Local Variables
  - [ ] Jumping Back and Forth
  - [ ] Calls and Functions
  - [ ] Closures
  - [ ] Garbage Collection
  - [ ] Classes and Instances
  - [ ] Methods and Initializers
  - [ ] Superclasses
  - [ ] Optimization
