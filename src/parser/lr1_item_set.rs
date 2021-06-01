use std::collections::{HashMap, HashSet};

use crate::parser::lr1_item::LR1Item;
use crate::parser::non_terminator::NonTerminator;
use crate::parser::production::{Production, ProductionRight};
use crate::tokenizer::token_type::TokenType;

/// # LR1 集合
#[derive(Clone, Debug)]
pub(crate) struct LR1ItemSet {
    pub(in crate::parser) items: Vec<LR1Item>,
}

impl PartialEq for LR1ItemSet {
    fn eq(&self, other: &Self) -> bool {
        self.items.iter().all(|item| other.items.contains(item))
            && self.items.len() == other.items.len()
    }
}

impl Eq for LR1ItemSet {}

impl LR1ItemSet {
    /// 新建
    pub(in crate::parser) fn new(item: &Vec<LR1Item>) -> Self {
        Self {
            items: item.clone(),
        }
    }
    /// 求闭包
    pub(crate) fn closure(
        &mut self,
        first_set: &HashMap<NonTerminator, HashSet<TokenType>>,
        nullable_set: &HashSet<NonTerminator>,
        props: &Vec<Production>,
    ) {
        let mut cnt = 0;
        loop {
            let size = self.items.len();
            for item in &self.items.clone()[cnt..] {
                // 获取闭包
                let new_items: Vec<LR1Item> = item
                    .closure(first_set, nullable_set, props)
                    .iter()
                    .filter(|item| !self.items.contains(&item))
                    .map(|x| x.clone())
                    .collect();
                // 插入新的项目
                new_items
                    .iter()
                    .for_each(|item| self.items.push(item.clone()));
            }
            cnt += 1;
            if self.items.len() == size {
                break;
            }
        }
    }
    /// 吃入一个字符，到达一个新状态
    pub fn go(
        &self,
        symbol: ProductionRight,
        first_set: &HashMap<NonTerminator, HashSet<TokenType>>,
        nullable_set: &HashSet<NonTerminator>,
        props: &Vec<Production>,
    ) -> LR1ItemSet {
        let mut dst = Self::new(&vec![]);
        // 找到 · 后字符为symbol的项目
        for item in &self.items {
            if item.is_move() && item.production.right[item.location] == symbol {
                let mut new_item = item.clone();
                new_item.location += 1;
                dst.items.push(new_item);
            }
        }
        dst.closure(first_set, nullable_set, props);
        dst
    }
}
