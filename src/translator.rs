use crate::parser::parser_item::{ParserItem, ParserType};

mod attribute;
mod identifier;
mod quad;
mod symbol_table;

pub fn translator(parser_items: &Vec<ParserItem>) {
    for parser_item in parser_items {
        match parser_item.action {
            // 如果是移进
            ParserType::Shift => {}
            ParserType::Reduce(r_index) => {}
        }
    }
}
