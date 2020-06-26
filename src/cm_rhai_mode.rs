use crate::codemirror;
use js_sys::RegExp;
use wasm_bindgen::prelude::*;
use web_sys::console;

mod token;

#[wasm_bindgen]
pub struct RhaiMode {
    indent_unit: u32,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct State {
    token_state: token::State,
    unclosed_bracket_count: i32,
    line_indent: u32,
}

thread_local! {
    static ELECTRIC_INPUT: RegExp = RegExp::new("^\\s*[}\\])]$", "").into();
}

#[wasm_bindgen]
impl RhaiMode {
    #[wasm_bindgen(constructor)]
    pub fn new(indent_unit: u32) -> Self {
        Self { indent_unit }
    }

    #[wasm_bindgen(js_name = startState)]
    pub fn start_state(&self) -> State {
        State {
            token_state: token::State::new(),
            unclosed_bracket_count: 0,
            line_indent: 0,
        }
    }

    #[wasm_bindgen(js_name = copyState)]
    pub fn copy_state(&self, state: &State) -> State {
        state.clone()
    }

    pub fn token(
        &self,
        stream: codemirror::StringStream,
        state: &mut State,
    ) -> Result<Option<String>, JsValue> {
        token(stream, state)
    }

    // #[wasm_bindgen(js_name = blankLine)]
    // pub fn blank_line(&self, state: &mut State) -> Result<(), JsValue> {
    //     Ok(())
    // }

    pub fn indent(&self, state: &mut State, text_after: String) -> JsValue {
        indent(self, state, text_after)
            .map(JsValue::from)
            .unwrap_or_else(|| codemirror::CODEMIRROR_PASS.clone())
    }

    #[wasm_bindgen(getter, js_name = electricInput)]
    pub fn electric_input(&self) -> RegExp {
        ELECTRIC_INPUT.with(|v| v.clone())
    }
}

fn token(stream: codemirror::StringStream, state: &mut State) -> Result<Option<String>, JsValue> {
    if stream.sol() {
        state.line_indent = stream.indentation();
        state.unclosed_bracket_count = 0;
    }

    let (next_token, _) = token::next_token(&mut state.token_state, &stream)
        .ok_or_else(|| "Failed to get next token")?;
    match next_token {
        token::Token::LeftBrace
        | token::Token::LeftBracket
        | token::Token::LeftParen
        | token::Token::MapStart => {
            if state.unclosed_bracket_count < 0 {
                state.unclosed_bracket_count = 0;
            }
            state.unclosed_bracket_count += 1;
        }
        token::Token::RightBrace | token::Token::RightBracket | token::Token::RightParen => {
            state.unclosed_bracket_count -= 1;
        }
        _ => {}
    };
    let res = match next_token {
        token::Token::IntegerConstant() => "number",
        token::Token::FloatConstant() => "number",
        token::Token::Identifier() => "variable",
        token::Token::CharConstant() => "string-2",
        token::Token::StringConst() => "string",
        token::Token::LeftBrace => "bracket",
        token::Token::RightBrace => "bracket",
        token::Token::LeftParen => "bracket",
        token::Token::RightParen => "bracket",
        token::Token::LeftBracket => "bracket",
        token::Token::RightBracket => "bracket",
        token::Token::Plus => "operator",
        token::Token::UnaryPlus => "operator",
        token::Token::Minus => "operator",
        token::Token::UnaryMinus => "operator",
        token::Token::Multiply => "operator",
        token::Token::Divide => "operator",
        token::Token::Modulo => "operator",
        token::Token::PowerOf => "operator",
        token::Token::LeftShift => "operator",
        token::Token::RightShift => "operator",
        token::Token::SemiColon => "operator",
        token::Token::Colon => "operator",
        token::Token::DoubleColon => "operator",
        token::Token::Comma => "operator",
        token::Token::Period => "operator",
        token::Token::MapStart => "bracket",
        token::Token::Equals => "operator",
        token::Token::True => "builtin",
        token::Token::False => "builtin",
        token::Token::Let => "keyword",
        token::Token::Const => "keyword",
        token::Token::If => "keyword",
        token::Token::Else => "keyword",
        token::Token::While => "keyword",
        token::Token::Loop => "keyword",
        token::Token::For => "keyword",
        token::Token::In => "keyword",
        token::Token::LessThan => "operator",
        token::Token::GreaterThan => "operator",
        token::Token::LessThanEqualsTo => "operator",
        token::Token::GreaterThanEqualsTo => "operator",
        token::Token::EqualsTo => "operator",
        token::Token::NotEqualsTo => "operator",
        token::Token::Bang => "operator",
        token::Token::Pipe => "operator",
        token::Token::Or => "operator",
        token::Token::XOr => "operator",
        token::Token::Ampersand => "operator",
        token::Token::And => "operator",
        token::Token::Fn => "keyword",
        token::Token::Continue => "keyword",
        token::Token::Break => "keyword",
        token::Token::Return => "keyword",
        token::Token::Throw => "keyword",
        token::Token::PlusAssign => "operator",
        token::Token::MinusAssign => "operator",
        token::Token::MultiplyAssign => "operator",
        token::Token::DivideAssign => "operator",
        token::Token::LeftShiftAssign => "operator",
        token::Token::RightShiftAssign => "operator",
        token::Token::AndAssign => "operator",
        token::Token::OrAssign => "operator",
        token::Token::XOrAssign => "operator",
        token::Token::ModuloAssign => "operator",
        token::Token::PowerOfAssign => "operator",
        token::Token::Private => "keyword",
        token::Token::Import => "keyword",
        token::Token::Export => "keyword",
        token::Token::As => "keyword",
        token::Token::LineComment => "comment",
        token::Token::BlockComment => "comment",
        token::Token::LexError(e) => {
            console::log_1(&JsValue::from_str(&format!("LexError: {}", e)));
            "error"
        }
        token::Token::EOF => return Ok(None),
    };
    Ok(Some(res.to_owned()))
}

fn indent(mode: &RhaiMode, state: &State, text_after: String) -> Option<u32> {
    let should_dedent = || {
        text_after
            .trim_start()
            .starts_with(['}', ']', ')'].as_ref())
    };
    if state.unclosed_bracket_count > 0 {
        if should_dedent() {
            Some(state.line_indent)
        } else {
            Some(state.line_indent + mode.indent_unit)
        }
    } else {
        if should_dedent() {
            Some(state.line_indent.saturating_sub(mode.indent_unit))
        } else {
            None
        }
    }
}
