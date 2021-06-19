use crate::parser::parser_item::{ParserItem, ParserType};
use crate::parser::production::{Production, ProductionRight};
use crate::tokenizer::position::Position;
use crate::tokenizer::token::Token;
use crate::tokenizer::token_type::TokenType;
use crate::translator::attribute::Attribute;
use crate::translator::identifier::{Identifier, IdentifierValue};
use crate::translator::quad::{DestValue, OptValue, Quad};
use crate::translator::symbol_table::{Blocks, SymbolTable};
use colored::Colorize;
use std::collections::{HashSet, LinkedList};

/// 回填
fn back_patch(p_chain: &HashSet<usize>, num: usize, quads: &mut Vec<Quad>) {
    for i in p_chain {
        if quads.len() > i - 1 {
            quads[i - 1].dest = DestValue::JumpNum(num)
        }
    }
}

/// # 语法制导翻译核心
///
///输入：产生式序号
pub(in crate::translator) fn syntax_directed(
    no: usize,
    production: &Production,
    attr_stack: &mut LinkedList<Attribute>,
    quad_no: &mut usize,
    quads: &mut Vec<Quad>,
    cur_scope: &mut i32,
    blocks: &mut Vec<SymbolTable>,
    content: &str,
    tmp_idx: &mut i32,
) {
    let mut cur_sym_table = None;
    if blocks.len() != 0 {
        let len = blocks.len();
        cur_sym_table = Some(&mut blocks[len - 1]);
    }
    // 归约后产生式左部的属性
    let mut res = Attribute::new(&Token::new(
        "".to_string(),
        TokenType::Epsilon,
        &Position::new(),
        &Position::new(),
    ));
    // 缓存一下弹出的属性
    let mut attr_cache = vec![];
    let r_size = production.right.len();
    // 处理epsilon
    if let ProductionRight::Terminator(TokenType::Epsilon) = production.right[0] {
    } else {
        // 属性栈缓存处理
        for _ in 0..r_size {
            attr_cache.push(attr_stack.pop_back().unwrap());
        }
    }
    match no {
        // Program : Block
        0 => {
            back_patch(&attr_cache[0].now_c, *quad_no, quads);
            quads.push(Quad::new(
                OptValue::End,
                DestValue::None,
                DestValue::None,
                DestValue::None,
            ));
            *quad_no += 1;
        }
        // Block : { Decls Stmts }
        1 => {
            res.now_c = attr_cache[1].now_c.clone();
        }
        // Decl : Type id ;
        4 => {
            res.value = attr_cache[2].value;
            let id = Identifier::new(attr_cache[2].value.unwrap(), cur_scope.clone());
            if !cur_sym_table
                .unwrap()
                .insert((&attr_cache[1]).token.lex.clone(), id)
            {
                // 重定义
                attr_cache[1]
                    .token
                    .show_error("multiple definition", content);
                std::process::exit(1);
            };
        }
        // Type : int
        6 => res.value = Some(IdentifierValue::Int(0)),
        // Type : real
        7 => res.value = Some(IdentifierValue::Real(0f64)),
        // Type : bool
        8 => res.value = Some(IdentifierValue::Bool(false)),
        // Stmts : Stmts M Stmt
        9 => {
            back_patch(&attr_cache[2].now_c, attr_cache[1].next_instr, quads);
            res.now_c = attr_cache[0].now_c.clone();
        }
        // Stmt : Var = Bool ;
        11 => {
            // 取消跳转指令
            *quad_no -= 2;
            quads.pop();
            quads.pop();
            let var_name = attr_cache[3].token.lex.clone();
            if let Some(t_scope) = blocks.find_id_scope(&var_name) {
                attr_cache[3].value = Some(blocks[t_scope].table.get(&var_name).unwrap().value);
                // 类型不一致出错
                if !attr_cache[1]
                    .value
                    .unwrap()
                    .check_type(&attr_cache[3].value.unwrap())
                {
                    attr_cache[2].token.show_error(
                        &format!(
                            "{:?} {:?}类型不匹配",
                            attr_cache[3].value, attr_cache[1].value
                        ),
                        content,
                    );
                    std::process::exit(1)
                };
                res.false_c = attr_cache[1].false_c.clone();
                res.true_c = attr_cache[1].true_c.clone();
                res.now_c = HashSet::new();
                quads.push(Quad::new(
                    OptValue::Assign,
                    attr_cache[1].temp_id.clone(),
                    DestValue::None,
                    DestValue::Name(attr_cache[3].token.lex.clone()),
                ));
                *quad_no += 1;
                blocks.set_value(&attr_cache[3].token.lex, attr_cache[1].value.unwrap());
            } else {
                attr_cache[3].token.show_error(
                    &format!("Undefined variant: {}", &attr_cache[3].token.lex),
                    content,
                );
                std::process::exit(1)
            }
        }
        // Stmt : if ( Bool ) M Stmt
        12 => {
            back_patch(&attr_cache[3].true_c, attr_cache[1].next_instr, quads);
            res.now_c = attr_cache[3]
                .false_c
                .union(&attr_cache[0].now_c)
                .map(|x| x.clone())
                .collect::<HashSet<_>>();
        }
        // Stmt : if ( Bool ) M Stmt HN else M Stmt
        13 => {
            back_patch(&attr_cache[7].true_c, attr_cache[5].next_instr, quads);
            back_patch(&attr_cache[7].false_c, attr_cache[1].next_instr, quads);
            let tmp_set = attr_cache[4]
                .now_c
                .union(&attr_cache[3].now_c)
                .map(|x| x.clone())
                .collect::<HashSet<_>>();
            res.now_c = tmp_set
                .union(&attr_cache[0].now_c)
                .map(|x| x.clone())
                .collect();
        }
        // Stmt : while M ( Bool ) M Stmt
        14 => {
            back_patch(&attr_cache[0].now_c, attr_cache[5].next_instr, quads);
            back_patch(&attr_cache[3].true_c, attr_cache[1].next_instr, quads);
            res.now_c = attr_cache[3].false_c.clone();
            quads.push(Quad::new(
                OptValue::Jump,
                DestValue::None,
                DestValue::None,
                DestValue::JumpNum(attr_cache[5].next_instr),
            ));
            *quad_no += 1;
        }
        // Stmt : break ;
        15 => {
            quads.push(Quad::new(
                OptValue::Jump,
                DestValue::None,
                DestValue::None,
                DestValue::JumpNum(2 + *quad_no),
            ));
            *quad_no += 1;
        }
        // Stmt : Block
        16 => {
            res.now_c = attr_cache[0].now_c.clone();
        }
        // Var : id
        18 | 33 => {
            res.value = attr_cache[0].value;
            res.temp_id = attr_cache[0].temp_id.clone();
            res.token = attr_cache[0].token.clone();
        }
        // Bool : Bool || M Join
        19 => {
            res.temp_id = DestValue::TempId(*tmp_idx);
            *tmp_idx += 1;
            if let (Some(IdentifierValue::Bool(x)), Some(IdentifierValue::Bool(y))) =
                (&attr_cache[3].value, &attr_cache[0].value)
            {
                res.value = Some(IdentifierValue::Bool(*x || *y));
            }
            back_patch(&attr_cache[3].false_c, attr_cache[1].next_instr, quads);
            res.true_c = attr_cache[3]
                .true_c
                .union(&attr_cache[0].true_c)
                .map(|x| x.clone())
                .collect();
            res.false_c = attr_cache[0].false_c.clone();
        }
        // Bool : Join
        20 | 22 => {
            res.temp_id = attr_cache[0].temp_id.clone();
            res.value = attr_cache[0].value;
            res.true_c = attr_cache[0].true_c.clone();
            res.false_c = attr_cache[0].false_c.clone();
        }
        // Join : Join && M Equality
        21 => {
            res.temp_id = DestValue::TempId(*tmp_idx);
            *tmp_idx += 1;
            if let (Some(IdentifierValue::Bool(x)), Some(IdentifierValue::Bool(y))) =
                (&attr_cache[3].value, &attr_cache[0].value)
            {
                res.value = Some(IdentifierValue::Bool(*x && *y));
            }
            back_patch(&attr_cache[3].false_c, attr_cache[1].next_instr, quads);
            res.false_c = attr_cache[3]
                .false_c
                .union(&attr_cache[0].false_c)
                .map(|x| x.clone())
                .collect();
            res.true_c = attr_cache[0].true_c.clone();
        }
        // Equality : Equality == Rel
        23 => {
            res.temp_id = DestValue::TempId(*tmp_idx);
            *tmp_idx += 1;
            quads.push(Quad::new(
                OptValue::Equal,
                attr_cache[2].temp_id.clone(),
                attr_cache[0].temp_id.clone(),
                res.temp_id.clone(),
            ));
            *quad_no += 1;
            res.value = Some(IdentifierValue::Bool(
                &attr_cache[0].value == &attr_cache[2].value,
            ))
        }
        // Equality : Equality != Rel
        24 => {
            res.temp_id = DestValue::TempId(*tmp_idx);
            *tmp_idx += 1;
            quads.push(Quad::new(
                OptValue::Equal,
                attr_cache[2].temp_id.clone(),
                attr_cache[0].temp_id.clone(),
                res.temp_id.clone(),
            ));
            *quad_no += 1;
            res.value = Some(IdentifierValue::Bool(
                &attr_cache[0].value != &attr_cache[2].value,
            ))
        }
        // Equality : Rel
        25 => {
            res.temp_id = attr_cache[0].temp_id.clone();
            res.value = attr_cache[0].value;
            res.true_c = HashSet::new();
            res.true_c.insert(*quad_no);
            res.false_c = HashSet::new();
            res.false_c.insert(*quad_no + 1);
            quads.push(Quad::new(
                OptValue::JTrue,
                attr_cache[0].temp_id.clone(),
                DestValue::None,
                DestValue::None,
            ));
            *quad_no += 1;
            quads.push(Quad::new(
                OptValue::Jump,
                DestValue::None,
                DestValue::None,
                DestValue::None,
            ));
            *quad_no += 1;
        }
        // Rel : Expr < Expr
        26 => {
            res.temp_id = DestValue::TempId(*tmp_idx);
            *tmp_idx += 1;
            quads.push(Quad::new(
                OptValue::Less,
                attr_cache[2].temp_id.clone(),
                attr_cache[0].temp_id.clone(),
                DestValue::TempId(*tmp_idx),
            ));
            *quad_no += 1;
            match (&attr_cache[2].value, &attr_cache[0].value) {
                (Some(IdentifierValue::Real(x)), Some(IdentifierValue::Real(y))) => {
                    res.value = Some(IdentifierValue::Bool(x < y))
                }
                (Some(IdentifierValue::Int(x)), Some(IdentifierValue::Int(y))) => {
                    res.value = Some(IdentifierValue::Bool(x < y))
                }
                _ => {
                    attr_cache[1].token.show_error(
                        &format!(
                            "{} {}不匹配",
                            attr_cache[0].token.lex, attr_cache[2].token.lex
                        ),
                        content,
                    );
                    std::process::exit(1)
                }
            };
        }
        // Rel : Expr <= Expr
        27 => {
            res.temp_id = DestValue::TempId(*tmp_idx);
            *tmp_idx += 1;
            quads.push(Quad::new(
                OptValue::LessEqual,
                attr_cache[2].temp_id.clone(),
                attr_cache[0].temp_id.clone(),
                DestValue::TempId(*tmp_idx),
            ));
            *quad_no += 1;
            match (&attr_cache[2].value, &attr_cache[0].value) {
                (Some(IdentifierValue::Real(x)), Some(IdentifierValue::Real(y))) => {
                    res.value = Some(IdentifierValue::Bool(x <= y))
                }
                (Some(IdentifierValue::Int(x)), Some(IdentifierValue::Int(y))) => {
                    res.value = Some(IdentifierValue::Bool(x <= y))
                }
                _ => {
                    attr_cache[1].token.show_error(
                        &format!(
                            "{} {}不匹配",
                            attr_cache[0].token.lex, attr_cache[2].token.lex
                        ),
                        content,
                    );
                    std::process::exit(1)
                }
            };
        }
        // Rel : Expr >= Expr
        28 => {
            res.temp_id = DestValue::TempId(*tmp_idx);
            *tmp_idx += 1;
            quads.push(Quad::new(
                OptValue::GreaterEqual,
                attr_cache[2].temp_id.clone(),
                attr_cache[0].temp_id.clone(),
                DestValue::TempId(*tmp_idx),
            ));
            *quad_no += 1;
            match (&attr_cache[2].value, &attr_cache[0].value) {
                (Some(IdentifierValue::Real(x)), Some(IdentifierValue::Real(y))) => {
                    res.value = Some(IdentifierValue::Bool(x >= y))
                }
                (Some(IdentifierValue::Int(x)), Some(IdentifierValue::Int(y))) => {
                    res.value = Some(IdentifierValue::Bool(x >= y))
                }
                _ => {
                    attr_cache[1].token.show_error(
                        &format!(
                            "{} {}不匹配",
                            attr_cache[0].token.lex, attr_cache[2].token.lex
                        ),
                        content,
                    );
                    std::process::exit(1)
                }
            };
        }
        // Rel : Expr > Expr
        29 => {
            res.temp_id = DestValue::TempId(*tmp_idx);
            *tmp_idx += 1;
            quads.push(Quad::new(
                OptValue::Greater,
                attr_cache[2].temp_id.clone(),
                attr_cache[0].temp_id.clone(),
                DestValue::TempId(*tmp_idx),
            ));
            *quad_no += 1;
            match (&attr_cache[2].value, &attr_cache[0].value) {
                (Some(IdentifierValue::Real(x)), Some(IdentifierValue::Real(y))) => {
                    res.value = Some(IdentifierValue::Bool(x > y))
                }
                (Some(IdentifierValue::Int(x)), Some(IdentifierValue::Int(y))) => {
                    res.value = Some(IdentifierValue::Bool(x > y))
                }
                _ => {
                    attr_cache[1].token.show_error(
                        &format!(
                            "{} {}不匹配",
                            attr_cache[0].token.lex, attr_cache[2].token.lex
                        ),
                        content,
                    );
                    std::process::exit(1)
                }
            };
        }
        // Rel : Expr
        30 | 36 | 39 => {
            res.value = attr_cache[0].value;
            res.temp_id = attr_cache[0].temp_id.clone();
        }
        // Expr : Expr + Term
        31 => {
            match (&attr_cache[2].value, &attr_cache[0].value) {
                (Some(IdentifierValue::Real(x)), Some(IdentifierValue::Real(y))) => {
                    res.value = Some(IdentifierValue::Real(x + y))
                }
                (Some(IdentifierValue::Int(x)), Some(IdentifierValue::Int(y))) => {
                    res.value = Some(IdentifierValue::Int(x + y))
                }
                _ => {
                    attr_cache[1].token.show_error(
                        &format!(
                            "{} {}不匹配",
                            attr_cache[0].token.lex, attr_cache[2].token.lex
                        ),
                        content,
                    );
                    std::process::exit(1)
                }
            };
            res.temp_id = DestValue::TempId(*tmp_idx);
            *tmp_idx += 1;
            quads.push(Quad::new(
                OptValue::Add,
                attr_cache[2].temp_id.clone(),
                attr_cache[0].temp_id.clone(),
                DestValue::TempId(*tmp_idx),
            ));
            *quad_no += 1;
        }
        // Expr : Expr - Term
        32 => {
            match (&attr_cache[2].value, &attr_cache[0].value) {
                (Some(IdentifierValue::Real(x)), Some(IdentifierValue::Real(y))) => {
                    res.value = Some(IdentifierValue::Real(x - y))
                }
                (Some(IdentifierValue::Int(x)), Some(IdentifierValue::Int(y))) => {
                    res.value = Some(IdentifierValue::Int(x - y))
                }
                _ => {
                    attr_cache[1].token.show_error(
                        &format!(
                            "{} {}不匹配",
                            attr_cache[0].token.lex, attr_cache[2].token.lex
                        ),
                        content,
                    );
                    std::process::exit(1)
                }
            };
            res.temp_id = DestValue::TempId(*tmp_idx);
            *tmp_idx += 1;
            quads.push(Quad::new(
                OptValue::Sub,
                attr_cache[2].temp_id.clone(),
                attr_cache[0].temp_id.clone(),
                DestValue::TempId(*tmp_idx),
            ));
            *quad_no += 1;
        }
        // Expr : Expr * Term
        34 => {
            match (&attr_cache[2].value, &attr_cache[0].value) {
                (Some(IdentifierValue::Real(x)), Some(IdentifierValue::Real(y))) => {
                    res.value = Some(IdentifierValue::Real(x * y))
                }
                (Some(IdentifierValue::Int(x)), Some(IdentifierValue::Int(y))) => {
                    res.value = Some(IdentifierValue::Int(x * y))
                }
                _ => {
                    attr_cache[1].token.show_error(
                        &format!(
                            "{} {}不匹配",
                            attr_cache[0].token.lex, attr_cache[2].token.lex
                        ),
                        content,
                    );
                    std::process::exit(1)
                }
            };
            res.temp_id = DestValue::TempId(*tmp_idx);
            *tmp_idx += 1;
            quads.push(Quad::new(
                OptValue::Mul,
                attr_cache[2].temp_id.clone(),
                attr_cache[0].temp_id.clone(),
                DestValue::TempId(*tmp_idx),
            ));
            *quad_no += 1;
        }
        // Expr : Expr / Term
        35 => {
            match (&attr_cache[2].value, &attr_cache[0].value) {
                (Some(IdentifierValue::Real(x)), Some(IdentifierValue::Real(y))) => {
                    res.value = Some(IdentifierValue::Real(x / y))
                }
                (Some(IdentifierValue::Int(x)), Some(IdentifierValue::Int(y))) => {
                    res.value = Some(IdentifierValue::Int(x / y))
                }
                _ => {
                    attr_cache[1].token.show_error(
                        &format!(
                            "{} {}不匹配",
                            attr_cache[0].token.lex, attr_cache[2].token.lex
                        ),
                        content,
                    );
                    std::process::exit(1)
                }
            };
            res.temp_id = DestValue::TempId(*tmp_idx);
            *tmp_idx += 1;
            quads.push(Quad::new(
                OptValue::Div,
                attr_cache[2].temp_id.clone(),
                attr_cache[0].temp_id.clone(),
                DestValue::TempId(*tmp_idx),
            ));
            *quad_no += 1;
        }
        // Unary : ! Unary
        37 => {
            res.temp_id = DestValue::TempId(*tmp_idx);
            *tmp_idx += 1;
            if let Some(IdentifierValue::Bool(x)) = attr_cache[0].value {
                res.value = Some(IdentifierValue::Bool(!x))
            }
            quads.push(Quad::new(
                OptValue::Not,
                attr_cache[0].temp_id.clone(),
                DestValue::None,
                DestValue::TempId(*tmp_idx),
            ));
            *quad_no += 1
        }
        // Unary : - Unary
        38 => {
            res.temp_id = DestValue::TempId(*tmp_idx);
            *tmp_idx += 1;
            match &attr_cache[0].value {
                Some(IdentifierValue::Real(x)) => res.value = Some(IdentifierValue::Real(-*x)),
                Some(IdentifierValue::Int(x)) => res.value = Some(IdentifierValue::Int(-*x)),
                _ => {
                    attr_cache[1].token.show_error(
                        &format!(
                            "{} {}不匹配",
                            attr_cache[0].token.lex, attr_cache[2].token.lex
                        ),
                        content,
                    );
                    std::process::exit(1)
                }
            };
            quads.push(Quad::new(
                OptValue::Sub,
                attr_cache[0].temp_id.clone(),
                DestValue::None,
                DestValue::TempId(*tmp_idx),
            ));
            *quad_no += 1
        }
        // Factor : ( Bool )
        40 => {
            res.value = attr_cache[1].value;
            res.temp_id = attr_cache[1].temp_id.clone();
        }
        // Factor : Var
        41 => {
            if let Some(_) = blocks.find_id_scope(&attr_cache[0].token.lex) {
                let var_name = &attr_cache[0].token.lex;
                if let Some(scope) = blocks.find_id_scope(var_name) {
                    let id = blocks[scope].table[var_name].clone();
                    res.temp_id = DestValue::Name(attr_cache[0].token.lex.clone());
                    res.value = Some(id.value);
                } else {
                    res.token
                        .show_error(&format!("Undefined variant {}", var_name), content);
                    std::process::exit(1)
                }
            } else {
                res.token.show_error(
                    &format!("Undefined variant {}", attr_cache[0].token.lex),
                    content,
                );
                std::process::exit(1)
            }
        }
        // Factor : int_num
        42 | 43 | 44 | 45 => {
            res.temp_id = DestValue::from(attr_cache[0].value);
            res.value = attr_cache[0].value;
        }
        46 => {
            res.now_c = HashSet::new();
            res.now_c.insert(*quad_no);
            quads.push(Quad::new(
                OptValue::Jump,
                DestValue::None,
                DestValue::None,
                DestValue::None,
            ));
            *quad_no += 1;
        }
        47 => {
            res.next_instr = *quad_no;
        }
        0..=47 => {}
        _ => {
            println!(
                "{}",
                format!("Syntax Directed Translation Err at {}!!!", no).red()
            );
            std::process::exit(1)
        }
    };
    attr_stack.push_back(res.clone());
}

