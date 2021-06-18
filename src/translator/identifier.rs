/// 变量的值
pub(in crate::translator) enum IdentifierValue {
    Int(i32),
    Real(f64),
    Bool(bool),
}
/// 变量（标识符）
pub(in crate::translator) struct Identifier {
    value: IdentifierValue,
    // 作用域(最外层为0)
    scope: i32,
}
