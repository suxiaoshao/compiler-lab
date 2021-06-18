use std::time::SystemTime;

mod args_reader;
mod parser;
mod tokenizer;
mod translator;

/// 清空控制台
fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn main() {
    let times = std::time::SystemTime::now();
    let (code_content, parser_content) = args_reader::read_code_parser();
    let tokens = tokenizer::get_tokens_from_string(&code_content);
    tokens.iter().for_each(|x| {
        println!("{}", x.show_string());
    });
    let (parser_items, productions) = parser::parser(&parser_content, &tokens);
    translator::translator(&parser_items, &productions, &code_content);
    println!(
        "{:?}",
        SystemTime::now().duration_since(times).unwrap().as_millis(),
    );
}
