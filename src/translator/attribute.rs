use crate::tokenizer::token::Token;
use crate::translator::identifier::IdentifierValue;
use std::collections::HashSet;

/// 语法单元属性
pub(in crate::translator) struct Attribute {
    /// 该属性为真时的跳转指令链
    true_c: HashSet<usize>,
    /// 该属性为假时的跳转指令链
    false_c: HashSet<usize>,
    /// 跳转到该代码后的跳转指令链
    now_c: HashSet<usize>,
    /// 下一条指令位置
    next_instr: usize,
    /// Token信息
    token: Token,
    /// 临时变量名
    temp_id_name: String,
    /// 类型和值
    value: Option<IdentifierValue>,
}
