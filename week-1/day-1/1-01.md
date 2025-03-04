# Day 1 - Rust Roadmap

Topics:

- [Day 1 - Rust Roadmap](#day-1---rust-roadmap)
  - [Installation](#installation)
  - [Creating a new project](#creating-a-new-project)
  - [Cargo](#cargo)
    - [Cargo.toml](#cargotoml)
    - [Building for release](#building-for-release)
  - [Resources](#resources)
  - [Exercises](#exercises)
  - [Key concepts](#key-concepts)

## Installation

To install Rust, we can use the [rustup](https://rustup.rs/) tool. This tool installs the Rust compiler and the Cargo package manager, which is used to manage Rust projects. We also have to set up an IDE to write Rust code. I suggest using Visual Studio Code with [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) extension.

## Creating a new project

To create a new project, we can use the `cargo new <project_name>` command. This command creates a new directory with the project structure and a `Cargo.toml` file, which contains the project configuration.

To execute the project, we can use the `cargo run` command. This command compiles the project and runs the executable.

## Cargo

Cargo is the package manager for Rust. It is used to manage Rust projects, including building, testing, and running the project. Cargo uses the `Cargo.toml` file to store the project configuration. Useful commands:

- `cargo new <project_name>`: Creates a new project
- `cargo build`: Builds the project
- `cargo run`: Builds and runs the project
- `cargo test`: Runs the tests
- `cargo check`: check if the project compiles without building it

### Cargo.toml

The `Cargo.toml` file contains the project configuration. It includes the project name, version, dependencies, and other settings. Example:

```toml
[package]
name = "hello_world"
version = "0.1.0"
edition = "2021"

[dependencies]
```

The `package` section contains the project metadata, such as the name, version, and edition. The `dependencies` section lists the project dependencies.

### Building for release

To build the project for release, we can use the `cargo build --release` command. This command compiles the project with optimizations enabled, resulting in a faster executable.

## Resources

From the [Rust Programming Language](https://doc.rust-lang.org/book/):

- 1.1: Installation
- 1.2: Hello, world!
- 1.3: Hello, Cargo!

## Exercises

No actual exercises are provided in this lesson. However, if you don't have any experience with Rust, you can try to write a simple "Hello, world!" program to get started and build confidence with the language and the tools.

```Rust
fn main() {
    println!("Hello, world!");
}
```

## Key concepts

- Rust is a compiled language
- Rust is designed for performance and safety
- Existence of `MACROs` in Rust, recognizable by the `!` at the end, which don't always follow the same rules as functions
- Cargo is the package manager for Rust, used to manage projects and dependencies
- `Cargo.toml` contains the project configuration
