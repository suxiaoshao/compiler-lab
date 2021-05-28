use crate::tokenizer::position::Position;
use crate::tokenizer::read_token_state::breaks::Breaks;
use crate::tokenizer::read_token_state::empty::Empty;
use crate::tokenizer::read_token_state::id::Id;
use crate::tokenizer::read_token_state::{check_special_symbols, ReadChar, ReadTokenState};
use crate::tokenizer::token::{Token, TokenType};

#[derive(Clone, Debug)]
pub(in crate::tokenizer) struct Bool {
    position: Position,
    value: String,
}
impl ReadChar for Bool {
    fn read_char(
        &self,
        c: char,
        position: &Position,
        pre_position: &Position,
    ) -> (Option<Token>, ReadTokenState, bool) {
        let position = position.clone();
        let empty = ReadTokenState::Empty(Empty::new(position.clone()));
        let now_str = self.value.to_string() + &*c.to_string();
        if check_special_symbols(c) {
            let token = if self.value.len() <= 3 {
                Token::new(
                    self.value.to_string(),
                    TokenType::Id,
                    &self.position,
                    &pre_position,
                )
            } else {
                Token::new(
                    self.value.to_string(),
                    TokenType::Bool,
                    &self.position,
                    &pre_position,
                )
            };
            return (Some(token), empty, false);
        };
        match c {
            ' ' | '\n' | '\r' => (
                Some(if self.value.len() <= 3 {
                    Token::new(
                        self.value.to_string(),
                        TokenType::Id,
                        &self.position,
                        &pre_position,
                    )
                } else {
                    Token::new(
                        self.value.to_string(),
                        TokenType::Bool,
                        &self.position,
                        &pre_position,
                    )
                }),
                empty,
                true,
            ),
            _ => match self.value.len() {
                1 => {
                    let state = match c {
                        'o' => ReadTokenState::Bool(Bool::new(self.position.clone(), &now_str)),
                        'r' => ReadTokenState::Break(Breaks::new(self.position.clone(), &now_str)),
                        _ => ReadTokenState::Id(Id::new(self.position.clone(), &now_str)),
                    };
                    (None, state, true)
                }
                2 => {
                    let state = if c == 'o' {
                        ReadTokenState::Bool(Bool::new(self.position.clone(), &now_str))
                    } else {
                        ReadTokenState::Id(Id::new(self.position.clone(), &now_str))
                    };
                    (None, state, true)
                }
                3 => {
                    let state = if c == 'l' {
                        ReadTokenState::Bool(Bool::new(self.position.clone(), &now_str))
                    } else {
                        ReadTokenState::Id(Id::new(self.position.clone(), &now_str))
                    };
                    (None, state, true)
                }
                4 => (
                    None,
                    ReadTokenState::Id(Id::new(self.position.clone(), &now_str)),
                    true,
                ),
                _ => panic!("解析 bool 出错"),
            },
        }
    }
}
impl Bool {
    pub(in crate::tokenizer::read_token_state) fn new(position: Position, value: &str) -> Self {
        Self {
            position,
            value: value.to_string(),
        }
    }
}
