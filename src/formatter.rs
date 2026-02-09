// The Minilux Programming Language
// Source code formatter
// Line-by-line formatting with comment preservation

use crate::lexer::{self, Lexer, Token, EXPR_BUILTINS, KEYWORDS, KEYWORD_ALIASES};

/// Case-insensitive keyword lookup for normalizing misspelled keywords.
fn try_keyword(name: &str) -> Option<Token> {
    let lower = name.to_lowercase();
    for &(kw, ref tok) in KEYWORDS.iter().chain(KEYWORD_ALIASES.iter()) {
        if lower == kw.to_lowercase() {
            return Some(tok.clone());
        }
    }
    None
}

/// Scan a line tracking string context. Calls `visitor` for each char outside strings.
/// Returns early with Some(result) if visitor returns Some.
fn scan_outside_strings<F, R>(line: &str, mut visitor: F) -> Option<R>
where
    F: FnMut(usize, char) -> Option<R>,
{
    let mut in_double = false;
    let mut in_single = false;
    let mut escape = false;

    for (i, ch) in line.char_indices() {
        if escape {
            escape = false;
            continue;
        }
        if ch == '\\' && (in_double || in_single) {
            escape = true;
            continue;
        }
        if ch == '"' && !in_single {
            in_double = !in_double;
        } else if ch == '\'' && !in_double {
            in_single = !in_single;
        } else if !in_double && !in_single {
            if let Some(r) = visitor(i, ch) {
                return Some(r);
            }
        }
    }
    None
}

/// Find the position of `#` comment marker outside of string literals.
fn find_comment_start(line: &str) -> Option<usize> {
    scan_outside_strings(line, |i, ch| if ch == '#' { Some(i) } else { None })
}

/// Collect all `$` positions outside of string literals.
fn find_dollar_positions(code: &str) -> std::collections::HashSet<usize> {
    let mut positions = std::collections::HashSet::new();
    scan_outside_strings(code, |i, ch| {
        if ch == '$' {
            positions.insert(i);
        }
        None::<()>
    });
    positions
}

/// Build indent levels for each line based on brace structure.
fn build_indent_map(lines: &[&str]) -> Vec<usize> {
    let mut indent_map = Vec::with_capacity(lines.len());
    let mut current_indent: i32 = 0;

    for line in lines {
        let trimmed = line.trim();

        let code = if let Some(pos) = find_comment_start(trimmed) {
            trimmed[..pos].trim()
        } else {
            trimmed
        };

        if code.is_empty() {
            indent_map.push(current_indent.max(0) as usize);
            continue;
        }

        if code.starts_with('}') {
            current_indent -= 1;
        }

        indent_map.push(current_indent.max(0) as usize);

        if code.ends_with('{') {
            current_indent += 1;
        }
    }

    indent_map
}

/// A formatting token that preserves whether an identifier had a $ prefix.
#[derive(Debug, Clone)]
enum FmtToken {
    Lexed(Token),
    BareIdent(String),
}

impl FmtToken {
    fn to_output(&self) -> String {
        match self {
            FmtToken::BareIdent(name) => name.clone(),
            FmtToken::Lexed(token) => match token {
                Token::Int(n) => n.to_string(),
                Token::String(s) => {
                    let escaped = s
                        .replace('\\', "\\\\")
                        .replace('\n', "\\n")
                        .replace('\t', "\\t")
                        .replace('\r', "\\r")
                        .replace('"', "\\\"");
                    format!("\"{}\"", escaped)
                }
                Token::Variable(name) => format!("${}", name),
                other => lexer::token_to_str(other).to_string(),
            },
        }
    }

    fn is_binary_op(&self) -> bool {
        matches!(
            self,
            FmtToken::Lexed(
                Token::Plus
                    | Token::Minus
                    | Token::Star
                    | Token::Slash
                    | Token::Percent
                    | Token::Equals
                    | Token::EqualEqual
                    | Token::NotEqual
                    | Token::Less
                    | Token::LessEqual
                    | Token::Greater
                    | Token::GreaterEqual
                    | Token::And
                    | Token::Or
            )
        )
    }

    fn is_opening(&self) -> bool {
        matches!(
            self,
            FmtToken::Lexed(Token::LeftParen | Token::LeftBracket | Token::LeftBrace)
        )
    }

    fn is_closing(&self) -> bool {
        matches!(
            self,
            FmtToken::Lexed(Token::RightParen | Token::RightBracket | Token::RightBrace)
        )
    }

    fn is_callable(&self) -> bool {
        match self {
            FmtToken::BareIdent(_) | FmtToken::Lexed(Token::Variable(_)) => true,
            FmtToken::Lexed(t) => EXPR_BUILTINS.contains(t),
        }
    }
}

