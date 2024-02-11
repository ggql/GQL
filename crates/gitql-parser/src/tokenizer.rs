#[derive(PartialEq)]
pub enum TokenKind {
    Set,
    Select,
    Distinct,
    From,
    Group,
    Where,
    Having,
    Limit,
    Offset,
    Order,
    By,
    In,
    Is,
    Not,
    Like,
    Glob,

    Case,
    When,
    Then,
    Else,
    End,

    Between,
    DotDot,

    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Equal,
    Bang,
    BangEqual,
    NullSafeEqual,

    As,

    LeftParen,
    RightParen,

    LogicalOr,
    LogicalAnd,
    LogicalXor,

    BitwiseOr,
    BitwiseAnd,
    BitwiseRightShift,
    BitwiseLeftShift,

    Symbol,
    GlobalVariable,
    Integer,
    Float,
    String,

    True,
    False,
    Null,

    ColonEqual,

    Plus,
    Minus,
    Star,
    Slash,
    Percentage,

    Comma,
    Dot,
    Semicolon,

    Ascending,
    Descending,
}

#[derive(Copy, Clone)]
pub struct Location {
    pub start: usize,
    pub end: usize,
}

pub struct Token {
    pub location: Location,
    pub kind: TokenKind,
    pub literal: String,
}

use crate::diagnostic::Diagnostic;

