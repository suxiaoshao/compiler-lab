use std::collections::HashSet;

use colored::{ColoredString, Colorize};
use serde::Deserialize;

use crate::parser::non_terminator::NonTerminator;
use crate::tokenizer::token_type::TokenType;

#[derive(Clone, Debug, Deserialize, Hash)]
pub struct Production {
    pub(in crate::parser) left: NonTerminator,
    pub(in crate::parser) right: Vec<ProductionRight>,
}

impl PartialEq for Production {
    fn eq(&self, other: &Self) -> bool {
        if self.left != other.left {
            return false;
        };
        self.right == other.right
    }
}
impl Eq for Production {}

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
        nullable_set: &HashSet<NonTerminator>,
    ) -> bool {
        self.right.iter().all(|item| match item {
            ProductionRight::NonTerminator(non) => nullable_set.contains(non),
            _ => false,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Hash)]
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
    use crate::{
        parser::{
            non_terminator::NonTerminator,
            production::{Production, ProductionRight},
        },
        tokenizer::token_type::TokenType,
    };

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
    #[test]
    fn eq() {
        let left = Production {
            left: crate::parser::non_terminator::NonTerminator::Block,
            right: vec![
                ProductionRight::NonTerminator(NonTerminator::Factor),
                ProductionRight::Terminator(TokenType::Assign),
            ],
        };
        let right = Production {
            left: crate::parser::non_terminator::NonTerminator::Block,
            right: vec![
                ProductionRight::NonTerminator(NonTerminator::Factor),
                ProductionRight::Terminator(TokenType::Assign),
            ],
        };
        assert_eq!(left, right);
        let right = Production {
            left: crate::parser::non_terminator::NonTerminator::Block,
            right: vec![
                ProductionRight::NonTerminator(NonTerminator::Factor),
                ProductionRight::Terminator(TokenType::Assign),
                ProductionRight::NonTerminator(NonTerminator::Program),
            ],
        };
        assert_ne!(left, right);
    }
}
