# CLI Mars Weight Calculator

## Data Types in Rust

Rust has 4 basic data types:

- Booleans
- Characters
- Integers
- Floats

### a) Integers

There integers can be grouped according to whether they are
signed(i)/unsigned(u) and also according to the number of bits they occupy in
memory.

Examples: `i8`, `u8` (unsigned integer using a single byte), `i16`, `u16`,
... , `i128`(signed integer using 16 bytes),`u128`

### b) Floats

Rust has 2 types of floating point numbers. `f32`, `f64`

### c) Boolean (`bool`)

Booleans occupy a single byte. Can either be `true` or `false`

### d) Char

Each char in Rust occupies 4 bytes in memory.

## Functions

Functions in Rust have the following signature

```rust
fn function_name(param:param_type) -> return_type {
    function_logic;
    return value;
}
```

The `return` keyword in rust is optional. Any value specified without the
return keyword will be implicitly returned.

```rust
fn function_name(param:param_type) -> return_type {
    function_logic;
    value;
} // Returns value
```

## Macros

Macros in rust end in an exclamation mark `!`. They are used to implement
metaprogramming (code that writes more code?).

Unlike function signatures which have to specify number and type of each
parameter, macros allow one to have a variable number of parameters with
different types. `println!` is a macro because it can be called with a variable
number of arguments.

## Variables

Variables in Rust can be declared using the `let` keyword. All variables are
immutable by default.

```rust
let variable_name: variable_type = value;
```

To make a variable mutable, you have to explicitly declare it as mutable.

```rust
let mut variable_name = value;
```

Rust supports type inferencing so you don't need to specify the variable type
when declaring it.

*Resource Acquisition is Initialization*: this is the process of deallocating 
memory at the end of a variable/objects lifetime. (destruction)

Because of Rust's memory borrowing model, there can only be 1 owner of a 
pointer at a time. This is the variable that initialized the value. This 
prevents memory corruption resulting from a *'double free error'* because 
if more than 1 variable references the same pointer, the destruction will 
occur for each of the referencing variables.

## Error Handling in Rust

A `Result` is the core of error handling. It can either have a success or an 
error state. 
Unwraping a `Result`in error state will cause the program to exit. If the 
`Result` is in success a value will be yielded.