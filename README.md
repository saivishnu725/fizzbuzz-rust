# FizzBuzz in Rust

This repository contains a simple implementation of the classic **FizzBuzz** game written in Rust. It is part of my learning journey with the [*The Rust Programming Language*](https://doc.rust-lang.org/book/) book.

## What is FizzBuzz?

FizzBuzz is a basic programming exercise often used to teach beginners. The rules are:

- Print numbers from 1 to `n`.
- (BONUS) If a number is divisible by `2`, print *"Fudge*" instead of the number.
- If a number is divisible by `3`, print *"Fizz*" instead of the number.
- (BONUS) If a number is divisible by both `2` and `3`, print *"FudgeFizz*" instead.
- If a number is divisible by `5`, print *"Buzz*" instead of the number.
- (BONUS) If a number is divisible by both `2` and `5`, print *"FudgeBuzz*" instead.
- If a number is divisible by both `3` and `5`, print *"FizzBuzz*" instead.
- (BONUS) If a number is divisible by `2`, `3` and `5`, print *"FudgeFizzBuzz*" instead.

## About the Project

This project is implemented as a Rust Cargo package. It demonstrates:

- Using `if` and `else if` statements for conditional logic.
- Writing a simple loop.
- Printing output to the console.

## Usage

### Prerequisites

- Rust and Cargo should be installed. If not, install them via [rustup](https://rustup.rs/).

### Running the Program

1. Clone this repository:

   ```bash
   git clone https://github.com/your-username/fizzbuzz-rust.git
   cd fizzbuzz-rust
   ```

2. Run the program:

   ```bash
   cargo run
   ```

3. The program will output FizzBuzz results up to a hardcoded number (e.g., 100). Feel free to modify the code to change the range.

### Example Output

```bash
1
Fudge
Fizz
Fudge
Buzz
FudgeFizz
7
Fudge
Fizz
FudgeBuzz
11
FudgeFizz
13
Fudge
FizzBuzz
...
```

## License

This project is licensed under the [**GPT-v2 License**](https://choosealicense.com/licenses/gpl-2.0/).

---
