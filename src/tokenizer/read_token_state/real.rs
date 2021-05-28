use crate::tokenizer::position::Position;
use crate::tokenizer::read_token_state::empty::Empty;
use crate::tokenizer::read_token_state::id::Id;
use crate::tokenizer::read_token_state::returns::Returns;
use crate::tokenizer::read_token_state::{check_special_symbols, ReadChar, ReadTokenState};
use crate::tokenizer::token::{Token, TokenType};

#[derive(Clone, Debug)]
pub(in crate::tokenizer) struct Real {
    position: Position,
    value: String,
}
impl ReadChar for Real {
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
        let real_state = ReadTokenState::Real(Real::new(self.position.clone(), &now_str));
        if check_special_symbols(c) && c != '.' {
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
                    TokenType::Real,
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
                        TokenType::Real,
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
                        'e' => real_state,
                        _ => id_state,
                    };
                    (None, state, true)
                }
                2 => {
                    let state = if c == 'a' {
                        real_state
                    } else if c == 't' {
                        ReadTokenState::Return(Returns::new(self.position.clone(), &now_str))
                    } else {
                        id_state
                    };
                    (None, state, true)
                }
                3 => {
                    let state = if c == 'l' { real_state } else { id_state };
                    (None, state, true)
                }
                4 => (None, id_state, true),
                _ => panic!("解析 bool 出错"),
            },
        }
    }
}
impl Real {
    pub(in crate::tokenizer::read_token_state) fn new(position: Position, value: &str) -> Self {
        Self {
            position,
            value: value.to_string(),
        }
    }
}
