mod args_reader;
mod parser;
mod tokenizer;

/// 清空控制台
fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn main() {
    let (code_content, parser_content) = args_reader::read_code_parser();
    let tokens = tokenizer::get_tokens_from_string(&code_content);
    tokens.iter().for_each(|x| {
        println!("{}", x.show_string());
    });
    parser::parser(&parser_content);
}
