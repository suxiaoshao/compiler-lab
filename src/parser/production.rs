use serde::Deserialize;

use crate::parser::non_terminator::NonTerminator;
use crate::tokenizer::token::TokenType;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct Production {
    left: NonTerminator,
    right: Vec<ProductionRight>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum ProductionRight {
    NonTerminator(NonTerminator),
    Terminator(TokenType),
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
"right": ["LeftBlock","Decls","Stmts","RightBlock"]
}
]"##;
        let result: Vec<Production> = serde_json::from_str(string_s).unwrap();
        println!("{:?}", result);
    }
}
