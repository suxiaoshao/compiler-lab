use crate::parser::lr1::LR1Set;

use super::production::ProductionRight;

/// LR1项目集规范族
#[derive(Debug)]
pub(in crate) struct CanonicalCollection {
    /// 项目集集合
    pub(in crate::parser) item_sets: Vec<LR1Set>,
    /// # 保存DFA的图
    /// []为当前状态序号
    ///
    /// `first` 是经什么转移（即吃掉的符号）
    ///
    /// `second` 为转移到的状态序号
    pub(in crate::parser) graph: Vec<Vec<(ProductionRight, usize)>>,
}
impl CanonicalCollection {
    /// 新建一个空
    pub(in crate::parser) fn new() -> Self {
        Self {
            item_sets: vec![],
            graph: vec![],
        }
    }
    /// 判断是否在项目集规范族中，若在返回序号
    pub(in crate::parser) fn find_index(&self, is: &LR1Set) -> Option<usize> {
        self.item_sets.iter().position(|item| item == is)
    }
    /// 在指定位置添加数据
    pub(in crate::parser) fn graph_push_back(
        &mut self,
        item: (ProductionRight, usize),
        index: usize,
    ) {
        // 添加到需要的大小
        while self.graph.len() <= index {
            self.graph.push(vec![]);
        }
        self.graph[index].push(item);
    }
}
