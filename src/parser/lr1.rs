use crate::parser::production::Production;
use crate::tokenizer::token_type::TokenType;

/// # 非终结符
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct LR1Item {
    /// 产生式
    production: Production,
    /// 点的位置
    location: i32,
    // 向前看符号
    next: TokenType,
}
impl LR1Item {
    pub(in crate::parser) fn new(production: &Production, location: i32, next: &TokenType) -> Self {
        Self {
            production: production.clone(),
            location,
            next: *next,
        }
    }
}
