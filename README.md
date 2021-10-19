# Relox
[![Check and Lint](https://github.com/themifi/relox/actions/workflows/check-and-lint.yaml/badge.svg)](https://github.com/themifi/relox/actions/workflows/check-and-lint.yaml)
[![Test with Code Coverage](https://github.com/themifi/relox/actions/workflows/test.yaml/badge.svg)](https://github.com/themifi/relox/actions/workflows/test.yaml)
[![codecov](https://codecov.io/gh/themifi/relox/branch/main/graph/badge.svg?token=F6ZU01G0EW)](https://codecov.io/gh/themifi/relox)

**Re**implementation of the **Lox** programming language.

[Crafting interpreters book](https://www.craftinginterpreters.com/) explains how to implement the Lox programming language and all its parts from scratch. I'll write my own implementation here in [the Rust programming lanugage](https://www.rust-lang.org/).

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

![A map of the territory](https://www.craftinginterpreters.com/image/a-map-of-the-territory/mountain.png)
