# minilux — minimal experimental language

<div align="center">
  <img src="minilux.png" alt="minilux logo" width="200"/>
  
  **A minimal experimental language for learning purposes**
</div>

---

## About

Minilux is a minimal scripting language designed for simplicity and learning. It features:

- **Variables** with dynamic typing (integers, strings, arrays)
- **Control flow** (if/elseif/else, while loops)
- **Functions** (user-defined and built-in)
- **Arrays** with indexing and manipulation operations
- **String operations** including indexing and interpolation
- **TCP sockets** for network programming
- **Shell integration** for executing system commands

## Quick Start

### Prerequisites

To build minilux from source, you need:

- **Rust toolchain** (cargo + rustc)

Install on Debian/Ubuntu:
```sh
sudo apt install cargo
```

Install on other systems:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Build

```sh
make
```

Or use Cargo directly:

```sh
cargo build --release
```

### Install

```sh
make install
```

This installs `minilux` to `/usr/bin`, allowing you to run scripts directly with a shebang.

### Run an Example

```sh
./minilux examples/test.mi
```

Or make the script executable:

```sh
chmod +x examples/test.mi
./examples/test.mi
```

## Language Reference

### Variables

Variables start with `$` and can hold integers, strings, or arrays:

```minilux
$name = "Alexia"
$age = 42
$result = 1 + 2
$list = [1, 2, 3]
```

### Control Structures

#### if / elseif / else

**Important:** Compound boolean conditions require double parentheses:

```minilux
if (($age >= 18) AND ($name == "Alexia")) {
    printf("Adult named Alexia\n")
}
elseif ($age >= 13) {
    printf("Teenager\n")
}
else {
    printf("Child\n")
}
```

Simple conditions work with single parentheses:

```minilux
if ($age >= 18) {
    printf("Adult\n")
}
```

#### while loops

```minilux
$i = 1
while ($i <= 5) {
    printf("Count: ", $i, "\n")
    inc $i + 1
}
```

### Operators

#### Comparison Operators
- `==` equal
- `!=` not equal
- `>` greater than
- `<` less than
- `>=` greater than or equal
- `<=` less than or equal

#### Logical Operators
- `AND` or `&&` logical and
- `OR` or `||` logical or

**Note:** When using AND/OR operators in conditions, use double parentheses:

```minilux
if (($x == 1) AND ($y == 2)) {
    printf("Both conditions met\n")
}

if (($a != 0) OR ($b != 0)) {
    printf("At least one is non-zero\n")
}
```

#### Arithmetic Operators
- `+` addition
- `-` subtraction
- `*` multiplication
- `/` division
- `%` modulo

Expressions support parentheses:

```minilux
$result = (10 + 5) * 2
$calc = 1 + (4 / 2)
```

### Arrays

Create and manipulate arrays:

```minilux
$arr = [1, 2, 3]
printf("Element 0: ", $arr[0], "\n")
printf("Length: ", len($arr), "\n")

push $arr, 4
pop $arr
shift $arr
unshift $arr, 0
```

String indexing also works:

```minilux
$text = "Hello"
printf("First char: ", $text[0], "\n")  # prints "H"
printf("Length: ", len($text), "\n")     # prints "5"
```

### Built-in Functions

#### printf() / print()

Print text and variables by concatenating all arguments:

```minilux
# Concatenate strings and variables
printf("Hello, ", $name, "!\n")
print("I am ", $age, " years old\n")

# Print numbers
printf("Number: ", 42, "\n")

# Mix text and variables
printf($name, " is ", $age, " years old\n")

# Single argument
printf("Hello world\n")
printf($name)
```

`print` is an alias of `printf`; both names behave identically and automatically append a trailing newline when one is missing.

Escape sequences:
- `\n` newline
- `\t` tab

**Note:** If the output doesn't end with `\n`, one is automatically added.

#### read()

Read a line from standard input (without the trailing newline) and store it in a variable:

```minilux
printf("What is your name?\n")
read($name)
printf("Hello ", $name, "!\n")
```

#### len()

Get the length of strings or arrays:

```minilux
$text = "Hello"
printf("Length: ", len($text), "\n")  # 5

$arr = [1, 2, 3]
printf("Array length: ", len($arr), "\n")  # 3
```

#### number()

