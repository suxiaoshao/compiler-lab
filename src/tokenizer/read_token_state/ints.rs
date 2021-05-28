use crate::tokenizer::position::Position;
use crate::tokenizer::read_token_state::empty::Empty;
use crate::tokenizer::read_token_state::id::Id;
use crate::tokenizer::read_token_state::ifs::Ifs;
use crate::tokenizer::read_token_state::{check_special_symbols, ReadChar, ReadTokenState};
use crate::tokenizer::token::{Token, TokenType};

#[derive(Clone, Debug)]
pub(in crate::tokenizer) struct Int {
    position: Position,
    value: String,
}
impl ReadChar for Int {
    fn read_char(
        &self,
        c: char,
        position: &Position,
        pre_position: &Position,
    ) -> (Option<Token>, ReadTokenState, bool) {
        let position = position.clone();
        let now_str = self.value.to_string() + &*c.to_string();
        let empty_state = ReadTokenState::Empty(Empty::new(position.clone()));
        let id_state = ReadTokenState::Id(Id::new(self.position.clone(), &now_str));
        let int_state = ReadTokenState::Int(Int::new(self.position.clone(), &now_str));
        if check_special_symbols(c) {
            let token = if self.value.len() <= 2 {
                Token::new(
                    self.value.to_string(),
                    TokenType::Id,
                    &self.position,
                    pre_position,
                )
            } else {
                Token::new(
                    self.value.to_string(),
                    TokenType::Int,
                    &self.position,
                    pre_position,
                )
            };
            return (Some(token), empty_state, false);
        };
        match c {
            ' ' | '\n' | '\r' => (
                Some(if self.value.len() <= 2 {
                    Token::new(
                        self.value.to_string(),
                        TokenType::Id,
                        &self.position,
                        pre_position,
                    )
                } else {
                    Token::new(
                        self.value.to_string(),
                        TokenType::Int,
                        &self.position,
                        pre_position,
                    )
                }),
                empty_state,
                true,
            ),
            _ => match self.value.len() {
                1 => {
                    let state = match c {
                        'n' => int_state,
                        'f' => ReadTokenState::If(Ifs::new(self.position.clone(), &now_str)),
                        _ => id_state,
                    };
                    (None, state, true)
                }
                2 => {
                    let state = if c == 't' { int_state } else { id_state };
                    (None, state, true)
                }
                3 => (None, id_state, true),
                _ => panic!("解析 bool 出错"),
            },
        }
    }
}
impl Int {
    pub(in crate::tokenizer::read_token_state) fn new(position: Position, value: &str) -> Self {
        Self {
            position,
            value: value.to_string(),
        }
    }
}
