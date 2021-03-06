use crate::tokenizer::position::Position;
use crate::tokenizer::read_token_state::and::And;
use crate::tokenizer::read_token_state::bool::Bool;
use crate::tokenizer::read_token_state::breaks::Breaks;
use crate::tokenizer::read_token_state::elses::Elses;
use crate::tokenizer::read_token_state::empty::Empty;
use crate::tokenizer::read_token_state::equal::Equal;
use crate::tokenizer::read_token_state::exclamation::Exclamation;
use crate::tokenizer::read_token_state::falses::Falses;
use crate::tokenizer::read_token_state::id::Id;
use crate::tokenizer::read_token_state::ifs::Ifs;
use crate::tokenizer::read_token_state::int_value::IntValue;
use crate::tokenizer::read_token_state::ints::Int;
use crate::tokenizer::read_token_state::less::Less;
use crate::tokenizer::read_token_state::more::More;
use crate::tokenizer::read_token_state::or::Or;
use crate::tokenizer::read_token_state::real::Real;
use crate::tokenizer::read_token_state::real_value::RealValue;
use crate::tokenizer::read_token_state::trues::Trues;
use crate::tokenizer::read_token_state::whiles::Whiles;
use crate::tokenizer::token::Token;

mod and;
mod bool;
mod breaks;
mod elses;
mod empty;
mod equal;
mod exclamation;
mod falses;
mod id;
mod ifs;
mod int_value;
mod ints;
mod less;
mod more;
mod or;
mod real;
mod real_value;
mod trues;
mod whiles;

#[derive(Clone, Debug)]
pub(in crate::tokenizer) enum ReadTokenState {
    Empty(Empty),
    Less(Less),
    More(More),
    Equal(Equal),
    Exclamation(Exclamation),
    And(And),
    Or(Or),
    IntValue(IntValue),
    RealValue(RealValue),
    Bool(Bool),
    Id(Id),
    Break(Breaks),
    Int(Int),
    If(Ifs),
    Else(Elses),
    False(Falses),
    True(Trues),
    Real(Real),
    While(Whiles),
}

/// # 状态
pub(in crate::tokenizer::read_token_state) trait ReadChar {
    /// # 读取字符,和位置返回 token 和 状态 和 是否读取下一个字符
    fn read_char(
        &self,
        c: char,
        position: &Position,
        pre_position: &Position,
    ) -> (Option<Token>, ReadTokenState, bool);
}

impl ReadTokenState {
    pub(in crate::tokenizer) fn read_char(
        &mut self,
        c: char,
        position: &Position,
        pre_position: &Position,
    ) -> (Option<Token>, bool) {
        let (token, state, if_next) = match self {
            ReadTokenState::Empty(e) => e.read_char(c, position, pre_position),
            ReadTokenState::Less(e) => e.read_char(c, position, pre_position),
            ReadTokenState::More(e) => e.read_char(c, position, pre_position),
            ReadTokenState::Equal(e) => e.read_char(c, position, pre_position),
            ReadTokenState::Exclamation(e) => e.read_char(c, position, pre_position),
            ReadTokenState::And(e) => e.read_char(c, position, pre_position),
            ReadTokenState::Or(e) => e.read_char(c, position, pre_position),
            ReadTokenState::IntValue(e) => e.read_char(c, position, pre_position),
            ReadTokenState::RealValue(e) => e.read_char(c, position, pre_position),
            ReadTokenState::Bool(e) => e.read_char(c, position, pre_position),
            ReadTokenState::Id(e) => e.read_char(c, position, pre_position),
            ReadTokenState::Break(e) => e.read_char(c, position, pre_position),
            ReadTokenState::Int(e) => e.read_char(c, position, pre_position),
            ReadTokenState::If(e) => e.read_char(c, position, pre_position),
            ReadTokenState::Else(e) => e.read_char(c, position, pre_position),
            ReadTokenState::False(e) => e.read_char(c, position, pre_position),
            ReadTokenState::True(e) => e.read_char(c, position, pre_position),
            ReadTokenState::Real(e) => e.read_char(c, position, pre_position),
            ReadTokenState::While(e) => e.read_char(c, position, pre_position),
        };
        *self = state;
        (token, if_next)
    }
    pub(in crate::tokenizer) fn new(position: &Position) -> Self {
        Self::Empty(Empty::new(position.clone()))
    }
}
/// 判断是否是特殊符号
pub(in crate::tokenizer::read_token_state) fn check_special_symbols(c: char) -> bool {
    match c {
        '%' | '#' | '@' | '^' | '`' | ':' | '\\' | '\'' | '"' | '?' | '.' | '+' | '*' | '-'
        | '/' | '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' | '=' | '!' | '&' | '|' | ';'
        | ',' => true,
        _ => false,
    }
}
