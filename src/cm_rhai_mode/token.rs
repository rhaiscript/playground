use crate::codemirror;
use std::str::FromStr;

type INT = i64;
type FLOAT = f64;
type LERR = LexError;

// Source: rhai//token.rs:132
/// Tokens.
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    IntegerConstant(/*INT*/),
    // #[cfg(not(feature = "no_float"))]
    FloatConstant(/*FLOAT*/),
    Identifier(/*String*/),
    CharConstant(/*char*/),
    StringConst(/*String*/),
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Plus,
    UnaryPlus,
    Minus,
    UnaryMinus,
    Multiply,
    Divide,
    Modulo,
    PowerOf,
    LeftShift,
    RightShift,
    SemiColon,
    Colon,
    DoubleColon,
    Comma,
    Period,
    MapStart,
    Equals,
    True,
    False,
    Let,
    Const,
    If,
    Else,
    While,
    Loop,
    For,
    In,
    LessThan,
    GreaterThan,
    LessThanEqualsTo,
    GreaterThanEqualsTo,
    EqualsTo,
    NotEqualsTo,
    Bang,
    Pipe,
    Or,
    XOr,
    Ampersand,
    And,
    // #[cfg(not(feature = "no_function"))]
    Fn,
    Continue,
    Break,
    Return,
    Throw,
    PlusAssign,
    MinusAssign,
    MultiplyAssign,
    DivideAssign,
    LeftShiftAssign,
    RightShiftAssign,
    AndAssign,
    OrAssign,
    XOrAssign,
    ModuloAssign,
    PowerOfAssign,
    // #[cfg(not(feature = "no_function"))]
    Private,
    // #[cfg(not(feature = "no_module"))]
    Import,
    // #[cfg(not(feature = "no_module"))]
    Export,
    // #[cfg(not(feature = "no_module"))]
    As,
    LineComment,
    BlockComment,
    LexError(Box<LexError>),
    EOF,
}

impl Token {
    // Source: rhai//token.rs:315
    // If another operator is after these, it's probably an unary operator
    // (not sure about fn name).
    pub fn is_next_unary(&self) -> bool {
        use Token::*;

        match self {
            LexError(_)      |
            LeftBrace        | // (+expr) - is unary
            // RightBrace    | {expr} - expr not unary & is closing
            LeftParen        | // {-expr} - is unary
            // RightParen    | (expr) - expr not unary & is closing
            LeftBracket      | // [-expr] - is unary
            // RightBracket  | [expr] - expr not unary & is closing
            Plus             |
            UnaryPlus        |
            Minus            |
            UnaryMinus       |
            Multiply         |
            Divide           |
            Colon            |
            Comma            |
            Period           |
            Equals           |
            LessThan         |
            GreaterThan      |
            Bang             |
            LessThanEqualsTo |
            GreaterThanEqualsTo |
            EqualsTo         |
            NotEqualsTo      |
            Pipe             |
            Or               |
            Ampersand        |
            And              |
            If               |
            While            |
            PlusAssign       |
            MinusAssign      |
            MultiplyAssign   |
            DivideAssign     |
            LeftShiftAssign  |
            RightShiftAssign |
            AndAssign        |
            OrAssign         |
            XOrAssign        |
            LeftShift        |
            RightShift       |
            XOr              |
            Modulo           |
            ModuloAssign     |
            Return           |
            Throw            |
            PowerOf          |
            In               |
            PowerOfAssign => true,

            LineComment => true,
            BlockComment => true,

            _ => false,
        }
    }
}

// Source: rhai//error.rs:8
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
#[non_exhaustive]
pub enum LexError {
    /// An unexpected character is encountered when tokenizing the script text.
    UnexpectedChar(char),
    /// A string literal is not terminated before a new-line or EOF.
    UnterminatedString,
    /// An identifier is in an invalid format.
    StringTooLong(usize),
    /// An string/character/numeric escape sequence is in an invalid format.
    MalformedEscapeSequence(String),
    /// An numeric literal is in an invalid format.
    MalformedNumber(String),
    /// An character literal is in an invalid format.
    MalformedChar(/*String*/),
    /// An identifier is in an invalid format.
    MalformedIdentifier(String),
    /// Bad symbol encountered when tokenizing the script text.
    ImproperSymbol(String),
}

