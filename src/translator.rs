use crate::parser::parser_item::ParserItem;
use crate::parser::production::Production;
use crate::translator::syntax::Syntax;
use std::collections::LinkedList;

mod attribute;
mod identifier;
mod quad;
mod symbol_table;
mod syntax;

pub fn translator(parser_items: &Vec<ParserItem>, productions: &Vec<Production>, content: &str) {
    let mut blocks = vec![];
    let mut attr_stack = LinkedList::new();
    let mut quad_no = 1;
    let mut quads = vec![];
    let mut cur_scope = 0;
    let mut tmp_idx = 0;
    for parser_item in parser_items {
        parser_item.syntax(
            &mut blocks,
            &mut attr_stack,
            &mut quad_no,
            &mut quads,
            &mut cur_scope,
            &mut tmp_idx,
            productions,
            content,
        );
    }
    syntax::syntax_directed(
        0,
        &productions[0],
        &mut attr_stack,
        &mut quad_no,
        &mut quads,
        &mut cur_scope,
        &mut blocks,
        content,
        &mut tmp_idx,
    );
    quads
        .iter()
        .enumerate()
        .for_each(|(index, x)| x.show_string(index))
}
