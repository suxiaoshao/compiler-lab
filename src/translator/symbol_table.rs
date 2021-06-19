use crate::translator::identifier::{Identifier, IdentifierValue};
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
#[derive(Clone, Debug)]
pub(in crate::translator) struct SymbolTable {
    /// 通过变量名找到对应的变量
    pub(in crate::translator) table: HashMap<String, Identifier>,
}

impl SymbolTable {
    pub(in crate::translator) fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }
    /// 插入
    pub(in crate::translator) fn insert(&mut self, key: String, value: Identifier) -> bool {
        if self.table.contains_key(&key) {
            false
        } else {
            self.table.insert(key, value);
            true
        }
    }
}

pub(in crate::translator) trait Blocks {
    /// 找到变量所在作用域（从近到远），返回scope
    fn find_id_scope(&self, id_name: &str) -> Option<usize>;
    fn set_value(&mut self, name: &str, value: IdentifierValue);
}

impl Blocks for Vec<SymbolTable> {
    fn find_id_scope(&self, id_name: &str) -> Option<usize> {
        self.iter().position(|x| x.table.contains_key(id_name))
    }

    fn set_value(&mut self, name: &str, value: IdentifierValue) {
        self.iter_mut()
            .enumerate()
            .rfind(|(_, x)| x.table.contains_key(name))
            .map(|(index, x)| x.insert(name.to_string(), Identifier::new(value, index as i32)));
    }
}
