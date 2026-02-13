# Minilux Standard Library Patterns

Reusable patterns and idioms for building standard library functions in Minilux. These patterns work within the language's constraints (global scope, no function parameters, no return values).

---

## Language Constraints to Remember

1. **No function parameters** — pass data via global variables before calling
2. **No return values** — functions write results to global variables
3. **Global scope only** — all variables are shared; use naming conventions to avoid collisions
4. **No local scope** — loop counters inside functions can overwrite outer variables
5. **Integer division only** — `7 / 2` yields `3`
6. **No string split/join** — must use shell for complex string manipulation
7. **No file I/O** — use `shell()` for reading/writing files
8. **No hash maps / dictionaries** — use parallel arrays as key-value pairs

---

## Convention: Function Argument/Return Protocol

Since functions can't take arguments or return values, use this naming convention:

```minilux
# Convention: prefix with underscore for "parameters"
# Convention: prefix with _ret_ for "return values"

$_arg_name = "input"
my_function
$result = $_ret_value

func my_function {
    # read from $_arg_name
    # write to $_ret_value
    $_ret_value = upper($_arg_name)
}
```

---

## Pattern: Shell Wrapper for File I/O

```minilux
# Read file contents
$_arg_path = "data.txt"
func file_read {
    $_ret_content = shell("cat " + $_arg_path)
}

# Write to file
$_arg_path = "output.txt"
$_arg_data = "Hello, World!"
func file_write {
    shell("printf '%s' '" + $_arg_data + "' > " + $_arg_path)
}

# Append to file
func file_append {
    shell("printf '%s\n' '" + $_arg_data + "' >> " + $_arg_path)
}

# Check if file exists (returns "yes" or "no" in $_ret_exists)
func file_exists {
    $_ret_exists = shell("test -f " + $_arg_path + " && echo yes || echo no")
}
```

---

## Pattern: String Utilities via Shell

```minilux
# Trim whitespace
$_arg_str = "  hello  "
func str_trim {
    $_ret_str = shell("echo '" + $_arg_str + "' | xargs")
}

# String contains check
$_arg_haystack = "hello world"
$_arg_needle = "world"
func str_contains {
    $_ret_found = shell("echo '" + $_arg_haystack + "' | grep -c '" + $_arg_needle + "'")
    $_ret_found = number($_ret_found)
}

# Replace substring
$_arg_str = "hello world"
$_arg_from = "world"
$_arg_to = "minilux"
func str_replace {
    $_ret_str = shell("echo '" + $_arg_str + "' | sed 's/" + $_arg_from + "/" + $_arg_to + "/g'")
}

# Split string into array (line by line)
$_arg_str = "a,b,c"
$_arg_delim = ","
func str_split {
    $_ret_str = shell("echo '" + $_arg_str + "' | tr '" + $_arg_delim + "' '\n'")
}
```

---

## Pattern: Parallel Arrays as Key-Value Store

```minilux
$_kv_keys = []
$_kv_vals = []

# Set a key-value pair
$_arg_key = "name"
$_arg_val = "Alexia"
func kv_set {
    # Check if key already exists
    $_kv_i = 0
    $_kv_found = 0
    while ($_kv_i < len($_kv_keys)) {
        if ($_kv_keys[$_kv_i] == $_arg_key) {
            $_kv_vals[$_kv_i] = $_arg_val
            $_kv_found = 1
        }
        inc $_kv_i + 1
    }
    if ($_kv_found == 0) {
        push $_kv_keys, $_arg_key
        push $_kv_vals, $_arg_val
    }
}

# Get a value by key
func kv_get {
    $_ret_value = ""
    $_kv_i = 0
    while ($_kv_i < len($_kv_keys)) {
        if ($_kv_keys[$_kv_i] == $_arg_key) {
            $_ret_value = $_kv_vals[$_kv_i]
        }
        inc $_kv_i + 1
    }
}
```

---

## Pattern: HTTP Request Helper

```minilux
$_arg_host = ""
$_arg_port = 80
$_arg_path = "/"

func http_get {
    $__req = "GET " + $_arg_path + " HTTP/1.1\r\nHost: " + $_arg_host + "\r\nConnection: close\r\n\r\n"
    sockopen("__http", $_arg_host, $_arg_port)
    sockwrite("__http", $__req)
    sockread("__http", $_ret_response)
    sockclose("__http")
}
```

---

## Pattern: JSON-like Parsing via Shell

```minilux
# Extract a value from simple JSON using jq (requires jq installed)
$_arg_json = ""
$_arg_field = ""
func json_get {
    # Write JSON to temp file and extract field
    shell("echo '" + $_arg_json + "' > /tmp/_minilux_json.tmp")
    $_ret_value = shell("cat /tmp/_minilux_json.tmp | jq -r '." + $_arg_field + "'")
    shell("rm /tmp/_minilux_json.tmp")
}
```

---

## Pattern: SQLite via Shell

```minilux
$_db_path = "app.db"

# Execute a SQL statement
$_arg_sql = ""
func db_exec {
    $_ret_result = shell("sqlite3 " + $_db_path + " \"" + $_arg_sql + "\"")
}

# Create a table
func db_create_users {
    $_arg_sql = "CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT, email TEXT);"
    db_exec
}

# Insert a row
$_arg_name = ""
$_arg_email = ""
func db_insert_user {
    $_arg_sql = "INSERT INTO users (name, email) VALUES ('" + $_arg_name + "', '" + $_arg_email + "');"
    db_exec
}

# Query all rows
func db_list_users {
    $_arg_sql = "SELECT * FROM users;"
    db_exec
    printf($_ret_result)
}
```

---

## Pattern: API Consumption via http.mi

```minilux
# Set base URL once, then use path-based requests
$_arg_url = "https://api.example.com"
http_init

# GET with path (auto-prepends base URL)
$_arg_url = "/users/1"
http_get
printf($_ret_content)

# POST with JSON content type
$_arg_url = "/users"
$_arg_data = '{"name": "Alice"}'
http_post_json
printf($_ret_content)
```

---

## Pattern: Error Handling Convention

Since Minilux has no exception system, use a global error flag:

```minilux
$_err = 0
$_err_msg = ""

func check_error {
    if ($_err != 0) {
        printf("ERROR: ", $_err_msg)
    }
}

# Usage in a function
func safe_db_exec {
    $_err = 0
    $_ret_result = shell("sqlite3 " + $_db_path + " \"" + $_arg_sql + "\" 2>&1")
    # Check if output contains "Error"
    $_check = shell("echo '" + $_ret_result + "' | grep -c 'Error'")
    if (number($_check) > 0) {
        $_err = 1
        $_err_msg = $_ret_result
    }
}
```

---

## Naming Conventions Summary

| Prefix       | Purpose                          | Example          |
|--------------|----------------------------------|------------------|
| `$_arg_`     | Function "input parameter"       | `$_arg_name`     |
| `$_ret_`     | Function "return value"          | `$_ret_result`   |
| `$_err`      | Error flag                       | `$_err = 1`      |
| `$_err_msg`  | Error message                    | `$_err_msg = ""` |
| `$__`        | Internal/private temp variable   | `$__req`         |
| `$_kv_`      | Key-value store internals        | `$_kv_keys`      |
