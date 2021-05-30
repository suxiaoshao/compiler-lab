use std::collections::BTreeMap;
use std::collections::BTreeSet;

use colored::Colorize;

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
    pub fn show(&self) {
        println!("\nNULLABLE集");
        let nullable_str = self
            .nullable_set
            .iter()
            .fold(String::new(), |x, y| format!("{} {:?}", x, y));
        println!("{{{} }}", nullable_str.yellow());
        println!("\nfirst 集");
        self.first_set.iter().for_each(|(x, y)| {
            let right = y
                .iter()
                .fold(String::new(), |x, y| format!("{} {}", x, y.show_string()));
            println!("{} {{{} }}", format!("{:?}", x).yellow(), right.cyan())
        })
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

#[cfg(test)]
mod test {
    use crate::parser::grammar::Grammar;
    use crate::parser::non_terminator::NonTerminator::{Decls, Equality, Program, Stmts};
    use crate::parser::production::Production;
    use crate::tokenizer::token_type::TokenType::{Eof, IntNum};

    static GRAMMAR_STR: &'static str = r#"[{"left":"Program","right":["Block"]},{"left":"Block","right":["{","Decls","Stmts","}"]},{"left":"Decls","right":["Decls","Decl"]},{"left":"Decls","right":["ε"]},{"left":"Decl","right":["Type","id",";"]},{"left":"Type","right":["Type","[","int_num","]"]},{"left":"Type","right":["Type","[","real_num","]"]},{"left":"Type","right":["int"]},{"left":"Type","right":["real"]},{"left":"Type","right":["bool"]},{"left":"Stmts","right":["Stmts","Stmt"]},{"left":"Stmts","right":["ε"]},{"left":"Stmt","right":["Var","=","Bool"]},{"left":"Stmt","right":["if","(","Bool",")","Stmt"]},{"left":"Stmt","right":["if","(","Bool",")","Stmt","else","Stmt"]},{"left":"Stmt","right":["while","(","Bool",")","Stmt"]},{"left":"Stmt","right":["break",";"]},{"left":"Stmt","right":["Block"]},{"left":"Var","right":["Var","[","int_num","]"]},{"left":"Var","right":["Var","[","real_num","]"]},{"left":"Var","right":["id"]},{"left":"Bool","right":["Bool","||","Join"]},{"left":"Bool","right":["Join"]},{"left":"Bool","right":["Join","&&","Equality"]},{"left":"Bool","right":["Equality"]},{"left":"Equality","right":["Equality","==","Rel"]},{"left":"Equality","right":["Equality","!=","Rel"]},{"left":"Equality","right":["Rel"]},{"left":"Rel","right":["Expr","<","Expr"]},{"left":"Rel","right":["Expr","<=","Expr"]},{"left":"Rel","right":["Expr",">=","Expr"]},{"left":"Rel","right":["Expr",">","Expr"]},{"left":"Rel","right":["Expr"]},{"left":"Expr","right":["Expr","+","Term"]},{"left":"Expr","right":["Expr","-","Term"]},{"left":"Expr","right":["Term"]},{"left":"Term","right":["Term","*","Unary"]},{"left":"Term","right":["Term","/","Unary"]},{"left":"Term","right":["Unary"]},{"left":"Unary","right":["!","Unary"]},{"left":"Unary","right":["-","Unary"]},{"left":"Unary","right":["Factor"]},{"left":"Factor","right":["(","Bool",")"]},{"left":"Factor","right":["Var"]},{"left":"Factor","right":["int_num"]},{"left":"Factor","right":["real_num"]},{"left":"Factor","right":["true"]},{"left":"Factor","right":["false"]}]"#;
    /// 测试 nullable获取
    #[test]
    fn test_grammar() {
        let productions: Vec<Production> = serde_json::from_str(GRAMMAR_STR).unwrap();
        let grammar = Grammar::new(productions);

        // nullable_set 的测试
        assert_eq!(grammar.nullable_set.len(), 2);
        assert_eq!(grammar.nullable_set.get(&Decls), Some(&Decls));
        assert_eq!(grammar.nullable_set.get(&Stmts), Some(&Stmts));

        // first_set 测试
        assert_eq!(grammar.first_set.get(&Program).unwrap().len(), 1);
        assert_eq!(grammar.first_set.get(&Decls).unwrap().len(), 4);
        let equality_set = grammar.first_set.get(&Equality).unwrap();
        assert_eq!(equality_set.len(), 8);
        assert_eq!(equality_set.contains(&IntNum), true);
        assert!(!equality_set.contains(&Eof));
    }
}