// Source: rhai//error.rs:32
impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedChar(c) => write!(f, "Unexpected '{}'", c),
            Self::MalformedEscapeSequence(s) => write!(f, "Invalid escape sequence: '{}'", s),
            Self::MalformedNumber(s) => write!(f, "Invalid number: '{}'", s),
            Self::MalformedChar(/*s*/) => write!(f, "Invalid character: '{}'", '?'/*s*/),
            Self::MalformedIdentifier(s) => write!(f, "Variable name is not proper: '{}'", s),
            Self::UnterminatedString => write!(f, "Open string is not terminated"),
            Self::StringTooLong(max) => write!(
                f,
                "Length of string literal exceeds the maximum limit ({})",
                max
            ),
            Self::ImproperSymbol(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Clone, Debug)]
pub struct State {
    can_be_unary: bool,
    in_between_token: InBetweenToken,
}

impl State {
    pub fn new() -> Self {
        Self {
            can_be_unary: true,
            in_between_token: InBetweenToken::Nothing,
        }
    }
}

#[derive(Clone, Debug)]
enum InBetweenToken {
    Nothing,
    BlockComment(u32),
    // StringLiteral,
}

// Source: rhai//token.rs:497
fn parse_string_literal(
    stream: &codemirror::StringStream,
    enclosing_char: char,
    // max_length: usize,
) -> Result<usize, (LexError, ())> {
    // let mut result = Vec::new();
    let mut chars_count = 0;
    let mut escape = String::with_capacity(12);

    loop {
        let next_char = stream.next().ok_or((LERR::UnterminatedString, ()))?;

        // self.advance();

        // if max_length > 0 && result.len() > max_length {
        //     return Err((LexError::StringTooLong(max_length), self.pos));
        // }

        match next_char {
            // \...
            '\\' if escape.is_empty() => {
                escape.push('\\');
            }
            // \\
            '\\' if !escape.is_empty() => {
                escape.clear();
                // result.push('\\');
                chars_count += 1;
            }
            // \t
            't' if !escape.is_empty() => {
                escape.clear();
                // result.push('\t');
                chars_count += 1;
            }
            // \n
            'n' if !escape.is_empty() => {
                escape.clear();
                // result.push('\n');
                chars_count += 1;
            }
            // \r
            'r' if !escape.is_empty() => {
                escape.clear();
                // result.push('\r');
                chars_count += 1;
            }
            // \x??, \u????, \U????????
            ch @ 'x' | ch @ 'u' | ch @ 'U' if !escape.is_empty() => {
                let mut seq = escape.clone();
                seq.push(ch);
                escape.clear();

                let mut out_val: u32 = 0;
                let len = match ch {
                    'x' => 2,
                    'u' => 4,
                    'U' => 8,
                    _ => unreachable!(),
                };

                for _ in 0..len {
                    let c = stream
                        .next()
                        .ok_or_else(|| (LERR::MalformedEscapeSequence(seq.to_string()), ()))?;

                    seq.push(c);
                    // self.advance();

                    out_val *= 16;
                    out_val += c
                        .to_digit(16)
                        .ok_or_else(|| (LERR::MalformedEscapeSequence(seq.to_string()), ()))?;
                }

                // result.push(
                //     char::from_u32(out_val)
                //         .ok_or_else(|| (LERR::MalformedEscapeSequence(seq), self.pos))?,
                // );
                chars_count += 1;
            }

            // \{enclosing_char} - escaped
            ch if enclosing_char == ch && !escape.is_empty() => {
                escape.clear();
                // result.push(ch)
                chars_count += 1;
            }

            // Close wrapper
            ch if enclosing_char == ch && escape.is_empty() => break,

            // Unknown escape sequence
            _ if !escape.is_empty() => return Err((LERR::MalformedEscapeSequence(escape), ())),

            // // Cannot have new-lines inside string literals
            // '\n' => {
            //     self.rewind();
            //     return Err((LERR::UnterminatedString, (self.pos)));
            // }

            // All other characters
            ch => {
                escape.clear();
                // result.push(ch);
                chars_count += 1;
            }
        }
    }

    // let s = result.iter().collect::<String>();

    // if max_length > 0 && s.len() > max_length {
    //     return Err((LexError::StringTooLong(max_length), self.pos));
    // }

    // Ok(s)
    Ok(chars_count)
}

