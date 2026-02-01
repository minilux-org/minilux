// The Minilux Programming Language
// Version: 0.1.0
// Author: Alexia Michelle <https://minilux.org>
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use crate::parser::Statement;
use crate::value::Value;
use std::collections::HashMap;
use std::net::TcpStream;

pub struct Runtime {
    variables: HashMap<String, Value>,
    sockets: HashMap<String, TcpStream>,
    functions: HashMap<String, Vec<Statement>>,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            variables: HashMap::new(),
            sockets: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn get_var(&self, name: &str) -> Value {
        self.variables.get(name).cloned().unwrap_or(Value::Nil)
    }

    pub fn set_var(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn get_socket(&mut self, name: &str) -> Option<&mut TcpStream> {
        self.sockets.get_mut(name)
    }

    pub fn set_socket(&mut self, name: String, stream: TcpStream) {
        self.sockets.insert(name, stream);
    }

    pub fn remove_socket(&mut self, name: &str) {
        self.sockets.remove(name);
    }

    #[allow(dead_code)]
    pub fn has_socket(&self, name: &str) -> bool {
        self.sockets.contains_key(name)
    }

    pub fn define_function(&mut self, name: String, body: Vec<Statement>) {
        self.functions.insert(name, body);
    }

    pub fn get_function(&self, name: &str) -> Option<Vec<Statement>> {
        self.functions.get(name).cloned()
    }

    #[allow(dead_code)]
    pub fn variables(&self) -> &HashMap<String, Value> {
        &self.variables
    }
}
