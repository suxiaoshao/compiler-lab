use colored::Colorize;

/// token 的 类型
#[derive(Clone, Copy, Debug)]
enum TokenType {
    Unknown,
    // 未知类型
    Int,
    IntNum,
    Real,
    // 实数
    RealNum,
    Bool,
    True,
    False,
    If,
    Else,
    For,
    Return,
    Break,
    Id,
    //标识符
    Add,
    Sub,
    Mul,
    Mod,
    Div,
    Assign,
    Equal,
    NotEqual,
    And,
    Or,
    Not,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Comma,
    // ,
    Semicolon,
    // ;
    LeftBlock,
    RightBlock,
    SqLeftBracket,
    // [
    SqRightBracket,
    // ]
    CirLeftBracket,
    // (
    CirRightBracket, // )
}

#[derive(Clone, Debug)]
pub struct Token {
    lex: String,
    token_type: TokenType,
}


impl Token {
    /// 新建一个 token
    fn new(lex: String, token_type: TokenType) -> Self {
        Self { lex, token_type }
    }
    pub fn display(&self) -> String {
        format!("<{} {}>", format!("'{}'", self.lex).green(), format!("{:?}", self.token_type).yellow())
    }
}

#[derive(Clone)]
enum ReadTokenState {
    Empty,
    Less,
    More,
    Equal,
    Exclamation,
    And,
    Or,
    IntValue(String),
    RealValue(String),
    Bool(String),
    Id(String),
    Break(String),
    Int(String),
    If(String),
    Else(String),
    False(String),
    For(String),
    True(String),
    Real(String),
    Return(String),
}

