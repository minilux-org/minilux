// The Minilux Programming Language
// Version: 0.1.0
// Author: Alexia Michelle <https://minilux.org>
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use crate::lexer::{self, Lexer, Token};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64),
    String(String),
    Variable(String),
    Binary {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Array(Vec<Expr>),
    Index {
        expr: Box<Expr>,
        index: Box<Expr>,
    },
    FunctionCall {
        name: String,
        #[allow(dead_code)]
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Not,
    Negate,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Assignment {
        var: String,
        value: Expr,
    },
    ArrayAssignment {
        var: String,
        index: Expr,
        value: Expr,
    },
    If {
        condition: Expr,
        then_body: Vec<Statement>,
        elseif_parts: Vec<(Expr, Vec<Statement>)>,
        else_body: Option<Vec<Statement>>,
    },
    While {
        condition: Expr,
        body: Vec<Statement>,
    },
    Printf {
        format: String,
        args: Vec<Expr>,
    },
    Read {
        var: String,
    },
    Inc {
        var: String,
        value: Expr,
    },
    Dec {
        var: String,
        value: Expr,
    },
    Push {
        array: String,
        value: Expr,
    },
    Pop {
        array: String,
    },
    Shift {
        array: String,
    },
    Unshift {
        array: String,
        value: Expr,
    },
    Sockopen {
        name: String,
        host: Expr,
        port: Expr,
    },
    Sockclose {
        name: String,
    },
    Sockwrite {
        name: String,
        data: Expr,
    },
    Sockread {
        name: String,
        var: String,
    },
    Include {
        path: String,
    },
    FunctionDef {
        name: String,
        body: Vec<Statement>,
    },
    FunctionCall {
        name: String,
        #[allow(dead_code)]
        args: Vec<Expr>,
    },
    Return {
        value: Option<Expr>,
    },
}

pub struct Parser {
    tokens: VecDeque<Token>,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        Parser {
            tokens: tokens.into_iter().collect(),
        }
    }

    fn current(&self) -> &Token {
        self.tokens.front().unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) {
        self.tokens.pop_front();
    }

    fn expect(&mut self, expected: Token) -> bool {
        if self.current() == &expected {
            self.advance();
            true
        } else {
            false
        }
    }

    fn skip_newlines(&mut self) {
        while self.current() == &Token::Newline {
            self.advance();
        }
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        self.skip_newlines();

        while self.current() != &Token::Eof {
            self.skip_newlines();
            if self.current() == &Token::Eof {
                break;
            }

            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.skip_newlines();
        }

        statements
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        self.skip_newlines();

        match self.current() {
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            Token::Printf => self.parse_printf(),
            Token::Read => self.parse_read(),
            Token::Inc => self.parse_inc(),
            Token::Dec => self.parse_dec(),
            Token::Push => self.parse_push(),
            Token::Pop => self.parse_pop(),
            Token::Shift => self.parse_shift(),
            Token::Unshift => self.parse_unshift(),
            Token::Sockopen => self.parse_sockopen(),
            Token::Sockclose => self.parse_sockclose(),
            Token::Sockwrite => self.parse_sockwrite(),
            Token::Sockread => self.parse_sockread(),
            Token::Include => self.parse_include(),
            Token::Function => self.parse_function_def(),
            Token::Return => self.parse_return(),
            Token::Sleep => self.parse_sleep(),
            Token::Elseif | Token::Else => {
                // These should have been consumed by the previous if statement
                // If we see them here, skip them to avoid treating them as separate statements
                self.advance();
                None
            }
            Token::Variable(name) => {
                let saved_name = name.clone();
                self.advance();

                if self.current() == &Token::LeftBrace {
                    self.tokens.push_front(Token::LeftBrace);
                    self.tokens.push_front(Token::Variable(saved_name.clone()));
                    self.parse_function_call()
                } else if self.current() == &Token::Equals || self.current() == &Token::LeftBracket
                {
                    self.tokens.push_front(Token::Variable(saved_name.clone()));
                    self.parse_assignment()
                } else if self.current() == &Token::Semicolon
                    || self.current() == &Token::Newline
                    || self.current() == &Token::Eof
                {
                    self.parse_function_call_simple(saved_name)
                } else {
                    self.tokens.push_front(Token::Variable(saved_name.clone()));
                    self.parse_assignment()
                }
            }
            _ => {
                self.advance();
                None
            }
        }
    }

    fn parse_if(&mut self) -> Option<Statement> {
        self.advance();

        if !self.expect(Token::LeftParen) {
            return None;
        }

        let condition = self.parse_expr();
        if !self.expect(Token::RightParen) {
            return None;
        }

        if !self.expect(Token::LeftBrace) {
            return None;
        }

        let then_body = self.parse_block();

        let mut elseif_parts = Vec::new();
        let mut else_body = None;

        self.skip_newlines();
        while self.current() == &Token::Elseif {
            self.advance();

            if !self.expect(Token::LeftParen) {
                break;
            }

            let cond = self.parse_expr();
            if !self.expect(Token::RightParen) {
                break;
            }

            if !self.expect(Token::LeftBrace) {
                break;
            }

            let body = self.parse_block();
            elseif_parts.push((cond, body));
            self.skip_newlines();
        }

        if self.current() == &Token::Else {
            self.advance();
            if self.expect(Token::LeftBrace) {
                else_body = Some(self.parse_block());
            }
        }

        Some(Statement::If {
            condition,
            then_body,
            elseif_parts,
            else_body,
        })
    }

    fn parse_while(&mut self) -> Option<Statement> {
        self.advance();

        if !self.expect(Token::LeftParen) {
            return None;
        }

        let condition = self.parse_expr();
        if !self.expect(Token::RightParen) {
            return None;
        }

        if !self.expect(Token::LeftBrace) {
            return None;
        }

        let body = self.parse_block();

        Some(Statement::While { condition, body })
    }

    fn parse_block(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        self.skip_newlines();

        while self.current() != &Token::RightBrace && self.current() != &Token::Eof {
            self.skip_newlines();

            if self.current() == &Token::RightBrace {
                break;
            }

            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.skip_newlines();
        }

        self.expect(Token::RightBrace);
        statements
    }

    fn parse_assignment(&mut self) -> Option<Statement> {
        if let Token::Variable(var) = self.current() {
            let var_name = var.clone();
            self.advance();

            if self.current() == &Token::LeftBracket {
                self.advance();
                let index = self.parse_expr();
                if !self.expect(Token::RightBracket) {
                    return None;
                }

                if !self.expect(Token::Equals) {
                    return None;
                }

                let value = self.parse_expr();
                self.skip_statement_end();

                return Some(Statement::ArrayAssignment {
                    var: var_name,
                    index,
                    value,
                });
            }

            if !self.expect(Token::Equals) {
                return None;
            }

            let value = self.parse_expr();
            self.skip_statement_end();

            Some(Statement::Assignment {
                var: var_name,
                value,
            })
        } else {
            None
        }
    }

    fn skip_statement_end(&mut self) {
        if self.current() == &Token::Semicolon {
            self.advance();
        }
        while self.current() == &Token::Newline {
            self.advance();
        }
    }

    fn parse_printf(&mut self) -> Option<Statement> {
        self.advance();

        if !self.expect(Token::LeftParen) {
            return None;
        }

        let first_expr = self.parse_expr();
        let mut args = Vec::new();

        // Treat first argument as format string if it's a string, otherwise empty format
        let format_str = if let Expr::String(s) = &first_expr {
            s.clone()
        } else {
            args.push(first_expr);
            String::new()
        };

        while self.current() == &Token::Comma {
            self.advance();
            args.push(self.parse_expr());
        }

        if !self.expect(Token::RightParen) {
            return None;
        }

        self.skip_statement_end();

        Some(Statement::Printf {
            format: format_str,
            args,
        })
    }

    fn parse_read(&mut self) -> Option<Statement> {
        self.advance();

        if !self.expect(Token::LeftParen) {
            return None;
        }

        let var = if let Token::Variable(name) = self.current() {
            let v = name.clone();
            self.advance();
            v
        } else {
            return None;
        };

        if !self.expect(Token::RightParen) {
            return None;
        }

        self.skip_statement_end();

        Some(Statement::Read { var })
    }

    fn parse_inc(&mut self) -> Option<Statement> {
        self.advance();

        if let Token::Variable(var) = self.current() {
            let var_name = var.clone();
            self.advance();

            if !self.expect(Token::Plus) {
                return None;
            }

            let value = self.parse_expr();
            self.skip_statement_end();

            return Some(Statement::Inc {
                var: var_name,
                value,
            });
        }

        None
    }

    fn parse_dec(&mut self) -> Option<Statement> {
        self.advance();

        if let Token::Variable(var) = self.current() {
            let var_name = var.clone();
            self.advance();

            if !self.expect(Token::Minus) {
                return None;
            }

            let value = self.parse_expr();
            self.skip_statement_end();

            return Some(Statement::Dec {
                var: var_name,
                value,
            });
        }

        None
    }

    fn parse_push(&mut self) -> Option<Statement> {
        self.advance();

        if let Token::Variable(var) = self.current() {
            let var_name = var.clone();
            self.advance();

            if !self.expect(Token::Comma) {
                return None;
            }

            let value = self.parse_expr();
            self.skip_statement_end();

            return Some(Statement::Push {
                array: var_name,
                value,
            });
        }

        None
    }

    fn parse_pop(&mut self) -> Option<Statement> {
        self.advance();

        if let Token::Variable(var) = self.current() {
            let var_name = var.clone();
            self.advance();
            self.skip_statement_end();

            return Some(Statement::Pop { array: var_name });
        }

        None
    }

    fn parse_shift(&mut self) -> Option<Statement> {
        self.advance();

        if let Token::Variable(var) = self.current() {
            let var_name = var.clone();
            self.advance();
            self.skip_statement_end();

            return Some(Statement::Shift { array: var_name });
        }

        None
    }

    fn parse_unshift(&mut self) -> Option<Statement> {
        self.advance();

        if let Token::Variable(var) = self.current() {
            let var_name = var.clone();
            self.advance();

            if !self.expect(Token::Comma) {
                return None;
            }

            let value = self.parse_expr();
            self.skip_statement_end();

            return Some(Statement::Unshift {
                array: var_name,
                value,
            });
        }

        None
    }

    fn parse_sockopen(&mut self) -> Option<Statement> {
        self.advance();

        if !self.expect(Token::LeftParen) {
            return None;
        }

        let name = if let Token::String(s) = self.current() {
            let name = s.clone();
            self.advance();
            name
        } else {
            return None;
        };

        if !self.expect(Token::Comma) {
            return None;
        }

        let host = self.parse_expr();

        if !self.expect(Token::Comma) {
            return None;
        }

        let port = self.parse_expr();

        if !self.expect(Token::RightParen) {
            return None;
        }

        self.skip_statement_end();

        Some(Statement::Sockopen { name, host, port })
    }

    fn parse_sockclose(&mut self) -> Option<Statement> {
        self.advance();

        if !self.expect(Token::LeftParen) {
            return None;
        }

        let name = if let Token::String(s) = self.current() {
            let n = s.clone();
            self.advance();
            n
        } else {
            return None;
        };

        if !self.expect(Token::RightParen) {
            return None;
        }

        self.skip_statement_end();

        Some(Statement::Sockclose { name })
    }

    fn parse_sockwrite(&mut self) -> Option<Statement> {
        self.advance();

        if !self.expect(Token::LeftParen) {
            return None;
        }

        let name = if let Token::String(s) = self.current() {
            let n = s.clone();
            self.advance();
            n
        } else {
            return None;
        };

        if !self.expect(Token::Comma) {
            return None;
        }

        let data = self.parse_expr();

        if !self.expect(Token::RightParen) {
            return None;
        }

        self.skip_statement_end();

        Some(Statement::Sockwrite { name, data })
    }

    fn parse_sockread(&mut self) -> Option<Statement> {
        self.advance();

        if !self.expect(Token::LeftParen) {
            return None;
        }

        let name = if let Token::String(s) = self.current() {
            let n = s.clone();
            self.advance();
            n
        } else {
            return None;
        };

        if !self.expect(Token::Comma) {
            return None;
        }

        let var = if let Token::Variable(v) = self.current() {
            let vname = v.clone();
            self.advance();
            vname
        } else {
            return None;
        };

        if !self.expect(Token::RightParen) {
            return None;
        }

        self.skip_statement_end();

        Some(Statement::Sockread { name, var })
    }

    fn parse_include(&mut self) -> Option<Statement> {
        self.advance();

        let path = if let Token::String(s) = self.current() {
            let p = s.clone();
            self.advance();
            p
        } else {
            return None;
        };

        self.skip_statement_end();

        Some(Statement::Include { path })
    }

    fn parse_sleep(&mut self) -> Option<Statement> {
        self.advance();

        if !self.expect(Token::LeftParen) {
            return None;
        }

        let seconds = self.parse_expr();

        if !self.expect(Token::RightParen) {
            return None;
        }

        self.skip_statement_end();

        Some(Statement::FunctionCall {
            name: lexer::token_to_str(&Token::Sleep).to_string(),
            args: vec![seconds],
        })
    }

    fn parse_function_def(&mut self) -> Option<Statement> {
        self.advance();

        let name = if let Token::Variable(n) = self.current() {
            let fname = n.clone();
            self.advance();
            fname
        } else {
            return None;
        };

        if self.current() == &Token::LeftParen {
            self.advance();
            self.expect(Token::RightParen);
        }

        if !self.expect(Token::LeftBrace) {
            return None;
        }

        let body = self.parse_block();

        Some(Statement::FunctionDef { name, body })
    }

    fn parse_return(&mut self) -> Option<Statement> {
        self.advance();

        let value = if self.current() != &Token::Semicolon
            && self.current() != &Token::Newline
            && self.current() != &Token::Eof
        {
            Some(self.parse_expr())
        } else {
            None
        };

        self.skip_statement_end();

        Some(Statement::Return { value })
    }

    fn parse_function_call(&mut self) -> Option<Statement> {
        if let Token::Variable(name) = self.current() {
            let fname = name.clone();
            self.advance();

            let _args: Vec<Expr> = Vec::new(); // TODO: parse args if needed

            if self.expect(Token::LeftBrace) {
                let _body = self.parse_block();
                return Some(Statement::FunctionCall {
                    name: fname,
                    args: vec![],
                });
            }

            self.skip_statement_end();
            Some(Statement::FunctionCall {
                name: fname,
                args: vec![],
            })
        } else {
            None
        }
    }

    fn parse_function_call_simple(&mut self, name: String) -> Option<Statement> {
        self.skip_statement_end();
        Some(Statement::FunctionCall { name, args: vec![] })
    }

    fn parse_expr(&mut self) -> Expr {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Expr {
        let mut left = self.parse_and();

        while matches!(self.current(), Token::Or | Token::Pipe) {
            if self.current() == &Token::Pipe {
                let next_is_pipe = self.tokens.get(1) == Some(&Token::Pipe);
                if !next_is_pipe {
                    break;
                }
                self.advance();
            }
            self.advance();

            let right = self.parse_and();
            left = Expr::Binary {
                left: Box::new(left),
                op: BinOp::Or,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_and(&mut self) -> Expr {
        let mut left = self.parse_equality();

        while matches!(self.current(), Token::And | Token::Ampersand) {
            if self.current() == &Token::Ampersand {
                let next_is_amp = self.tokens.get(1) == Some(&Token::Ampersand);
                if !next_is_amp {
                    break;
                }
                self.advance();
            }
            self.advance();

            let right = self.parse_equality();
            left = Expr::Binary {
                left: Box::new(left),
                op: BinOp::And,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_equality(&mut self) -> Expr {
        let mut left = self.parse_comparison();

        while let Some(op) = match self.current() {
            Token::EqualEqual => Some(BinOp::Equal),
            Token::NotEqual => Some(BinOp::NotEqual),
            _ => None,
        } {
            self.advance();
            let right = self.parse_comparison();
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_comparison(&mut self) -> Expr {
        let mut left = self.parse_additive();

        while let Some(op) = match self.current() {
            Token::Less => Some(BinOp::Less),
            Token::LessEqual => Some(BinOp::LessEqual),
            Token::Greater => Some(BinOp::Greater),
            Token::GreaterEqual => Some(BinOp::GreaterEqual),
            _ => None,
        } {
            self.advance();
            let right = self.parse_additive();
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_additive(&mut self) -> Expr {
        let mut left = self.parse_multiplicative();

        while let Some(op) = match self.current() {
            Token::Plus => Some(BinOp::Add),
            Token::Minus => Some(BinOp::Subtract),
            _ => None,
        } {
            self.advance();
            let right = self.parse_multiplicative();
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_multiplicative(&mut self) -> Expr {
        let mut left = self.parse_unary();

        while let Some(op) = match self.current() {
            Token::Star => Some(BinOp::Multiply),
            Token::Slash => Some(BinOp::Divide),
            Token::Percent => Some(BinOp::Modulo),
            _ => None,
        } {
            self.advance();
            let right = self.parse_unary();
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_unary(&mut self) -> Expr {
        match self.current() {
            Token::Not => {
                self.advance();
                Expr::Unary {
                    op: UnaryOp::Not,
                    expr: Box::new(self.parse_unary()),
                }
            }
            Token::Minus => {
                self.advance();
                Expr::Unary {
                    op: UnaryOp::Negate,
                    expr: Box::new(self.parse_unary()),
                }
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Expr {
        let mut expr = self.parse_primary();

        loop {
            match self.current() {
                Token::LeftBracket => {
                    self.advance();
                    let index = self.parse_expr();
                    self.expect(Token::RightBracket);
                    expr = Expr::Index {
                        expr: Box::new(expr),
                        index: Box::new(index),
                    };
                }
                _ => break,
            }
        }

        expr
    }

    fn parse_builtin_call(&mut self, token: &Token, default: Expr) -> Expr {
        self.advance();
        if self.expect(Token::LeftParen) {
            let arg = self.parse_expr();
            self.expect(Token::RightParen);
            Expr::FunctionCall {
                name: lexer::token_to_str(token).to_string(),
                args: vec![arg],
            }
        } else {
            default
        }
    }

    fn parse_primary(&mut self) -> Expr {
        match self.current().clone() {
            Token::Int(n) => {
                self.advance();
                Expr::Int(n)
            }
            Token::String(s) => {
                self.advance();
                Expr::String(s)
            }
            Token::Len | Token::Number | Token::Sleep => {
                let tok = self.current().clone();
                self.parse_builtin_call(&tok, Expr::Int(0))
            }
            Token::Shell | Token::Lower | Token::Upper => {
                let tok = self.current().clone();
                self.parse_builtin_call(&tok, Expr::String(String::new()))
            }
            Token::Variable(name) => {
                self.advance();

                if self.current() == &Token::LeftParen {
                    self.advance();
                    let mut args = Vec::new();

                    while self.current() != &Token::RightParen && self.current() != &Token::Eof {
                        args.push(self.parse_expr());
                        if self.current() == &Token::Comma {
                            self.advance();
                        }
                    }

                    self.expect(Token::RightParen);

                    Expr::FunctionCall { name, args }
                } else {
                    Expr::Variable(name)
                }
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.parse_expr();
                self.expect(Token::RightParen);
                expr
            }
            Token::LeftBracket => {
                self.advance();
                let mut elements = Vec::new();

                while self.current() != &Token::RightBracket && self.current() != &Token::Eof {
                    elements.push(self.parse_expr());
                    if self.current() == &Token::Comma {
                        self.advance();
                    }
                }

                self.expect(Token::RightBracket);
                Expr::Array(elements)
            }
            _ => {
                self.advance();
                Expr::Int(0)
            }
        }
    }
}