pub fn tokenize(script: String) -> Result<Vec<Token>, Box<Diagnostic>> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut position = 0;
    let mut column_start;

    let characters: Vec<char> = script.chars().collect();
    let len = characters.len();

    while position < len {
        column_start = position;

        let char = characters[position];

        // Symbol
        if char.is_alphabetic() {
            tokens.push(consume_identifier(
                &characters,
                &mut position,
                &mut column_start,
            ));
            continue;
        }

        // Global Variable Symbol
        if char == '@' {
            tokens.push(consume_global_variable_name(
                &characters,
                &mut position,
                &mut column_start,
            )?);
            continue;
        }

        // Number
        if char.is_numeric() {
            if char == '0' && position + 1 < len {
                if characters[position + 1] == 'x' {
                    position += 2;
                    column_start += 2;
                    tokens.push(consume_hex_number(
                        &characters,
                        &mut position,
                        &mut column_start,
                    )?);
                    continue;
                }

                if characters[position + 1] == 'b' {
                    position += 2;
                    column_start += 2;
                    tokens.push(consume_binary_number(
                        &characters,
                        &mut position,
                        &mut column_start,
                    )?);
                    continue;
                }

                if characters[position + 1] == 'o' {
                    position += 2;
                    column_start += 2;
                    tokens.push(consume_octal_number(
                        &characters,
                        &mut position,
                        &mut column_start,
                    )?);
                    continue;
                }
            }

            tokens.push(consume_number(
                &characters,
                &mut position,
                &mut column_start,
            )?);
            continue;
        }

        // String literal
        if char == '"' {
            tokens.push(consume_string(
                &characters,
                &mut position,
                &mut column_start,
            )?);
            continue;
        }

        // All chars between two backticks should be consumed as identifier
        if char == '`' {
            tokens.push(consume_backticks_identifier(
                &characters,
                &mut position,
                &mut column_start,
            )?);
            continue;
        }

        // Plus
        if char == '+' {
            let location = Location {
                start: column_start,
                end: position,
            };

            let token = Token {
                location,
                kind: TokenKind::Plus,
                literal: "+".to_owned(),
            };

            tokens.push(token);
            position += 1;
            continue;
        }

        // Minus
        if char == '-' {
            // Ignore single line comment which from -- until the end of the current line
            if position + 1 < characters.len() && characters[position + 1] == '-' {
                ignore_single_line_comment(&characters, &mut position);
                continue;
            }

            let location = Location {
                start: column_start,
                end: position,
            };

            let token = Token {
                location,
                kind: TokenKind::Minus,
                literal: "-".to_owned(),
            };

            tokens.push(token);
            position += 1;
            continue;
        }

        // Star
        if char == '*' {
            let location = Location {
                start: column_start,
                end: position,
            };

            let token = Token {
                location,
                kind: TokenKind::Star,
                literal: "*".to_owned(),
            };

            tokens.push(token);
            position += 1;
            continue;
        }

        // Slash
        if char == '/' {
            // Ignore C style comment which from /* comment */
            if position + 1 < characters.len() && characters[position + 1] == '*' {
                ignore_c_style_comment(&characters, &mut position)?;
                continue;
            }

            let location = Location {
                start: column_start,
                end: position,
            };

            let token = Token {
                location,
                kind: TokenKind::Slash,
                literal: "/".to_owned(),
            };

            tokens.push(token);
            position += 1;
            continue;
        }

        // Percentage
        if char == '%' {
            let location = Location {
                start: column_start,
                end: position,
            };

            let token = Token {
                location,
                kind: TokenKind::Percentage,
                literal: "%".to_owned(),
            };

            tokens.push(token);
            position += 1;
            continue;
        }

        // Or
        if char == '|' {
            let location = Location {
                start: column_start,
                end: position,
            };

            position += 1;

            let mut kind = TokenKind::BitwiseOr;
            let literal = if position < len && characters[position] == '|' {
                position += 1;
                kind = TokenKind::LogicalOr;
                "||"
            } else {
                "|"
            };

            let token = Token {
                location,
                kind,
                literal: literal.to_string(),
            };

            tokens.push(token);
            continue;
        }

        // And
        if char == '&' {
            let location = Location {
                start: column_start,
                end: position,
            };

            position += 1;
            let mut kind = TokenKind::BitwiseAnd;
            let literal = if position < len && characters[position] == '&' {
                position += 1;
                kind = TokenKind::LogicalAnd;
                "&&"
            } else {
                "&"
            };

            let token = Token {
                location,
                kind,
                literal: literal.to_string(),
            };

            tokens.push(token);
            continue;
        }

        // xor
        if char == '^' {
            let location = Location {
                start: column_start,
                end: position,
            };

            let token = Token {
                location,
                kind: TokenKind::LogicalXor,
                literal: "^".to_owned(),
            };

            tokens.push(token);
            position += 1;
            continue;
        }

        // Comma
        if char == ',' {
            let location = Location {
                start: column_start,
                end: position,
            };

            let token = Token {
                location,
                kind: TokenKind::Comma,
                literal: ",".to_owned(),
            };

            tokens.push(token);
            position += 1;
            continue;
        }

        // Dot or Range (DotDot)
        if char == '.' {
            let location = Location {
                start: column_start,
                end: position,
            };

            position += 1;

            let mut kind = TokenKind::Dot;
            let literal = if position < len && characters[position] == '.' {
                position += 1;
                kind = TokenKind::DotDot;
                ".."
            } else {
                "."
            };

            let token = Token {
                location,
                kind,
                literal: literal.to_string(),
            };

            tokens.push(token);
            continue;
        }

        // Greater or GreaterEqual
        if char == '>' {
            let location = Location {
                start: column_start,
                end: position,
            };

            position += 1;

            let mut kind = TokenKind::Greater;
            let literal = if position < len && characters[position] == '=' {
                position += 1;
                kind = TokenKind::GreaterEqual;
                ">="
            } else if position < len && characters[position] == '>' {
                position += 1;
                kind = TokenKind::BitwiseRightShift;
                ">>"
            } else {
                ">"
            };

            let token = Token {
                location,
                kind,
                literal: literal.to_string(),
            };

            tokens.push(token);
            continue;
        }

        // Less, LessEqual or NULL-safe equal
        if char == '<' {
            let location = Location {
                start: column_start,
                end: position,
            };

            position += 1;

            let mut kind = TokenKind::Less;
            let literal = if position < len && characters[position] == '=' {
                position += 1;
                if position < len && characters[position] == '>' {
                    position += 1;
                    kind = TokenKind::NullSafeEqual;
                    "<=>"
                } else {
                    kind = TokenKind::LessEqual;
                    "<="
                }
            } else if position < len && characters[position] == '<' {
                position += 1;
                kind = TokenKind::BitwiseLeftShift;
                "<<"
            } else if position < len && characters[position] == '>' {
                position += 1;
                kind = TokenKind::BangEqual;
                "<>"
            } else {
                "<"
            };

            let token = Token {
                location,
                kind,
                literal: literal.to_owned(),
            };

            tokens.push(token);
            continue;
        }

        // Equal
        if char == '=' {
            let location = Location {
                start: column_start,
                end: position,
            };

            let token = Token {
                location,
                kind: TokenKind::Equal,
                literal: "=".to_owned(),
            };

            tokens.push(token);
            position += 1;
            continue;
        }

        // Colon Equal
        if char == ':' {
            if position + 1 < len && characters[position + 1] == '=' {
                let location = Location {
                    start: column_start,
                    end: position,
                };

                let token = Token {
                    location,
                    kind: TokenKind::ColonEqual,
                    literal: ":=".to_owned(),
                };

                tokens.push(token);
                position += 2;
                continue;
            }

            return Err(Diagnostic::error("Expect `=` after `:`")
                .add_help("Only token that has `:` is `:=` so make sure you add `=` after `:`")
                .with_location_span(column_start, position)
                .as_boxed());
        }

        // Bang or Bang Equal
        if char == '!' {
            let location = Location {
                start: column_start,
                end: position,
            };

            position += 1;

            let mut kind = TokenKind::Bang;
            let literal = if position < len && characters[position] == '=' {
                position += 1;
                kind = TokenKind::BangEqual;
                "!="
            } else {
                "!"
            };

            let token = Token {
                location,
                kind,
                literal: literal.to_owned(),
            };

            tokens.push(token);
            continue;
        }

        // Left Paren
        if char == '(' {
            let location = Location {
                start: column_start,
                end: position,
            };

            let token = Token {
                location,
                kind: TokenKind::LeftParen,
                literal: "(".to_owned(),
            };

            tokens.push(token);
            position += 1;
            continue;
        }

        // Right Paren
        if char == ')' {
            let location = Location {
                start: column_start,
                end: position,
            };

            let token = Token {
                location,
                kind: TokenKind::RightParen,
                literal: ")".to_owned(),
            };

            tokens.push(token);
            position += 1;
            continue;
        }

        // Semicolon
        if char == ';' {
            let location = Location {
                start: column_start,
                end: position,
            };

            let token = Token {
                location,
                kind: TokenKind::Semicolon,
                literal: ";".to_owned(),
            };

            tokens.push(token);
            position += 1;
            continue;
        }

        // Characters to ignoring
        if char == ' ' || char == '\n' || char == '\t' {
            position += 1;
            continue;
        }

        return Err(Diagnostic::error("Unexpected character")
            .with_location_span(column_start, position)
            .as_boxed());
    }

    Ok(tokens)
}

