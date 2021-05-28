use crate::tokenizer::position::Position;
use crate::tokenizer::read_token_state::empty::Empty;
use crate::tokenizer::read_token_state::{check_special_symbols, ReadChar, ReadTokenState};
use crate::tokenizer::token::{Token, TokenType};

#[derive(Clone, Debug)]
pub(in crate::tokenizer) struct Id {
    position: Position,
    value: String,
}
impl ReadChar for Id {
    fn read_char(&self, c: char, position: &Position) -> (Option<Token>, ReadTokenState, bool) {
        let position = position.clone();
        let empty = ReadTokenState::Empty(Empty::new(position.clone()));
        let now_str = self.value.to_string() + &*c.to_string();
        if check_special_symbols(c) {
            (
                Some(Token::new(
                    self.value.to_string(),
                    TokenType::Id,
                    &self.position,
                    &position,
                )),
                empty,
                false,
            )
        } else {
            match c {
                ' ' | '\r' | '\n' => (
                    Some(Token::new(
                        self.value.to_string(),
                        TokenType::Id,
                        &self.position,
                        &position,
                    )),
                    empty,
                    true,
                ),
                _ => (
                    None,
                    ReadTokenState::Id(Id::new(self.position.clone(), &now_str)),
                    true,
                ),
            }
        }
    }
}
impl Id {
    pub(in crate::tokenizer::read_token_state) fn new(position: Position, value: &str) -> Self {
        Self {
            position,
            value: value.to_string(),
        }
    }
}