pub(in crate::translator) trait Syntax {
    fn syntax(
        &self,
        blocks: &mut Vec<SymbolTable>,
        attr_stack: &mut LinkedList<Attribute>,
        quad_no: &mut usize,
        quads: &mut Vec<Quad>,
        cur_scope: &mut i32,
        tmp_idx: &mut i32,
        productions: &Vec<Production>,
        content: &str,
    ) -> ();
}

impl Syntax for ParserItem {
    fn syntax(
        &self,
        blocks: &mut Vec<SymbolTable>,
        attr_stack: &mut LinkedList<Attribute>,
        quad_no: &mut usize,
        quads: &mut Vec<Quad>,
        cur_scope: &mut i32,
        tmp_idx: &mut i32,
        productions: &Vec<Production>,
        content: &str,
    ) -> () {
        match self.action {
            ParserType::Shift => {
                self.token.syntax(
                    blocks,
                    attr_stack,
                    quad_no,
                    quads,
                    cur_scope,
                    tmp_idx,
                    productions,
                    content,
                );
            }
            ParserType::Reduce(r_size) => syntax_directed(
                r_size,
                &productions[r_size].clone(),
                attr_stack,
                quad_no,
                quads,
                cur_scope,
                blocks,
                content,
                tmp_idx,
            ),
        }
    }
}

