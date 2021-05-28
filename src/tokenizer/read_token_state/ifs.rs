use crate::tokenizer::position::Position;
use crate::tokenizer::read_token_state::empty::Empty;
use crate::tokenizer::read_token_state::id::Id;
use crate::tokenizer::read_token_state::{check_special_symbols, ReadChar, ReadTokenState};
use crate::tokenizer::token::{Token, TokenType};

#[derive(Clone)]
pub(in crate::tokenizer) struct Ifs {
    position: Position,
    value: String,
}
impl ReadChar for Ifs {
    fn read_char(&self, c: char, position: &Position) -> (Option<Token>, ReadTokenState, bool) {
        let position = position.clone();
        let now_str = self.value.to_string() + &*c.to_string();
        let empty_state = ReadTokenState::Empty(Empty::new(position.clone()));
        let id_state = ReadTokenState::Id(Id::new(self.position.clone(), &now_str));
        if check_special_symbols(c) {
            return (
                Some(Token::new(self.value.to_string(), TokenType::If)),
                empty_state,
                false,
            );
        };
        match c {
            ' ' | '\n' | '\r' => (
                Some(Token::new(self.value.to_string(), TokenType::If)),
                empty_state,
                true,
            ),
            _ => (None, id_state, true),
        }
    }
}
impl Ifs {
    pub(in crate::tokenizer::read_token_state) fn new(position: Position, value: &str) -> Self {
        Self {
            position,
            value: value.to_string(),
        }
    }
}
