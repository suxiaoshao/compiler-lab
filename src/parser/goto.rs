use std::collections::HashMap;

use super::{
    canonical_collection::CanonicalCollection, non_terminator::NonTerminator,
    production::ProductionRight,
};

/// action表
/// 行是项目集序号
/// 列是非终结符
#[derive(Debug)]
pub(in crate::parser) struct Goto(pub(crate) Vec<HashMap<NonTerminator, Option<usize>>>);
impl Goto {
    /// 新建表
    pub(in crate::parser) fn new(cc: &CanonicalCollection) -> Self {
        let mut action = vec![];
        for _ in 0..(cc.item_sets.len()) {
            let mut action_item = HashMap::new();
            for token in NonTerminator::get_all_vec() {
                action_item.insert(token, None);
            }
            action.push(action_item);
        }
        Self(action)
    }
    /// 构建GOTO表
    pub(in crate::parser) fn update(&mut self, graph: &Vec<(ProductionRight, usize)>, i: usize) {
        for p in graph {
            if let ProductionRight::NonTerminator(symbol) = p.0 {
                self.0[i].insert(symbol, Some(p.1));
            }
        }
    }
}
