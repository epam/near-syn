# near-syn

[![Build Status](https://github.com/epam/near-syn/actions/workflows/near-syn.yml/badge.svg)](https://github.com/epam/near-syn/actions/)
[![Crates.io](https://img.shields.io/crates/v/near-syn)](https://crates.io/crates/near-syn/)
[![docs.rs](https://img.shields.io/docsrs/near-syn)](https://docs.rs/near-syn/)
![License](https://img.shields.io/crates/l/near-syn.svg)

`near-syn` is a library and command line utility to ease contract development for the NEAR platform.
It leverages Rust `syn` to generate TypeScript bindings and Markdown docs.

The `near-syn` command line utility contains two sub-commands:

- `ts` generates TypeScript bindings from Rust source files.
- `md` generates Markdown documentation from Rust source files.

For more details see `near-syn --help`.

## Installation

To install the `near-syn` command line utilities use

```sh
cargo install near-syn
```

Or alternatively you can install it directly from GitHub

```sh
cargo install --git https://github.com/epam/near-syn --branch main
```

## Usage

The `near-syn ts` utility takes a group of Rust source files,
and outputs the generated TypeScript bindings.

```sh
near-syn ts path/to/src/lib.rs > src/contract.ts
```

Similarly, the `near-syn md` utility takes a group of Rust source files,
and outputs the generated Markdown documentation.

```sh
near-syn md path/to/src/lib.rs > path/to/README.md
```
