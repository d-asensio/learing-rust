# Variables

## Mutability
In Rust variables are immutable by default.

However, it is allowed to opt out and declare a mutable variable, as mentioned before, it can be done by using the `mut` keyword:

Immutable variable:
```rust
let foo = 1
```

Mutable variable:
```rust
let mut foo = 1
```

## Constants

In rust is also possible to declare constants, those were mentioned before as well, but let's cover them deeply.

Constants have some differences compared to regular variables:

- Can be declared in the global scope
- Are always immutable (the keyword `mut` cannot be used with constants)
- Must be declared using uppercase and snake-case.
- The type of constants must be explicitly annotated when they are declared.
- Are evaluated at compile time, so its value is computed before the code runs. This means that not all operations can be assigned to constants, more on this topic [here](https://doc.rust-lang.org/reference/const_eval.html).

## Shadowing

The concept of shadowing in rust is similar to shadowing in JavaScript, but not exactly equal. In Rust a variable can be shadowed inside the same scope by redeclaring it with the same name using the `let` keyword.

Shadowing does not mutate the variable, it effectively creates another variable (which can have a different type). I see it as a feature that contributes to code ergonomics, it is common to need a temporary variable that is only used to store a value before it is casted or converted to another value type.

An example of this is making a http request and then parsing it to json. In many cases you end up having a `response` variable and a line after another variable named `parsedResponse`. Using shadowing you could reuse the same name, which for me makes sense since it reduces cognitive effort to think of a new name for essentially the same thing.

Shadowing in rust works per scope, so inner scopes shadowing outer variables do not affect the variable value in the outer scope, this is logic since (as aforementioned) shadowing does not mutate the original variable. 