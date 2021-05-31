use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::parser::production::Production;
use crate::tokenizer::token_type::TokenType;

use super::non_terminator::NonTerminator;
use super::production::ProductionRight;

/// # LR1项目
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(in crate::parser) struct LR1Item {
    /// 产生式
    pub(in crate::parser::lr1) production: Production,
    /// 点的位置
    pub(in crate::parser::lr1) location: usize,
    // 向前看符号
    pub(in crate::parser::lr1) next: TokenType,
}
impl LR1Item {
    /// 新建
    pub(in crate::parser) fn new(
        production: &Production,
        location: usize,
        next: &TokenType,
    ) -> Self {
        Self {
            production: production.clone(),
            location,
            next: *next,
        }
    }
    /// 求新闭包
    pub(in crate) fn closure(
        &self,
        first_set: &BTreeMap<NonTerminator, BTreeSet<TokenType>>,
        nullable_set: &BTreeSet<NonTerminator>,
        props: &Vec<Production>,
    ) -> Vec<LR1Item> {
        // 如果是非终结符，求闭包
        if let Some(ProductionRight::NonTerminator(symbol)) =
            self.production.right.get(self.location)
        {
            let first = self.get_first_set(first_set, nullable_set);
            let result = symbol.get_items_by_n(&first, props);
            result
        } else {
            vec![]
        }
    }
    /// 获取 · 后的非终结符之后的first集
    fn get_first_set(
        &self,
        first_set: &BTreeMap<NonTerminator, BTreeSet<TokenType>>,
        nullable_set: &BTreeSet<NonTerminator>,
    ) -> BTreeSet<TokenType> {
        let mut first = BTreeSet::new();
        // 模拟吃了一个符号
        let mut loc = self.location + 1;
        let size = self.production.right.len();
        while loc < size {
            let symbol = &self.production.right[loc];
            match symbol {
                // 终结符 直接是first
                ProductionRight::Terminator(t) => {
                    first.insert(t.clone());
                    return first;
                }
                ProductionRight::NonTerminator(non_t) => {
                    if let Some(s) = first_set.get(non_t) {
                        for i in s {
                            // 插入不为空的
                            if let &TokenType::Epsilon = i {
                                first.insert(i.clone());
                            }
                        }
                        // 该非终结符不在nullable内
                        if nullable_set.contains(non_t) {
                            return first;
                        }
                    }
                    loc += 1;
                }
            }
        }
        first.insert(self.next);
        first
    }
}
/// # LR1 集合
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate) struct LR1Set {
    pub(in crate::parser) items: Vec<LR1Item>,
}
impl LR1Set {
    /// 新建
    pub(in crate::parser) fn new(item: &Vec<LR1Item>) -> Self {
        Self {
            items: item.clone(),
        }
    }
    /// 求闭包
    pub(in crate) fn closure(
        &mut self,
        first_set: &BTreeMap<NonTerminator, BTreeSet<TokenType>>,
        nullable_set: &BTreeSet<NonTerminator>,
        props: &Vec<Production>,
    ) {
        let mut cnt = 0;
        loop {
            let size = self.items.len();
            for item in &self.items.clone()[cnt..] {
                // 获取闭包
                let new_items = item.closure(first_set, nullable_set, props);
                // 插入新的项目
                for item in new_items {
                    if let None = self.items.iter().position(|x| x == &item) {
                        self.items.push(item)
                    }
                }
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
        first_set: &BTreeMap<NonTerminator, BTreeSet<TokenType>>,
        nullable_set: &BTreeSet<NonTerminator>,
        props: &Vec<Production>,
    ) -> LR1Set {
        let mut dst = Self::new(&vec![]);
        // 找到 · 后字符为symbol的项目
        for item in &self.items {
            if item.location < item.production.right.len()
                && item.production.right[item.location] == symbol
            {
                match item.production.right[0] {
                    ProductionRight::Terminator(TokenType::Epsilon) => {}
                    _ => {
                        let mut new_item = item.clone();
                        new_item.location += 1;
                        dst.items.push(new_item);
                    }
                }
            }
        }
        dst.closure(first_set, nullable_set, props);
        dst
    }
}
