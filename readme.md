# rust_roadmap

This repo contains my personal roadmap for learning Rust. It includes topics, resources, exercises, and key concepts to help me learn Rust effectively. The main source of information is the [Rust Programming Language](https://doc.rust-lang.org/book/). As long as the book, I suggest to be familiar with almost one programming language, before starting Rust.

This repo is not intended to be a comprehensive guide, but rather a structured plan to help me learn Rust in a systematic way, with notes, exercises, and projects to consolidate my knowledge.

## Index

- [Who is this roadmap for?](#who-is-this-roadmap-for)
- [Set-up your time table](#set-up-your-time-table)
- [Table of contents](#table-of-contents)

## Who is this roadmap for?

First of all, it is for myself. I tried to design a schedule than can accomplish my weakness in learning programming languages, focusing both on fundamentals and language-specific features. If you ever want to follow this roadmap, I strongly encourgae you to modify it according to your needs.

## Set-up your time table

I added a `.ics` file with my personal schedule, which can be imported to any calendar and follow the roadmap. Each day is described in this way:

```txt
BEGIN:VEVENT
UID:rust-roadmap-weekx-dayy
DTSTAMP:20250226T000000Z
DTSTART:<MONx>T093000
DTEND:<MONx>T113000
SUMMARY:Week x - Day y
DESCRIPTION:...
END:VEVENT
```

How to Replace the Date Placeholders

- MON1, TUE1, WED1, THU1, FRI1 refer to Week 1, Monday through Friday.
- MON2, TUE2, WED2, etc. refer to Week 2, and so on, up to MON8, TUE8, WED8, THU8, FRI8.

For each placeholder (e.g., \<MON1>), insert the numeric date in YYYYMMDD format. For example, if Week 1, Day 1 is March 3rd, 2025, you replace \<MON1> with 20250303. The time portion (T093000) or (T150000) etc. remains attached to the date.

Example:

```txt
DTSTART:<MON1>T093000
```

becomes

```txt
DTSTART:20250303T093000
```

I suggest you to modify dates and times according to your needs (e.g. prompting ChatGPT to modify dates and times with specifyc patterns for each day).

### Setting up `rust-analizer` in VSCode

For better organization, I choose to create a new crate for each day. The main drawback is that `rust-analizer` struggles to find `Cargo.toml` file if it is not in the root directory. In my opinion, the best solution is to modify the extension settings in VSCode. To do so, follow these steps:

1. Serve the command `Preferences: Open Settings (UI)` in the command palette.
2. Paste the following string in the search bar: `rust-analyzer: Linked Projects`
3. Click on `Edit in settings.json` in the first result.
4. Add an entry for each day, like this:

```json
  "rust-analyzer.linkedProjects": [
        "week-1/day-3/exercises-day-3/Cargo.toml",
        "week-1/day-4/exercises-day-4/Cargo.toml",

  ]

```

Unfortunately, I didn't find a way to add multiple entries at once, and moreover the extension seems to not support wildcards.

---

## Table of contents

The *roadmap* is divided into weeks, and each week is divided into days.  
I assume **no more than 2 hours** of study per day, for **5 days** a week.

- [Week 1](#week-1): Install and configure Rust, Cargo, basic syntax, flow control, and functions  
  - [Day 1](#week-1---day-1): Getting started with Rust: installation, hello world, and Cargo  
  - [Day 2](#week-1---day-2): Variables, mutability, and data types  
  - [Day 3](#week-1---day-3): Functions, control flow
  - [Day 4](#week-1---day-4): A basic guess-the-number game
  - [Day 5](#week-1---day-5): Recap and exercises  

- [Week 2](#week-2): Ownership, references, and borrowing  
  - [Day 1](#week-2---day-1): Basics of ownership and borrowing
  - [Day 2](#week-2---day-2): References and borrowing
  - [Day 3](#week-2---day-3): String slices and slices
  - [Day 4](#week-2---day-4): Lifetime annotation
  - [Day 5](#week-2---day-5): Recap and exercises

- [Week 3](#week-3): Structs, enums, pattern matching (advanced)
  - [Day 1](#week-3---day-1): Defining and implementing `struct`s
  - [Day 2](#week-3---day-2): Enumerations and pattern matching
  - [Day 3](#week-3---day-3): Working with `Option` and `Result`
  - [Day 4](#week-3---day-4): Advanced pattern matching
  - [Day 5](#week-3---day-5): Recap and exercises  
- [Week 4](#week-4): Generics, traits, OOP
  - [Day 1](#week-4---day-1): Generics
  - [Day 2](#week-4---day-2): Traits
  - [Day 3](#week-4---day-3): OOP in Rust
  - [Day 4](#week-4---day-4): Smart pointers
  - [Day 5](#week-4---day-5): Interior mutability
- [Week 5](#week-5): Collections, iterators, and closures
  - [Day 1](#week-5---day-1): Core collections
  - [Day 2](#week-5---day-2): `HashMap<K, V>`
  - [Day 3](#week-5---day-3): Functional programming in Rust
  - [Day 4](#week-5---day-4): Iterators
  - [Day 5](#week-5---day-5): Recap and exercises
- [Week 6](#week-6): Concurrency: threads, channels, mutex  
- [Week 7](#week-7): Macros, unsafe, smart pointers, and advanced features  
- [Week 8](#week-8): Final project and consolidation  

---

## Week 1

### Week 1 - Day 1

**Topics**:

- Installing Rust (via `rustup`), setting up your editor or IDE.  
- Your first “Hello World” project using `cargo new`.  
- Running the project with `cargo run`.  

**Goals**:

- Understand how to install, update, and manage Rust toolchains.  
- Familiarize yourself with the basic project structure and Cargo.  

### Week 1 - Day 2

**Topics**:

- Variables, mutability (`let` vs `let mut`), shadowing.  
- Data types: integers, floating-point numbers, booleans, chars, tuples, arrays.  

**Goals**:

- Deepen your understanding of Rust’s strict typing.  
- Experiment with storing and printing different types.  

### Week 1 - Day 3

**Topics**:

- Functions (`fn`), parameters, return types.  
- Control flow: `if`, `else if`, `else`, `match`, loops (`for`, `while`, `loop`).  

**Goals**:

- Write small, modular functions.  
- Practice branching logic and looping constructs.

### Week 1 - Day 4

**Topics**:

- Building a basic “guess the number” game (inspired by the example in *The Rust Book*).  
- Using `std::io` for input, generating random numbers (via `rand` crate), basic control flow.  

**Goals**:

- Practice reading user input and handling conversions (string → integer).  
- Learn how to handle external crates via `Cargo.toml`.  

### Week 1 - Day 5

**Recap and Exercises**:

- Consolidate all concepts from the previous four days (Cargo, data types, control flow, functions).  
- Possibly expand the guess-the-number game or create small challenges (e.g., a calculator).  

---

## Week 2

Ownership, references, and borrowing

### Week 2 - Day 1

**Topics**:

- The ownership model: stack vs heap, moves, copying, dropping.  
- Basics concepts of `move`, `clone` and `copy`.

**Goals**:

- Understand the core of Rust’s memory safety.  
- Learn differences between stack-allocated and heap-allocated data.

### Week 2 - Day 2

**Topics**:

- Borrowing and references (`&`), mutable references (`&mut`).
- Borrow checker, rules for borrowing.

**Goals**:

- Practice borrowing data without moving it, without duplicating it.
- Understand the rules of borrowing and how to avoid dangling references.

### Week 2 - Day 3

**Topics**:

- Slices
- Differences between *strings* and *string slices*.

**Goals**:

- Understand concepts behind slices.
- How slice parameters work in functions.

### Week 2 - Day 4

**Topics**:

- Lifetime: introduction and motivation.
- Lifetime elision rules.  

**Goals**:

- Understand why references have to be valid for a certain lifetime.
- Use lifetime annotations to specify the relationship between references.

### Week 2 - Day 5

**Recap and Exercises**:  

- Recap and insights on `match` case.
- Practice exercises on borrowing, references, and slices.

---

## Week 3

Structs, enums, pattern matching (advanced)

### Week 3 - Day 1

**Topics**:

- Defining `struct`s, implementing methods, deriving traits (like `Debug`).
- Implementing methods for `struct`s.

**Goals**:

- Understand how to define custom data types.
- Familiarize with traits and how to derive them.

### Week 3 - Day 2

**Topics**:

- Enumerations (`enum`), differences between `struct` and `enum`.
- `Option` and `Result` enums, pattern matching on enums.

**Goals**:

- Learn how to store different data in variants.
- Practice pattern matching on `enum`, handling different cases.

### Week 3 - Day 3

**Topics**:

- Error handling with `match`, `unwrap()`, `expect()`.
- `panic!` and `Result` for error handling.

**Goals**:

- Understand how to handle optional and error-prone values.
- Combine borrowing/ownership with enums for error handling.

### Week 3 - Day 4

**Topics**:

- Advanced pattern matching: `if let`, `while let`.
- Pattern matching with advanced destructuring.

**Goals**:

- Practice more complex pattern matching scenarios.
- Master the `match` syntax and its variations.

### Week 3 - Day 5

**Recap**:

- Build a small system (e.g., a booking manager) using `struct`s and `enum`s.
- Consolidate knowledge on `struct`s, `enum`s, and pattern matching.
- Practice creating a small project from scratch.

---

## Week 4

Generics, traits, and error handling

### Week 4 - Day 1

**Topics**:

- Generics `<T>`
- Generics in functions, structs, enums.

**Goals**:

- Understand how to write functions and structs that work with any type.
- Learn how to constrain generics with traits.

### Week 4 - Day 2

**Topics**:

- Traits: definition, implementation.
- Default implementations, trait inheritance.

**Goals**:

- Define custom traits and implement them for custom types.
- Understand how to use default methods in traits.

### Week 4 - Day 3

**Topics**:

- OOP in Rust: `impl Trait`, `dyn Trait`.
- Trait objects: `Box<dyn Trait>`, dynamic dispatch.

**Goals**:

- Understand how Rust implements object-oriented programming.
- Learn how to use trait objects for dynamic dispatch.

### Week 4 - Day 4

**Topics**:

- Smart pointers
- `Box<T>`, `Rc<T>`

**Goals**:

- Understand the differences between smart pointers and references.
- Learn how to use smart pointers for shared ownership.

### Week 4 - Day 5

**Topics**:

- Interior mutability
- `RefCell<T>`, usage with `Rc<T>`

**Goals**:

- Understand how to mutate data inside an immutable reference.
- Learn how to use `RefCell` for interior mutability.

---

## Week 5

Collections, iterators, and closures

### Week 5 - Day 1

**Topics**:

- Core collections: `Vec<T>`, `String`
- Basic operations (push/pop, indexing, iteration).

**Goals**:

- Understand how to use vectors and strings.
- Practice basic operations on collections.

### Week 5 - Day 2

**Topics**:

- `HashMap<K, V>`
- Key-value pairs, insertion, removal, iteration.

**Goals**:

- Learn how to use hash maps for key-value storage.
- Deal with missing keys and handle errors.

### Week 5 - Day 3

**Topics**:

- Functional programming in Rust: closures.
- Usage with iterators (e.g., `map(|x| x+1)`).

**Goals**:

- Learn how to write closures in Rust.
- Practice using closures with iterators.

### Week 5 - Day 4

**Topics**:

- Iterators: `map()`, `filter()`, `fold()`.
- Differences between `iter()`, `iter_mut()`, `into_iter()`.

**Goals**:

- Understand how to use iterators for data processing.
- Learn the differences between iterator adaptors.

### Week 5 - Day 5

**Recap**:

- Create a small data-processing tool that uses collections, iterators, and closures.
- Consolidate knowledge on collections, iterators, and closures.

---

## Week 6

Concurrency

### Day 1

**Topics**:

- Threads with `std::thread::spawn`
- Joining threads, basic concurrency overview.

**Goals**:

- Understand how to create and manage threads in Rust.
- Learn how to join threads and handle thread safety.

### Day 2

**Topics**:

- Channels (`mpsc::channel`)
- Sending and receiving messages, multiple producers.

**Goals**:

- Understand how to use channels for communication between threads.
- Handle multiple producers concurrently.

### Day 3

**Topics**:

- Mutex
- `Arc<Mutex<T>>` for thread-safe shared state.

**Goals**:

- Avoid Race Conditions with Mutex.
- Requirements for secure concurrency in Rust.

### Day 4

**Topics**:

- `Send` and `Sync` traits.
- Rust's requirements for thread safety.

**Goals**:

- Understand how Rust guarantees memory safety.
- Learn how to use `Send` and `Sync` traits for thread safety.

### Day 5

**Topics**:

- `std::sync::RwLock`
- Producer-consumer exercises.

**Goals**:

- Understand how to use `RwLock` for read-write locks.
- Practice building a producer-consumer scenario with threads and channels.

---

## Week 7

Macros, unsafe, smart pointers, and advanced features

- **Day 1**: Macros with `macro_rules!`, creating simple macros, parameterized macros.  
- **Day 2**: Procedural macros (brief introduction), deriving custom traits, advanced code generation.  
- **Day 3**: `unsafe` Rust: raw pointers, `extern "C"`, when (not) to use `unsafe`.  
- **Day 4**: Smart pointers: `Box<T>`, `Rc<T>`, `RefCell<T>`, building linked lists.  
- **Day 5**: Recap: revisit existing code to see if macros or smart pointers might optimize or simplify it.  

---

## Week 8

Final project and consolidation

- **Day 1**: Plan your final project (could be a CLI tool, mini-game, web server, etc.), set up Git repo.  
- **Day 2**: Structure modules, define core data types, draft main functionalities.  
- **Day 3**: Implement key features, handle errors properly; add concurrency if it fits.  
- **Day 4**: Refactoring, testing (`cargo test`), linting with `cargo clippy`, formatting with `cargo fmt`.  
- **Day 5**: Finalize the project, add documentation (`///` doc comments), publish or share if desired.  

---
