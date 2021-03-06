use crate::tokenizer::position::Position;
use crate::tokenizer::read_token_state::and::And;
use crate::tokenizer::read_token_state::bool::Bool;
use crate::tokenizer::read_token_state::elses::Elses;
use crate::tokenizer::read_token_state::equal::Equal;
use crate::tokenizer::read_token_state::exclamation::Exclamation;
use crate::tokenizer::read_token_state::falses::Falses;
use crate::tokenizer::read_token_state::id::Id;
use crate::tokenizer::read_token_state::int_value::IntValue;
use crate::tokenizer::read_token_state::ints::Int;
use crate::tokenizer::read_token_state::less::Less;
use crate::tokenizer::read_token_state::more::More;
use crate::tokenizer::read_token_state::or::Or;
use crate::tokenizer::read_token_state::real::Real;
use crate::tokenizer::read_token_state::trues::Trues;
use crate::tokenizer::read_token_state::whiles::Whiles;
use crate::tokenizer::read_token_state::{ReadChar, ReadTokenState};
use crate::tokenizer::token::Token;
use crate::tokenizer::token_type::TokenType;

#[derive(Clone, Debug)]
pub(in crate::tokenizer) struct Empty {
    position: Position,
}
impl ReadChar for Empty {
    fn read_char(
        &self,
        c: char,
        position: &Position,
        _pre_position: &Position,
    ) -> (Option<Token>, ReadTokenState, bool) {
        let position = position.clone();
        let string = c.to_string();
        match c {
            ' ' | '\n' | '\r' => (None, ReadTokenState::Empty(Empty::new(position)), true),
            '+' => (
                Some(Token::new(string, TokenType::Add, &position, &position)),
                ReadTokenState::Empty(Empty::new(position)),
                true,
            ),
            '*' => (
                Some(Token::new(string, TokenType::Mul, &position, &position)),
                ReadTokenState::Empty(Empty::new(position)),
                true,
            ),
            '-' => (
                Some(Token::new(string, TokenType::Sub, &position, &position)),
                ReadTokenState::Empty(Empty::new(position)),
                true,
            ),
            '/' => (
                Some(Token::new(string, TokenType::Div, &position, &position)),
                ReadTokenState::Empty(Empty::new(position)),
                true,
            ),
            '(' => (
                Some(Token::new(
                    string,
                    TokenType::CirLeftBracket,
                    &position,
                    &position,
                )),
                ReadTokenState::Empty(Empty::new(position)),
                true,
            ),
            ')' => (
                Some(Token::new(
                    string,
                    TokenType::CirRightBracket,
                    &position,
                    &position,
                )),
                ReadTokenState::Empty(Empty::new(position)),
                true,
            ),
            '[' => (
                Some(Token::new(
                    string,
                    TokenType::SqLeftBracket,
                    &position,
                    &position,
                )),
                ReadTokenState::Empty(Empty::new(position)),
                true,
            ),
            ']' => (
                Some(Token::new(
                    string,
                    TokenType::SqRightBracket,
                    &position,
                    &position,
                )),
                ReadTokenState::Empty(Empty::new(position)),
                true,
            ),
            '{' => (
                Some(Token::new(
                    string,
                    TokenType::LeftBlock,
                    &position,
                    &position,
                )),
                ReadTokenState::Empty(Empty::new(position)),
                true,
            ),
            '}' => (
                Some(Token::new(
                    string,
                    TokenType::RightBlock,
                    &position,
                    &position,
                )),
                ReadTokenState::Empty(Empty::new(position)),
                true,
            ),
            ',' => (
                Some(Token::new(string, TokenType::Comma, &position, &position)),
                ReadTokenState::Empty(Empty::new(position)),
                true,
            ),
            '%' | '#' | '@' | '^' | '`' | ':' | '\\' | '\'' | '"' | '?' | '.' => (
                Some(Token::new(string, TokenType::Epsilon, &position, &position)),
                ReadTokenState::Empty(Empty::new(position)),
                true,
            ),
            '<' => (None, ReadTokenState::Less(Less::new(position)), true),
            '>' => (None, ReadTokenState::More(More::new(position)), true),
            '=' => (None, ReadTokenState::Equal(Equal::new(position)), true),
            '!' => (
                None,
                ReadTokenState::Exclamation(Exclamation::new(position)),
                true,
            ),
            '&' => (None, ReadTokenState::And(And::new(position)), true),
            '|' => (None, ReadTokenState::Or(Or::new(position)), true),
            ';' => (
                Some(Token::new(
                    string,
                    TokenType::Semicolon,
                    &position,
                    &position,
                )),
                ReadTokenState::Empty(Empty::new(position)),
                true,
            ),
            '0'..='9' => (
                None,
                ReadTokenState::IntValue(IntValue::new(position, &c.to_string())),
                true,
            ),
            'b' => (
                None,
                ReadTokenState::Bool(Bool::new(position, &string)),
                true,
            ),
            'w' => (
                None,
                ReadTokenState::While(Whiles::new(position, &string)),
                true,
            ),
            'i' => (None, ReadTokenState::Int(Int::new(position, &string)), true),
            'e' => (
                None,
                ReadTokenState::Else(Elses::new(position, &string)),
                true,
            ),
            'f' => (
                None,
                ReadTokenState::False(Falses::new(position, &string)),
                true,
            ),
            't' => (
                None,
                ReadTokenState::True(Trues::new(position, &string)),
                true,
            ),
            'r' => (
                None,
                ReadTokenState::Real(Real::new(position, &string)),
                true,
            ),
            _ => (None, ReadTokenState::Id(Id::new(position, &string)), true),
        }
    }
}

impl Empty {
    pub(in crate::tokenizer::read_token_state) fn new(position: Position) -> Self {
        Self { position }
    }
}
