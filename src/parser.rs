use std::collections::LinkedList;

use colored::Colorize;

use crate::parser::goto::Goto;
use crate::parser::grammar::Grammar;
use crate::parser::production::Production;
use crate::parser::production::ProductionRight;
use crate::tokenizer::position::Position;
use crate::tokenizer::token::Token;
use crate::tokenizer::token_type::TokenType;

use self::action::Action;

mod action;
mod canonical_collection;
mod goto;
mod grammar;
mod lr1_item;
mod lr1_item_set;
mod non_terminator;
mod production;

pub fn parser(parser_content: &str, tokens: &Vec<Token>) {
    println!("解析 grammar 文件\n");
    let productions: Vec<Production> =
        serde_json::from_str(parser_content).expect("文法文件解析出错");
    productions
        .iter()
        .for_each(|i| println!("{}", i.show_string()));
    let grammar = Grammar::new(productions);
    grammar.show();
    println!("\n 开始构造 DFA");
    let cc = grammar.dfa();
    println!(
        "DFA's size is {}\n",
        format!("{}", cc.item_sets.len()).green()
    );
    println!("构造预测分析表");
    let (action, goto) = cc.build_predict_table(&grammar.productions);
    println!("语法分析（使用分析栈）");
    syntax_parser(tokens, &action, &grammar.productions, &goto);
}
/// 语法分析（使用分析栈）
fn syntax_parser(tokens: &Vec<Token>, action: &Action, prods: &Vec<Production>, goto: &Goto) {
    let token = Token::new(
        "$".to_string(),
        TokenType::Eof,
        &Position::new_(0, 0),
        &Position::new_(0, 0),
    );
    let mut tokens = tokens.clone();
    tokens.push(token);
    // 分析栈
    let mut a_stack = LinkedList::new();
    a_stack.push_back((0, ProductionRight::Terminator(TokenType::Eof)));
    let mut ip = 0;
    let mut step = 0;
    while let Some(item) = a_stack.back().clone() {
        let (top_state, ..) = item;
        let cur_token = &tokens[ip];
        let symbol = cur_token.token_type;
        match action.0[top_state.clone()][&symbol] {
            action::ActionType::Accept => {
                println!("{}", "success".green());
                break;
            }
            // 移进
            action::ActionType::Shift(e) => {
                a_stack.push_back((e, ProductionRight::Terminator(symbol)));
                step += 1;
                println!(
                    "{} {} {}",
                    step,
                    "Shift".bright_blue(),
                    symbol.show_string().cyan()
                );
                ip += 1;
            }
            // 规约
            action::ActionType::Reduce(r_index) => {
                let p = &prods[r_index];
                // 弹出产生式(除了A -> ε)
                if let ProductionRight::Terminator(TokenType::Epsilon) = p.right[0] {
                } else {
                    for _ in 0..p.right.len() {
                        a_stack.pop_back();
                    }
                }
                step += 1;
                println!(
                    "{} {} {}",
                    step,
                    "Reduce".truecolor(255, 152, 0),
                    p.show_string()
                );
                let (top_state, ..) = a_stack.back().unwrap();
                let goto_map = &goto.0[top_state.clone()];
                a_stack.push_back((
                    goto_map[&p.left].unwrap(),
                    ProductionRight::NonTerminator(p.left),
                ));
            }
            action::ActionType::Error => {
                println!("{}", "error".red());
                break;
            }
        }
    }
}
