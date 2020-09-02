//! 规则表达式的解析实现。

use super::error::Error;
use super::result::Result;

/// 单个 Token。
#[derive(Debug)]
pub struct Token {
    /// Token 的类型。
    pub type_: TokenType,
    /// Token 的值。
    pub value: String,
}

/// Token 的类型。
#[derive(Debug)]
pub enum TokenType {
    /// 左小括号。
    OpenParenthesis,
    /// 右小括号。
    CloseParenthesis,
    /// 字段。
    Field,
    /// 运算符。
    Operator,
    /// 左大括号。
    OpenBrace,
    /// 右大括号。
    CloseBrace,
    /// 引号。
    Quote,
    /// 值。
    Value,
    /// 结束。
    NIL,
    /// end token
    EOF,
}

type Input = Vec<char>;

/// 词法分析器。
#[derive(Debug)]
pub struct Lexer<'a> {
    /// 输入。
    pub input: &'a Input,
    /// 指针位置。
    pub pos: usize,
    /// 当前字符。
    pub current_char: Option<&'a char>,
    /// 全部的 Token。
    pub tokens: Vec<Token>,
}

impl Token {
    pub fn new(type_: TokenType, value: String) -> Self {
        Self { type_, value }
    }
}

impl<'a> Lexer<'a> {
    /// 以字符列表作为输入创建分析器。
    pub fn new(input: &'a Input) -> Self {
        Self {
            input: input,
            pos: 0,
            current_char: input.get(0),
            tokens: vec![],
        }
    }

    pub fn lex(&mut self) -> Result<()> {
        while !self.current_char.is_nil() {
            self.skip_white_space();
            let current_char = self.current_char.unwrap();

            let token = match current_char {
                '(' => Token::new(TokenType::OpenParenthesis, "(".to_owned()),
                ')' => Token::new(TokenType::CloseParenthesis, ")".to_owned()),
                '{' => Token::new(TokenType::OpenBrace, "{".to_owned()),
                '}' => Token::new(TokenType::CloseBrace, "}".to_owned()),
                '"' => Token::new(TokenType::Quote, "\"".to_owned()),
                _ => {
                    return Err(Error::ParseFailed {
                        column: self.pos + 1,
                    })
                }
            };

            self.tokens.push(token);
            self.next_char();
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "EOF".to_owned()));

        Ok(())
    }

    // 访问下一个字符。此方法调用后会自增指针位置。
    fn next_char(&mut self) -> Option<&char> {
        self.pos += 1;
        self.current_char = self.input.get(self.pos);

        self.current_char
    }

    fn skip_white_space(&mut self) {
        let mut current_char = self.current_char;
        while current_char.is_white_space() {
            current_char = self.next_char()
        }
    }

    pub fn is_end(&self) -> bool {
        self.pos > self.input.len() - 1
    }
}

trait IsNil {
    fn is_nil(self) -> bool;
}
trait IsWhiteSpace {
    fn is_white_space(self) -> bool;
}

impl IsNil for Option<&char> {
    fn is_nil(self) -> bool {
        if let Some(c) = self {
            *c == '\n'
        } else {
            true
        }
    }
}

impl IsWhiteSpace for Option<&char> {
    fn is_white_space(self) -> bool {
        if let Some(c) = self {
            *c == ' '
        } else {
            false
        }
    }
}

#[test]
fn test_lex() {
    let expression = "{}()\"";
    let input = expression.chars().collect::<Vec<_>>();

    let mut lexer = Lexer::new(&input);
    lexer.lex().unwrap();

    // println!("{:?}", lexer.tokens);
}