impl ReadTokenState {
    fn get_token_empty(&mut self, s: char) -> Option<Token> {
        let string = s.to_string();
        match s {
            ' ' | '\n' | '\r' => None,
            '+' => Some(Token::new(string, TokenType::Add)),
            '*' => Some(Token::new(string, TokenType::Mul)),
            '-' => Some(Token::new(string, TokenType::Sub)),
            '/' => Some(Token::new(string, TokenType::Div)),
            '(' => Some(Token::new(string, TokenType::CirLeftBracket)),
            ')' => Some(Token::new(string, TokenType::CirRightBracket)),
            '[' => Some(Token::new(string, TokenType::SqLeftBracket)),
            ']' => Some(Token::new(string, TokenType::SqRightBracket)),
            '{' => Some(Token::new(string, TokenType::LeftBlock)),
            '}' => Some(Token::new(string, TokenType::RightBlock)),
            '%' => Some(Token::new(string, TokenType::Mod)),
            '<' => {
                *self = ReadTokenState::Less;
                None
            }
            '>' => {
                *self = ReadTokenState::More;
                None
            }
            '=' => {
                *self = ReadTokenState::Equal;
                None
            }
            '!' => {
                *self = ReadTokenState::Exclamation;
                None
            }
            '&' => {
                *self = ReadTokenState::And;
                None
            }
            '|' => {
                *self = ReadTokenState::Or;
                None
            }
            ';' => Some(Token::new(string, TokenType::Semicolon)),
            ',' => Some(Token::new(string, TokenType::Comma)),
            '0'..='9' => {
                *self = ReadTokenState::IntValue(string);
                None
            }
            'b' => {
                *self = ReadTokenState::Bool(string);
                None
            }
            'i' => {
                *self = ReadTokenState::Int(string);
                None
            }
            'e' => {
                *self = ReadTokenState::Else(string);
                None
            }
            'f' => {
                *self = ReadTokenState::False(string);
                None
            }
            't' => {
                *self = ReadTokenState::True(string);
                None
            }
            'r' => {
                *self = ReadTokenState::Real(string);
                None
            }
            _ => {
                *self = ReadTokenState::Id(string);
                None
            }
        }
    }
    fn get_token_less(&mut self, s: char) -> Vec<Token> {
        match s {
            ' ' | '\n' | '\r' => {
                vec![]
            }
            '=' => {
                *self = ReadTokenState::Empty;
                vec![(Token::new("<=".to_string(), TokenType::LessEqual))]
            }
            _ => {
                *self = ReadTokenState::Empty;
                let mut tokens = vec![Token::new("<".to_string(), TokenType::Less)];
                if let Some(token) = self.get_token_empty(s) {
                    tokens.push(token);
                };
                tokens
            }
        }
    }
    fn get_tokens_more(&mut self, s: char) -> Vec<Token> {
        match s {
            ' ' | '\n' | '\r' => {
                vec![]
            }
            '=' => {
                *self = ReadTokenState::Empty;
                vec![Token::new(">=".to_string(), TokenType::GreaterEqual)]
            }
            _ => {
                let mut tokens = vec![];
                tokens.push(Token::new(">".to_string(), TokenType::Greater));
                *self = ReadTokenState::Empty;
                if let Some(token) = self.get_token_empty(s) {
                    tokens.push(token);
                };
                tokens
            }
        }
    }
    fn get_tokens_equal(&mut self, s: char) -> Vec<Token> {
        match s {
            ' ' | '\n' | '\r' => {
                vec![]
            }
            '=' => {
                let mut tokens = vec![];
                tokens.push(Token::new("==".to_string(), TokenType::Equal));
                *self = ReadTokenState::Empty;
                tokens
            }
            _ => {
                let mut tokens = vec![];
                tokens.push(Token::new("=".to_string(), TokenType::Assign));
                *self = ReadTokenState::Empty;
                if let Some(token) = self.get_token_empty(s) {
                    tokens.push(token);
                };
                tokens
            }
        }
    }
    fn get_tokens_exclamation(&mut self, s: char) -> Vec<Token> {
        match s {
            ' ' | '\n' | '\r' => {
                vec![]
            }
            '=' => {
                let mut tokens = vec![];
                tokens.push(Token::new("!=".to_string(), TokenType::NotEqual));
                *self = ReadTokenState::Empty;
                tokens
            }
            _ => {
                let mut tokens = vec![];
                tokens.push(Token::new("!".to_string(), TokenType::Not));
                *self = ReadTokenState::Empty;
                if let Some(token) = self.get_token_empty(s) {
                    tokens.push(token);
                };
                tokens
            }
        }
    }
}

