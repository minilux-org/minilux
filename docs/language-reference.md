# Minilux Language Reference

Complete reference for the Minilux programming language v0.1.0.

## Table of Contents

- [Variables](#variables)
- [Data Types](#data-types)
- [Operators](#operators)
- [Control Flow](#control-flow)
- [Built-in Functions](#built-in-functions)
- [Array Operations](#array-operations)
- [Socket Operations](#socket-operations)
- [User-Defined Functions](#user-defined-functions)
- [Include](#include)
- [Comments and Shebangs](#comments-and-shebangs)

---

## Variables

Variables start with `$` and can hold integers, strings, or arrays. The language uses dynamic typing — variables can change types at runtime.

```minilux
$name = "Alexia"
$age = 42
$pi = 3
$isActive = 1
$list = [1, 2, 3]
```

### Naming Rules

- Must start with `$`
- Followed by letters, numbers, or underscores
- Case-sensitive (`$Name` and `$name` are different)

### Scope

All variables share **global scope**. Variables declared inside functions or control structures are accessible everywhere in the program.

### Reassignment

Variables can be reassigned to different types at any time:

```minilux
$x = 42
$x = "now a string"
$x = [1, 2, 3]
```

---

## Data Types

### Integers

Whole number values including negative numbers and zero:

```minilux
$age = 42
$negative = -10
$zero = 0
```

### Strings

Strings use double or single quotes with escape sequences:

```minilux
$greeting = "Hello, World!"
$single = 'Also valid'
```

**Escape sequences:** `\n` (newline), `\t` (tab), `\r` (carriage return), `\\` (backslash), `\"` (double quote), `\'` (single quote).

**String interpolation:** Variables can be embedded within double-quoted strings:

```minilux
$name = "World"
printf("Hello $name")
```

**String indexing:** 0-based character access with bracket notation:

```minilux
$text = "Hello"
printf($text[0])    # prints "H"
printf(len($text))  # prints 5
```

### Arrays

Arrays can hold multiple values of any type:

```minilux
$numbers = [1, 2, 3, 4, 5]
$mixed = [1, "hello", 42]
$empty = []
```

**Array indexing:** 0-based with bracket notation. Elements can be reassigned:

```minilux
$arr = [10, 20, 30]
printf($arr[0])     # prints 10
$arr[1] = 99        # modify element
```

### Nil

The `Nil` value represents the absence of a value. Returned by unsupported operations.

### Mixed-Type Operations

- `Int + String` produces string concatenation
- Unsupported type combinations return `Nil`

---

## Operators

### Arithmetic Operators

| Operator | Operation      | Example          |
|----------|---------------|------------------|
| `+`      | Addition       | `5 + 3` → `8`   |
| `-`      | Subtraction    | `10 - 4` → `6`  |
| `*`      | Multiplication | `6 * 7` → `42`  |
| `/`      | Division       | `20 / 4` → `5`  |
| `%`      | Modulo         | `17 % 5` → `2`  |

Parentheses control evaluation order:

```minilux
$result = (10 + 5) * 2   # 30
$calc = 1 + (4 / 2)      # 3
```

### Comparison Operators

Evaluate to `1` (true) or `0` (false):

| Operator | Meaning               |
|----------|-----------------------|
| `==`     | Equal                 |
| `!=`     | Not equal             |
| `>`      | Greater than          |
| `<`      | Less than             |
| `>=`     | Greater than or equal |
| `<=`     | Less than or equal    |

### Logical Operators

| Operator    | Meaning     |
|-------------|-------------|
| `AND` / `&&` | Logical AND |
| `OR` / `||`  | Logical OR  |
| `!`          | Logical NOT |

**CRITICAL:** When using `AND` or `OR` in conditions, each sub-expression must be wrapped in parentheses (double-parentheses pattern):

```minilux
# CORRECT — double parentheses
if (($age >= 18) AND ($name == "Alexia")) {
    printf("Match")
}

# INCORRECT — will cause parse error
if ($age >= 18 AND $name == "Alexia") {
    printf("This won't work")
}
```

Simple conditions use a single set of parentheses:

```minilux
if ($age >= 18) {
    printf("Adult")
}
```

Logical NOT:

```minilux
if (!($flag)) {
    printf("Flag is false/zero")
}
```

### Operator Precedence (highest to lowest)

1. Parentheses `()`
2. Unary: `!`, `-`
3. Multiplication, Division, Modulo: `*`, `/`, `%`
4. Addition, Subtraction: `+`, `-`
5. Comparison: `<`, `<=`, `>`, `>=`
6. Equality: `==`, `!=`
7. Logical AND: `AND`, `&&`
8. Logical OR: `OR`, `||`

---

## Control Flow

### if / elseif / else

```minilux
if ($score >= 90) {
    printf("Grade: A")
} elseif ($score >= 80) {
    printf("Grade: B")
} elseif ($score >= 70) {
    printf("Grade: C")
} else {
    printf("Grade: F")
}
```

Nested conditionals are supported:

```minilux
if ($age >= 18) {
    if ($hasID == 1) {
        printf("Access granted")
    }
}
```

### while

```minilux
$i = 1
while ($i <= 5) {
    printf("Count: ", $i)
    inc $i + 1
}
```

Countdown pattern:

```minilux
$countdown = 10
while ($countdown > 0) {
    printf($countdown, "...")
    sleep(1)
    dec $countdown - 1
}
printf("Go!")
```

---

## Built-in Functions

### printf() / print()

Print text by concatenating all arguments. Auto-appends `\n` if output doesn't end with one. `print` is an alias for `printf`.

```minilux
printf("Hello, ", $name, "!")
printf("Number: ", 42)
printf($name, " is ", $age, " years old")
```

### read()

Read a line from stdin (without trailing newline) into a variable:

```minilux
printf("What is your name?")
read($name)
printf("Hello ", $name, "!")
```

### len() / strlen()

Get the length of strings or arrays. `strlen` is an alias for `len`.

```minilux
$text = "Hello"
printf(len($text))      # 5

$arr = [1, 2, 3]
printf(len($arr))       # 3
```

### number()

Convert strings to numeric values for arithmetic. Returns `0` if parsing fails.

```minilux
read($input)
$value = number($input)
printf("Twice is ", $value * 2)
```

### lower() / upper()

Normalize string casing:

```minilux
$answer = "YeS"
if (lower($answer) == "yes") {
    printf("Confirmed")
}
printf(upper("minilux"))   # "MINILUX"
```

### shell()

Execute system commands and capture output. Trailing newline is automatically removed.

```minilux
$user = shell("whoami")
$date = shell("date +%Y-%m-%d")
$count = shell("ls -l | wc -l")
```

### sleep()

Pause execution for specified seconds. Can be used as both statement and expression.

```minilux
sleep(1)
sleep(3)
```

### inc / dec

Increment or decrement variables (statement form, not a function call):

```minilux
$counter = 0
inc $counter + 1    # counter is now 1
inc $counter + 5    # counter is now 6
dec $counter - 2    # counter is now 4
```

---

## Array Operations

Statement-level operations on arrays:

| Statement              | Effect                    |
|------------------------|---------------------------|
| `push $arr, value`     | Append element to end     |
| `pop $arr`             | Remove last element       |
| `shift $arr`           | Remove first element      |
| `unshift $arr, value`  | Prepend element to start  |

```minilux
$list = [1, 2, 3]
push $list, 4        # [1, 2, 3, 4]
pop $list            # [1, 2, 3]
shift $list          # [2, 3]
unshift $list, 0     # [0, 2, 3]
```

Iterate an array:

```minilux
$arr = [10, 20, 30]
$i = 0
while ($i < len($arr)) {
    printf("Index ", $i, ": ", $arr[$i])
    inc $i + 1
}
```

---

## Socket Operations

Minilux supports TCP socket programming with named sockets:

| Statement                                  | Effect                         |
|--------------------------------------------|--------------------------------|
| `sockopen("name", "host", port)`           | Open TCP connection            |
| `sockwrite("name", data)`                  | Send data through socket       |
| `sockread("name", $var)`                   | Read response into variable    |
| `sockclose("name")`                        | Close the socket               |

Example HTTP request:

```minilux
$host = "example.com"
$port = 80
$request = "GET / HTTP/1.1\r\nHost: " + $host + "\r\nConnection: close\r\n\r\n"

sockopen("web", $host, $port)
sockwrite("web", $request)
sockread("web", $response)
printf($response)
sockclose("web")
```

---

## User-Defined Functions

Define reusable code blocks with `func` (or `function`). Functions have **no parameters** and **no local scope** — they share global state.

```minilux
func greet {
    printf("Hello from a custom function!")
}

func show_version {
    printf("Version: ", $app_version)
}

# Call functions by name
greet
show_version
```

### return

Exit a function early with `return`:

```minilux
func check_access {
    if ($role != "admin") {
        printf("Access denied")
        return
    }
    printf("Welcome, admin")
}
```

### Global Scope Behavior

Functions access and modify global variables:

```minilux
$counter = 0
func increment {
    inc $counter + 1
}
increment
printf($counter)    # prints 1
```

---

## Include

Import and execute other Minilux files. Variables and functions defined in the included file become available globally.

```minilux
include "lib.mi"
include "config.mi"
```

Included files are resolved relative to the including file's directory.

**Pattern — shared library:**

```minilux
# lib.mi
$app_name = "My App"
func banner {
    printf("=== ", $app_name, " ===")
}

# main.mi
include "lib.mi"
banner
```

---

## Comments and Shebangs

Lines starting with `#` are comments:

```minilux
# This is a comment
$x = 42  # inline comments are NOT supported (this is part of the string/token)
```

Shebang support for executable scripts:

```minilux
#!/usr/bin/env minilux
$name = "World"
printf("Hello, ", $name)
```

---

## File Extension

Minilux scripts use the `.mi` extension.

## Statement Terminators

Semicolons are optional; newlines act as statement terminators.

## Formatter

The built-in formatter (`minilux fmt`) normalizes:

- 4-space indentation
- Consistent spacing around operators and after commas
- Keyword casing: keywords lowercase, `AND`/`OR` uppercase
- Comment preservation
- Blank line collapsing

```sh
minilux fmt script.mi          # stdout
minilux fmt -w script.mi       # in-place
minilux fmt script.mi > out.mi # redirect
```
