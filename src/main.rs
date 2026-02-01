// The Minilux Programming Language
// Version: 0.1.0
// Author: Alexia Michelle <https://minilux.org>
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

mod interpreter;
mod lexer;
mod parser;
mod runtime;
mod value;

use interpreter::Interpreter;
use parser::Parser;
use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        if let Err(e) = execute_file(&args[1]) {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    } else {
        run_repl();
    }
}

fn execute_file(path: &str) -> Result<(), String> {
    let content = fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;

    let mut parser = Parser::new(&content);
    let statements = parser.parse();

    let mut interpreter = Interpreter::new();
    let absolute_path = {
        let provided = Path::new(path);
        if provided.is_absolute() {
            provided.to_path_buf()
        } else {
            env::current_dir()
                .map_err(|e| format!("Failed to determine current directory: {}", e))?
                .join(provided)
        }
    };

    let base_dir = absolute_path.parent().map(|p| p.to_path_buf());
    if let Some(dir) = base_dir.clone() {
        interpreter.push_base_dir(dir);
    }

    let result = interpreter.execute(statements);

    if base_dir.is_some() {
        interpreter.pop_base_dir();
    }

    result
}

fn run_repl() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();

    println!("Minilux Interpreter Console (REPL)");
    println!("Version 0.1.0 on {} -- [Rust]", get_system_info());
    println!("Type \"exit\" to quit");
    println!();

    loop {
        input.clear();
        print!("> ");
        std::io::stdout().flush().ok();

        if reader.read_line(&mut input).is_err() {
            break;
        }

        let trimmed = input.trim();
        if trimmed == "exit" {
            break;
        }

        if trimmed.is_empty() {
            continue;
        }

        let mut parser = Parser::new(trimmed);
        let statements = parser.parse();

        let mut interpreter = Interpreter::new();
        if let Err(e) = interpreter.execute(statements) {
            eprintln!("Error: {}", e);
        }
    }
}

fn get_system_info() -> String {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    format!("{}/{}", os, arch)
}
