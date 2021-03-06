use crate::translator::identifier::IdentifierValue;
use colored::{ColoredString, Colorize};
#[derive(Debug, Clone)]
pub(in crate::translator) enum DestValue {
    None,
    JumpNum(usize),
    TempId(i32),
    Name(String),
    Int(i32),
    Real(f64),
    Bool(bool),
}

impl From<Option<IdentifierValue>> for DestValue {
    fn from(other: Option<IdentifierValue>) -> Self {
        match other {
            Some(IdentifierValue::Real(x)) => DestValue::Real(x),
            Some(IdentifierValue::Int(x)) => DestValue::Int(x),
            Some(IdentifierValue::Bool(x)) => DestValue::Bool(x),
            _ => DestValue::None,
        }
    }
}

impl DestValue {
    fn show_string(&self) -> ColoredString {
        match self {
            DestValue::None => "_".to_string().truecolor(255, 152, 0),
            DestValue::JumpNum(num) => format!("jump({})", num).yellow(),
            DestValue::TempId(id) => format!("temp_{}", id).yellow(),
            DestValue::Name(name) => format!("Var({})", name).yellow(),
            DestValue::Int(x) => format!("{}", x).blue(),
            DestValue::Real(x) => format!("{}", x).blue(),
            DestValue::Bool(x) => format!("{}", x).blue(),
        }
    }
}

#[derive(Debug)]
pub(in crate::translator) enum OptValue {
    Jump,
    Assign,
    Equal,
    JTrue,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Add,
    Sub,
    Mul,
    Div,
    Not,
    End,
}

impl OptValue {
    fn show_string(&self) -> ColoredString {
        match self {
            OptValue::Jump => "Jump",
            OptValue::Assign => "=",
            OptValue::Equal => "==",
            OptValue::JTrue => "JTrue",
            OptValue::Less => "<",
            OptValue::LessEqual => "<=",
            OptValue::Greater => ">",
            OptValue::GreaterEqual => ">=",
            OptValue::Add => "+",
            OptValue::Sub => "-",
            OptValue::Mul => "*",
            OptValue::Div => "/",
            OptValue::Not => "!",
            &OptValue::End => "End",
        }
        .to_string()
        .cyan()
    }
}

#[derive(Debug)]
pub(in crate::translator) struct Quad {
    /// ??????
    pub(in crate::translator) opt: OptValue,
    /// ???????????????
    pub(in crate::translator) lhs: DestValue,
    /// ???????????????
    pub(in crate::translator) rhs: DestValue,
    /// ?????????
    pub(in crate::translator) dest: DestValue,
}

impl Quad {
    pub(in crate::translator) fn new(
        opt: OptValue,
        lhs: DestValue,
        rhs: DestValue,
        dest: DestValue,
    ) -> Self {
        Self {
            opt,
            lhs,
            rhs,
            dest,
        }
    }
    pub fn show_string(&self, index: usize) {
        println!(
            "{} <{},{},{},{}>",
            index + 1,
            self.opt.show_string(),
            self.lhs.show_string(),
            self.rhs.show_string(),
            self.dest.show_string()
        )
    }
}
