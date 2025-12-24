# REI

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.0+-orange.svg)](https://www.rust-lang.org)

**REI** (Recursive Execution Interpreter) is a modern, dynamically-typed programming language with a tree-walk AST interpreter written in Rust. REI combines the simplicity of scripting languages with powerful features like classes, inheritance, closures, and a comprehensive standard library.

## What is REI?

REI is an interpreted programming language designed for rapid development and ease of use. It features:

- **Clean, readable syntax** inspired by modern scripting languages
- **Object-oriented programming** with classes and inheritance
- **First-class functions** and closures
- **Comprehensive standard library** for common tasks
- **Native performance** through Rust-backed native functions
- **JIT compilation support** (experimental, using Cranelift)

## Features

### Language Features

- **Dynamic typing** with runtime type checking
- **Classes and inheritance** with method overriding
- **Functions** as first-class citizens with closures
- **Control flow**: `if/else`, `while`, `for`, `loop` statements
- **Error handling** with `do/fail` blocks
- **Module system** with `use` statements
- **Static methods** and instance methods
- **Property access** and assignment

### Standard Library

REI includes a rich standard library covering:

- **Math**: Trigonometric functions, logarithms, random numbers, constants
- **Collections**: Vectors, arrays, and data structures
- **I/O**: Standard input/output operations
- **File System**: File and directory operations
- **Networking**: HTTP client and server capabilities
- **Chronology**: Date and time operations
- **Process**: System process management

### Native Functions

High-performance native functions implemented in Rust for:
- Mathematical operations
- Network I/O
- File system operations
- System calls
- Memory management

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- Cargo (comes with Rust)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/rei.git
cd rei

# Build the project
cargo build --release

# Install the standard library
cargo run --release -- setup

# The binary will be in target/release/rei
```

### Setting Up the Standard Library

After building, install the standard library:

```bash
rei setup
```

This installs the standard library to:
- **Linux**: `/usr/share/rei/std`
- **macOS**: `/usr/local/share/rei/std`
- **Windows**: `C:\ProgramData\rei\std`

You can customize the installation path by setting the `REI_HOME` environment variable.

## Quick Start

### Hello, World!

Create a file `hello.reix`:

```reix
println "Hello, World!";
```

Run it:

```bash
rei hello.reix
```

### Creating a New Project

```bash
rei new myproject
```

This creates a new project structure:

```
myproject/
├── main.reix
├── lib/
│   └── std/
└── .gitignore
```

### Example: Classes and Inheritance

```reix
class Person {
    init(name, age) {
        this.name = name;
        this.age = age;
    }

    greet() {
        println "Hello, I'm " + this.name;
    }
}

class Student < Person {
    init(name, age, school) {
        base.init(name, age);
        this.school = school;
    }

    study() {
        println this.name + " is studying at " + this.school;
    }
}

let student = Student("Alice", 20, "University");
student.greet();
student.study();
```

### Example: HTTP Server

```reix
use std/net as Net;
use std/request as Request;

let app = Net();

fn handler() {
    return Request.get("https://api.example.com/data");
}

app.set_get("/api", handler);
app.serve("127.0.0.1:8080");
```

### Example: Using the Standard Library

```reix
use std/math as Math;

let pi = Math.PI();
let result = Math.pow(2, 10);
println "2^10 = " + result;

let random = Math.random_range(1, 100);
println "Random number: " + random;
```

## Usage

### Running Files

```bash
rei <file.reix>
```

### Interactive Mode

```bash
rei
```

### Running Tests

```bash
rei test <test_number>
```

### AST Code Generation

```bash
rei gen
```

## Language Syntax

### Variables

```reix
let x = 10;
let name = "REI";
let isActive = true;
```

### Functions

```reix
fn add(a, b) {
    return a + b;
}

fn greet(name) {
    println "Hello, " + name;
}
```

### Classes

```reix
class Rectangle {
    init(width, height) {
        this.width = width;
        this.height = height;
    }

    area() {
        return this.width * this.height;
    }

    static create_square(side) {
        return Rectangle(side, side);
    }
}
```

### Control Flow

```reix
// If/Else
if (x > 10) {
    println "Large";
} else {
    println "Small";
}

// While loop
let i = 0;
while (i < 10) {
    println i;
    i = i + 1;
}

// For loop
for (let i = 0; i < 10; i = i + 1) {
    println i;
}
```

### Error Handling

```reix
do {
    let result = risky_operation();
    println result;
} fail {
    println "An error occurred";
}
```

### Modules

```reix
use std/math as Math;
use std/io/std_in as Input;

let value = Math.sqrt(16);
let userInput = Input.read_line();
```

## Project Structure

```
rei/
├── src/
│   ├── backend/          # Interpreter, JIT compiler, native functions
│   ├── crux/             # Core runner and utilities
│   ├── frontend/          # Lexer, parser, AST
│   ├── std/              # Standard library (.reix files)
│   ├── tests/            # Test suite
│   └── tools/            # Development tools
├── Cargo.toml
└── README.md
```

## Architecture

REI uses a tree-walk AST interpreter architecture:

1. **Lexer**: Tokenizes source code into tokens
2. **Parser**: Builds an Abstract Syntax Tree (AST) from tokens
3. **Resolver**: Performs variable resolution and scope analysis
4. **Interpreter**: Executes the AST by walking the tree
5. **JIT Compiler**: (Experimental) Compiles hot paths to native code using Cranelift

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Setup

```bash
# Clone and build
git clone https://github.com/yourusername/rei.git
cd rei
cargo build

# Run tests
cargo test

# Run specific test file
cargo run -- test <test_number>
```

### Code Generation

The project uses code generation for AST nodes. To regenerate:

```bash
cargo run -- gen
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- JIT compilation powered by [Cranelift](https://github.com/bytecodealliance/wasmtime/tree/main/cranelift)
- Language design inspired by modern scripting languages

## Resources

- **Grammar**: See [src/frontend/grammer.md](src/frontend/grammer.md) for the complete language grammar
- **Test Examples**: Check [src/tests/code/](src/tests/code/) for example REI programs
- **Standard Library**: Browse [src/std/](src/std/) for available modules

## Status

REI is currently in active development. The core language features are stable, but some advanced features (like full JIT compilation) are still experimental.

---

**Note**: REI stands for "Recursive Execution Interpreter" - a name that reflects the tree-walk interpreter architecture, though the execution model is more nuanced than pure recursion.
