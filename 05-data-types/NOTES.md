# Data Types

## Integers

Since rust is a *statically typed* language, it must know variable types at compile type. In the case of integers, rust allows multiple type variants regarding signed and unsigned values for different bit sizes: 8, 16, 32, 64, 128 and size (that means the size of the current architecture, typically 64 or 32 bits).

In debug mode rust checks for overflows and makes the program panic if such situation occurs. However, in release mode rust will perform two’s complement wrapping over the overflowed value without panicking and this is considered an error.

To avoid/check for overflows or explicitly wrap a value when an operation overflows, rust's standard library provides a set of functions preceded by `wrapping_`, `cehcked_`, `overflowing_` and `saturating_`.