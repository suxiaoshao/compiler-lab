use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::parser::non_terminator::NonTerminator;
use crate::parser::production::Production;
use crate::tokenizer::token_type::TokenType;

use super::production::ProductionRight;

/// 语法结构
#[derive(Debug, Clone)]
pub struct Grammar {
    /// 产生式
    productions: Vec<Production>,
    /// NULLABLE集(可能会推导出ε的非终结符)
    nullable_set: BTreeSet<NonTerminator>,
    /// first 集
    first_set: BTreeMap<NonTerminator, BTreeSet<TokenType>>,
}
impl Grammar {
    pub(in crate::parser) fn new(productions: Vec<Production>) -> Grammar {
        let nullable_set = get_nullable_set(&productions);
        let first_set = get_first_set(&productions, &nullable_set);
        Self {
            productions,
            nullable_set,
            first_set,
        }
    }
}

/// 求NULLABLE集
fn get_nullable_set(productions: &Vec<Production>) -> BTreeSet<NonTerminator> {
    let mut nullable_set = BTreeSet::new();
    loop {
        let len = nullable_set.len();
        for production in productions {
            if production.is_nullable() {
                nullable_set.insert(production.left);
            } else if production.is_next_nullable(&nullable_set) {
                nullable_set.insert(production.left);
            }
        }
        if len == nullable_set.len() {
            break;
        }
    }
    nullable_set
}
/// 初始化FIRST集
fn get_first_set(
    productions: &Vec<Production>,
    nullable_set: &BTreeSet<NonTerminator>,
) -> BTreeMap<NonTerminator, BTreeSet<TokenType>> {
    // 初始化 first集
    let mut first_set: BTreeMap<NonTerminator, BTreeSet<TokenType>> = BTreeMap::new();
    // first 插入数据
    let add = move |first_set: &mut BTreeMap<NonTerminator, BTreeSet<TokenType>>,
                    left: &NonTerminator,
                    right: &TokenType| {
        if first_set.contains_key(left) {
            let left_value = first_set.get_mut(&left).unwrap();
            left_value.insert(right.clone());
        } else {
            let mut left_value = BTreeSet::new();
            left_value.insert(right.clone());
            first_set.insert(left.clone(), left_value);
        }
    };

    // 集合联合
    let union_set = move |first_set: &mut BTreeMap<NonTerminator, BTreeSet<TokenType>>,
                          left: &NonTerminator,
                          other: &NonTerminator| {
        let other = match first_set.get(other) {
            Some(o) => o.clone(),
            None => BTreeSet::new(),
        };
        match first_set.get_mut(left) {
            Some(l) => {
                *l = l.union(&other).map(|x| x.clone()).collect();
            }
            None => {
                first_set.insert(left.clone(), other);
            }
        }
    };
    // 获取 first 集元素长度
    let get_len = |first_set: &BTreeMap<NonTerminator, BTreeSet<TokenType>>| {
        first_set.iter().fold(0, |x, (_, y)| x + y.len())
    };
    loop {
        let len = get_len(&first_set);
        for production in productions {
            for right in &production.right {
                match right {
                    // 首个符号为终结符，直接添加
                    ProductionRight::Terminator(e) => {
                        add(&mut first_set, &production.left, e);
                        break;
                    }
                    // 非终结符
                    ProductionRight::NonTerminator(e) => {
                        union_set(&mut first_set, &production.left, e);
                        if !nullable_set.contains(e) {
                            break;
                        }
                    }
                }
            }
        }
        if len == get_len(&first_set) {
            break;
        }
    }
    first_set
}
