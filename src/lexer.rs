// The Minilux Programming Language
// Version: 0.1.0
// Author: Alexia Michelle <https://minilux.org>
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0
// - Control
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Int(i64),
    String(String),
    Variable(String),

    // Keywords
    If,
    Elseif,
    Else,
    While,
    Printf,
    Shell,
    Len,
    Sleep,
    Inc,
    Dec,
    Array,
    Push,
    Pop,
    Shift,
    Unshift,
    Sockopen,
    Sockclose,
    Sockwrite,
    Sockread,
    Sockstatus,
    Read,
    Lower,
    Upper,
    Number,
    Include,
    Function,
    Return,
    And,
    Or,
    Not,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equals,
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Ampersand,
    Pipe,
    At,

    // Delimiters
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Semicolon,
    Comma,
    Dot,

    // Special
    Newline,
    Eof,
}

/// Tokens that act as expression-level built-in functions (called with parens).
/// Used by both the parser and formatter to avoid repeating this list.
pub const EXPR_BUILTINS: &[Token] = &[
    Token::Shell,
    Token::Len,
    Token::Lower,
    Token::Upper,
    Token::Number,
    Token::Printf,
    Token::Sleep,
];

/// (canonical_name, Token) — single source of truth for keyword ↔ string mapping.
pub const KEYWORDS: &[(&str, Token)] = &[
    ("if", Token::If),
    ("elseif", Token::Elseif),
    ("else", Token::Else),
    ("while", Token::While),
    ("printf", Token::Printf),
    ("shell", Token::Shell),
    ("len", Token::Len),
    ("sleep", Token::Sleep),
    ("inc", Token::Inc),
    ("dec", Token::Dec),
    ("array", Token::Array),
    ("push", Token::Push),
    ("pop", Token::Pop),
    ("shift", Token::Shift),
    ("unshift", Token::Unshift),
    ("sockopen", Token::Sockopen),
    ("sockclose", Token::Sockclose),
    ("sockwrite", Token::Sockwrite),
    ("sockread", Token::Sockread),
    ("sockstatus", Token::Sockstatus),
    ("read", Token::Read),
    ("lower", Token::Lower),
    ("upper", Token::Upper),
    ("number", Token::Number),
    ("include", Token::Include),
    ("func", Token::Function),
    ("return", Token::Return),
    ("AND", Token::And),
    ("OR", Token::Or),
];

/// Aliases that also resolve to keyword tokens (not canonical output names).
pub const KEYWORD_ALIASES: &[(&str, Token)] = &[
    ("print", Token::Printf),
    ("strlen", Token::Len),
    ("function", Token::Function),
    ("not", Token::Not),
];

/// Exact-match keyword lookup (used by the lexer itself).
pub fn match_keyword(name: &str) -> Option<Token> {
    for &(kw, ref tok) in KEYWORDS.iter().chain(KEYWORD_ALIASES.iter()) {
        if name == kw {
            return Some(tok.clone());
        }
    }
    None
}

