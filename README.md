# Rust

## Table of Contents

- [Introduction](#introduction)
  - [Updates](#updates)
  - [Hello, World!](#hello-world)
  - [Cargo Commands](#cargo-commands)
  - [Creating a New Project with Cargo](#creating-a-new-project-with-cargo)
  - [Getting Started](#getting-started)
  - [Clippy](#clippy)
- [Syntax](#syntax)
- [Glossary](#glossary)

## Introduction

### Updates

``` rust 
// Check for updates
rustup check

// Update Rust
rustup update
```

### Hello World

``` rust
// Hello World
cargo new testrust && cd $_

// Run your code
cargo run
```

### Cargo Commands

``` rust
// Check prj & depenedices for basic structural errors
cargo check

// Compile but do NOT run*
cargo build

// Rm entire $TargerDirectory
cargo clean

// *Cargo run/build ::> built into DEBUG mode == slow!
cargo run

// Compile/run prgm in RELEASE mode (NO debugger support) == fast!
cargo run --release

// Format the *entire* prj to Rust's std layout (run throughout prj)
cargo fmt

// Find issues w/ the *content* of your code (see Glossary: cargo > clippy)
cargo clippy

// Search for available crates
cargo search [search_term]  // bracket-terminal | slotmap
```

### Creating a New Project with Cargo

:bulb: Use snake_case: `snake_case`

``` rust
// In project's root directory
cargo new [prj name]

// -OR- without git integration
cargo new --vcs=none [prj name]

// Run your program
cargo run
```

## Getting Started

``` rust
## Open source file
vim Cargo.toml

```

### Clippy

``` rust
// Increase clippy pedantic-ness
#![warn(clippy::all, clippy::pedantic)]
```

## Syntax

### Basic Syntax

``` rust
// functions main
fn main() {}
// functions
fn name() {}

// exclamation mark: marks a MACRO
println!("Hello, world!")
```

## Glossary

***clippy:*** a bossy little mascot who criticizes all of your code. Ultimately a learning tool and can help you avoid mistakes.

***cargo:*** Rust's package manager; also manages collections of crates.
  - ***`Cargo.toml`:*** Project meta-data, describing your project.
  - ***`Cargo.toml` > `[package]`:*** *(Metadata)* section of the `Cargo.toml` file describes your crate.
  - ***`Cargo.toml` > `authors =`:*** *(Metadata)* a list of author names in quotation marks denoted by sqare brackets.
  - ***`Cargo.toml` > `edition =`:*** *(Metadata)* the *major edition* of Rust.
  - ***`Cargo.toml` > `name =`:*** *(Metadata)* the name of your program.
  - ***`Cargo.toml` > `version =`:*** *(Metadata)* the version of your project.
  - ***`cargo clippy`:*** provides hints & guidance while coding.

***crates:*** Rust's name for packeages; available for free on the [crates.io](https://crates.io) system.

***`./src`:*** a dir containing source code.
  - ***`./src/main.rs`:*** a minimal source code file containing the code required.

***TOML (Tom's Obvious, Minimal Language):*** a format used to describe your program `Cargo.toml` and divides information about your crate into sections.
