use crate::tokenizer::position::Position;
use crate::tokenizer::read_token_state::empty::Empty;
use crate::tokenizer::read_token_state::{ReadChar, ReadTokenState};
use crate::tokenizer::token::Token;
use crate::tokenizer::token_type::TokenType;

/// # and 状态
///
/// 匹配到 `&` 符号时进入这个状态
/// 会尝试匹配 `&&` 符号
#[derive(Clone, Debug)]
pub(in crate::tokenizer) struct And {
    position: Position,
}
impl ReadChar for And {
    fn read_char(
        &self,
        c: char,
        position: &Position,
        pre_position: &Position,
    ) -> (Option<Token>, ReadTokenState, bool) {
        let empty = ReadTokenState::Empty(Empty::new(position.clone()));
        match c {
            ' ' | '\n' | '\r' => (
                Some(Token::new(
                    "&".to_string(),
                    TokenType::Epsilon,
                    &self.position,
                    pre_position,
                )),
                empty,
                true,
            ),
            '&' => (
                Some(Token::new(
                    "&&".to_string(),
                    TokenType::And,
                    &self.position,
                    position,
                )),
                empty,
                true,
            ),
            _ => (
                Some(Token::new(
                    "&".to_string(),
                    TokenType::Epsilon,
                    &self.position,
                    pre_position,
                )),
                empty,
                false,
            ),
        }
    }
}
impl And {
    pub(in crate::tokenizer::read_token_state) fn new(position: Position) -> Self {
        Self { position }
    }
}
#[cfg(test)]
mod test {
    use crate::tokenizer::position::Position;
    use crate::tokenizer::read_token_state::and::And;
    use crate::tokenizer::read_token_state::{ReadChar, ReadTokenState};
    use crate::tokenizer::token::Token;
    use crate::tokenizer::token_type::TokenType;

    #[test]
    fn read_char_() {
        let position = Position::new();
        let and = And::new(position.clone());
        let (token, state, is_next) = and.read_char(' ', &position, &position);
        if let Some(Token {
            lex, token_type, ..
        }) = token
        {
            assert_eq!(lex, "&");
            assert_eq!(TokenType::Epsilon, token_type);
        } else {
            panic!("返回 token 错误")
        }
        match state {
            ReadTokenState::Empty(_) => {}
            _ => panic!("状态出错"),
        }
        assert!(is_next);
    }
    #[test]
    fn read_char_and() {
        let position = Position::new();
        let and = And::new(position.clone());
        let (token, state, is_next) = and.read_char('&', &position, &position);
        if let Some(Token {
            lex, token_type, ..
        }) = token
        {
            assert_eq!(lex, "&&");
            assert_eq!(TokenType::And, token_type);
        } else {
            panic!("返回 token 错误")
        }
        match state {
            ReadTokenState::Empty(_) => {}
            _ => panic!("状态出错"),
        }
        assert!(is_next);
    }
}
