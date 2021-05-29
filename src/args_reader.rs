use std::fs;

use clap::{App, Arg};
use colored::Colorize;

/// 解析命令行获取待编译的源文件 和 parser语法文件
fn get_code_and_parser_file() -> (String, String) {
    let matches = App::new("c-like compile")
        .version("0.0.5")
        .author("sushao <https://github.com/suxiaoshao>")
        .about("一个简单的 c-like 语言编译器前端")
        .arg(
            Arg::with_name("code")
                .value_name("FILE")
                .help("待编译的源文件")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("parser")
                .short("p")
                .long("parser")
                .help("文法的配置 json 文件")
                .value_name("FILE")
                .takes_value(true),
        )
        .get_matches();
    let code_file = matches.value_of("code").unwrap();
    let parser_file = matches.value_of("parser").unwrap_or("./grammar.json");
    (code_file.to_string(), parser_file.to_string())
}

/// # 读取解析文件和语法文件
pub fn read_code_parser() -> (String, String) {
    // 读取控制台输入
    let (code_file, parser_file) = get_code_and_parser_file();

    // 读取文件
    crate::clear();
    (read_file(&code_file), read_file(&parser_file))
}

/// 读取文件
fn read_file(file_name: &str) -> String {
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
