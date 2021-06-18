/// 变量的值
#[derive(Clone, Copy, PartialEq, Debug)]
pub(in crate::translator) enum IdentifierValue {
    Int(i32),
    Real(f64),
    Bool(bool),
}

impl IdentifierValue {
    /// 判断是否类型相等
    pub(in crate::translator) fn check_type(&self, other: &Self) -> bool {
        use IdentifierValue::{Bool, Int, Real};
        match (self, other) {
            (Bool(_), Int(_)) => false,
            (Real(_), Int(_)) => false,
            (Bool(_), Real(_)) => false,
            (Real(_), Bool(_)) => false,
            (Int(_), Bool(_)) => false,
            (Int(_), Real(_)) => false,
            _ => true,
        }
    }
}

/// 变量（标识符）
#[derive(Clone, Debug)]
pub(in crate::translator) struct Identifier {
    pub(in crate::translator) value: IdentifierValue,
    // 作用域(最外层为0)
    pub(in crate::translator) scope: i32,
}

impl Identifier {
    pub(in crate::translator) fn new(value: IdentifierValue, scope: i32) -> Self {
        Self { value, scope }
    }
}
