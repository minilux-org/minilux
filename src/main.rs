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

    let mut text: String =  "".to_owned();

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

        if trimmed == "show" {
            println!("{}", text);
            continue;
        }

        if trimmed == "clear" {
            text.clear();
            continue;
        }

        if trimmed == "ls" {
            let paths = fs::read_dir("./").unwrap();

            for path in paths {
                println!("{}", path.unwrap().path().display())
            }
            continue;
        }

        if trimmed == "save" {
            input.clear();
            if reader.read_line(&mut input).is_err() {
                continue;
            }
            let file = input.trim();
            fs::write( file, text.clone()).expect("err");
            println!("save file: {}", file);
            continue;
        }

        if trimmed == "read" {
            text.clear();
            input.clear();
            if reader.read_line(&mut input).is_err() {
                continue;
            }
            let file = input.trim();
            text = fs::read_to_string(file).expect("err");
            println!("load file: {}", file);
            continue;
        }

        if trimmed == "run" {
            let mut parser = Parser::new(&text);
            let statements = parser.parse();

            let mut interpreter = Interpreter::new();
            if let Err(e) = interpreter.execute(statements) {
                eprintln!("Error: {}", e);
            }
            continue;
        }


        text.push_str(trimmed);
        text.push_str("\n");
    }
}

fn get_system_info() -> String {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    format!("{}/{}", os, arch)
}
