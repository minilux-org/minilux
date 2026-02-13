# Minilux Examples Reference

Annotated collection of all example programs from the repository.

---

## 1. Common Utilities (common.mi)

Shared helper library included by other examples. Demonstrates `func` definitions and `include` usage.

```minilux
$helper_message = "Helpers loaded from include"

func banner {
    printf("==============================")
    printf("   Minilux Feature Showcase   ")
    printf("==============================")
}

func divider {
    printf("------------------------------")
}

func wait_briefly {
    printf("Pausing briefly...")
    sleep(1)
    printf("Resume after sleep.")
}

func show_return_demo {
    printf("About to exit helper early.")
    return
    printf("This line never executes.")
}
```

**Concepts:** `func`, `printf`, `sleep()`, `return`, global variables.

---

## 2. Feature Showcase (showcase.mi)

Comprehensive demonstration of most language features.

```minilux
include "common.mi"

banner
printf($helper_message)
divider

# Variables and arithmetic
$name = "Alexia"
$language = "Minilux"
$year = 2026
printf("Welcome ", $name, " to ", $language, " in ", $year, "!")

# inc/dec
$age = 20
printf("Starting age: ", $age)
inc $age + 2
printf("After inc: ", $age)
dec $age - 1
printf("After dec: ", $age)

# Compound conditions with AND
if (($age >= 18) AND ($language == "Minilux")) {
    printf("You are an adult Minilux fan.")
} elseif ($age >= 13) {
    printf("You are a teen explorer.")
} else {
    printf("You are a young learner.")
}

# Array operations
$numbers = [1, 2, 3]
$numbers[1] = 99
printf("numbers[1] adjusted to ", $numbers[1])

push $numbers, 4
unshift $numbers, 0
divider
printf("Iterating array after push/unshift")
$i = 0
while ($i < len($numbers)) {
    printf("Index ", $i, ": ", $numbers[$i])
    inc $i + 1
}

pop $numbers
shift $numbers
divider
printf("Array trimmed again; length now ", len($numbers))

# String functions
$word = "Minilux"
printf("Word length: ", len($word), ", first char: ", $word[0])
printf("lower(\"MIXED\"): ", lower("MIXED"))
printf("upper($word): ", upper($word))

# Logical NOT
$flag = 0
if (!($flag)) {
    printf("Logical NOT verified.")
}

# OR operator
if (($numbers[0] == 1) OR ($age > 100)) {
    printf("OR condition evaluated true.")
}

# Shell integration
divider
$today = shell("date +%Y-%m-%d")
printf("Today from shell: ", $today)

# User-defined function using global arrays
func inspect_numbers {
    divider
    printf("Inspecting array contents...")
    $idx = 0
    while ($idx < len($numbers)) {
        printf(" -> ", $numbers[$idx])
        inc $idx + 1
    }
    divider
}

inspect_numbers
wait_briefly
show_return_demo
printf("Returned from helper call.")

divider
printf("Showcase complete.")
```

**Concepts:** `include`, `inc`/`dec`, `if`/`elseif`/`else`, `AND`/`OR`/`!`, arrays, `push`/`pop`/`shift`/`unshift`, `len()`, `lower()`/`upper()`, `shell()`, `func`, `while`, string indexing.

---

## 3. Temperature Converter (temperature_converter.mi)

Interactive program with user input and arithmetic.

```minilux
include "common.mi"

banner
divider
printf("Temperature converter")

printf("Enter the temperature value:")
read($raw_temp)
$temp = number($raw_temp)

printf("Is this value in Celsius or Fahrenheit? [C/F]")
read($scale_input)
$scale = lower($scale_input)

divider
if ($scale == "c") {
    $result = ($temp * 9 / 5) + 32
    printf($temp, " °C is ", $result, " °F")
} elseif ($scale == "f") {
    $result = ($temp - 32) * 5 / 9
    printf($temp, " °F is ", $result, " °C")
} else {
    printf("Please answer with C or F.")
}

divider
printf("Conversion complete.")
```

**Concepts:** `read()`, `number()`, `lower()`, arithmetic expressions, `if`/`elseif`/`else`.

---

## 4. Input Demo (input_demo.mi)

Simple interactive input handling.

```minilux
include "common.mi"

banner
divider
printf("Interactive input demo (press Ctrl+C to exit)")

printf("What is your name?")
read($name)
printf("Hi ", upper($name), "!")

printf("Do you like Minilux? [y/n]")
read($answer)
$choice = lower($answer)

if ($choice == "y") {
    printf("Great! Let's build something together.")
} elseif ($choice == "n") {
    printf("Thanks for giving it a try anyway.")
} else {
    printf("I'll take that as a maybe.")
}

divider
printf("Demo complete.")
```

**Concepts:** `read()`, `upper()`, `lower()`, conditional branching.

---

## 5. Network Request (network_request.mi)

TCP socket HTTP GET request.

```minilux
include "common.mi"

banner
divider
$host = "example.com"
$port = 80

printf("Starting HTTP GET to ", $host, "\n")

$request = "GET / HTTP/1.1\r\n" + "Host: " + $host + "\r\nConnection: close\r\n\r\n"

sockopen("web", $host, $port)
sockwrite("web", $request)
sockread("web", $response)

if (len($response) == 0) {
    printf("No data received.\n")
} else {
    printf("Received ", len($response), " bytes.\n")
}

divider
printf("Response preview:\n")
printf($response)

sockclose("web")
divider
printf("Network demo finished.\n")
```

**Concepts:** `sockopen`, `sockwrite`, `sockread`, `sockclose`, string concatenation, `len()`.

---

## 6. FizzBuzz (fmt/fluzz_buzz.mi)

Classic programming challenge — shows pre-formatter style.

```minilux
$i = 1
while ($i <= 20) {
    $mod3 = $i % 3
    $mod5 = $i % 5

    if ((($mod3 == 0) AND ($mod5 == 0))) {
        printf("FizzBuzz")
    } elseif ($mod3 == 0) {
        printf("Fizz")
    } elseif ($mod5 == 0) {
        printf("Buzz")
    } else {
        printf($i)
    }

    inc $i + 1
}
```

**Concepts:** `%` modulo, `while`, compound `AND` conditions, `inc`.

---

## Common Patterns

### Iterate an array
```minilux
$i = 0
while ($i < len($arr)) {
    printf($arr[$i])
    inc $i + 1
}
```

### Read and convert user input
```minilux
printf("Enter a number:")
read($raw)
$num = number($raw)
```

### Accumulate a sum
```minilux
$sum = 0
$i = 1
while ($i <= 100) {
    $sum = $sum + $i
    inc $i + 1
}
printf("Sum: ", $sum)
```

### Find maximum in array
```minilux
$arr = [3, 7, 2, 9, 4]
$max = $arr[0]
$i = 1
while ($i < len($arr)) {
    if ($arr[$i] > $max) {
        $max = $arr[$i]
    }
    inc $i + 1
}
printf("Max: ", $max)
```

### Build a string from shell output
```minilux
$user = shell("whoami")
$host = shell("hostname")
$date = shell("date +%Y-%m-%d")
printf("User: ", $user, " Host: ", $host, " Date: ", $date)
```

### Simulate a progress bar
```minilux
$i = 0
while ($i < 10) {
    printf(".")
    sleep(1)
    inc $i + 1
}
printf("Done!")
```
