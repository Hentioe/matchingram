//! 规则表达式的词法分析实现。
//!
//! 词法分析器并不保证语法上的正确性，但仍然具备一定的错误识别能力。例如通过前后关系才能确定的”字段“和”操作符“两个 token 类型。
//! 一个例子：
//! ```
//! use matchingram::lexer::Lexer;
//! use matchingram::lexer::Token::*;
//!
//! let rule = "(message.text contains_all \"bye\" and message.text contains_one {parent world}) or (message.text contains_one {see you})";
//! let input = rule.chars().collect::<Vec<_>>();
//!
//! let mut lexer = Lexer::new(&input);
//! lexer.tokenize()?;
//!
//! let truthy = [
//!     (OpenParenthesis, String::from("(")),
//!     (Field, String::from("message.text")),
//!     (Operator, String::from("contains_all")),
//!     (Quote, String::from("\"")),
//!     (Value, String::from("bye")),
//!     (Quote, String::from("\"")),
//!     (And, String::from("and")),
//!     (Field, String::from("message.text")),
//!     (Operator, String::from("contains_one")),
//!     (OpenBrace, String::from("{")),
//!     (Value, String::from("parent world")),
//!     (CloseBrace, String::from("}")),
//!     (CloseParenthesis, String::from(")")),
//!     (Or, String::from("or")),
//!     (OpenParenthesis, String::from("(")),
//!     (Field, String::from("message.text")),
//!     (Operator, String::from("contains_one")),
//!     (OpenBrace, String::from("{")),
//!     (Value, String::from("see you")),
//!     (CloseBrace, String::from("}")),
//!     (CloseParenthesis, String::from(")")),
//!     (EOF, String::from("")),
//! ];
//!
//! assert_eq!(truthy.len(), lexer.output().len());
//! for (i, mapping) in lexer.token_data_owner()?.into_iter().enumerate() {
//!     assert_eq!(truthy[i], mapping);
//! }
//! # Ok::<(), matchingram::Error>(())
//! ```

use super::error::Error;
use super::result::Result;

/// 所有的 Token。
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Token {
    /// 左小括号。
    OpenParenthesis, // (
    /// 右小括号。
    CloseParenthesis, // )
    /// 字段。
    Field, // string
    /// 运算符。
    Operator, // string
    /// 左大括号。
    OpenBrace, // {
    /// 右大括号。
    CloseBrace, // }
    /// 引号。
    Quote, // "
    /// 值。
    Value, // string
    /// and 关键字。
    And, // and
    /// or 关键字。
    Or, // or
    /// 结束
    EOF,
}

type Input = [char];

/// 词法分析器。
#[derive(Debug)]
pub struct Lexer<'a> {
    /// 输入。
    pub input: &'a Input,
    // 当前指针。
    pos: usize,
    // 当前字符。
    current_char: Option<&'a char>,
    // 全部的 Token。
    tokens: Vec<Token>,
    // 位置序列。
    positions: Vec<Position>,
}

#[derive(Debug)]
pub struct Position {
    pub begin: usize,
    pub end: usize,
}

impl<'a> Lexer<'a> {
    /// 以字符列表作为输入创建分析器。
    pub fn new(input: &'a Input) -> Self {
        Self {
            input: input,
            pos: 0,
            current_char: input.get(0),
            tokens: vec![],
            positions: vec![],
        }
    }

    /// 获取输出（token 序列）。
    pub fn output(&self) -> &Vec<Token> {
        &self.tokens
    }

    /// 获取位置序列。
    pub fn positions(&self) -> &Vec<Position> {
        &self.positions
    }

