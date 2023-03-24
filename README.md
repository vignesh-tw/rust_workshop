# Workshop Agenda
| Content                                                                  | Duration |
|--------------------------------------------------------------------------|----------|
| 1. Workshop Context/Expectations                                         | 2 mins   |
| 2. [What is Rust?](#what-is-rust)                                        | 5 mins   |
| 3. [Why Rust?](#Why Rust?)                                               | 10 mins  |
| 4. [Learning resources](#Resources for Learning Rust)                    | 5 mins   |
| 5. [Installation](#Install Rust )                                        | -        |
| 6. [Rust Basics](#Rust Basics)                                           | 10 mins  |
| 7. [Differentiating features of Rust](#Differentiating features of Rust) | 20 mins  |
| 8. Rust Exercise Context                                                 | 5 mins   |
| 9. Iteration #1                                                          | 20 mins  |
| 10. Iteration #2                                                         | 20 mins  |
| 11. Iteration #3                                                         | 20 mins  |
| 12. Iteration #4                                                         | 20 mins  |



# Getting Started with Rust 🦀❤️

## What is Rust?
Rust is a modern and efficient programming language for developing systems applications. Performance, Reliability and Productivity are three pillars basis which Rust was created which are the reasons why rust was created.

## Why Rust?

### P1. Performance 
Rust is blazingly fast and memory-efficient: with no runtime or garbage collector, it can power performance-critical services, run on embedded devices, and easily integrate with other languages.

### P2. Reliability
Rust’s rich type system and ownership model guarantee memory-safety and thread-safety — enabling you to eliminate many classes of bugs at compile-time.

How safety and reliability is achieved,
1. A channel transfers ownership of the messages sent along it, so you can send a pointer from one thread to another without fear of the threads later racing for access through that pointer. **Rust's channels enforce thread isolation.**

2. A lock knows what data it protects, and Rust guarantees that the data can only be accessed when the lock is held. State is never accidentally shared. **"Lock data, not code" is enforced in Rust.**

3. Every data type knows whether it can safely be sent between or accessed by multiple threads, and Rust enforces this safe usage; there are no data races, even for lock-free data structures. **Thread safety isn't just documentation; it's law.**

4. You can even share stack frames between threads, and Rust will statically ensure that the frames remain active while other threads are using them. **Even the most daring forms of sharing are guaranteed safe in Rust.**

### P3.  Productivity
Rust has great documentation, a friendly compiler with useful error messages, and top-notch tooling — an integrated package manager and build tool, smart multi-editor support with auto-completion and type inspections, an auto-formatter, and more.

# Resources for Learning Rust
For a first-time Rust learner, there are several other resources:

- [The Book](https://doc.rust-lang.org/book/index.html) - The most comprehensive resource for learning Rust, but a bit theoretical sometimes. You will be using this along with Rustlings!
- [Rustlings](https://github.com/rust-lang/rustlings) - Small exercises to get you used to reading and writing Rust code. Includes practice reading and responding to compiler messages!
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html) - Learn Rust by solving little exercises! It's almost like `rustlings`, but online
- [Compilation of Rust Learning Materials](https://github.com/ctjhoa/rust-learning) - Exhaustive list all the standard learning materials shared by Rustlang as well as detailed list of other important blogs, videos, articles and repository for learning Rust

# Install Rust 

You will need to have Rust installed. You can get it by visiting https://rustup.rs. This'll also install Cargo, Rust's package/project manager.

### MacOS/Linux

Just run:

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

* Update using `$ rustup update` this works for both unix and windows based machines

### Windows

Download the installer from this [link](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe)
> **Note:** Windows installer will download around 250MB for rust and 1.5GB for visual studio with C++ toolchain which is a dependency to run rust applications

_**If your distribution/OS is different than above you can also install Rust with steps as mentioned in this [site](https://www.rust-lang.org/tools/install)**_

# Rust Basics

# Differentiating features of Rust