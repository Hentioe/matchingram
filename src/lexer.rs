//! 规则表达式的词法分析实现。
//!
//! 词法分析器并不保证语法上的正确性，但仍然具备一定的错误识别能力。例如通过前后关系才能确定的”字段“和”操作符“两个 token 类型。
//! 一个例子：
//! ```
//! use matchingram::lexer::Lexer;
//! use matchingram::lexer::Token::*;
//!
//! let expression = "(message.text contains_all \"bye\" and message.text contains_one {parent world}) or (message.text contains_one {see you})";
//! let input = expression.chars().collect::<Vec<_>>();
//!
//! let mut lexer = Lexer::new(&input);
//! lexer.tokenize().unwrap();
//!
//! let truthy = [
//!     OpenParenthesis,
//!     Field("message.text".to_owned()),
//!     Operator("contains_all".to_owned()),
//!     Quote,
//!     Value("bye".to_owned()),
//!     Quote,
//!     And,
//!     Field("message.text".to_owned()),
//!     Operator("contains_one".to_owned()),
//!     OpenBrace,
//!     Value("parent world".to_owned()),
//!     CloseBrace,
//!     CloseParenthesis,
//!     Or,
//!     OpenParenthesis,
//!     Field("message.text".to_owned()),
//!     Operator("contains_one".to_owned()),
//!     OpenBrace,
//!     Value("see you".to_owned()),
//!     CloseBrace,
//!     CloseParenthesis,
//!     EOF,
//! ];
//!
//! assert_eq!(truthy.len(), lexer.tokens.len());
//! for (i, token) in lexer.tokens.iter().enumerate() {
//!     assert_eq!(truthy[i], *token);
//! }
//! # Ok::<(), matchingram::Error>(())
//! ```

use super::error::Error;
use super::result::Result;

