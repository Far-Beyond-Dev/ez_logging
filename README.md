# ez_logging

`ez_logging` is a simple, easy-to-use logging library for Rust projects. It overrides the standard `println!` macro to log messages to both the console and a file simultaneously, with automatic timestamping.

## Features

- Override `println!` to log messages to console and file simultaneously
- Automatic timestamping of log messages
- Thread-safe logging
- Easy integration into existing projects without changing print statements

## Installation

Run `cargo add ez_logging`, or add the following to `Cargo.toml`

```toml
[dependencies]
ez_logging = v0.1.1
```

## Usage

1. Add the following to the root of your main.rs or lib.rs:

```rust
#[macro_use]
extern crate ez_logging;
```

2. Initialize the logging system at the start of your program:

```rust
fn main() {
    ez_logging::init();
    
    // Your code here
}
```

3. Use `println!` as you normally would. It will now log to both console and file:

```rust
println!("This is a log message");
println!("You can use {} too", "formatting");
```

4. Log messages will appear in both the console and a file named `server.log` in your project directory.

## Example

```rust
#[macro_use]
extern crate ez_logging;

fn main() {
    ez_logging::init();
    
    println!("Starting application");
    
    for i in 1..=5 {
        println!("Processing item {}", i);
    }
    
    println!("Application finished");
}
```

## Output

Console and `server.log`:

```
[2023-05-20 15:30:45] Logging system initialized
[2023-05-20 15:30:45] Starting application
[2023-05-20 15:30:45] Processing item 1
[2023-05-20 15:30:45] Processing item 2
[2023-05-20 15:30:45] Processing item 3
[2023-05-20 15:30:45] Processing item 4
[2023-05-20 15:30:45] Processing item 5
[2023-05-20 15:30:45] Application finished
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
