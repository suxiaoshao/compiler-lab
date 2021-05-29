use crate::parser::grammar::Grammar;
use crate::parser::production::Production;

mod grammar;
mod non_terminator;
mod production;

pub fn parser(parser_content: &str) {
    let productions: Vec<Production> =
        serde_json::from_str(parser_content).expect("文法文件解析出错");
    productions
        .iter()
        .for_each(|i| println!("{}", i.show_string()));
    let grammar = Grammar::new(productions);
    println!("{:?}", grammar);
}