fn consume_global_variable_name(
    chars: &Vec<char>,
    pos: &mut usize,
    start: &mut usize,
) -> Result<Token, Box<Diagnostic>> {
    // Consume `@`
    *pos += 1;

    // Make sure first character is  alphabetic
    if *pos < chars.len() && !chars[*pos].is_alphabetic() {
        return Err(
            Diagnostic::error("Global variable name must start with alphabetic character")
                .add_help("Add at least one alphabetic character after @")
                .with_location_span(*start, *pos)
                .as_boxed(),
        );
    }

    while *pos < chars.len() && (chars[*pos] == '_' || chars[*pos].is_alphanumeric()) {
        *pos += 1;
    }

    // Identifier is be case-insensitive by default, convert to lowercase to be easy to compare and lookup
    let literal = &chars[*start..*pos];
    let string = String::from_utf8(literal.iter().map(|&c| c as u8).collect())
        .unwrap()
        .to_lowercase();

    let location = Location {
        start: *start,
        end: *pos,
    };

    Ok(Token {
        location,
        kind: TokenKind::GlobalVariable,
        literal: string,
    })
}

fn consume_identifier(chars: &Vec<char>, pos: &mut usize, start: &mut usize) -> Token {
    while *pos < chars.len() && (chars[*pos] == '_' || chars[*pos].is_alphanumeric()) {
        *pos += 1;
    }

    // Identifier is be case-insensitive by default, convert to lowercase to be easy to compare and lookup
    let literal = &chars[*start..*pos];
    let string = String::from_utf8(literal.iter().map(|&c| c as u8).collect())
        .unwrap()
        .to_lowercase();

    let location = Location {
        start: *start,
        end: *pos,
    };

    Token {
        location,
        kind: resolve_symbol_kind(string.to_string()),
        literal: string,
    }
}

fn consume_number(
    chars: &Vec<char>,
    pos: &mut usize,
    start: &mut usize,
) -> Result<Token, Box<Diagnostic>> {
    let mut kind = TokenKind::Integer;

    while *pos < chars.len() && (chars[*pos].is_numeric() || chars[*pos] == '_') {
        *pos += 1;
    }

    if *pos < chars.len() && chars[*pos] == '.' {
        *pos += 1;

        kind = TokenKind::Float;
        while *pos < chars.len() && (chars[*pos].is_numeric() || chars[*pos] == '_') {
            *pos += 1;
        }
    }

    let literal = &chars[*start..*pos];
    let string = String::from_utf8(literal.iter().map(|&c| c as u8).collect()).unwrap();
    let literal_num = string.replace('_', "");

    let location = Location {
        start: *start,
        end: *pos,
    };

    Ok(Token {
        location,
        kind,
        literal: literal_num,
    })
}

fn consume_backticks_identifier(
    chars: &Vec<char>,
    pos: &mut usize,
    start: &mut usize,
) -> Result<Token, Box<Diagnostic>> {
    *pos += 1;

    while *pos < chars.len() && chars[*pos] != '`' {
        *pos += 1;
    }

    if *pos >= chars.len() {
        return Err(Diagnostic::error("Unterminated backticks")
            .add_help("Add ` at the end of the identifier")
            .with_location_span(*start, *pos)
            .as_boxed());
    }

    *pos += 1;

    let literal = &chars[*start + 1..*pos - 1];
    let identifier = String::from_utf8(literal.iter().map(|&c| c as u8).collect()).unwrap();

    let location = Location {
        start: *start,
        end: *pos,
    };

    let string_literal = Token {
        location,
        kind: TokenKind::Symbol,
        literal: identifier,
    };

    Ok(string_literal)
}

