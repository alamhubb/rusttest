#[derive(Debug, PartialEq)]
enum DfaState {
    Initial,
    Id,
    GT,
    GE,
    IntLiteral,
    Keyword,
}

#[derive(Debug, PartialEq)]
enum TokenType {
    Identifier,
    GT,
    GE,
    IntLiteral,
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    text: String,
}

fn is_alpha(ch: char) -> bool {
    ch.is_ascii_alphabetic()
}

fn is_digit(ch: char) -> bool {
    ch.is_ascii_digit()
}



fn init_token(token: &mut Token, ch: Option<char>) -> DfaState {
    // 完成当前 token，重置状态
    if let Some(c) = ch {
        if is_alpha(c) {
            token.token_type = TokenType::Identifier;
            token.text.push(c);
            return DfaState::Id;
        } else if is_digit(c) {
            token.token_type = TokenType::IntLiteral;
            token.text.push(c);
            return DfaState::IntLiteral;
        } else if c == '>' {
            token.token_type = TokenType::GT;
            token.text.push(c);
            return DfaState::GT;
        }
    }
    DfaState::Initial
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    //状态设置为初始
    let mut state = DfaState::Initial;
    // 这里初始化可以避免在
    let mut token = Token { token_type: TokenType::Identifier, text: String::new() };

    for ch in input.chars() {
        match state {
            //如果为初始，则根据第一个字母改编为其他状态，并将文字加入
            DfaState::Initial => {
                state = init_token(&mut token, Some(ch));
            }
            DfaState::Id => {
                //如果为id，则符合要求不改变状态，继续加入
                if is_alpha(ch) || is_digit(ch) {
                    token.text.push(ch);
                } else {
                    //不符合状态，阶段token，改变token状态
                    tokens.push(token);
                    token = Token { token_type: TokenType::Identifier, text: String::new() };
                    state = init_token(&mut token, Some(ch));
                }
            }
            DfaState::Keyword => {
                //如果为id，则符合要求不改变状态，继续加入
                if is_alpha(ch) || is_digit(ch) {
                    token.text.push(ch);
                    token.token_type = TokenType::Identifier
                } else {
                    //不符合状态，阶段token，改变token状态
                    tokens.push(token);
                    token = Token { token_type: TokenType::Identifier, text: String::new() };
                    state = init_token(&mut token, Some(ch));
                }
            }
            DfaState::GT => {
                if ch == '=' {
                    token.token_type = TokenType::GE;
                    token.text.push(ch);
                    state = DfaState::GE;
                } else {
                    tokens.push(token);
                    token = Token { token_type: TokenType::Identifier, text: String::new() };
                    state = init_token(&mut token, Some(ch));
                }
            }
            DfaState::GE => {
                tokens.push(token);
                token = Token { token_type: TokenType::Identifier, text: String::new() };
                state = init_token(&mut token, Some(ch));
            }
            DfaState::IntLiteral => {
                if is_digit(ch) {
                    token.text.push(ch);
                } else {
                    tokens.push(token);
                    token = Token { token_type: TokenType::Identifier, text: String::new() };
                    state = init_token(&mut token, Some(ch));
                }
            }
        }
    }

    if !token.text.is_empty() {
        tokens.push(token);
    }

    tokens
}

fn main() {
    let input = "abc 123 >= 456";
    let tokens = tokenize(input);

    for token in tokens {
        println!("{:?}", token);
    }
}
