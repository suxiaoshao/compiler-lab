use crate::tokenizer::position::Position;
use crate::tokenizer::read_token_state::empty::Empty;
use crate::tokenizer::read_token_state::id::Id;
use crate::tokenizer::read_token_state::{check_special_symbols, ReadChar, ReadTokenState};
use crate::tokenizer::token::Token;
use crate::tokenizer::token_type::TokenType;

#[derive(Clone, Debug)]
pub(in crate::tokenizer) struct Trues {
    position: Position,
    value: String,
}
impl ReadChar for Trues {
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
        let true_state = ReadTokenState::True(Trues::new(self.position.clone(), &now_str));
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
                    TokenType::True,
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
                        TokenType::True,
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
                        'r' => true_state,
                        _ => id_state,
                    };
                    (None, state, true)
                }
                2 => {
                    let state = if c == 'u' { true_state } else { id_state };
                    (None, state, true)
                }
                3 => {
                    let state = if c == 'e' { true_state } else { id_state };
                    (None, state, true)
                }
                4 => (None, id_state, true),
                _ => panic!("?????? bool ??????"),
            },
        }
    }
}
impl Trues {
    pub(in crate::tokenizer::read_token_state) fn new(position: Position, value: &str) -> Self {
        Self {
            position,
            value: value.to_string(),
        }
    }
}
