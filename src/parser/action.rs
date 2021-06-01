use std::collections::HashMap;

use crate::parser::lr1_item_set::LR1ItemSet;
use crate::parser::production::{Production, ProductionRight};
use crate::tokenizer::token_type::TokenType;

use super::canonical_collection::CanonicalCollection;

/// 动作表
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::parser) enum ActionType {
    /// 接受
    Accept(usize),
    /// 移进
    Shift(usize),
    /// 规约
    Reduce(usize),
    /// 错误，拒绝    
    Error,
}
/// action表
/// 行是项目集序号
/// 列是终结符
#[derive(Debug)]
pub(in crate::parser) struct Action(pub(in crate::parser) Vec<HashMap<TokenType, ActionType>>);

impl Action {
    pub(in crate::parser) fn new(cc: &CanonicalCollection) -> Self {
        let mut action = vec![];
        for _ in 0..(cc.item_sets.len()) {
            let mut action_item = HashMap::new();
            let action_item_item = ActionType::Error;
            for token in TokenType::get_all_vec() {
                action_item.insert(token, action_item_item.clone());
            }
            action.push(action_item);
        }
        Self(action)
    }
    /// 构建ACTION表
    pub(in crate) fn update(
        &mut self,
        items: &LR1ItemSet,
        graph: &Vec<(ProductionRight, usize)>,
        prods: &Vec<Production>,
        i: usize,
    ) {
        // 构建ACTION表
        for item in &items.items {
            if let Some(symbol) = item.production.right.get(item.location) {
                // 移进 or 待约项
                // 形如 A -> ε 的，直接规约就行
                if item.is_move() {
                    if let ProductionRight::Terminator(t) = symbol {
                        graph.iter().find(|&p| p.0 == *symbol).map(|p| {
                            self.0[i].insert(t.clone(), ActionType::Shift(p.1));
                        });
                    }
                }
                // 规约项
                else {
                    if item.production.left == prods[0].left && item.next == TokenType::Eof {
                        self.0[i].insert(item.next, ActionType::Accept(0));
                    } else {
                        for j in 0..prods.len() {
                            if item.production == prods[j] {
                                self.0[i].insert(item.next, ActionType::Reduce(j));
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}
