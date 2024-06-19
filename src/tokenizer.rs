#[derive(Debug)]
pub enum Type {

}

#[derive(Debug)]
pub enum Token {
    NUMBER(u128),
    TYPE(Type),
    LPAREN,
    RPAREN,
    LCURLY,
    RCURLY,
    LSQUARE,
    RSQUARE,
    STRING(String),
    DOT,
    COMMA,
    IDENT(String),
}

macro_rules! push {
    ($tokens:expr, $cur_token:expr) => {
        if $cur_token.len() > 0 {
            $tokens.push(Token::IDENT($cur_token));
        }
    };
}

pub fn tokenize(line: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut cur_token: String = String::from("");
    let mut in_str = false;
    for c in line.chars() {
        if !in_str {
            match c {
                '(' => {
                    push!(tokens, cur_token);
                    cur_token = String::from("");
                    tokens.push(Token::LPAREN);
                }
                ')' => {
                    push!(tokens, cur_token);
                    cur_token = String::from("");
                    tokens.push(Token::RPAREN);
                }
                '{' => {
                    push!(tokens, cur_token);
                    cur_token = String::from("");
                    tokens.push(Token::LCURLY);
                }
                '}' => {
                    push!(tokens, cur_token);
                    cur_token = String::from("");
                    tokens.push(Token::RCURLY);
                }
                '[' => {
                    push!(tokens, cur_token);
                    cur_token = String::from("");
                    tokens.push(Token::LSQUARE);
                }
                ']' => {
                    push!(tokens, cur_token);
                    cur_token = String::from("");
                    tokens.push(Token::RSQUARE);
                }
                '"' => {
                    in_str = true;
                }
                '.' => {
                    push!(tokens, cur_token);
                    cur_token = String::from("");
                    tokens.push(Token::DOT);
                }
                ',' => {
                    push!(tokens, cur_token);
                    cur_token = String::from("");
                    tokens.push(Token::COMMA);
                }
                ' ' => {
                    if cur_token.len() > 0 {
                        tokens.push(Token::IDENT(cur_token));
                        cur_token = String::from("");
                    }
                }
                '\r' | '\n' => {
                    if cur_token.len() > 0 {
                        tokens.push(Token::IDENT(cur_token));
                        cur_token = String::from("");
                    }
                }
                _ => cur_token.push(c)
            }
        } else {
            // yes im using match for this
            match c {
                '"' => {
                    tokens.push(Token::STRING(cur_token));
                    cur_token = String::from("");
                }
                _ => cur_token.push(c)
            }
        }
    }

    return tokens;
}