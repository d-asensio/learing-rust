# Building a guessing game

## Mutable variables
Mutable variables should be declared using the `mut` keyword, like so: `let mut foo = 3`.

## Constant variables

Constants must be explicitly typed and should be written in upper-snake-case.

```rust
const MAX_ATTEMPTS: u32 = 10;
```

## Type inference
The `.parse()` method of the `String` type infers the assignment type and attempts to parse the string value according to such type. In this example this is used to parse the user input to a unsigned 32 bits number, like so:

```rust
let guess: u32 = match guess
    .trim()
    .parse()
    .expect("Wrong input!")
```

## The `match` operator

The match operator is used to assert the value of an operation or variable and act in consequence. Match expressions are made up of *arms*, an arm is a pattern to match against and represent one of the paths a program can take. Rust has mechanisms to help the developer know if all the possible arms that an expression can take are taken into account.

### Examples of use:

Decide what to do with a `Result` (a rust type that have two variants, `Ok` and `Err`):

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(number) => {
        attempts += 1;
        number
    },
    Err(_) => {
        println!("Wrong input! Please, input only positive numbers");
        continue;
    },
};
```

Handle the result of an order comparison:
```rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small"),
    Ordering::Greater => println!("Too big"),
    Ordering::Equal => {
        println!("You win! The secret number was {} and you guessed it in {} attempts", secret_number, attempts);
        break;
    }
}
```

## Loops

`loop { ... }` is used to indicate infinite loops.

`while <condition> { ... }` is a regular while loop that breaks on a condition.

## Docs

It is helpful to run `cargo doc --open`, it will compile the documentation of all the dependencies used by the project and open a webpage with it.