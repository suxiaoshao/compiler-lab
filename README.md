# 项目介绍

本项目为 fzu 的编译实验代码

## 开发环境

### 工具链

- rustc 1.52.1 (9bc8c42bb 2021-05-09)
- cargo 1.52.0 (69767412a 2021-04-21)
- WSL Ubuntu 20.04

### 编译

在项目根目录下

编译

```sh
 cargo build --release
```

编译并运行

```sh
 cargo build --release && strip -s target/release/compiler-lab && ./target/release/compiler-lab -p ./grammar.json  ./test.lex
```

快速编译运行

```sh
 cargo run -- -p ./grammar.json  ./test.lex
```

### 运行

#### 参数介绍

```sh
./target/release/compiler-lab ./test.lex -p ./grammar.json
```

`-p` 或 `--parser` 后面是文法描述文件,以 `json` 文件给出,非必选项,默认为 `./grammar.json`

第一个参数为测试文件,内容是待分析的源代码,必选项

## 实验内容

### 词法分析✔

#### 关键字

- int
- real
- bool

- true
- false
- if
- else if
- else
- while
- break

#### 标识符

变量名（形如real num中的`num`）

#### 常数

比如`3.14`

#### 运算符

- `+`,`-`,`*`,`/`

- `=`,`==`,`!=`
- `&&`,`||`,`!`
- `>`,`>=`,`<`,`<=`

#### 分隔符

- `,`,`;`
- `{`,`}`
- `[`,`]`
- `(`,`)`
- 支持`//`的行注释

#### 变量

支持所有非数字开头除了几个运算符开头的以外的 `unicode` 字符,如中文/emoji 等

### 语法分析✔

#### 文法配置文件

文法是程序运行时提供的参数,默认为 `./grammar.json`,可自定义

文法G[Program]如下

```json
[
    {
        "left": "Program",
        "right": [
            "Block"
        ]
    },
    {
        "left": "Block",
        "right": [
            "{",
            "Decls",
            "Stmts",
            "}"
        ]
    },
    {
        "left": "Decls",
        "right": [
            "Decls",
            "Decl"
        ]
    },
    {
        "left": "Decls",
        "right": [
            "ε"
        ]
    },
    {
        "left": "Decl",
        "right": [
            "Type",
            "id",
            ";"
        ]
    },
    {
        "left": "Type",
        "right": [
            "Type",
            "[",
            "int_num",
            "]"
        ]
    },
    {
        "left": "Type",
        "right": [
            "Type",
            "[",
            "real_num",
            "]"
        ]
    },
    {
        "left": "Type",
        "right": [
            "int"
        ]
    },
    {
        "left": "Type",
        "right": [
            "real"
        ]
    },
    {
        "left": "Type",
        "right": [
            "bool"
        ]
    },
    {
        "left": "Stmts",
        "right": [
            "Stmts",
            "Stmt"
        ]
    },
    {
        "left": "Stmts",
        "right": [
            "ε"
        ]
    },
    {
        "left": "Stmt",
        "right": [
            "Var",
            "=",
            "Bool",
            ";"
        ]
    },
    {
        "left": "Stmt",
        "right": [
            "if",
            "(",
            "Bool",
            ")",
            "Stmt"
        ]
    },
    {
        "left": "Stmt",
        "right": [
            "if",
            "(",
            "Bool",
            ")",
            "Stmt",
            "else",
            "Stmt"
        ]
    },
    {
        "left": "Stmt",
        "right": [
            "while",
            "(",
            "Bool",
            ")",
            "Stmt"
        ]
    },
    {
        "left": "Stmt",
        "right": [
            "break",
            ";"
        ]
    },
    {
        "left": "Stmt",
        "right": [
            "Block"
        ]
    },
    {
        "left": "Var",
        "right": [
            "Var",
            "[",
            "int_num",
            "]"
        ]
    },
    {
        "left": "Var",
        "right": [
            "Var",
            "[",
            "real_num",
            "]"
        ]
    },
    {
        "left": "Var",
        "right": [
            "id"
        ]
    },
    {
        "left": "Bool",
        "right": [
            "Bool",
            "||",
            "Join"
        ]
    },
    {
        "left": "Bool",
        "right": [
            "Join"
        ]
    },
    {
        "left": "Join",
        "right": [
            "Join",
            "&&",
            "Equality"
        ]
    },
    {
        "left": "Join",
        "right": [
            "Equality"
        ]
    },
    {
        "left": "Equality",
        "right": [
            "Equality",
            "==",
            "Rel"
        ]
    },
    {
        "left": "Equality",
        "right": [
            "Equality",
            "!=",
            "Rel"
        ]
    },
    {
        "left": "Equality",
        "right": [
            "Rel"
        ]
    },
    {
        "left": "Rel",
        "right": [
            "Expr",
            "<",
            "Expr"
        ]
    },
    {
        "left": "Rel",
        "right": [
            "Expr",
            "<=",
            "Expr"
        ]
    },
    {
        "left": "Rel",
        "right": [
            "Expr",
            ">=",
            "Expr"
        ]
    },
    {
        "left": "Rel",
        "right": [
            "Expr",
            ">",
            "Expr"
        ]
    },
    {
        "left": "Rel",
        "right": [
            "Expr"
        ]
    },
    {
        "left": "Expr",
        "right": [
            "Expr",
            "+",
            "Term"
        ]
    },
    {
        "left": "Expr",
        "right": [
            "Expr",
            "-",
            "Term"
        ]
    },
    {
        "left": "Expr",
        "right": [
            "Term"
        ]
    },
    {
        "left": "Term",
        "right": [
            "Term",
            "*",
            "Unary"
        ]
    },
    {
        "left": "Term",
        "right": [
            "Term",
            "/",
            "Unary"
        ]
    },
    {
        "left": "Term",
        "right": [
            "Unary"
        ]
    },
    {
        "left": "Unary",
        "right": [
            "!",
            "Unary"
        ]
    },
    {
        "left": "Unary",
        "right": [
            "-",
            "Unary"
        ]
    },
    {
        "left": "Unary",
        "right": [
            "Factor"
        ]
    },
    {
        "left": "Factor",
        "right": [
            "(",
            "Bool",
            ")"
        ]
    },
    {
        "left": "Factor",
        "right": [
            "Var"
        ]
    },
    {
        "left": "Factor",
        "right": [
            "int_num"
        ]
    },
    {
        "left": "Factor",
        "right": [
            "real_num"
        ]
    },
    {
        "left": "Factor",
        "right": [
            "true"
        ]
    },
    {
        "left": "Factor",
        "right": [
            "false"
        ]
    }
]
```