/// Tokenize code for formatting, preserving $ vs bare identifier distinction
/// and normalizing misspelled keywords.
fn fmt_tokenize(code: &str) -> Vec<FmtToken> {
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize();
    let dollar_positions = find_dollar_positions(code);

    let mut result = Vec::new();
    let mut source_pos = 0;

    for token in &tokens {
        if matches!(token, Token::Newline | Token::Eof) {
            continue;
        }

        match token {
            Token::Variable(name) => {
                let dollar_pat = format!("${}", name);
                let remaining = &code[source_pos..];

                if let Some(offset) = remaining.find(&dollar_pat) {
                    let abs_pos = source_pos + offset;
                    if dollar_positions.contains(&abs_pos) {
                        result.push(FmtToken::Lexed(token.clone()));
                    } else if let Some(kw) = try_keyword(name) {
                        result.push(FmtToken::Lexed(kw));
                    } else {
                        result.push(FmtToken::BareIdent(name.clone()));
                    }
                    source_pos = abs_pos + dollar_pat.len();
                } else if let Some(offset) = remaining.find(name.as_str()) {
                    if let Some(kw) = try_keyword(name) {
                        result.push(FmtToken::Lexed(kw));
                    } else {
                        result.push(FmtToken::BareIdent(name.clone()));
                    }
                    source_pos = source_pos + offset + name.len();
                } else {
                    result.push(FmtToken::Lexed(token.clone()));
                }
            }
            Token::String(_) => {
                let remaining = &code[source_pos..];
                if let Some(offset) = remaining.find('"').or_else(|| remaining.find('\'')) {
                    let quote_char = remaining.as_bytes()[offset] as char;
                    let mut pos = source_pos + offset + 1;
                    let bytes = code.as_bytes();
                    while pos < bytes.len() {
                        if bytes[pos] == b'\\' {
                            pos += 2;
                        } else if bytes[pos] as char == quote_char {
                            pos += 1;
                            break;
                        } else {
                            pos += 1;
                        }
                    }
                    source_pos = pos;
                }
                result.push(FmtToken::Lexed(token.clone()));
            }
            Token::Int(n) => {
                let n_str = n.to_string();
                if let Some(offset) = code[source_pos..].find(&n_str) {
                    source_pos = source_pos + offset + n_str.len();
                }
                result.push(FmtToken::Lexed(token.clone()));
            }
            _ => {
                let tok_str = lexer::token_to_str(token);
                if !tok_str.is_empty() {
                    let remaining = &code[source_pos..];
                    let remaining_lower = remaining.to_lowercase();
                    let tok_lower = tok_str.to_lowercase();
                    if let Some(offset) = remaining_lower.find(&tok_lower) {
                        source_pos = source_pos + offset + tok_str.len();
                    }
                }
                result.push(FmtToken::Lexed(token.clone()));
            }
        }
    }

    result
}

/// Determine whether a space is needed before the current token.
fn needs_space_before(token: &FmtToken, prev: &FmtToken) -> bool {
    if token.is_closing() {
        return false;
    }
    if matches!(token, FmtToken::Lexed(Token::LeftBracket)) {
        return prev.is_binary_op();
    }
    if matches!(token, FmtToken::Lexed(Token::LeftParen)) {
        return !prev.is_callable() && !prev.is_opening() && !matches!(prev, FmtToken::Lexed(Token::Not));
    }
    if matches!(token, FmtToken::Lexed(Token::Comma | Token::Semicolon)) {
        return false;
    }
    if token.is_binary_op() {
        return true;
    }
    if prev.is_opening() || matches!(prev, FmtToken::Lexed(Token::Not | Token::At)) {
        return false;
    }
    if matches!(prev, FmtToken::Lexed(Token::Comma)) || prev.is_binary_op() {
        return true;
    }
    true
}

/// Format a code portion (without comments) by tokenizing and reconstructing.
fn format_code(code: &str) -> String {
    let tokens = fmt_tokenize(code);

    if tokens.is_empty() {
        return String::new();
    }

    let mut result = String::new();

    for (i, ft) in tokens.iter().enumerate() {
        let text = ft.to_output();
        if text.is_empty() {
            continue;
        }

        if i > 0 && needs_space_before(ft, &tokens[i - 1]) {
            result.push(' ');
        }
        result.push_str(&text);
    }

    result
}

/// Format Minilux source code.
pub fn format_source(source: &str) -> String {
    let lines: Vec<&str> = source.lines().collect();
    let indent_map = build_indent_map(&lines);
    let mut output = Vec::new();
    let mut consecutive_blanks = 0;

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            consecutive_blanks += 1;
            if consecutive_blanks <= 1 {
                output.push(String::new());
            }
            continue;
        }
        consecutive_blanks = 0;

        let indent = "    ".repeat(indent_map[i]);

        if trimmed.starts_with('#') {
            output.push(format!("{}{}", indent, trimmed));
            continue;
        }

        let (code_part, comment_part) = if let Some(pos) = find_comment_start(trimmed) {
            (trimmed[..pos].trim(), Some(trimmed[pos..].trim()))
        } else {
            (trimmed, None)
        };

        let formatted_code = if code_part.is_empty() {
            String::new()
        } else {
            format_code(code_part)
        };

        let formatted_line = match comment_part {
            Some(comment) if formatted_code.is_empty() => format!("{}{}", indent, comment),
            Some(comment) => format!("{}{}  {}", indent, formatted_code, comment),
            None => format!("{}{}", indent, formatted_code),
        };

        output.push(formatted_line.trim_end().to_string());
    }

    let mut result = output.join("\n");
    if !result.ends_with('\n') {
        result.push('\n');
    }
    result
}