fn consume_binary_number(
    chars: &Vec<char>,
    pos: &mut usize,
    start: &mut usize,
) -> Result<Token, Box<Diagnostic>> {
    let mut has_digit = false;
    while *pos < chars.len() && ((chars[*pos] == '0' || chars[*pos] == '1') || chars[*pos] == '_') {
        *pos += 1;
        has_digit = true;
    }

    if !has_digit {
        return Err(
            Diagnostic::error("Missing digits after the integer base prefix")
                .add_help("Expect at least one binary digits after the prefix 0b")
                .add_help("Binary digit mean 0 or 1")
                .with_location_span(*start, *pos)
                .as_boxed(),
        );
    }

    let literal = &chars[*start..*pos];
    let string = String::from_utf8(literal.iter().map(|&c| c as u8).collect()).unwrap();
    let literal_num = string.replace('_', "");
    let convert_result = i64::from_str_radix(&literal_num, 2);

    if convert_result.is_err() {
        return Err(Diagnostic::error("Invalid binary number")
            .with_location_span(*start, *pos)
            .as_boxed());
    }

    let location = Location {
        start: *start,
        end: *pos,
    };

    Ok(Token {
        location,
        kind: TokenKind::Integer,
        literal: convert_result.ok().unwrap().to_string(),
    })
}

fn consume_octal_number(
    chars: &Vec<char>,
    pos: &mut usize,
    start: &mut usize,
) -> Result<Token, Box<Diagnostic>> {
    let mut has_digit = false;
    while *pos < chars.len() && ((chars[*pos] >= '0' || chars[*pos] < '8') || chars[*pos] == '_') {
        *pos += 1;
        has_digit = true;
    }

    if !has_digit {
        return Err(
            Diagnostic::error("Missing digits after the integer base prefix")
                .add_help("Expect at least one octal digits after the prefix 0o")
                .add_help("Octal digit mean 0 to 8 number")
                .with_location_span(*start, *pos)
                .as_boxed(),
        );
    }

    let literal = &chars[*start..*pos];
    let string = String::from_utf8(literal.iter().map(|&c| c as u8).collect()).unwrap();
    let literal_num = string.replace('_', "");
    let convert_result = i64::from_str_radix(&literal_num, 8);

    if convert_result.is_err() {
        return Err(Diagnostic::error("Invalid octal number")
            .with_location_span(*start, *pos)
            .as_boxed());
    }

    let location = Location {
        start: *start,
        end: *pos,
    };

    Ok(Token {
        location,
        kind: TokenKind::Integer,
        literal: convert_result.ok().unwrap().to_string(),
    })
}

fn consume_hex_number(
    chars: &Vec<char>,
    pos: &mut usize,
    start: &mut usize,
) -> Result<Token, Box<Diagnostic>> {
    let mut has_digit = false;
    while *pos < chars.len() && (chars[*pos].is_ascii_hexdigit() || chars[*pos] == '_') {
        *pos += 1;
        has_digit = true;
    }

    if !has_digit {
        return Err(
            Diagnostic::error("Missing digits after the integer base prefix")
                .add_help("Expect at least one hex digits after the prefix 0x")
                .add_help("Hex digit mean 0 to 9 and a to f")
                .with_location_span(*start, *pos)
                .as_boxed(),
        );
    }

    let literal = &chars[*start..*pos];
    let string = String::from_utf8(literal.iter().map(|&c| c as u8).collect()).unwrap();
    let literal_num = string.replace('_', "");
    let convert_result = i64::from_str_radix(&literal_num, 16);

    if convert_result.is_err() {
        return Err(Diagnostic::error("Invalid hex decimal number")
            .with_location_span(*start, *pos)
            .as_boxed());
    }

    let location = Location {
        start: *start,
        end: *pos,
    };

    Ok(Token {
        location,
        kind: TokenKind::Integer,
        literal: convert_result.ok().unwrap().to_string(),
    })
}

fn consume_string(
    chars: &Vec<char>,
    pos: &mut usize,
    start: &mut usize,
) -> Result<Token, Box<Diagnostic>> {
    *pos += 1;

    while *pos < chars.len() && chars[*pos] != '"' {
        *pos += 1;
    }

    if *pos >= chars.len() {
        return Err(Diagnostic::error("Unterminated double quote string")
            .add_help("Add \" at the end of the String literal")
            .with_location_span(*start, *pos)
            .as_boxed());
    }

    *pos += 1;

    let literal = &chars[*start + 1..*pos - 1];
    let string = String::from_utf8(literal.iter().map(|&c| c as u8).collect()).unwrap();

    let location = Location {
        start: *start,
        end: *pos,
    };

    let string_literal = Token {
        location,
        kind: TokenKind::String,
        literal: string,
    };

    Ok(string_literal)
}

