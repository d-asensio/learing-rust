# Data Types

## Integers

Since rust is a *statically typed* language, it must know variable types at compile type. In the case of integers, rust allows multiple type variants regarding signed and unsigned values for different bit sizes: 8, 16, 32, 64, 128 and size (that means the size of the current architecture, typically 64 or 32 bits).

In debug mode rust checks for overflows and makes the program panic if such situation occurs. However, in release mode rust will perform two’s complement wrapping over the overflowed value without panicking and this is considered an error.

To avoid/check for overflows or explicitly wrap a value when an operation overflows, rust's standard library provides a set of functions preceded by `wrapping_`, `cehcked_`, `overflowing_` and `saturating_`.

## Floating-Point

Not much to say here, in rust floating-point numbers are represented according to the IEEE-754 standard. The f32 type is a single-precision float, and f64 has double precision.


## Numeric operators

In this aspect rust is like many other languages, a list of the available numeric operators can be found [here](https://doc.rust-lang.org/book/appendix-02-operators.html).

An important thing to know is that the `++`/`--` operators that are present in most other languages like C++, Java, JavaScript, Python, etc. are not available in rust, instead, you have to use the `foo =+ 1` form, which in my opinion is much more semantic and intentional.

## The boolean type

Booleans are 1 byte in size, nothing very relevant here.

## The character type

Character literals must be specified with single quotes, as opposed to string literals which are specified with double quotes. Like in C++.

In rust, characters are four bytes in size, meaning that they can represent a wide variety of characters, not just ASCII.

# Compound types

## Tuples

Have a fixed length and can have different values with different types.

Cannot be directly printed using the `println!` macro.

Can be destructured, (like arrays in javascript, but they do not support the spread operator). Like that:

```rust
const TUPLE: (i8, char) = (8, 'A');
let (number, character) = TUPLE;
```

Tuple values can also be accessed thrioug indexes, like that:

```rust
const TUPLE: (i8, char) = (8, 'A');
println!("A tuple looks like this: ({}, {})", TUPLE.0, TUPLE.1);

```

## Arrays

Arrays are like tuples, thay have a fixed length too. The difference is that all values in a array must be of the same type.

To get the same (growing/shrinking) functionality as arrays in other languages, rust have another data structure: `vectors`.

An array is declared like so:

```rust
const ARRAY: [char; 2] = ['A', 'B'];
```

Arrays can also be destructured, and accessed by index, however, the syntax is different than tuples:

```rust
const ARRAY: [char; 2] = ['A', 'B'];
println!("Access by index: [{}, {}]", ARRAY[0], ARRAY[1])


let [first, second] = ARRAY
println!("Access by destructuring: [{}, {}]", first, second)
```

Invalid access to array keys in runtime make rust panic and exit the program. 