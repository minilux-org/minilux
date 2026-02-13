# Minilux Knowledge Base

Internal documentation and patterns for developing Minilux programs and standard libraries.

## Contents

| Document | Description |
|----------|-------------|
| [language-reference.md](language-reference.md) | Complete language reference — variables, types, operators, control flow, built-ins, sockets, functions |
| [examples-reference.md](examples-reference.md) | Annotated examples from the repository with concepts breakdown and common patterns |
| [stdlib-patterns.md](stdlib-patterns.md) | Reusable patterns for building stdlib — file I/O, key-value stores, HTTP, SQLite ORM, API consumption, error handling |
| [grammar.ebnf](../grammar.ebnf) | Formal EBNF grammar specification |

## Language Constraints Summary

- **No function parameters** — pass data via global `$_arg_*` variables
- **No return values** — write results to global `$_ret_*` variables
- **Global scope only** — all variables are shared across the entire program
- **Integer arithmetic only** — no floating point (`7 / 2` yields `3`)
- **No file I/O built-in** — use `shell()` to call system commands
- **No hash maps** — use parallel arrays as key-value pairs
- **No string split/join** — use `shell()` with `tr`, `sed`, `awk`

## External Resources

- Website: https://minilux.org
- Docs: https://minilux.org/docs/intro
- Repository: https://github.com/minilux-org/minilux
