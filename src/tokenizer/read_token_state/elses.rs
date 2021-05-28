use crate::tokenizer::position::Position;
use crate::tokenizer::read_token_state::empty::Empty;
use crate::tokenizer::read_token_state::id::Id;
use crate::tokenizer::read_token_state::{check_special_symbols, ReadChar, ReadTokenState};
use crate::tokenizer::token::{Token, TokenType};

#[derive(Clone, Debug)]
pub(in crate::tokenizer) struct Elses {
    position: Position,
    value: String,
}
impl ReadChar for Elses {
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
        let else_state = ReadTokenState::Else(Elses::new(self.position.clone(), &now_str));
        if check_special_symbols(c) {
            let token = if self.value.len() <= 3 {
                Token::new(
                    self.value.to_string(),
                    TokenType::Id,
                    &self.position,
                    pre_position,
                )
            } else {
                Token::new(
                    self.value.to_string(),
                    TokenType::Else,
                    &self.position,
                    pre_position,
                )
            };
            return (Some(token), empty_state, false);
        };
        match c {
            ' ' | '\n' | '\r' => (
                Some(if self.value.len() <= 3 {
                    Token::new(
                        self.value.to_string(),
                        TokenType::Id,
                        &self.position,
                        pre_position,
                    )
                } else {
                    Token::new(
                        self.value.to_string(),
                        TokenType::Else,
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
                        'l' => else_state,
                        _ => id_state,
                    };
                    (None, state, true)
                }
                2 => {
                    let state = if c == 's' { else_state } else { id_state };
                    (None, state, true)
                }
                3 => {
                    let state = if c == 'e' { else_state } else { id_state };
                    (None, state, true)
                }
                4 => (None, id_state, true),
                _ => panic!("解析 bool 出错"),
            },
        }
    }
}
impl Elses {
    pub(in crate::tokenizer::read_token_state) fn new(position: Position, value: &str) -> Self {
        Self {
            position,
            value: value.to_string(),
        }
    }
}