fn ignore_single_line_comment(chars: &Vec<char>, pos: &mut usize) {
    *pos += 2;

    while *pos < chars.len() && chars[*pos] != '\n' {
        *pos += 1;
    }

    *pos += 1;
}

fn ignore_c_style_comment(chars: &Vec<char>, pos: &mut usize) -> Result<(), Box<Diagnostic>> {
    *pos += 2;

    while *pos + 1 < chars.len() && (chars[*pos] != '*' && chars[*pos + 1] != '/') {
        *pos += 1;
    }

    if *pos + 2 > chars.len() {
        return Err(Diagnostic::error("C Style comment must end with */")
            .add_help("Add */ at the end of C Style comments")
            .with_location_span(*pos, *pos)
            .as_boxed());
    }

    *pos += 2;
    Ok(())
}

fn resolve_symbol_kind(literal: String) -> TokenKind {
    match literal.to_lowercase().as_str() {
        // Reserved keywords
        "set" => TokenKind::Set,
        "select" => TokenKind::Select,
        "distinct" => TokenKind::Distinct,
        "from" => TokenKind::From,
        "group" => TokenKind::Group,
        "where" => TokenKind::Where,
        "having" => TokenKind::Having,
        "limit" => TokenKind::Limit,
        "offset" => TokenKind::Offset,
        "order" => TokenKind::Order,
        "by" => TokenKind::By,
        "case" => TokenKind::Case,
        "when" => TokenKind::When,
        "then" => TokenKind::Then,
        "else" => TokenKind::Else,
        "end" => TokenKind::End,
        "between" => TokenKind::Between,
        "in" => TokenKind::In,
        "is" => TokenKind::Is,
        "not" => TokenKind::Not,
        "like" => TokenKind::Like,
        "glob" => TokenKind::Glob,

        // Logical Operators
        "or" => TokenKind::LogicalOr,
        "and" => TokenKind::LogicalAnd,
        "xor" => TokenKind::LogicalXor,

        // True, False and Null
        "true" => TokenKind::True,
        "false" => TokenKind::False,
        "null" => TokenKind::Null,

        "as" => TokenKind::As,

        // Order by DES and ASC
        "asc" => TokenKind::Ascending,
        "desc" => TokenKind::Descending,

        // Identifier
        _ => TokenKind::Symbol,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        // Symbol: NAME
        let script = "NAME".to_string();
        let tokens = tokenize(script);
        if tokens.is_ok() {
            assert_eq!(1, tokens.as_ref().ok().unwrap().len());
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.start);
            assert_eq!(4, tokens.as_ref().ok().unwrap()[0].location.end);
            assert_eq!("name", tokens.as_ref().ok().unwrap()[0].literal);
            if tokens.as_ref().ok().unwrap()[0].kind != TokenKind::Symbol {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // GlobalVariable: @NAME
        let script = "@NAME".to_string();
        let tokens = tokenize(script);
        if tokens.is_ok() {
            assert_eq!(1, tokens.as_ref().ok().unwrap().len());
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.start);
            assert_eq!(5, tokens.as_ref().ok().unwrap()[0].location.end);
            assert_eq!("@name", tokens.as_ref().ok().unwrap()[0].literal);
            if tokens.as_ref().ok().unwrap()[0].kind != TokenKind::GlobalVariable {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // Integer: 0x01
        let script = "0x01".to_string();
        let tokens = tokenize(script);
        if tokens.is_ok() {
            assert_eq!(1, tokens.as_ref().ok().unwrap().len());
            assert_eq!(2, tokens.as_ref().ok().unwrap()[0].location.start);
            assert_eq!(4, tokens.as_ref().ok().unwrap()[0].location.end);
            assert_eq!("1", tokens.as_ref().ok().unwrap()[0].literal);
            if tokens.as_ref().ok().unwrap()[0].kind != TokenKind::Integer {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // Integer: 0b01
        let script = "0b01".to_string();
        let tokens = tokenize(script);
        if tokens.is_ok() {
            assert_eq!(1, tokens.as_ref().ok().unwrap().len());
            assert_eq!(2, tokens.as_ref().ok().unwrap()[0].location.start);
            assert_eq!(4, tokens.as_ref().ok().unwrap()[0].location.end);
            assert_eq!("1", tokens.as_ref().ok().unwrap()[0].literal);
            if tokens.as_ref().ok().unwrap()[0].kind != TokenKind::Integer {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // Integer: 0o01
        let script = "0o01".to_string();
        let tokens = tokenize(script);
        if tokens.is_ok() {
            assert_eq!(1, tokens.as_ref().ok().unwrap().len());
            assert_eq!(2, tokens.as_ref().ok().unwrap()[0].location.start);
            assert_eq!(4, tokens.as_ref().ok().unwrap()[0].location.end);
            assert_eq!("1", tokens.as_ref().ok().unwrap()[0].literal);
            if tokens.as_ref().ok().unwrap()[0].kind != TokenKind::Integer {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // Integer: 1
        let script = "1".to_string();
        let tokens = tokenize(script);
        if tokens.is_ok() {
            assert_eq!(1, tokens.as_ref().ok().unwrap().len());
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.start);
            assert_eq!(1, tokens.as_ref().ok().unwrap()[0].location.end);
            assert_eq!("1", tokens.as_ref().ok().unwrap()[0].literal);
            if tokens.as_ref().ok().unwrap()[0].kind != TokenKind::Integer {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // Float: 0.1
        let script = "0.1".to_string();
        let tokens = tokenize(script);
        if tokens.is_ok() {
            assert_eq!(1, tokens.as_ref().ok().unwrap().len());
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.start);
            assert_eq!(3, tokens.as_ref().ok().unwrap()[0].location.end);
            assert_eq!("0.1", tokens.as_ref().ok().unwrap()[0].literal);
            if tokens.as_ref().ok().unwrap()[0].kind != TokenKind::Float {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // String: "name"
        let script = "\"name\"".to_string();
        let tokens = tokenize(script);
        if tokens.is_ok() {
            assert_eq!(1, tokens.as_ref().ok().unwrap().len());
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.start);
            assert_eq!(6, tokens.as_ref().ok().unwrap()[0].location.end);
            assert_eq!("name", tokens.as_ref().ok().unwrap()[0].literal);
            if tokens.as_ref().ok().unwrap()[0].kind != TokenKind::String {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // Symbol: `name`
        let script = "`name`".to_string();
        let tokens = tokenize(script);
        if tokens.is_ok() {
            assert_eq!(1, tokens.as_ref().ok().unwrap().len());
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.start);
            assert_eq!(6, tokens.as_ref().ok().unwrap()[0].location.end);
            assert_eq!("name", tokens.as_ref().ok().unwrap()[0].literal);
            if tokens.as_ref().ok().unwrap()[0].kind != TokenKind::Symbol {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // Plus: +
        let script = "+".to_string();
        let tokens = tokenize(script);
        if tokens.is_ok() {
            assert_eq!(1, tokens.as_ref().ok().unwrap().len());
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.start);
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.end);
            assert_eq!("+", tokens.as_ref().ok().unwrap()[0].literal);
            if tokens.as_ref().ok().unwrap()[0].kind != TokenKind::Plus {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // Minus: -
        let script = "-".to_string();
        let tokens = tokenize(script);
        if tokens.is_ok() {
            assert_eq!(1, tokens.as_ref().ok().unwrap().len());
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.start);
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.end);
            assert_eq!("-", tokens.as_ref().ok().unwrap()[0].literal);
            if tokens.as_ref().ok().unwrap()[0].kind != TokenKind::Minus {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // Star: *
        let script = "*".to_string();
        let tokens = tokenize(script);
        if tokens.is_ok() {
            assert_eq!(1, tokens.as_ref().ok().unwrap().len());
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.start);
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.end);
            assert_eq!("*", tokens.as_ref().ok().unwrap()[0].literal);
            if tokens.as_ref().ok().unwrap()[0].kind != TokenKind::Star {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // Slash: /
        let script = "/".to_string();
        let tokens = tokenize(script);
        if tokens.is_ok() {
            assert_eq!(1, tokens.as_ref().ok().unwrap().len());
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.start);
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.end);
            assert_eq!("/", tokens.as_ref().ok().unwrap()[0].literal);
            if tokens.as_ref().ok().unwrap()[0].kind != TokenKind::Slash {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // Percentage: %
        let script = "%".to_string();
        let tokens = tokenize(script);
        if tokens.is_ok() {
            assert_eq!(1, tokens.as_ref().ok().unwrap().len());
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.start);
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.end);
            assert_eq!("%", tokens.as_ref().ok().unwrap()[0].literal);
            if tokens.as_ref().ok().unwrap()[0].kind != TokenKind::Percentage {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // BitwiseOr: |
        let script = "|".to_string();
        let tokens = tokenize(script);
        if tokens.is_ok() {
            assert_eq!(1, tokens.as_ref().ok().unwrap().len());
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.start);
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.end);
            assert_eq!("|", tokens.as_ref().ok().unwrap()[0].literal);
            if tokens.as_ref().ok().unwrap()[0].kind != TokenKind::BitwiseOr {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // LogicalOr: ||
        let script = "||".to_string();
        let tokens = tokenize(script);
        if tokens.is_ok() {
            assert_eq!(1, tokens.as_ref().ok().unwrap().len());
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.start);
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.end);
            assert_eq!("||", tokens.as_ref().ok().unwrap()[0].literal);
            if tokens.as_ref().ok().unwrap()[0].kind != TokenKind::LogicalOr {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // BitwiseAnd: &
        let script = "&".to_string();
        let tokens = tokenize(script);
        if tokens.is_ok() {
            assert_eq!(1, tokens.as_ref().ok().unwrap().len());
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.start);
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.end);
            assert_eq!("&", tokens.as_ref().ok().unwrap()[0].literal);
            if tokens.as_ref().ok().unwrap()[0].kind != TokenKind::BitwiseAnd {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // LogicalAnd: &&
        let script = "&&".to_string();
        let tokens = tokenize(script);
        if tokens.is_ok() {
            assert_eq!(1, tokens.as_ref().ok().unwrap().len());
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.start);
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.end);
            assert_eq!("&&", tokens.as_ref().ok().unwrap()[0].literal);
            if tokens.as_ref().ok().unwrap()[0].kind != TokenKind::LogicalAnd {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // LogicalXor: ^
        let script = "^".to_string();
        let tokens = tokenize(script);
        if tokens.is_ok() {
            assert_eq!(1, tokens.as_ref().ok().unwrap().len());
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.start);
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.end);
            assert_eq!("^", tokens.as_ref().ok().unwrap()[0].literal);
            if tokens.as_ref().ok().unwrap()[0].kind != TokenKind::LogicalXor {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // Comma: ,
        let script = ",".to_string();
        let tokens = tokenize(script);
        if tokens.is_ok() {
            assert_eq!(1, tokens.as_ref().ok().unwrap().len());
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.start);
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.end);
            assert_eq!(",", tokens.as_ref().ok().unwrap()[0].literal);
            if tokens.as_ref().ok().unwrap()[0].kind != TokenKind::Comma {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // Dot: .
        let script = ".".to_string();
        let tokens = tokenize(script);
        if tokens.is_ok() {
            assert_eq!(1, tokens.as_ref().ok().unwrap().len());
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.start);
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.end);
            assert_eq!(".", tokens.as_ref().ok().unwrap()[0].literal);
            if tokens.as_ref().ok().unwrap()[0].kind != TokenKind::Dot {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // DotDot: ..
        let script = "..".to_string();
        let tokens = tokenize(script);
        if tokens.is_ok() {
            assert_eq!(1, tokens.as_ref().ok().unwrap().len());
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.start);
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.end);
            assert_eq!("..", tokens.as_ref().ok().unwrap()[0].literal);
            if tokens.as_ref().ok().unwrap()[0].kind != TokenKind::DotDot {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // Greater: >
        let script = ">".to_string();
        let tokens = tokenize(script);
        if tokens.is_ok() {
            assert_eq!(1, tokens.as_ref().ok().unwrap().len());
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.start);
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.end);
            assert_eq!(">", tokens.as_ref().ok().unwrap()[0].literal);
            if tokens.as_ref().ok().unwrap()[0].kind != TokenKind::Greater {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // GreaterEqual: >=
        let script = ">=".to_string();
        let tokens = tokenize(script);
        if tokens.is_ok() {
            assert_eq!(1, tokens.as_ref().ok().unwrap().len());
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.start);
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.end);
            assert_eq!(">=", tokens.as_ref().ok().unwrap()[0].literal);
            if tokens.as_ref().ok().unwrap()[0].kind != TokenKind::GreaterEqual {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // BitwiseRightShift: >>
        let script = ">>".to_string();
        let tokens = tokenize(script);
        if tokens.is_ok() {
            assert_eq!(1, tokens.as_ref().ok().unwrap().len());
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.start);
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.end);
            assert_eq!(">>", tokens.as_ref().ok().unwrap()[0].literal);
            if tokens.as_ref().ok().unwrap()[0].kind != TokenKind::BitwiseRightShift {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // Less: <
        let script = "<".to_string();
        let tokens = tokenize(script);
        if tokens.is_ok() {
            assert_eq!(1, tokens.as_ref().ok().unwrap().len());
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.start);
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.end);
            assert_eq!("<", tokens.as_ref().ok().unwrap()[0].literal);
            if tokens.as_ref().ok().unwrap()[0].kind != TokenKind::Less {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // NulllSafeEqual: <=>
        let script = "<=>".to_string();
        let tokens = tokenize(script);
        if tokens.is_ok() {
            assert_eq!(1, tokens.as_ref().ok().unwrap().len());
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.start);
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.end);
            assert_eq!("<=>", tokens.as_ref().ok().unwrap()[0].literal);
            if tokens.as_ref().ok().unwrap()[0].kind != TokenKind::NullSafeEqual {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // LessEqual: <=
        let script = "<=".to_string();
        let tokens = tokenize(script);
        if tokens.is_ok() {
            assert_eq!(1, tokens.as_ref().ok().unwrap().len());
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.start);
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.end);
            assert_eq!("<=", tokens.as_ref().ok().unwrap()[0].literal);
            if tokens.as_ref().ok().unwrap()[0].kind != TokenKind::LessEqual {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // BitwiseLeftShift: <<
        let script = "<<".to_string();
        let tokens = tokenize(script);
        if tokens.is_ok() {
            assert_eq!(1, tokens.as_ref().ok().unwrap().len());
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.start);
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.end);
            assert_eq!("<<", tokens.as_ref().ok().unwrap()[0].literal);
            if tokens.as_ref().ok().unwrap()[0].kind != TokenKind::BitwiseLeftShift {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // BangEqual: <>
        let script = "<>".to_string();
        let tokens = tokenize(script);
        if tokens.is_ok() {
            assert_eq!(1, tokens.as_ref().ok().unwrap().len());
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.start);
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.end);
            assert_eq!("<>", tokens.as_ref().ok().unwrap()[0].literal);
            if tokens.as_ref().ok().unwrap()[0].kind != TokenKind::BangEqual {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // Equal: =
        let script = "=".to_string();
        let tokens = tokenize(script);
        if tokens.is_ok() {
            assert_eq!(1, tokens.as_ref().ok().unwrap().len());
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.start);
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.end);
            assert_eq!("=", tokens.as_ref().ok().unwrap()[0].literal);
            if tokens.as_ref().ok().unwrap()[0].kind != TokenKind::Equal {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // ColonEqual: :
        let script = ":".to_string();
        let tokens = tokenize(script);
        if tokens.is_err() {
            assert_eq!("Expect `=` after `:`", tokens.err().unwrap().message());
        } else {
            assert!(false);
        }

        // ColonEqual: :=
        let script = ":=".to_string();
        let tokens = tokenize(script);
        if tokens.is_ok() {
            assert_eq!(1, tokens.as_ref().ok().unwrap().len());
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.start);
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.end);
            assert_eq!(":=", tokens.as_ref().ok().unwrap()[0].literal);
            if tokens.as_ref().ok().unwrap()[0].kind != TokenKind::ColonEqual {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // BangEqual: !
        let script = "!".to_string();
        let tokens = tokenize(script);
        if tokens.is_ok() {
            assert_eq!(1, tokens.as_ref().ok().unwrap().len());
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.start);
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.end);
            assert_eq!("!", tokens.as_ref().ok().unwrap()[0].literal);
            if tokens.as_ref().ok().unwrap()[0].kind != TokenKind::Bang {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // BangEqual: !=
        let script = "!=".to_string();
        let tokens = tokenize(script);
        if tokens.is_ok() {
            assert_eq!(1, tokens.as_ref().ok().unwrap().len());
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.start);
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.end);
            assert_eq!("!=", tokens.as_ref().ok().unwrap()[0].literal);
            if tokens.as_ref().ok().unwrap()[0].kind != TokenKind::BangEqual {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // LeftParen: (
        let script = "(".to_string();
        let tokens = tokenize(script);
        if tokens.is_ok() {
            assert_eq!(1, tokens.as_ref().ok().unwrap().len());
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.start);
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.end);
            assert_eq!("(", tokens.as_ref().ok().unwrap()[0].literal);
            if tokens.as_ref().ok().unwrap()[0].kind != TokenKind::LeftParen {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // RightParen: (
        let script = ")".to_string();
        let tokens = tokenize(script);
        if tokens.is_ok() {
            assert_eq!(1, tokens.as_ref().ok().unwrap().len());
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.start);
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.end);
            assert_eq!(")", tokens.as_ref().ok().unwrap()[0].literal);
            if tokens.as_ref().ok().unwrap()[0].kind != TokenKind::RightParen {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // Semicolon: ;
        let script = ";".to_string();
        let tokens = tokenize(script);
        if tokens.is_ok() {
            assert_eq!(1, tokens.as_ref().ok().unwrap().len());
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.start);
            assert_eq!(0, tokens.as_ref().ok().unwrap()[0].location.end);
            assert_eq!(";", tokens.as_ref().ok().unwrap()[0].literal);
            if tokens.as_ref().ok().unwrap()[0].kind != TokenKind::Semicolon {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // Invalid: ?
        let script = "?".to_string();
        let tokens = tokenize(script);
        if tokens.is_err() {
            assert_eq!("Unexpected character", tokens.err().unwrap().message());
        } else {
            assert!(false);
        }
    }

    fn test_consume_global_variable_name() {
    }

    fn test_consume_identifier() {
    }

    fn test_consume_number() {
    }

    fn test_consume_backticks_identifier() {
    }

    fn test_consume_binary_number() {
    }

    fn test_consume_octal_number() {
    }

    fn test_consume_hex_number() {
    }

    fn test_consume_string() {
    }

    fn test_ignore_single_line_comment() {
    }

    fn test_ignore_c_style_comment() {
    }

    fn test_resolve_symbol_kind() {
    }
}