impl Syntax for Token {
    fn syntax(
        &self,
        blocks: &mut Vec<SymbolTable>,
        attr_stack: &mut LinkedList<Attribute>,
        _quad_no: &mut usize,
        _quads: &mut Vec<Quad>,
        _cur_scope: &mut i32,
        _tmp_idx: &mut i32,
        _productions: &Vec<Production>,
        _content: &str,
    ) -> () {
        match self.token_type {
            // 整数
            TokenType::IntNum => {
                let value = self.lex.parse().unwrap();
                attr_stack.push_back(Attribute::new_value(self, IdentifierValue::Int(value)));
            }
            TokenType::Int => {
                attr_stack.push_back(Attribute::new_value(self, IdentifierValue::Int(0)));
            }
            // 实数
            TokenType::RealNum => {
                let value = self.lex.parse().unwrap();
                attr_stack.push_back(Attribute::new_value(self, IdentifierValue::Real(value)))
            }
            TokenType::Real => {
                attr_stack.push_back(Attribute::new_value(self, IdentifierValue::Real(0.0)));
            }
            // true
            TokenType::True => {
                attr_stack.push_back(Attribute::new_value(self, IdentifierValue::Bool(true)))
            }
            // false
            TokenType::False => {
                attr_stack.push_back(Attribute::new_value(self, IdentifierValue::Bool(false)))
            }
            TokenType::Bool => {
                attr_stack.push_back(Attribute::new_value(self, IdentifierValue::Bool(false)));
            }
            // 左大括号
            TokenType::LeftBlock => {
                blocks.push(SymbolTable::new());
                attr_stack.push_back(Attribute::new(self))
            }
            // 右大括号
            TokenType::RightBlock => {
                blocks.pop();
                attr_stack.push_back(Attribute::new(self))
            }
            _ => attr_stack.push_back(Attribute::new(self)),
        }
    }
}
