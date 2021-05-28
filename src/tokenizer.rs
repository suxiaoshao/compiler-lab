use crate::tokenizer::position::Position;
use read_token_state::ReadTokenState;
use token::Token;

mod position;
mod read_token_state;
mod token;

pub fn get_tokens_from_string(content: &str) -> Vec<Token> {
    // 字符向量
    let chars: Vec<char> = content.chars().collect();
    // 结果
    let mut tokens = vec![];
    // 位置
    let mut position = Position::new();
    // 状态
    let mut read_token_state: ReadTokenState = ReadTokenState::new(&position);
    // 下标
    let mut index = 0;
    while index < chars.len() {
        let c = chars[index];
        let (token, is_next) = read_token_state.read_char(c, &position);
        if let Some(token) = token {
            tokens.push(token)
        }
        if is_next {
            position.change_from_char(c);
            index += 1;
        }
    }
    let (token, ..) = read_token_state.read_char(' ', &position);
    if let Some(token) = token {
        tokens.push(token)
    }
    tokens.iter_mut().for_each(|token| token.check_position());
    tokens
}
#[cfg(test)]
mod test {
    use crate::tokenizer::get_tokens_from_string;
    use crate::tokenizer::token::TokenType;
    use crate::tokenizer::token::TokenType::{IntNum, RealNum};

    #[test]
    fn get_tokens_test() {
        let content = r##"
real elsee,iff,fal,tru,哎;
if (aa>=boo || false && 1) {
    aa = boo + 3.0;
}
else{int elsee;elsee=3;}
int intt = 44;
real reall = 44.44;
bool booll = true;
a%b = $q
else"##;
        let tokens = get_tokens_from_string(content);
        assert_eq!(60, tokens.len());
        assert_eq!(tokens[9].lex, "哎");
        assert_eq!(tokens[1].token_type, TokenType::Id);
        assert_eq!(tokens[26].token_type, RealNum);
        assert_eq!(tokens[26].lex, "3.0");
        assert_eq!(tokens[36].token_type, IntNum);
        assert_eq!(tokens[58].token_type, TokenType::Id);
        assert_eq!(tokens[58].start.y, 10);
        assert_eq!(tokens[58].start.x, 7);
        assert_eq!(tokens[58].end.y, 10);
        assert_eq!(tokens[58].end.x, 8);
    }
}
