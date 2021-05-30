use std::collections::BTreeSet;

use colored::{ColoredString, Colorize};
use serde::Deserialize;

use crate::parser::non_terminator::NonTerminator;
use crate::tokenizer::token_type::TokenType;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Ord, PartialOrd)]
pub struct Production {
    pub(in crate::parser) left: NonTerminator,
    pub(in crate::parser) right: Vec<ProductionRight>,
}

impl Production {
    /// 返回显示的字符串
    pub fn show_string(&self) -> String {
        let right = self
            .right
            .iter()
            .fold(String::new(), |x, y| format!("{} {}", x, y.show_string()));
        format!("{} -> {}", format!("{:?}", self.left).yellow(), right)
    }
    pub(in crate::parser) fn is_nullable(&self) -> bool {
        self.right.len() == 1
            && match self.right[0] {
                ProductionRight::Terminator(TokenType::Epsilon) => true,
                _ => false,
            }
    }
    pub(in crate::parser) fn is_next_nullable(
        &self,
        nullable_set: &BTreeSet<NonTerminator>,
    ) -> bool {
        self.right.iter().all(|item| match item {
            ProductionRight::NonTerminator(non) => nullable_set.contains(non),
            _ => false,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Ord, PartialOrd)]
#[serde(untagged)]
pub enum ProductionRight {
    NonTerminator(NonTerminator),
    Terminator(TokenType),
}
impl ProductionRight {
    fn show_string(&self) -> ColoredString {
        match self {
            ProductionRight::NonTerminator(e) => format!("{:?}", e).yellow(),
            ProductionRight::Terminator(e) => format!("{}", e.show_string()).cyan(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parser::production::Production;

    #[test]
    fn test() {
        let string_s = r##"[
{
  "left": "Program",
  "right": ["Block"]
},
{
"left": "Block",
"right": ["{","Decls","Stmts","}"]
}
]"##;
        let result: Vec<Production> = serde_json::from_str(string_s).unwrap();
        println!("{:?}", result);
    }
}