Convert strings (or existing integers) into numeric values for arithmetic:
IMPORTANT: parser fails return 0 (e.g: giving a non-numeric value for example number(hello))

```minilux
read($input)
$value = number($input)
printf("Twice is ", $value * 2, "\n")
```

#### lower() / upper()

Normalize string casing in expressions:

```minilux
$answer = "YeS"
if (lower($answer) == "yes") {
    printf("Confirmed\n")
}

printf("Shouting: ", upper("minilux"), "\n")
```

#### shell()

Execute system shell commands and capture output:

```minilux
$user = shell("whoami")
printf("Current user: ", $user, "\n")

# Commands with pipes
$count = shell("ls -l | wc -l")
printf("File count: ", $count, "\n")

# Date commands
$date = shell("date +%Y-%m-%d")
printf("Today: ", $date, "\n")
```

**Note:** The trailing newline is automatically removed from command output.

#### inc / dec

Increment or decrement variables:

```minilux
$counter = 0
inc $counter + 1   # counter is now 1
inc $counter + 5   # counter is now 6
dec $counter - 2   # counter is now 4
```

#### Array Operations

- `push $array, value` - Add element to end
- `pop $array` - Remove element from end
- `shift $array` - Remove element from beginning
- `unshift $array, value` - Add element to beginning

```minilux
$list = [1, 2, 3]
push $list, 4        # [1, 2, 3, 4]
pop $list            # [1, 2, 3]
shift $list          # [2, 3]
unshift $list, 0     # [0, 2, 3]
```

#### Socket Operations

Minilux supports TCP socket programming:

- `sockopen("name", "host", port)` - Open a TCP connection
- `sockwrite("name", "data")` - Send data to socket
- `sockread("name", $var)` - Read data into variable
- `sockclose("name")` - Close the socket

Example HTTP request:

```minilux
sockopen("web", "example.com", 80)
sockwrite("web", "GET / HTTP/1.1\r\nHost: example.com\r\n\r\n")
sockread("web", $response)
printf("Response: $response\n")
sockclose("web")
```

#### include

Include and execute other minilux files:

```minilux
include "lib.mi"
```

### User-Defined Functions

Define reusable code blocks with `func`:

```minilux
func greet {
    printf("Hello from function!\n")
}

func show_name {
    printf("My name is $name\n")
}

greet
show_name
```

### Comments

Lines starting with `#` are comments (primarily for shebangs).

### File Extension

Minilux scripts use the `.mi` extension.

### Shebang Support

Make scripts executable with a shebang:

```minilux
#!/usr/bin/minilux

$name = "World"
printf("Hello, $name!\n")
```

Then run directly:

```sh
chmod +x script.mi
./script.mi
```

## Examples

See the `examples/` directory for sample programs:
- `test.mi` - Basic language features
- `array_test_1.mi` - Array operations and string indexing
- `array_test_2.mi` - Array modification
- `http_test.mi` - TCP socket usage

## Project Structure

```
minilux/
├── src/
│   ├── main.rs         # Entry point and CLI
│   ├── value.rs        # Value type system
│   ├── lexer.rs        # Tokenization
│   ├── parser.rs       # AST generation
│   ├── interpreter.rs  # Execution engine
│   └── runtime.rs      # Runtime state management
├── examples/           # Example scripts
├── Cargo.toml          # Rust dependencies
├── Makefile            # Build automation
└── README.md           # This file
```

## Make Targets

- `make` — Build the release binary
- `make install` — Install to /usr/bin (requires sudo)
- `make uninstall` — Remove from /usr/bin
- `make clean` — Remove build artifacts

## Building from Source

Requirements:
- Rust 1.85.0 or later
- Cargo (comes with Rust)

```sh
# Build debug version
cargo build

# Build optimized release version
cargo build --release

# Run tests
./target/release/minilux examples/test.mi
```

## Why Rust?

This implementation was rewritten from C to Rust to provide:

- **Memory safety** - No segfaults or buffer overflows
- **Better error handling** - Proper error propagation
- **Modern tooling** - Cargo for builds and dependencies
- **Maintainability** - Cleaner, more organized code structure
- **Type safety** - Catch more bugs at compile time

## Author

**Alexia Michelle**  
✉️ <alexia@goldendoglinux.org>

## License

See LICENSE file for details.

---

*This interpreter is intentionally small and experimental — a great starting point for learning language design and implementation.*