use crate::tokenizer::position::Position;
use crate::tokenizer::read_token_state::empty::Empty;
use crate::tokenizer::read_token_state::{ReadChar, ReadTokenState};
use crate::tokenizer::token::Token;
use crate::tokenizer::token_type::TokenType;

#[derive(Clone, Debug)]
pub(in crate::tokenizer) struct RealValue {
    position: Position,
    value: String,
}
impl ReadChar for RealValue {
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
                ReadTokenState::RealValue(RealValue::new(
                    self.position.clone(),
                    &(self.value.to_string() + &*c.to_string()),
                )),
                true,
            ),
            _ => (
                Some(Token::new(
                    self.value.to_string(),
                    TokenType::RealNum,
                    &self.position,
                    pre_position,
                )),
                empty,
                false,
            ),
        }
    }
}
impl RealValue {
    pub(in crate::tokenizer::read_token_state) fn new(position: Position, value: &str) -> Self {
        Self {
            position,
            value: value.to_string(),
        }
    }
}