终结符与词法分析中的 `TokenType` 一一对应,

`TokenType` 的 `rust` 定义如下

```rs
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize, Hash)]
pub enum TokenType {
    #[serde(rename = "ε")]
    Epsilon, // 未知类型
    #[serde(rename = "int")]
    Int, // int
    #[serde(rename = "int_num")]
    IntNum, // 整数
    #[serde(rename = "real")]
    Real, // real
    #[serde(rename = "real_num")]
    RealNum, // 实数
    #[serde(rename = "bool")]
    Bool, // bool
    #[serde(rename = "true")]
    True, // true
    #[serde(rename = "false")]
    False, // false
    #[serde(rename = "if")]
    If, // if
    #[serde(rename = "else")]
    Else, // else
    #[serde(rename = "break")]
    Break, // break
    #[serde(rename = "id")]
    Id, //标识符
    #[serde(rename = "+")]
    Add, // +
    #[serde(rename = "-")]
    Sub, // -
    #[serde(rename = "*")]
    Mul, // *
    #[serde(rename = "/")]
    Div, // /
    #[serde(rename = "=")]
    Assign, // =
    #[serde(rename = "==")]
    Equal, // ==
    #[serde(rename = "!=")]
    NotEqual, // !=
    #[serde(rename = "&&")]
    And, // &&
    #[serde(rename = "||")]
    Or, // ||
    #[serde(rename = "!")]
    Not, // !
    #[serde(rename = ">")]
    Greater, // >
    #[serde(rename = ">=")]
    GreaterEqual, // >=
    #[serde(rename = "<")]
    Less, // <
    #[serde(rename = "<=")]
    LessEqual, // <=
    #[serde(rename = ";")]
    Semicolon, // ;
    #[serde(rename = "{")]
    LeftBlock, // {
    #[serde(rename = "}")]
    RightBlock, // }
    #[serde(rename = "[")]
    SqLeftBracket, // [
    #[serde(rename = "]")]
    SqRightBracket, // ]
    #[serde(rename = "(")]
    CirLeftBracket, // (
    #[serde(rename = ")")]
    CirRightBracket, // )
    #[serde(rename = "while")]
    While,
    #[serde(rename = "$")]
    Eof, // $,
    #[serde(rename = ",")]
    Comma,
}
```

非终结符与 `NonTerminator`意义对应

定义如下

```rs
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Hash)]
pub enum NonTerminator {
    Program,
    Block,
    Decls,
    Decl,
    Type,
    Stmts,
    Stmt,
    Var,
    Bool,
    Join,
    Equality,
    Rel,
    Expr,
    Term,
    Unary,
    Factor,
}
```

这里使用 `serde` 和 `serde_json` 做 json 数据的反序列化

### 语法制导翻译（未完成）

### 中间代码生成（未完成）

## 参考项目

主要是参考 [qizong](https://github.com/qizong007) 写的 <https://github.com/Pikapika-sk/compiler-lab> (其实就是把他的 c++ 代码翻译了一下)

还有就是 [qizong](https://github.com/qizong007) 推荐的这个项目 <https://github.com/khuqen/gramma_analysis>
