# 🎮 Guess the Number Game

A delightful command-line game where players try to guess a randomly generated number between 1 and 100. Built with Rust's powerful features and safety guarantees.

## 📦 Understanding Rust Crates

In Rust, a crate is the fundamental unit of code organization - similar to packages in Python, libraries in C++, or modules in JavaScript. Crates can be:

- Binary crates (executable programs)
- Library crates (reusable code)

In this project, we use:

- `rand`: For generating random numbers
- `std`: The standard library, providing core functionality
  - `io`: For input/output operations
  - `cmp`: For comparison operations

## 🛠️ How to Add Dependencies

In Rust, you can add dependencies to your project using the `cargo add` command. To remove a dependency, use the `cargo remove` command.

```bash
cargo add rand
```

This will add the `rand` crate to your project and update the `Cargo.toml` file with the dependency.

## 📚 Key Rust Concepts Used

### Variables and Mutability

- Variables are declared using the `let` keyword
- Variables are immutable by default in Rust
- To make a variable mutable, use the `mut` keyword
- Example: `let mut guess = String::new();`

### Data Types

- `String`: A growable string type (used for user input)
- `u32`: 32-bit unsigned integer (used for the secret number and guess)
- Type annotations can be explicit: `let secret_number: u32 = ...`

### Control Flow

#### Loops

- `loop`: Creates an infinite loop
- `break`: Exits the loop
- Example:

```rust
loop {
    // code
    if condition {
        break;
    }
}
```

#### Pattern Matching vs If-Else

In our code, we have two ways to compare the guess with the secret number:

1. Using `match` with `cmp` (commented out in our code):

```rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
}
```

2. Using `if-else` (currently used in our code):

```rust
if guess < secret_number {
    println!("Too small!");
} else if guess > secret_number {
    println!("Too big!");
} else {
    println!("You win!");
}
```

For JavaScript/TypeScript developers:

- `match` is similar to `switch` but more powerful:
  - It's exhaustive (must handle all possible cases)
  - It can destructure and pattern match
  - It's more idiomatic in Rust
  - Similar to TypeScript's discriminated unions
- `if-else` works just like in JavaScript/TypeScript
  - More familiar syntax
  - More flexible for complex conditions
  - Can be easier to read for simple comparisons

The `cmp` method returns an `Ordering` enum which has three variants:

- `Less`: when the first value is smaller
- `Greater`: when the first value is larger
- `Equal`: when the values are equal

This is similar to how JavaScript's comparison operators work (`<`, `>`, `===`), but wrapped in a type-safe enum.

### Input/Output

- `println!`: Macro for printing to console
- `io::stdin()`: For reading user input
- `read_line()`: Reads a line of input
- `expect()`: Handles potential errors
- Example:

```rust
io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
```

### Error Handling

- `expect()`: Used for basic error handling
- `parse()`: Converts string to number, returns Result type
- Example: `guess.trim().parse().expect("Please type a valid number!")`

### String Methods

- `trim()`: Removes whitespace
- `parse()`: Converts string to number
- Example: `guess.trim().parse()`

### Random Number Generation

- Using the `rand` crate
- `rand::thread_rng()`: Creates a random number generator
- `gen_range()`: Generates a random number in a range
- Example: `rand::thread_rng().gen_range(1..=100)`

## 🎯 How to Play

1. Run the game using `cargo run`
2. Enter a number between 1 and 100
3. The game will tell you if your guess is too high or too low
4. Keep guessing until you find the correct number!

## 🚀 Running the Project

```bash
cargo run
```