/// Canonical string representation for any Token.
pub fn token_to_str(token: &Token) -> &'static str {
    for &(name, ref tok) in KEYWORDS {
        if tok == token {
            return name;
        }
    }
    match token {
        Token::Not => "!",
        Token::Plus => "+",
        Token::Minus => "-",
        Token::Star => "*",
        Token::Slash => "/",
        Token::Percent => "%",
        Token::Equals => "=",
        Token::EqualEqual => "==",
        Token::NotEqual => "!=",
        Token::Less => "<",
        Token::LessEqual => "<=",
        Token::Greater => ">",
        Token::GreaterEqual => ">=",
        Token::Ampersand => "&",
        Token::Pipe => "|",
        Token::At => "@",
        Token::LeftBrace => "{",
        Token::RightBrace => "}",
        Token::LeftParen => "(",
        Token::RightParen => ")",
        Token::LeftBracket => "[",
        Token::RightBracket => "]",
        Token::Semicolon => ";",
        Token::Comma => ",",
        Token::Dot => ".",
        _ => "",
    }
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    current: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input: input.chars().peekable(),
            current: None,
        };
        lexer.advance();
        lexer
    }

    fn advance(&mut self) {
        self.current = self.input.next();
    }

    #[allow(dead_code)]
    fn peek(&mut self) -> Option<char> {
        self.input.peek().copied()
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current {
            if ch == ' ' || ch == '\t' || ch == '\r' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_comment(&mut self) {
        if self.current == Some('#') {
            while self.current.is_some() && self.current != Some('\n') {
                self.advance();
            }
        }
    }

    fn read_string(&mut self, quote: char) -> String {
        let mut result = String::new();
        self.advance();

        while let Some(ch) = self.current {
            if ch == quote {
                self.advance();
                break;
            } else if ch == '\\' {
                self.advance();
                match self.current {
                    Some('n') => result.push('\n'),
                    Some('t') => result.push('\t'),
                    Some('r') => result.push('\r'),
                    Some('\\') => result.push('\\'),
                    Some('"') => result.push('"'),
                    Some('\'') => result.push('\''),
                    Some(c) => result.push(c),
                    None => break,
                }
                self.advance();
            } else {
                result.push(ch);
                self.advance();
            }
        }

        result
    }

    fn read_number(&mut self) -> i64 {
        let mut num_str = String::new();
        while let Some(ch) = self.current {
            if ch.is_ascii_digit() {
                num_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        num_str.parse().unwrap_or(0)
    }

    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();
        while let Some(ch) = self.current {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        ident
    }

    pub fn next_token(&mut self) -> Token {
        loop {
            self.skip_whitespace();

            if self.current == Some('#') {
                self.skip_comment();
                continue;
            }
            break;
        }

        match self.current {
            None => Token::Eof,
            Some('\n') => {
                self.advance();
                Token::Newline
            }
            Some('+') => {
                self.advance();
                Token::Plus
            }
            Some('-') => {
                self.advance();
                Token::Minus
            }
            Some('*') => {
                self.advance();
                Token::Star
            }
            Some('/') => {
                self.advance();
                Token::Slash
            }
            Some('%') => {
                self.advance();
                Token::Percent
            }
            Some('=') => {
                self.advance();
                if self.current == Some('=') {
                    self.advance();
                    Token::EqualEqual
                } else {
                    Token::Equals
                }
            }
            Some('!') => {
                self.advance();
                if self.current == Some('=') {
                    self.advance();
                    Token::NotEqual
                } else {
                    Token::Not
                }
            }
            Some('<') => {
                self.advance();
                if self.current == Some('=') {
                    self.advance();
                    Token::LessEqual
                } else {
                    Token::Less
                }
            }
            Some('>') => {
                self.advance();
                if self.current == Some('=') {
                    self.advance();
                    Token::GreaterEqual
                } else {
                    Token::Greater
                }
            }
            Some('&') => {
                self.advance();
                if self.current == Some('&') {
                    self.advance();
                    Token::And
                } else {
                    Token::Ampersand
                }
            }
            Some('|') => {
                self.advance();
                if self.current == Some('|') {
                    self.advance();
                    Token::Or
                } else {
                    Token::Pipe
                }
            }
            Some('$') => {
                self.advance();
                let name = self.read_identifier();
                Token::Variable(name)
            }
            Some('@') => {
                self.advance();
                Token::At
            }
            Some('{') => {
                self.advance();
                Token::LeftBrace
            }
            Some('}') => {
                self.advance();
                Token::RightBrace
            }
            Some('(') => {
                self.advance();
                Token::LeftParen
            }
            Some(')') => {
                self.advance();
                Token::RightParen
            }
            Some('[') => {
                self.advance();
                Token::LeftBracket
            }
            Some(']') => {
                self.advance();
                Token::RightBracket
            }
            Some(';') => {
                self.advance();
                Token::Semicolon
            }
            Some(',') => {
                self.advance();
                Token::Comma
            }
            Some('.') => {
                self.advance();
                Token::Dot
            }
            Some('"') => Token::String(self.read_string('"')),
            Some('\'') => Token::String(self.read_string('\'')),
            Some(ch) if ch.is_ascii_digit() => Token::Int(self.read_number()),
            Some(ch) if ch.is_alphabetic() || ch == '_' => {
                let ident = self.read_identifier();
                match_keyword(&ident).unwrap_or(Token::Variable(ident))
            }
            Some(_) => {
                self.advance();
                self.next_token()
            }
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            if token == Token::Eof {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }
        tokens
    }
}
