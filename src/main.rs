mod tokenizer;

use std::env;
use colored::Colorize;
use std::fs;


/// 清空控制台
fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

/// 读取文件
fn read_file_chars() -> String {
    // 读取控制台输入
    let args: Vec<String> = env::args().collect();
    let file_name = match args.get(1) {
        None => {
            println!("{}", "未输入文件".red());
            std::process::exit(1);
        }
        Some(v) => v
    };

    // 读取文件
    clear();
    let file_name = std::path::Path::new(file_name);
    let file_display = file_name.canonicalize().unwrap().to_str().unwrap().green();
    println!("开始读取文件 {}", file_display);
    let file = match fs::read(file_name) {
        Ok(f) => f,
        Err(err) => {
            println!("{}", format!("读取文件失败 : {}", err).red());
            std::process::exit(1);
        }
    };

    // 读取文件内容
    println!("开始读取文件内容");
    match String::from_utf8(file) {
        Ok(s) => {
            println!("读取 {} 文件完成", file_display);
            s
        }
        Err(err) => {
            println!("{}", format!("解析文件内容失败 : {}", err).red());
            std::process::exit(1);
        }
    }
}

fn main() {
    let content=read_file_chars();
    let tokens= tokenizer::get_tokens_from_string(content);
    tokens.iter().for_each(|x|{
        println!("{}",x.display());
    })
}
