# Rust Programming Quiz

### Q1. Rust is syntatically similar to which programming language?
- [ ] C#
- [x] C++
- [ ] Python
- [ ] Java

### Q2. Which `cargo` command checks a program for error without creating a binary executable?

- [ ] cargo --version
- [ ] cargo init
- [ ] cargo build
- [x] cargo check

### Q3. Which among the following is not a valid keyword in Rust?

- [ ] let
- [ ] impl
- [ ] mut
- [x] var

### Q4. Which keyword is used to write a function in rust?

- [x] fn
- [ ] function
- [ ] func
- [ ] method

### Q5. Rust is 

- [x] statically typed 
- [ ] dynamically typed 
- [ ] strongly typed 
- [ ] untyped

### Q6. Rust is known to be memory safe. Which feature is the main reason for the memory safety of Rust?

- [ ] lifetime
- [x] ownership
- [ ] reference
- [ ] borrowing

### Q7. In Rust, unsafe function and block allows 3 features. Which one feature is not allowed in Rust "unsafe" ?
```
type Name = String; 
let x: Name = "opengenus".to_string();
```
- [ ] Dereference a raw pointer
- [ ] Access or update a static mutable variable
- [x] turn off the borrow checker
- [ ] Call unsafe functions

### Q8. What is the output of this Rust program?
```
struct S {
f: fn(),
}

impl S {
fn f(&self) {
print!("1");
}
}

fn main() {
let print2 = || print!("2");
S { f: print2 }.f();
}
```
- [ ] The program exhibits undefined behavior
- [ ] The program does not compiler
- [x] The program is guaranteed to output: 1

### Q9. Which are the two primitive compound types supported by Rust?

- [x] tuples and arrays
- [ ] tuples and lists
- [ ] lists and arrays
- [ ] maps and arrays

### Q10. What is the output of the program ?
```
fn main() {
    let (.., x, y) = (0, 1, ..);
    print!("{}", b"066"[y][x]);
}
```
**Answer:** 54