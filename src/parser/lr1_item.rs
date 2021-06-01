use std::collections::HashMap;
use std::collections::HashSet;

use crate::parser::production::Production;
use crate::tokenizer::token_type::TokenType;

use super::non_terminator::NonTerminator;
use super::production::ProductionRight;

/// # LR1项目
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub(in crate::parser) struct LR1Item {
    /// 产生式
    pub(in crate::parser) production: Production,
    /// 点的位置
    pub(in crate::parser) location: usize,
    // 向前看符号
    pub(in crate::parser) next: TokenType,
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
        first_set: &HashMap<NonTerminator, HashSet<TokenType>>,
        nullable_set: &HashSet<NonTerminator>,
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
        first_set: &HashMap<NonTerminator, HashSet<TokenType>>,
        nullable_set: &HashSet<NonTerminator>,
    ) -> HashSet<TokenType> {
        let mut first = HashSet::new();
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
                    let s = &first_set[non_t];
                    // 插入不为空的
                    s.iter()
                        .filter(|i| *i != &TokenType::Epsilon)
                        .for_each(|i| {
                            first.insert(i.clone());
                        });
                    // 该非终结符不在nullable内
                    if !nullable_set.contains(non_t) {
                        return first;
                    }
                    loc += 1;
                }
            }
        }
        first.insert(self.next);
        first
    }
    /// 判断是否移进
    pub(in crate) fn is_move(&self) -> bool {
        if self.location < self.production.right.len() {
            match self.production.right[0] {
                ProductionRight::Terminator(TokenType::Epsilon) => false,
                _ => true,
            }
        } else {
            false
        }
    }
}
