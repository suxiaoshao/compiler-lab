use std::collections::HashSet;

use serde::Deserialize;

use crate::tokenizer::token_type::TokenType;

use super::{lr1_item::LR1Item, production::Production};

/// # 非终结符
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Hash)]
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
impl NonTerminator {
    /// # 获取产生式左部为symbol的项目集
    /// `symbol`: 上一个式子·后的非终结符
    ///
    /// `first`: symbol后的符号求出来的first集
    pub(in crate::parser) fn get_items_by_n(
        &self,
        first: &HashSet<TokenType>,
        propd: &Vec<Production>,
    ) -> Vec<LR1Item> {
        let mut items = vec![];
        propd
            .iter()
            .filter(|prop| *self == prop.left)
            .for_each(|prop| {
                for i in first {
                    items.push(LR1Item::new(prop, 0, i));
                }
            });
        items
    }
    /// 遍历每个非终结符
    pub(in crate::parser) fn get_all_vec() -> Vec<NonTerminator> {
        vec![
            NonTerminator::Program,
            NonTerminator::Block,
            NonTerminator::Decls,
            NonTerminator::Decl,
            NonTerminator::Type,
            NonTerminator::Stmts,
            NonTerminator::Stmt,
            NonTerminator::Var,
            NonTerminator::Bool,
            NonTerminator::Join,
            NonTerminator::Equality,
            NonTerminator::Rel,
            NonTerminator::Expr,
            NonTerminator::Term,
            NonTerminator::Unary,
            NonTerminator::Factor,
        ]
    }
}
