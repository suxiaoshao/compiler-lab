use crate::tokenizer::position::Position;
use read_token_state::ReadTokenState;
use token::Token;

mod position;
mod read_token_state;
mod token;

pub fn get_tokens_from_string(content: String) -> Vec<Token> {
    // 字符向量
    let chars: Vec<char> = content.chars().collect();
    // 结果
    let mut tokens = vec![];
    // 位置
    let mut position = Position::new();
    // 状态
    let mut read_token_state: ReadTokenState = ReadTokenState::new(&position);
    // 下标
    let mut index = 0;
    while index < chars.len() {
        let c = chars[index];
        let (token, is_next) = read_token_state.read_char(c, &position);
        if let Some(token) = token {
            tokens.push(token)
        }
        if is_next {
            position.change_from_char(c);
            index += 1;
        }
    }
    let (token, ..) = read_token_state.read_char(' ', &position);
    if let Some(token) = token {
        tokens.push(token)
    }
    tokens.iter_mut().for_each(|token| token.check_position());
    tokens
}