/// 所有的 Token。
#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    /// 左小括号。
    OpenParenthesis, // (
    /// 右小括号。
    CloseParenthesis, // )
    /// 字段。
    Field(String), // string
    /// 运算符。
    Operator(String), // string
    /// 左大括号。
    OpenBrace, // {
    /// 右大括号。
    CloseBrace, // }
    /// 引号。
    Quote, // "
    /// 值。
    Value(String), // string
    /// and 关键字。
    And, // and
    // or 关键字。
    Or, // or
    /// 结束
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

    pub fn tokenize(&mut self) -> Result<()> {
        while self.current_char.is_some() {
            self.skip_white_space();
            if let Some(current_char) = self.current_char {
                match current_char {
                    '(' => {
                        self.push_token(Token::OpenParenthesis);
                        self.scan();
                        self.skip_white_space();
                        if !self.tokenize_field() {
                            return Err(Error::MissingField {
                                column: self.pos + 1,
                            });
                        }
                        self.skip_white_space();
                        if !self.tokenize_operator() {
                            return Err(Error::MissingOperator {
                                column: self.pos + 1,
                            });
                        }
                    }
                    ')' => self.push_token(Token::CloseParenthesis),
                    '{' => {
                        self.push_token(Token::OpenBrace);
                        self.tokenize_value(Token::CloseBrace);
                    }
                    '}' => {
                        self.push_token(Token::CloseBrace);
                        self.skip_white_space();
                    }
                    '"' => {
                        self.push_token(Token::Quote);
                        self.tokenize_value(Token::Quote);
                    }
                    'o' => {
                        if !self.tokenize_or() {
                            return Err(Error::ParseFailed {
                                column: self.pos + 1,
                            });
                        }
                    }
                    'a' => {
                        if self.tokenize_and() {
                            self.scan();
                            self.skip_white_space();
                            if !self.tokenize_field() {
                                return Err(Error::MissingField {
                                    column: self.pos + 1,
                                });
                            }
                            self.skip_white_space();
                            if !self.tokenize_operator() {
                                return Err(Error::MissingOperator {
                                    column: self.pos + 1,
                                });
                            }
                        } else {
                            return Err(Error::ParseFailed {
                                column: self.pos + 1,
                            });
                        }
                    }
                    _ => {
                        return Err(Error::ParseFailed {
                            column: self.pos + 1,
                        });
                    }
                }

                self.scan();
            }
        }

        self.tokens.push(Token::EOF);

        Ok(())
    }

    /// 追加一个 Token。
    pub fn push_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn tokenize_field(&mut self) -> bool {
        let begin_pos = self.pos;
        let mut cur_pos = begin_pos;
        let mut next_char = self.at_char(cur_pos);

        while next_char.is_some() {
            if next_char.is_white_space() {
                break;
            }

            cur_pos += 1;
            next_char = self.at_char(cur_pos);
        }

        let is_field = cur_pos > begin_pos;

        if is_field {
            let value = self.input[begin_pos..cur_pos].iter().collect::<String>();
            self.tokens.push(Token::Field(value));
            self.scan_at(cur_pos);
        }

        return is_field;
    }

    fn tokenize_operator(&mut self) -> bool {
        let begin_pos = self.pos;
        let mut cur_pos = begin_pos;
        let mut next_char = self.at_char(cur_pos);

        while next_char.is_some() {
            if next_char.is_white_space() {
                break;
            }

            cur_pos += 1;
            next_char = self.at_char(cur_pos);
        }

        let is_operator = cur_pos > begin_pos;

        if is_operator {
            let value = self.input[begin_pos..cur_pos].iter().collect::<String>();
            self.tokens.push(Token::Operator(value));
            self.scan_at(cur_pos);
        }

        return is_operator;
    }

    fn tokenize_value(&mut self, end_token: Token) -> bool {
        let begin_pos = self.pos + 1;
        let end_char = match end_token {
            Token::CloseBrace => '}',
            // 如果上上个是值，表示上一个引号是结束引号
            Token::Quote => match self.tokens.get(self.tokens.len() - 2) {
                Some(Token::Value(_)) => return false,
                _ => '"',
            },
            _ => return false,
        };

        let mut cur_pos = begin_pos;
        let mut next_char = self.at_char(cur_pos);

        while next_char.is_some() {
            if next_char == Some(&end_char) {
                break;
            }

            cur_pos += 1;
            next_char = self.at_char(cur_pos);
        }

        let value = self.input[begin_pos..cur_pos].iter().collect::<String>();
        let value = value.trim();

        let is_value = cur_pos > begin_pos;

        if is_value {
            self.tokens.push(Token::Value(value.to_string()));
            self.scan_at(cur_pos - 1);
        }

        return is_value;
    }

    fn tokenize_or(&mut self) -> bool {
        if self.at_char(self.pos + 1) == Some(&'r') && self.at_char(self.pos + 2) == Some(&' ') {
            self.scan_at(self.pos + 1);
            self.push_token(Token::Or);

            true
        } else {
            false
        }
    }

    fn tokenize_and(&mut self) -> bool {
        if self.at_char(self.pos + 1) == Some(&'n')
            && self.at_char(self.pos + 2) == Some(&'d')
            && self.at_char(self.pos + 3) == Some(&' ')
        {
            self.scan_at(self.pos + 2);
            self.push_token(Token::And);

            true
        } else {
            false
        }
    }

    // 扫描下一个字符。此方法调用后会自增指针位置。
    fn scan(&mut self) -> Option<&char> {
        self.pos += 1;
        self.current_char = self.input.get(self.pos);

        self.current_char
    }

    // 访问指定位置的字符。此方法不移动位置。
    fn at_char(&self, pos: usize) -> Option<&char> {
        self.input.get(pos)
    }

    // 扫描指定位置的字符。此方法将改变指针位置。
    fn scan_at(&mut self, pos: usize) -> Option<&char> {
        self.pos = pos;
        self.current_char = self.input.get(self.pos);

        self.current_char
    }

    // 跳过空白字符。返回跳过的字符数量。
    fn skip_white_space(&mut self) -> usize {
        let mut current_char = self.current_char;
        let begin_pos = self.pos;
        while current_char.is_white_space() {
            current_char = self.scan();
        }

        return self.pos - begin_pos;
    }

    pub fn is_end(&self) -> bool {
        self.pos > self.input.len() - 1
    }
}

trait IsWhiteSpace {
    fn is_white_space(self) -> bool;
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