    pub fn tokenize(&mut self) -> Result<()> {
        while self.current_char.is_some() {
            self.skip_white_space();
            if let Some(current_char) = self.current_char {
                match current_char {
                    '(' => {
                        self.push_token(Token::OpenParenthesis)?;
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
                    ')' => self.push_token(Token::CloseParenthesis)?,
                    '{' => {
                        self.push_token(Token::OpenBrace)?;
                        self.tokenize_value(Token::CloseBrace);
                    }
                    '}' => {
                        self.push_token(Token::CloseBrace)?;
                        self.skip_white_space();
                    }
                    '"' => {
                        self.push_token(Token::Quote)?;
                        self.tokenize_value(Token::Quote);
                    }
                    'o' => {
                        if !self.tokenize_or()? {
                            return Err(Error::ParseFailed {
                                column: self.pos + 1,
                            });
                        }
                    }
                    'a' => {
                        if self.tokenize_and()? {
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

        self.push_token(Token::EOF)?;

        Ok(())
    }

    /// 追加一个 Token。
    pub fn push_token(&mut self, token: Token) -> Result<()> {
        use Token::*;

        let position = match &token {
            OpenParenthesis => Position {
                begin: self.pos,
                end: self.pos + 1,
            },
            CloseParenthesis => Position {
                begin: self.pos,
                end: self.pos + 1,
            },
            OpenBrace => Position {
                begin: self.pos,
                end: self.pos + 1,
            },
            CloseBrace => Position {
                begin: self.pos,
                end: self.pos + 1,
            },
            Quote => Position {
                begin: self.pos,
                end: self.pos + 1,
            },
            And => Position {
                begin: self.pos - 2,
                end: self.pos + 1,
            },
            Or => Position {
                begin: self.pos - 1,
                end: self.pos + 1,
            },
            EOF => Position {
                begin: self.pos,
                end: self.pos,
            },
            _ => return Err(Error::InferPositionFailed { token: token }),
        };
        self.positions.push(position);
        self.tokens.push(token);

        Ok(())
    }

    fn push_token_position(&mut self, token: Token, position: Position) {
        self.positions.push(position);
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
            self.push_token_position(
                Token::Field,
                Position {
                    begin: begin_pos,
                    end: cur_pos,
                },
            );
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
            self.push_token_position(
                Token::Operator,
                Position {
                    begin: begin_pos,
                    end: cur_pos,
                },
            );
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
                Some(Token::Value) => return false,
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

        let is_value = cur_pos > begin_pos;

        if is_value {
            self.push_token_position(
                Token::Value,
                Position {
                    begin: begin_pos,
                    end: cur_pos,
                },
            );
            self.scan_at(cur_pos - 1);
        }

        return is_value;
    }

    fn tokenize_or(&mut self) -> Result<bool> {
        if self.at_char(self.pos + 1) == Some(&'r') && self.at_char(self.pos + 2) == Some(&' ') {
            self.scan_at(self.pos + 1);
            self.push_token(Token::Or)?;

            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn tokenize_and(&mut self) -> Result<bool> {
        if self.at_char(self.pos + 1) == Some(&'n')
            && self.at_char(self.pos + 2) == Some(&'d')
            && self.at_char(self.pos + 3) == Some(&' ')
        {
            self.scan_at(self.pos + 2);
            self.push_token(Token::And)?;

            Ok(true)
        } else {
            Ok(false)
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

    /// 分析是否结束。
    pub fn is_end(&self) -> bool {
        self.pos > self.input.len() - 1
    }

    /// 生成 token 与数据（引用）的映射序列。
    pub fn token_data(&self) -> Result<Vec<(&Token, &[char])>> {
        let mut mapping = vec![];

        for (i, token) in self.tokens.iter().enumerate() {
            if let Some(position) = self.positions.get(i) {
                let data = &self.input[position.begin..position.end];

                mapping.push((token, data));
            } else {
                return Err(Error::MissingTokenPosition {
                    position: i + 1,
                    token: *token,
                });
            }
        }

        Ok(mapping)
    }

    /// 生成 token 与数据的映射序列。
    pub fn token_data_owner(&self) -> Result<Vec<(Token, String)>> {
        let mut mapping = vec![];

        for (token, data) in self.token_data()? {
            mapping.push((*token, data.to_vec().iter().collect()));
        }

        Ok(mapping)
    }

    /// 生成数据（引用）序列。
    pub fn data(&self) -> Vec<&[char]> {
        let mut sequence = vec![];

        for position in self.positions.iter() {
            sequence.push(&self.input[position.begin..position.end]);
        }

        sequence
    }

    /// 生成数据序列。
    pub fn data_owner(&self) -> Vec<String> {
        let mut sequence = vec![];

        for position in self.positions.iter() {
            sequence.push(self.input[position.begin..position.end].iter().collect());
        }

        sequence
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