pub fn get_tokens_from_string(content: String) -> Vec<Token> {
    // 字符向量
    let mut chars = content.chars();
    // 结果
    let mut tokens = vec![];
    let mut read_token_state = ReadTokenState::Empty;
    while let Some(s) = chars.next() {
        // 空状态
        match read_token_state.clone() {
            // 空开始
            ReadTokenState::Empty => {
                if let Some(token) = read_token_state.get_token_empty(s) {
                    tokens.push(token);
                };
            }
            // 小于号
            ReadTokenState::Less => {
                read_token_state.get_token_less(s).iter().for_each(|token| {
                    tokens.push(token.clone());
                });
            }
            ReadTokenState::More => {
                read_token_state
                    .get_tokens_more(s)
                    .iter()
                    .for_each(|token| {
                        tokens.push(token.clone());
                    });
            }
            ReadTokenState::Equal => {
                read_token_state
                    .get_tokens_equal(s)
                    .iter()
                    .for_each(|token| {
                        tokens.push(token.clone());
                    });
            }
            ReadTokenState::Exclamation => {
                read_token_state
                    .get_tokens_exclamation(s)
                    .iter()
                    .for_each(|token| {
                        tokens.push(token.clone());
                    });
            }
            ReadTokenState::And => match s {
                ' ' | '\n' | '\r' => {}
                '&' => {
                    tokens.push(Token::new("&&".to_string(), TokenType::And));
                    read_token_state = ReadTokenState::Empty;
                }
                _ => {
                    tokens.push(Token::new("&".to_string(), TokenType::Unknown));
                    read_token_state = ReadTokenState::Empty;
                    if let Some(token) = read_token_state.get_token_empty(s) {
                        tokens.push(token);
                    };
                }
            },
            ReadTokenState::Or => match s {
                ' ' | '\n' | '\r' => {}
                '|' => {
                    tokens.push(Token::new("||".to_string(), TokenType::Or));
                    read_token_state = ReadTokenState::Empty;
                }
                _ => {
                    tokens.push(Token::new("|".to_string(), TokenType::Unknown));
                    read_token_state = ReadTokenState::Empty;
                    if let Some(token) = read_token_state.get_token_empty(s) {
                        tokens.push(token);
                    };
                }
            },
            ReadTokenState::IntValue(nums) => match s {
                '0'..='9' => {
                    read_token_state = ReadTokenState::IntValue(nums + &*s.to_string());
                }
                '.' => {
                    read_token_state = ReadTokenState::RealValue(nums + &*s.to_string());
                }
                _ => {
                    tokens.push(Token::new(nums, TokenType::IntNum));
                    read_token_state = ReadTokenState::Empty;
                    if let Some(token) = read_token_state.get_token_empty(s) {
                        tokens.push(token);
                    };
                }
            },
            ReadTokenState::RealValue(nums) => match s {
                '0'..='9' => read_token_state = ReadTokenState::RealValue(nums + &*s.to_string()),
                _ => {
                    tokens.push(Token::new(nums, TokenType::RealNum));
                    read_token_state = ReadTokenState::Empty;
                    if let Some(token) = read_token_state.get_token_empty(s) {
                        tokens.push(token);
                    };
                }
            },
            ReadTokenState::Bool(nums) => match s {
                ' ' | '\n' | '\r' => {
                    if nums.len() <= 3 {
                        tokens.push(Token::new(nums, TokenType::Id));
                        read_token_state = ReadTokenState::Empty;
                    } else {
                        tokens.push(Token::new(nums, TokenType::Bool));
                        read_token_state = ReadTokenState::Empty;
                    }
                }
                '%' | '+' | '*' | '-' | '/' | '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' | '=' | '!' | '&' | '|' | ';' | ',' | '0'..='9' => {
                    if nums.len() <= 3 {
                        tokens.push(Token::new(nums, TokenType::Id));
                        read_token_state = ReadTokenState::Empty;
                    } else {
                        tokens.push(Token::new(nums, TokenType::Bool));
                        read_token_state = ReadTokenState::Empty;
                    }
                    if let Some(token) = read_token_state.get_token_empty(s) {
                        tokens.push(token);
                    };
                }
                _ => {
                    match nums.len() {
                        1 => {
                            if s == 'o' {
                                read_token_state = ReadTokenState::Bool(nums + &*s.to_string());
                            } else if s == 'r' {
                                read_token_state = ReadTokenState::Break(nums + &*s.to_string());
                            } else {
                                read_token_state = ReadTokenState::Id(nums + &*s.to_string());
                            }
                        }
                        2 => {
                            if s == 'o' {
                                read_token_state = ReadTokenState::Bool(nums + &*s.to_string());
                            } else {
                                read_token_state = ReadTokenState::Id(nums + &*s.to_string());
                            }
                        }
                        3 => {
                            if s == 'l' {
                                read_token_state = ReadTokenState::Bool(nums + &*s.to_string());
                            } else {
                                read_token_state = ReadTokenState::Id(nums + &*s.to_string());
                            }
                        }
                        4 => {
                            read_token_state = ReadTokenState::Id(nums + &*s.to_string());
                        }
                        _ => {}
                    };
                }
            },
            ReadTokenState::Id(nums) => match s {
                ' ' | '\n' | '\r' => {
                    tokens.push(Token::new(nums, TokenType::Id));
                    read_token_state = ReadTokenState::Empty;
                }
                '%' | '+' | '*' | '-' | '/' | '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' | '=' | '!' | '&' | '|' | ';' | ',' | '0'..='9' => {
                    tokens.push(Token::new(nums, TokenType::Id));
                    read_token_state = ReadTokenState::Empty;
                    if let Some(token) = read_token_state.get_token_empty(s) {
                        tokens.push(token);
                    };
                }
                _ => {
                    read_token_state = ReadTokenState::Id(nums + &*s.to_string());
                }
            },
            ReadTokenState::Break(nums) => match s {
                ' ' | '\n' | '\r' => {
                    if nums.len() <= 4 {
                        tokens.push(Token::new(nums, TokenType::Id));
                        read_token_state = ReadTokenState::Empty;
                    } else {
                        tokens.push(Token::new(nums, TokenType::Break));
                        read_token_state = ReadTokenState::Empty;
                    }
                }
                '%' | '+' | '*' | '-' | '/' | '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' | '=' | '!' | '&' | '|' | ';' | ',' | '0'..='9' => {
                    if nums.len() <= 4 {
                        tokens.push(Token::new(nums, TokenType::Id));
                        read_token_state = ReadTokenState::Empty;
                    } else {
                        tokens.push(Token::new(nums, TokenType::Break));
                        read_token_state = ReadTokenState::Empty;
                    }
                    if let Some(token) = read_token_state.get_token_empty(s) {
                        tokens.push(token);
                    };
                }
                _ => match nums.len() {
                    2 => {
                        if s == 'e' {
                            read_token_state = ReadTokenState::Break(nums + &*s.to_string());
                        } else {
                            read_token_state = ReadTokenState::Id(nums + &*s.to_string());
                        }
                    }
                    3 => {
                        if s == 'a' {
                            read_token_state = ReadTokenState::Break(nums + &*s.to_string());
                        } else {
                            read_token_state = ReadTokenState::Id(nums + &*s.to_string());
                        }
                    }
                    4 => {
                        if s == 'k' {
                            read_token_state = ReadTokenState::Break(nums + &*s.to_string());
                        } else {
                            read_token_state = ReadTokenState::Id(nums + &*s.to_string());
                        }
                    }
                    5 => {
                        read_token_state = ReadTokenState::Id(nums + &*s.to_string());
                    }
                    _ => {}
                },
            },
            ReadTokenState::Int(nums) => match s {
                ' ' | '\n' | '\r' => {
                    if nums.len() <= 2 {
                        tokens.push(Token::new(nums, TokenType::Id));
                        read_token_state = ReadTokenState::Empty;
                    } else {
                        tokens.push(Token::new(nums, TokenType::Int));
                        read_token_state = ReadTokenState::Empty;
                    }
                }
                '%' | '+' | '*' | '-' | '/' | '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' | '=' | '!' | '&' | '|' | ';' | ',' | '0'..='9' => {
                    if nums.len() <= 2 {
                        tokens.push(Token::new(nums, TokenType::Id));
                        read_token_state = ReadTokenState::Empty;
                    } else {
                        tokens.push(Token::new(nums, TokenType::Int));
                        read_token_state = ReadTokenState::Empty;
                    }
                    if let Some(token) = read_token_state.get_token_empty(s) {
                        tokens.push(token);
                    };
                }
                _ => {
                    match nums.len() {
                        1 => {
                            if s == 'n' {
                                read_token_state = ReadTokenState::Int(nums + &*s.to_string());
                            } else if s == 'f' {
                                read_token_state = ReadTokenState::If(nums + &*s.to_string());
                            } else {
                                read_token_state = ReadTokenState::Id(nums + &*s.to_string());
                            }
                        }
                        2 => {
                            if s == 't' {
                                read_token_state = ReadTokenState::Int(nums + &*s.to_string());
                            } else {
                                read_token_state = ReadTokenState::Id(nums + &*s.to_string());
                            }
                        }
                        3 => {
                            read_token_state = ReadTokenState::Id(nums + &*s.to_string());
                        }
                        _ => {}
                    };
                }
            },
            ReadTokenState::If(nums) => match s {
                ' ' | '\n' | '\r' => {
                    tokens.push(Token::new(nums, TokenType::If));
                    read_token_state = ReadTokenState::Empty;
                }
                _ => {
                    read_token_state = ReadTokenState::Id(nums + &*s.to_string());
                }
            },
            ReadTokenState::Else(nums) => match s {
                ' ' | '\n' | '\r' => {
                    if nums.len() <= 3 {
                        tokens.push(Token::new(nums, TokenType::Id));
                        read_token_state = ReadTokenState::Empty;
                    } else {
                        tokens.push(Token::new(nums, TokenType::Else));
                        read_token_state = ReadTokenState::Empty;
                    }
                }
                '%' | '+' | '*' | '-' | '/' | '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' | '=' | '!' | '&' | '|' | ';' | ',' | '0'..='9' => {
                    if nums.len() <= 3 {
                        tokens.push(Token::new(nums, TokenType::Id));
                        read_token_state = ReadTokenState::Empty;
                    } else {
                        tokens.push(Token::new(nums, TokenType::Else));
                        read_token_state = ReadTokenState::Empty;
                    }
                    if let Some(token) = read_token_state.get_token_empty(s) {
                        tokens.push(token);
                    };
                }
                _ => {
                    match nums.len() {
                        1 => {
                            if s == 'l' {
                                read_token_state = ReadTokenState::Else(nums + &*s.to_string());
                            } else {
                                read_token_state = ReadTokenState::Id(nums + &*s.to_string());
                            }
                        }
                        2 => {
                            if s == 's' {
                                read_token_state = ReadTokenState::Else(nums + &*s.to_string());
                            } else {
                                read_token_state = ReadTokenState::Id(nums + &*s.to_string());
                            }
                        }
                        3 => {
                            if s == 'e' {
                                read_token_state = ReadTokenState::Else(nums + &*s.to_string());
                            } else {
                                read_token_state = ReadTokenState::Id(nums + &*s.to_string());
                            }
                        }
                        4 => {
                            read_token_state = ReadTokenState::Id(nums + &*s.to_string());
                        }
                        _ => {}
                    };
                }
            },
            ReadTokenState::False(nums) => match s {
                ' ' | '\n' | '\r' => {
                    if nums.len() <= 4 {
                        tokens.push(Token::new(nums, TokenType::Id));
                        read_token_state = ReadTokenState::Empty;
                    } else {
                        tokens.push(Token::new(nums, TokenType::False));
                        read_token_state = ReadTokenState::Empty;
                    }
                }
                '%' | '+' | '*' | '-' | '/' | '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' | '=' | '!' | '&' | '|' | ';' | ',' | '0'..='9' => {
                    if nums.len() <= 4 {
                        tokens.push(Token::new(nums, TokenType::Id));
                        read_token_state = ReadTokenState::Empty;
                    } else {
                        tokens.push(Token::new(nums, TokenType::False));
                        read_token_state = ReadTokenState::Empty;
                    }
                    if let Some(token) = read_token_state.get_token_empty(s) {
                        tokens.push(token);
                    };
                }
                _ => {
                    match nums.len() {
                        1 => {
                            if s == 'a' {
                                read_token_state = ReadTokenState::False(nums + &*s.to_string());
                            } else if s == 'r' {
                                read_token_state = ReadTokenState::Break(nums + &*s.to_string());
                            } else {
                                read_token_state = ReadTokenState::For(nums + &*s.to_string());
                            }
                        }
                        2 => {
                            if s == 'l' {
                                read_token_state = ReadTokenState::False(nums + &*s.to_string());
                            } else {
                                read_token_state = ReadTokenState::Id(nums + &*s.to_string());
                            }
                        }
                        3 => {
                            if s == 's' {
                                read_token_state = ReadTokenState::False(nums + &*s.to_string());
                            } else {
                                read_token_state = ReadTokenState::Id(nums + &*s.to_string());
                            }
                        }
                        4 => {
                            if s == 'e' {
                                read_token_state = ReadTokenState::False(nums + &*s.to_string());
                            } else {
                                read_token_state = ReadTokenState::Id(nums + &*s.to_string());
                            }
                        }
                        5 => {
                            read_token_state = ReadTokenState::Id(nums + &*s.to_string());
                        }
                        _ => {}
                    };
                }
            },
            ReadTokenState::For(nums) => match s {
                ' ' | '\n' | '\r' => {
                    if nums.len() <= 2 {
                        tokens.push(Token::new(nums, TokenType::Id));
                        read_token_state = ReadTokenState::Empty;
                    } else {
                        tokens.push(Token::new(nums, TokenType::For));
                        read_token_state = ReadTokenState::Empty;
                    }
                }
                '%' | '+' | '*' | '-' | '/' | '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' | '=' | '!' | '&' | '|' | ';' | ',' | '0'..='9' => {
                    if nums.len() <= 2 {
                        tokens.push(Token::new(nums, TokenType::Id));
                        read_token_state = ReadTokenState::Empty;
                    } else {
                        tokens.push(Token::new(nums, TokenType::For));
                        read_token_state = ReadTokenState::Empty;
                    }
                    if let Some(token) = read_token_state.get_token_empty(s) {
                        tokens.push(token);
                    };
                }
                _ => match nums.len() {
                    2 => {
                        if s == 'r' {
                            read_token_state = ReadTokenState::For(nums + &*s.to_string());
                        } else {
                            read_token_state = ReadTokenState::Id(nums + &*s.to_string());
                        }
                    }
                    3 => {
                        read_token_state = ReadTokenState::Id(nums + &*s.to_string());
                    }
                    _ => {}
                },
            },
            ReadTokenState::True(nums) => match s {
                ' ' | '\n' | '\r' => {
                    if nums.len() <= 3 {
                        tokens.push(Token::new(nums, TokenType::Id));
                        read_token_state = ReadTokenState::Empty;
                    } else {
                        tokens.push(Token::new(nums, TokenType::True));
                        read_token_state = ReadTokenState::Empty;
                    }
                }
                '%' | '+' | '*' | '-' | '/' | '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' | '=' | '!' | '&' | '|' | ';' | ',' | '0'..='9' => {
                    if nums.len() <= 3 {
                        tokens.push(Token::new(nums, TokenType::Id));
                        read_token_state = ReadTokenState::Empty;
                    } else {
                        tokens.push(Token::new(nums, TokenType::True));
                        read_token_state = ReadTokenState::Empty;
                    }
                    if let Some(token) = read_token_state.get_token_empty(s) {
                        tokens.push(token);
                    };
                }
                _ => {
                    match nums.len() {
                        1 => {
                            if s == 'r' {
                                read_token_state = ReadTokenState::True(nums + &*s.to_string());
                            } else {
                                read_token_state = ReadTokenState::Id(nums + &*s.to_string());
                            }
                        }
                        2 => {
                            if s == 'u' {
                                read_token_state = ReadTokenState::True(nums + &*s.to_string());
                            } else {
                                read_token_state = ReadTokenState::Id(nums + &*s.to_string());
                            }
                        }
                        3 => {
                            if s == 'e' {
                                read_token_state = ReadTokenState::True(nums + &*s.to_string());
                            } else {
                                read_token_state = ReadTokenState::Id(nums + &*s.to_string());
                            }
                        }
                        4 => {
                            read_token_state = ReadTokenState::Id(nums + &*s.to_string());
                        }
                        _ => {}
                    };
                }
            },
            ReadTokenState::Real(nums) => match s {
                ' ' | '\n' | '\r' => {
                    if nums.len() <= 3 {
                        tokens.push(Token::new(nums, TokenType::Id));
                        read_token_state = ReadTokenState::Empty;
                    } else {
                        tokens.push(Token::new(nums, TokenType::Real));
                        read_token_state = ReadTokenState::Empty;
                    }
                }
                '%' | '+' | '*' | '-' | '/' | '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' | '=' | '!' | '&' | '|' | ';' | ',' | '0'..='9' => {
                    if nums.len() <= 3 {
                        tokens.push(Token::new(nums, TokenType::Id));
                        read_token_state = ReadTokenState::Empty;
                    } else {
                        tokens.push(Token::new(nums, TokenType::Real));
                        read_token_state = ReadTokenState::Empty;
                    }
                    if let Some(token) = read_token_state.get_token_empty(s) {
                        tokens.push(token);
                    };
                }
                _ => {
                    match nums.len() {
                        1 => {
                            if s == 'e' {
                                read_token_state = ReadTokenState::Real(nums + &*s.to_string());
                            } else {
                                read_token_state = ReadTokenState::Id(nums + &*s.to_string());
                            }
                        }
                        2 => {
                            if s == 'a' {
                                read_token_state = ReadTokenState::Real(nums + &*s.to_string());
                            } else if s == 't' {
                                read_token_state = ReadTokenState::Return(nums + &*s.to_string());
                            } else {
                                read_token_state = ReadTokenState::Id(nums + &*s.to_string());
                            }
                        }
                        3 => {
                            if s == 'l' {
                                read_token_state = ReadTokenState::Real(nums + &*s.to_string());
                            } else {
                                read_token_state = ReadTokenState::Id(nums + &*s.to_string());
                            }
                        }
                        4 => {
                            read_token_state = ReadTokenState::Id(nums + &*s.to_string());
                        }
                        _ => {}
                    };
                }
            },
            ReadTokenState::Return(nums) => match s {
                ' ' | '\n' | '\r' => {
                    if nums.len() <= 5 {
                        tokens.push(Token::new(nums, TokenType::Id));
                        read_token_state = ReadTokenState::Empty;
                    } else {
                        tokens.push(Token::new(nums, TokenType::Return));
                        read_token_state = ReadTokenState::Empty;
                    }
                }
                '%' | '+' | '*' | '-' | '/' | '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' | '=' | '!' | '&' | '|' | ';' | ',' | '0'..='9' => {
                    if nums.len() <= 5 {
                        tokens.push(Token::new(nums, TokenType::Id));
                        read_token_state = ReadTokenState::Empty;
                    } else {
                        tokens.push(Token::new(nums, TokenType::Return));
                        read_token_state = ReadTokenState::Empty;
                    }
                    if let Some(token) = read_token_state.get_token_empty(s) {
                        tokens.push(token);
                    };
                }
                _ => match nums.len() {
                    3 => {
                        if s == 'u' {
                            read_token_state = ReadTokenState::Return(nums + &*s.to_string());
                        } else {
                            read_token_state = ReadTokenState::Id(nums + &*s.to_string());
                        }
                    }
                    4 => {
                        if s == 'r' {
                            read_token_state = ReadTokenState::Return(nums + &*s.to_string());
                        } else {
                            read_token_state = ReadTokenState::Id(nums + &*s.to_string());
                        }
                    }
                    5 => {
                        if s == 'n' {
                            read_token_state = ReadTokenState::Return(nums + &*s.to_string());
                        } else {
                            read_token_state = ReadTokenState::Id(nums + &*s.to_string());
                        }
                    }
                    6 => {
                        read_token_state = ReadTokenState::Id(nums + &*s.to_string());
                    }
                    _ => {}
                },
            },
        }
    }
    tokens
}
