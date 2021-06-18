use crate::translator::identifier::Identifier;
use std::collections::HashMap;

/// # 符号表
///
/// 一个block对应一张表=>作用域
///
/// 编译过程中使用的所有符号表都保存在 vector<SymbolTable> 中
///
/// 每进入一个块 (读入 { )，都要新增一个符号表用来存储该块内的变量信息
///
/// 每退出一个块 (读入 } )，都要删除该块对应的符号表
struct SymbolTable {
    /// 通过变量名找到对应的变量
    table: HashMap<String, Identifier>,
}
