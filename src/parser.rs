use crate::parser::grammar::Grammar;
use crate::parser::production::Production;

mod canonical_collection;
mod grammar;
mod lr1;
mod non_terminator;
mod production;

pub fn parser(parser_content: &str) {
    println!("解析 grammar 文件\n");
    let productions: Vec<Production> =
        serde_json::from_str(parser_content).expect("文法文件解析出错");
    productions
        .iter()
        .for_each(|i| println!("{}", i.show_string()));
    let grammar = Grammar::new(productions);
    grammar.show();
    let cc = grammar.dfa();
    println!("{:?}", cc.graph.len());
}
