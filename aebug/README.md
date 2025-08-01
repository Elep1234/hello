# aebug: The AI-Powered Debugger

`aebug` is a fast, portable, and secure AI-powered debugger CLI agent that helps you find and fix bugs in your code.

## Features

- **Multi-language support:** Can be extended to support any programming language. Currently supports Python.
- **Static analysis:** Analyzes code for syntax errors and other issues without running it.
- **Dynamic analysis:** Runs code in a sandboxed environment to detect runtime errors.
- **AI-powered suggestions:** Provides helpful suggestions to fix detected bugs.
- **Root cause detection:** Differentiates between syntax errors, runtime errors, and hangs.

## Usage

To debug a file, simply run the following command:

```bash
cargo run -- <path/to/your/file>
```

For example, to debug a Python file named `my_script.py`:

```bash
cargo run -- my_script.py
```

## Building from source

To build the project from source, you will need to have Rust installed. You can find instructions on how to install Rust [here](https://www.rust-lang.org/tools/install).

Once you have Rust installed, you can build the project by running the following command in the root of the project directory:

```bash
cargo build
```

The compiled binary will be located in the `target/debug/` directory.