// Source: rhai//token.rs:615
fn next_token_inner(state: &mut State, stream: &codemirror::StringStream) -> Option<(Token, ())> {
    match state.in_between_token {
        InBetweenToken::BlockComment(mut level) => {
            // Copied from later
            while let Some(c) = stream.next() {
                // self.advance();

                match c {
                    '/' => {
                        if stream.next() == Some('*') {
                            level += 1;
                        }
                        // self.advance();
                    }
                    '*' => {
                        if stream.next() == Some('/') {
                            level -= 1;
                        }
                        // self.advance();
                    }
                    // '\n' => self.new_line(),
                    _ => (),
                }

                if level == 0 {
                    break;
                }
            }
            if level != 0 {
                state.in_between_token = InBetweenToken::BlockComment(level);
            } else {
                state.in_between_token = InBetweenToken::Nothing;
            }
            return Some((Token::BlockComment, ()));
        }
        InBetweenToken::Nothing => {}
    }

    let mut negated = false;
    let pos = ();

    while let Some(c) = stream.next() {
        // self.advance();

        // let pos = self.pos;

        // match (c, stream.peek().unwrap_or('\0')) {
        match (c, stream.peek().unwrap_or_else(||stream.look_ahead(1).map(|_|'\n').unwrap_or('\0'))) {
            // \n
            ('\n', _) => /*self.new_line()*/ (),

            // digit ...
            ('0'..='9', _) => {
                let mut result = Vec::new();
                let mut radix_base: Option<u32> = None;
                result.push(c);

                while let Some(next_char) = stream.peek() {
                    match next_char {
                        '0'..='9' | '_' => {
                            result.push(next_char);
                            // self.eat_next();
                            let _ = stream.next();
                        }
                        // #[cfg(not(feature = "no_float"))]
                        '.' => {
                            result.push(next_char);
                            // self.eat_next();
                            let _ = stream.next();
                            while let Some(next_char_in_float) = stream.peek() {
                                match next_char_in_float {
                                    '0'..='9' | '_' => {
                                        result.push(next_char_in_float);
                                        // self.eat_next();
                                        let _ = stream.next();
                                    }
                                    _ => break,
                                }
                            }
                        }
                        // 0x????, 0o????, 0b????
                        ch @ 'x' | ch @ 'X' | ch @ 'o' | ch @ 'O' | ch @ 'b' | ch @ 'B'
                            if c == '0' =>
                        {
                            result.push(next_char);
                            // self.eat_next();
                            let _ = stream.next();

                            let valid = match ch {
                                'x' | 'X' => [
                                    'a', 'b', 'c', 'd', 'e', 'f', 'A', 'B', 'C', 'D', 'E', 'F',
                                    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '_',
                                ],
                                'o' | 'O' => [
                                    '0', '1', '2', '3', '4', '5', '6', '7', '_', '_', '_', '_',
                                    '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_',
                                ],
                                'b' | 'B' => [
                                    '0', '1', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_',
                                    '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_',
                                ],
                                _ => unreachable!(),
                            };

                            radix_base = Some(match ch {
                                'x' | 'X' => 16,
                                'o' | 'O' => 8,
                                'b' | 'B' => 2,
                                _ => unreachable!(),
                            });

                            while let Some(next_char_in_escape_seq) = stream.peek() {
                                if !valid.contains(&next_char_in_escape_seq) {
                                    break;
                                }

                                result.push(next_char_in_escape_seq);
                                // self.eat_next();
                                let _ = stream.next();
                            }
                        }

                        _ => break,
                    }
                }

                if negated {
                    result.insert(0, '-');
                }

                // Parse number
                if let Some(radix) = radix_base {
                    let out: String = result.iter().skip(2).filter(|&&c| c != '_').collect();

                    return Some((
                        INT::from_str_radix(&out, radix)
                            .map(|_|Token::IntegerConstant())
                            .unwrap_or_else(|_| {
                                Token::LexError(Box::new(LERR::MalformedNumber(
                                    result.into_iter().collect(),
                                )))
                            }),
                        pos,
                    ));
                } else {
                    let out: String = result.iter().filter(|&&c| c != '_').collect();
                    let num = INT::from_str(&out).map(|_|Token::IntegerConstant());

                    // If integer parsing is unnecessary, try float instead
                    #[cfg(not(feature = "no_float"))]
                    let num = num.or_else(|_| FLOAT::from_str(&out).map(|_|Token::FloatConstant()));

                    return Some((
                        num.unwrap_or_else(|_| {
                            Token::LexError(Box::new(LERR::MalformedNumber(
                                result.into_iter().collect(),
                            )))
                        }),
                        pos,
                    ));
                }
            }

            // letter or underscore ...
            ('A'..='Z', _) | ('a'..='z', _) | ('_', _) => {
                let mut result = Vec::new();
                result.push(c);

                while let Some(next_char) = stream.peek() {
                    match next_char {
                        x if x.is_ascii_alphanumeric() || x == '_' => {
                            result.push(x);
                            // self.eat_next();
                            let _ = stream.next();
                        }
                        _ => break,
                    }
                }

                let is_valid_identifier = result
                    .iter()
                    .find(|&ch| char::is_ascii_alphanumeric(ch)) // first alpha-numeric character
                    .map(char::is_ascii_alphabetic) // is a letter
                    .unwrap_or(false); // if no alpha-numeric at all - syntax error

                let identifier: String = result.iter().collect();

                if !is_valid_identifier {
                    return Some((
                        Token::LexError(Box::new(LERR::MalformedIdentifier(identifier))),
                        pos,
                    ));
                }

                return Some((
                    match identifier.as_str() {
                        "true" => Token::True,
                        "false" => Token::False,
                        "let" => Token::Let,
                        "const" => Token::Const,
                        "if" => Token::If,
                        "else" => Token::Else,
                        "while" => Token::While,
                        "loop" => Token::Loop,
                        "continue" => Token::Continue,
                        "break" => Token::Break,
                        "return" => Token::Return,
                        "throw" => Token::Throw,
                        "for" => Token::For,
                        "in" => Token::In,
                        // #[cfg(not(feature = "no_function"))]
                        "private" => Token::Private,
                        // #[cfg(not(feature = "no_module"))]
                        "import" => Token::Import,
                        // #[cfg(not(feature = "no_module"))]
                        "export" => Token::Export,
                        // #[cfg(not(feature = "no_module"))]
                        "as" => Token::As,

                        // #[cfg(not(feature = "no_function"))]
                        "fn" => Token::Fn,

                        _ => Token::Identifier(/*identifier*/),
                    },
                    pos,
                ));
            }

            // " - string literal
            ('"', _) => {
                return parse_string_literal(stream, '"')
                    .map_or_else(
                        |err| Some((Token::LexError(Box::new(err.0)), err.1)),
                        |out| Some((Token::StringConst(/*out*/), pos)),
                    );
            }

            // ' - character literal
            ('\'', '\'') => {
                return Some((
                    Token::LexError(Box::new(LERR::MalformedChar())),
                    pos,
                ));
            }
            ('\'', _) => {
                return Some(
                    parse_string_literal(stream, '\'')
                        .map_or_else(
                            |err| (Token::LexError(Box::new(err.0)), err.1),
                            |result| {
                                // let mut chars = result.chars();
                                // let first = chars.next();

                                // if chars.next().is_some() {
                                //     (
                                //         Token::LexError(Box::new(LERR::MalformedChar(result))),
                                //         pos,
                                //     )
                                // } else {
                                //     (Token::CharConstant(first.expect("should be Some")), pos)
                                // }
                                if result > 1 {
                                    (
                                        Token::LexError(Box::new(LERR::MalformedChar())),
                                        pos,
                                    )
                                } else {
                                    assert_eq!(result, 1);
                                    (Token::CharConstant(), pos)
                                }
                            },
                        ),
                );
            }

            // Braces
            ('{', _) => return Some((Token::LeftBrace, pos)),
            ('}', _) => return Some((Token::RightBrace, pos)),

            // Parentheses
            ('(', _) => return Some((Token::LeftParen, pos)),
            (')', _) => return Some((Token::RightParen, pos)),

            // Indexing
            ('[', _) => return Some((Token::LeftBracket, pos)),
            (']', _) => return Some((Token::RightBracket, pos)),

            // Map literal
            #[cfg(not(feature = "no_object"))]
            ('#', '{') => {
                // self.eat_next();
                let _ = stream.next();
                return Some((Token::MapStart, pos));
            }

            // Operators
            ('+', '=') => {
                // self.eat_next();
                let _ = stream.next();
                return Some((Token::PlusAssign, pos));
            }
            ('+', _) if state.can_be_unary => return Some((Token::UnaryPlus, pos)),
            ('+', _) => return Some((Token::Plus, pos)),

            ('-', '0'..='9') if state.can_be_unary => negated = true,
            ('-', '0'..='9') => return Some((Token::Minus, pos)),
            ('-', '=') => {
                // self.eat_next();
                let _ = stream.next();
                return Some((Token::MinusAssign, pos));
            }
            ('-', '>') => {
                return Some((
                    Token::LexError(Box::new(LERR::ImproperSymbol(
                        "'->' is not a valid symbol. This is not C or C++!".to_string(),
                    ))),
                    pos,
                ))
            }
            ('-', _) if state.can_be_unary => return Some((Token::UnaryMinus, pos)),
            ('-', _) => return Some((Token::Minus, pos)),

            ('*', '=') => {
                // self.eat_next();
                let _ = stream.next();
                return Some((Token::MultiplyAssign, pos));
            }
            ('*', _) => return Some((Token::Multiply, pos)),

            // Comments
            ('/', '/') => {
                // self.eat_next();
                // let _ = stream.next();

                // while let Some(c) = stream.next() {
                //     // if c == '\n' {
                //     //     self.new_line();
                //     //     break;
                //     // }

                //     // self.advance();
                // }
                stream.skip_to_end();
                return Some((Token::LineComment, ()))
            }
            ('/', '*') => {
                let mut level = 1;

                // self.eat_next();
                let _ = stream.next();

                while let Some(c) = stream.next() {
                    // self.advance();

                    match c {
                        '/' => {
                            if stream.next() == Some('*') {
                                level += 1;
                            }
                            // self.advance();
                        }
                        '*' => {
                            if stream.next() == Some('/') {
                                level -= 1;
                            }
                            // self.advance();
                        }
                        // '\n' => self.new_line(),
                        _ => (),
                    }

                    if level == 0 {
                        break;
                    }
                }
                if level != 0 {
                    state.in_between_token = InBetweenToken::BlockComment(level);
                }
                return Some((Token::BlockComment, ()))
            }

            ('/', '=') => {
                // self.eat_next();
                let _ = stream.next();
                return Some((Token::DivideAssign, pos));
            }
            ('/', _) => return Some((Token::Divide, pos)),

            (';', _) => return Some((Token::SemiColon, pos)),
            (',', _) => return Some((Token::Comma, pos)),
            ('.', _) => return Some((Token::Period, pos)),

            ('=', '=') => {
                // self.eat_next();
                let _ = stream.next();

                // Warn against `===`
                if stream.peek() == Some('=') {
                    return Some((
                            Token::LexError(Box::new(LERR::ImproperSymbol(
                                "'===' is not a valid operator. This is not JavaScript! Should it be '=='?"
                                    .to_string(),
                            ))),
                            pos,
                        ));
                }

                return Some((Token::EqualsTo, pos));
            }
            ('=', '>') => {
                return Some((
                    Token::LexError(Box::new(LERR::ImproperSymbol(
                        "'=>' is not a valid symbol. This is not Rust! Should it be '>='?"
                            .to_string(),
                    ))),
                    pos,
                ))
            }
            ('=', _) => return Some((Token::Equals, pos)),

            (':', ':') => {
                // self.eat_next();
                let _ = stream.next();
                return Some((Token::DoubleColon, pos));
            }
            (':', '=') => {
                return Some((
                    Token::LexError(Box::new(LERR::ImproperSymbol(
                        "':=' is not a valid assignment operator. This is not Pascal! Should it be simply '='?"
                            .to_string(),
                    ))),
                    pos,
                ))
            }
            (':', _) => return Some((Token::Colon, pos)),

            ('<', '=') => {
                // self.eat_next();
                let _ = stream.next();
                return Some((Token::LessThanEqualsTo, pos));
            }
            ('<', '-') => {
                return Some((
                    Token::LexError(Box::new(LERR::ImproperSymbol(
                        "'<-' is not a valid symbol. Should it be '<='?".to_string(),
                    ))),
                    pos,
                ))
            }
            ('<', '<') => {
                // self.eat_next();
                let _ = stream.next();

                return Some((
                    if stream.peek() == Some('=') {
                        // self.eat_next();
                        let _ = stream.next();
                        Token::LeftShiftAssign
                    } else {
                        Token::LeftShift
                    },
                    pos,
                ));
            }
            ('<', _) => return Some((Token::LessThan, pos)),

            ('>', '=') => {
                // self.eat_next();
                let _ = stream.next();
                return Some((Token::GreaterThanEqualsTo, pos));
            }
            ('>', '>') => {
                // self.eat_next();
                let _ = stream.next();

                return Some((
                    if stream.peek() == Some('=') {
                        // self.eat_next();
                        let _ = stream.next();
                        Token::RightShiftAssign
                    } else {
                        Token::RightShift
                    },
                    pos,
                ));
            }
            ('>', _) => return Some((Token::GreaterThan, pos)),

            ('!', '=') => {
                // self.eat_next();
                let _ = stream.next();

                // Warn against `!==`
                if stream.peek() == Some('=') {
                    return Some((
                            Token::LexError(Box::new(LERR::ImproperSymbol(
                                "'!==' is not a valid operator. This is not JavaScript! Should it be '!='?"
                                    .to_string(),
                            ))),
                            pos,
                        ));
                }

                return Some((Token::NotEqualsTo, pos));
            }
            ('!', _) => return Some((Token::Bang, pos)),

            ('|', '|') => {
                // self.eat_next();
                let _ = stream.next();
                return Some((Token::Or, pos));
            }
            ('|', '=') => {
                // self.eat_next();
                let _ = stream.next();
                return Some((Token::OrAssign, pos));
            }
            ('|', _) => return Some((Token::Pipe, pos)),

            ('&', '&') => {
                // self.eat_next();
                let _ = stream.next();
                return Some((Token::And, pos));
            }
            ('&', '=') => {
                // self.eat_next();
                let _ = stream.next();
                return Some((Token::AndAssign, pos));
            }
            ('&', _) => return Some((Token::Ampersand, pos)),

            ('^', '=') => {
                // self.eat_next();
                let _ = stream.next();
                return Some((Token::XOrAssign, pos));
            }
            ('^', _) => return Some((Token::XOr, pos)),

            ('%', '=') => {
                // self.eat_next();
                let _ = stream.next();
                return Some((Token::ModuloAssign, pos));
            }
            ('%', _) => return Some((Token::Modulo, pos)),

            ('~', '=') => {
                // self.eat_next();
                let _ = stream.next();
                return Some((Token::PowerOfAssign, pos));
            }
            ('~', _) => return Some((Token::PowerOf, pos)),

            ('\0', _) => unreachable!(),

            (ch, _) if ch.is_whitespace() => (),
            (ch, _) => return Some((Token::LexError(Box::new(LERR::UnexpectedChar(ch))), pos)),
        }
    }

    Some((Token::EOF, ()))
}

pub fn next_token(state: &mut State, stream: &codemirror::StringStream) -> Option<(Token, ())> {
    let result = next_token_inner(state, stream);
    if let Some((token, _)) = result.as_ref() {
        state.can_be_unary = token.is_next_unary();
    }
    result
}
