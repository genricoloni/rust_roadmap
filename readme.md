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
  - [Day 2](#week-2---day-2)  
  - [Day 3](#week-2---day-3)  
  - [Day 4](#week-2---day-4)  
  - [Day 5](#week-2---day-5)  

- [Week 3](#week-3): Structs, enums, pattern matching (advanced)  
- [Week 4](#week-4): Generics, traits, and error handling  
- [Week 5](#week-5): Collections, iterators, and closures  
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

- More pattern matching with `match` (especially on slices and references).  
- Combining ownership rules with pattern matching.  

**Goals**:

- Become comfortable matching different data structures and references.  
- Incorporate safe borrowing while destructuring values.  

### Week 2 - Day 5

**Recap and Exercises**:  

- Recap and insights on `match` case.
- Practice exercises on borrowing, references, and slices.

---

## Week 3

Structs, enums, pattern matching (advanced)

- **Day 1**: Defining `struct`s, implementing methods, deriving traits (like `Debug`).  
- **Day 2**: Enumerations (`enum`), storing different data in variants, pattern matching on `enum`.  
- **Day 3**: Working with `Option` and `Result`, error handling with `match`, `unwrap()`, `expect()`.  
- **Day 4**: Advanced pattern matching: `if let`, `while let`, destructuring nested types.  
- **Day 5**: Recap: build a small system (e.g., a booking manager) using `struct`s and `enum`s.  

---

## Week 4

Generics, traits, and error handling

- **Day 1**: Generics (`<T>`) in functions, `impl<T>` blocks, partial ordering constraints (e.g., `PartialOrd`).  
- **Day 2**: Traits: defining your own, implementing them for custom types, default methods.  
- **Day 3**: Trait objects (`Box<dyn Trait>`), dynamic dispatch, building heterogeneous collections.  
- **Day 4**: Advanced error handling, `Result`, the `?` operator, returning custom error types.  
- **Day 5**: Recap: create or extend a small project that uses generics, custom traits, and proper error handling.  

---

## Week 5

Collections, iterators, and closures

- **Day 1**: Core collections: `Vec<T>`, `String`, basic operations (push/pop, indexing, iteration).  
- **Day 2**: `HashMap<K, V>`: insertion, removal, iteration, handling missing keys.  
- **Day 3**: Iterators: `map()`, `filter()`, `fold()`. Differences between `iter()`, `iter_mut()`, `into_iter()`.  
- **Day 4**: Closures: syntax, capturing variables, usage with iterators (e.g., `map(|x| x+1)`).  
- **Day 5**: Recap: create a small data-processing tool that uses collections, iterators, closures.  

---

## Week 6

Concurrency

- **Day 1**: Threads with `std::thread::spawn`, joining threads, basic concurrency overview.  
- **Day 2**: Channels (`mpsc::channel`): sending and receiving messages, multiple producers.  
- **Day 3**: Mutex, `Arc<Mutex<T>>` for shared state, avoiding data races.  
- **Day 4**: More complex concurrency patterns (multiple consumers, chat-like scenarios, `RwLock`).  
- **Day 5**: Recap: Producer-consumer demo with concurrency tools (channels, threads, mutex).  

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
