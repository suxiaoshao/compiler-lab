use crate::tokenizer::position::Position;
use crate::tokenizer::read_token_state::empty::Empty;
use crate::tokenizer::read_token_state::real_value::RealValue;
use crate::tokenizer::read_token_state::{ReadChar, ReadTokenState};
use crate::tokenizer::token::{Token, TokenType};

#[derive(Clone, Debug)]
pub(in crate::tokenizer) struct IntValue {
    position: Position,
    value: String,
}
impl ReadChar for IntValue {
    fn read_char(
        &self,
        c: char,
        position: &Position,
        pre_position: &Position,
    ) -> (Option<Token>, ReadTokenState, bool) {
        let position = position.clone();
        let empty = ReadTokenState::Empty(Empty::new(position.clone()));
        match c {
            '0'..='9' => (
                None,
                ReadTokenState::IntValue(IntValue::new(
                    self.position.clone(),
                    &*(self.value.to_string() + &*c.to_string()),
                )),
                true,
            ),
            '.' => (
                None,
                ReadTokenState::RealValue(RealValue::new(
                    self.position.clone(),
                    &*(self.value.to_string() + &*c.to_string()),
                )),
                true,
            ),
            _ => (
                Some(Token::new(
                    self.value.to_string(),
                    TokenType::IntNum,
                    &self.position,
                    pre_position,
                )),
                empty,
                false,
            ),
        }
    }
}
impl IntValue {
    pub(in crate::tokenizer::read_token_state) fn new(position: Position, value: &str) -> Self {
        Self {
            position,
            value: value.to_string(),
        }
    }
}
