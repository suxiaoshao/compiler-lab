use crate::tokenizer::token::Token;
use crate::translator::identifier::IdentifierValue;
use crate::translator::quad::DestValue;
use std::collections::HashSet;

/// 语法单元属性
#[derive(Debug, Clone)]
pub(in crate::translator) struct Attribute {
    /// 该属性为真时的跳转指令链
    pub(in crate::translator) true_c: HashSet<usize>,
    /// 该属性为假时的跳转指令链
    pub(in crate::translator) false_c: HashSet<usize>,
    /// 跳转到该代码后的跳转指令链
    pub(in crate::translator) now_c: HashSet<usize>,
    /// 下一条指令位置
    pub(in crate::translator) next_instr: usize,
    /// Token信息
    pub(in crate::translator) token: Token,
    /// 临时变量名
    pub(in crate::translator) temp_id: DestValue,
    /// 类型和值
    pub(in crate::translator) value: Option<IdentifierValue>,
}

impl Attribute {
    pub(in crate::translator) fn new(token: &Token) -> Self {
        Self {
            true_c: HashSet::new(),
            false_c: HashSet::new(),
            now_c: HashSet::new(),
            next_instr: 0,
            token: token.clone(),
            temp_id: DestValue::None,
            value: None,
        }
    }
    pub(in crate::translator) fn new_value(token: &Token, value: IdentifierValue) -> Self {
        Self {
            true_c: HashSet::new(),
            false_c: HashSet::new(),
            now_c: HashSet::new(),
            next_instr: 0,
            token: token.clone(),
            temp_id: DestValue::None,
            value: Some(value),
        }
    }
}
