use serde::Deserialize;

/// # 非终结符
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize)]
pub enum NonTerminator {
    Program,
    Block,
    Decls,
    Decl,
    Type,
    Stmts,
    Stmt,
    Var,
    Bool,
    Join,
    Equality,
    Rel,
    Expr,
    Term,
    Unary,
    Factor,
}
