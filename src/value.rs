// The Minilux Programming Language
// Version: 0.1.0
// Author: Alexia Michelle <https://minilux.org>
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use std::cmp::Ordering;
use std::fmt;

/// Represents a value in the minilux language
#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    String(String),
    Array(Vec<Value>),
    Nil,
}

impl Value {
    /// Convert to string representation
    pub fn to_string(&self) -> String {
        match self {
            Value::Int(n) => n.to_string(),
            Value::String(s) => s.clone(),
            Value::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                format!("[{}]", items.join(", "))
            }
            Value::Nil => "nil".to_string(),
        }
    }

    /// Convert to integer
    pub fn to_int(&self) -> i64 {
        match self {
            Value::Int(n) => *n,
            Value::String(s) => s.parse().unwrap_or(0),
            Value::Array(_) => 0,
            Value::Nil => 0,
        }
    }

    /// Check if value is truthy
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Int(n) => *n != 0,
            Value::String(s) => !s.is_empty(),
            Value::Array(arr) => !arr.is_empty(),
            Value::Nil => false,
        }
    }

    /// Compare two values for equality
    pub fn equals(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Nil, Value::Nil) => true,
            (Value::Int(a), Value::String(b)) => a.to_string() == *b,
            (Value::String(a), Value::Int(b)) => a == &b.to_string(),
            _ => false,
        }
    }

    /// Compare two values
    pub fn compare(&self, other: &Value) -> Option<Ordering> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Some(a.cmp(b)),
            (Value::String(a), Value::String(b)) => Some(a.cmp(b)),
            (Value::Int(a), Value::String(b)) => {
                if let Ok(b_int) = b.parse::<i64>() {
                    Some(a.cmp(&b_int))
                } else {
                    None
                }
            }
            (Value::String(a), Value::Int(b)) => {
                if let Ok(a_int) = a.parse::<i64>() {
                    Some(a_int.cmp(b))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Add two values
    pub fn add(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a + b),
            (Value::String(a), Value::String(b)) => Value::String(format!("{}{}", a, b)),
            (Value::Int(a), Value::String(b)) => Value::String(format!("{}{}", a, b)),
            (Value::String(a), Value::Int(b)) => Value::String(format!("{}{}", a, b)),
            _ => Value::Nil,
        }
    }

    /// Subtract two values
    pub fn subtract(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a - b),
            _ => Value::Nil,
        }
    }

    /// Multiply two values
    pub fn multiply(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a * b),
            _ => Value::Nil,
        }
    }

    /// Divide two values
    pub fn divide(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => {
                if *b == 0 {
                    Value::Nil
                } else {
                    Value::Int(a / b)
                }
            }
            _ => Value::Nil,
        }
    }

    /// Modulo two values
    pub fn modulo(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => {
                if *b == 0 {
                    Value::Nil
                } else {
                    Value::Int(a % b)
                }
            }
            _ => Value::Nil,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
