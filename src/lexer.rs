//! 规则表达式的词法分析实现。
//!
//! 词法分析器并不保证语法上的正确性，但仍然具备一定的错误识别能力。例如通过前后关系才能确定的”字段“和”操作符“两个 token 类型。
//! 一个例子：
//! ```
//! use matchingram::lexer::Lexer;
//! use matchingram::lexer::Token::*;
//!
//! let rule = r#"(message.text all "bye" and message.text any {"parent" "world"}) or (message.text any {"see" "you"})"#;
//! let input = rule.chars().collect::<Vec<_>>();
//!
//! let mut lexer = Lexer::new(&input);
//! lexer.tokenize()?;
//!
//! let truthy = [
//!     (OpenParenthesis, String::from("(")),
//!     (Field, String::from("message.text")),
//!     (Operator, String::from("all")),
//!     (Quote, String::from("\"")),
//!     (Letter, String::from("bye")),
//!     (Quote, String::from("\"")),
//!     (And, String::from("and")),
//!     (Field, String::from("message.text")),
//!     (Operator, String::from("any")),
//!     (OpenBrace, String::from("{")),
//!     (Quote, String::from("\"")),
//!     (Letter, String::from("parent")),
//!     (Quote, String::from("\"")),
//!     (Quote, String::from("\"")),
//!     (Letter, String::from("world")),
//!     (Quote, String::from("\"")),
//!     (CloseBrace, String::from("}")),
//!     (CloseParenthesis, String::from(")")),
//!     (Or, String::from("or")),
//!     (OpenParenthesis, String::from("(")),
//!     (Field, String::from("message.text")),
//!     (Operator, String::from("any")),
//!     (OpenBrace, String::from("{")),
//!     (Quote, String::from("\"")),
//!     (Letter, String::from("see")),
//!     (Quote, String::from("\"")),
//!     (Quote, String::from("\"")),
//!     (Letter, String::from("you")),
//!     (Quote, String::from("\"")),
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
    /// 文字值。
    Letter,
    /// 整数。
    Integer,
    /// 小数。
    Decimal,
    /// and 关键字。
    And, // and
    /// or 关键字。
    Or, // or
    /// not 关键字。
    Not, // not
    /// 结束
    EOF,
}

type Input = [char];

/// 词法分析器。
#[derive(Debug)]
pub struct Lexer<'a> {
    /// 输入。
    pub input: &'a Input,
    // 当前指针位置。
    pos: usize,
    // 当前字符。
    cc: Option<&'a char>,
    // 全部的 Token。
    tokens: Vec<Token>,
    // 位置序列。
    positions: Vec<Position>,
    // 是否处在引号内部。
    is_inside_quotes: bool,
}

#[derive(Debug)]
pub struct Position {
    pub begin: usize,
    pub end: usize,
}

impl<'a> Lexer<'a> {
    /// 以字符序列作为输入创建分析器。
    pub fn new(input: &'a Input) -> Self {
        Self {
            input: input,
            pos: 0,
            cc: input.get(0),
            tokens: vec![],
            positions: vec![],
            is_inside_quotes: false,
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

    /// 将输入转换为 token 序列。
    pub fn tokenize(&mut self) -> Result<()> {
        while self.cc.is_some() {
            self.skip_white_space();
            if let Some(cc) = self.cc {
                match cc {
                    '(' => {
                        self.push_token(Token::OpenParenthesis)?;
                        self.scan();
                        self.skip_white_space();
                        if !self.scan_field()? {
                            return Err(Error::MissingField {
                                column: self.pos + 1,
                            });
                        }
                        self.scan();
                        self.skip_white_space();
                        if !self.scan_operator()? {
                            self.back();
                        }
                    }
                    ')' => self.push_token(Token::CloseParenthesis)?,
                    '{' => self.push_token(Token::OpenBrace)?,
                    '}' => self.push_token(Token::CloseBrace)?,
                    '"' => {
                        self.is_inside_quotes = !self.is_inside_quotes;
                        self.push_token(Token::Quote)?;
                        if self.is_inside_quotes {
                            self.scan();
                            if !self.scan_letter()? {
                                return Err(Error::MissingQuote {
                                    column: self.pos + 1,
                                });
                            }
                        }
                    }
                    _ => {
                        if !self.scan_keywords()? && !self.scan_number()? {
                            return Err(Error::ParseFailed {
                                column: self.pos + 1,
                            });
                        }
                    }
                }

                self.scan();
            }
        }

        self.push_token(Token::EOF)?;

        Ok(())
    }

    // 扫描关键字。
    fn scan_keywords(&mut self) -> Result<bool> {
        if let Some(cc) = self.cc {
            match cc {
                'a' => {
                    if self.tokenize_and()? {
                        self.scan();
                        self.skip_white_space();
                        if !self.scan_field()? {
                            return Err(Error::MissingField {
                                column: self.pos + 1,
                            });
                        }
                        self.scan();
                        self.skip_white_space();
                        if !self.scan_operator()? {
                            self.back();
                        }

                        Ok(true)
                    } else {
                        Ok(false)
                    }
                }
                'o' => self.tokenize_or(),
                'n' => self.tokenize_not(),
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    // 扫描数字
    // 包括整数、小数
    // TODO: 支持符合扫描（负数）。
    fn scan_number(&mut self) -> Result<bool> {
        let begin_pos = self.pos;
        let mut end_pos = begin_pos;

        while self.at_char(end_pos).is_integer() {
            end_pos += 1;
        }

        let end_char = self.at_char(end_pos);
        let is_integer = end_pos > begin_pos
            // 检查是否合法结束
            && (end_char.is_white_space() || match end_char {
                Some(&'}') => true,
                Some(&')') => true,
                _ => false,
            });

        if is_integer {
            self.scan_at(end_pos - 1);
            self.push_token_position(
                Token::Integer,
                Position {
                    begin: begin_pos,
                    end: end_pos,
                },
            );

            Ok(true)
        } else if end_char == Some(&'.') {
            // 可能是小数
            let mut con_pos = end_pos + 1; // 继续扫描的位置

            while self.at_char(con_pos).is_integer() {
                con_pos += 1;
            }

            let end_char = self.at_char(con_pos);
            let is_decimal = con_pos > end_pos + 1
            // 检查是否合法结束
            && (end_char.is_white_space() || match end_char {
                Some(&'}') => true,
                Some(&')') => true,
                _ => false,
            });

            if is_decimal {
                self.scan_at(con_pos - 1);
                self.push_token_position(
                    Token::Decimal,
                    Position {
                        begin: begin_pos,
                        end: con_pos,
                    },
                );

                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    // 扫描字面值（字符串）
    fn scan_letter(&mut self) -> Result<bool> {
        // 如果不在引号内部，则不扫描。
        if !self.is_inside_quotes {
            return Ok(false);
        }

        let begin_pos = self.pos;
        let mut end_pos = begin_pos;
        let mut separator = self.at_char(end_pos);
        // 如果没有被双引号或换行截断，继续扫描
        while separator.is_some() && separator != Some(&'"') && separator != Some(&'\n') {
            end_pos += 1;
            separator = self.at_char(end_pos);
        }

        // 合法结束检查条件：以双引号截断（而不是换行）
        let is_letter = end_pos > begin_pos && separator == Some(&'"');

        if is_letter {
            self.scan_at(end_pos - 1);
            self.push_token_position(
                Token::Letter,
                Position {
                    begin: begin_pos,
                    end: end_pos,
                },
            );

            Ok(true)
        } else {
            Ok(false)
        }
    }

    // 追加一个 Token。
    fn push_token(&mut self, token: Token) -> Result<()> {
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
            Not => Position {
                begin: self.pos - 2,
                end: self.pos + 1,
            },
            EOF => Position {
                begin: self.pos,
                end: self.pos,
            },
            _ => return Err(Error::InferPositionFailed { token: token }),
        };
        self.push_token_position(token, position);

        Ok(())
    }

    fn push_token_position(&mut self, token: Token, position: Position) {
        self.positions.push(position);
        self.tokens.push(token);
    }

    fn tokenize_not(&mut self) -> Result<bool> {
        if self.at_char(self.pos + 1) == Some(&'o')
            && self.at_char(self.pos + 2) == Some(&'t')
            && self.at_char(self.pos + 3).is_white_space()
        {
            self.scan_at(self.pos + 2);
            self.push_token(Token::Not)?;

            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn scan_field(&mut self) -> Result<bool> {
        if self.cc == Some(&'n') && self.tokenize_not()? {
            self.scan();
            self.skip_white_space();
        }
        let begin_pos = self.pos;
        let mut cur_pos = begin_pos;
        let mut end_char = self.at_char(cur_pos);

        while end_char != Some(&')') && !end_char.is_white_space() {
            cur_pos += 1;
            end_char = self.at_char(cur_pos);
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
            self.scan_at(cur_pos - 1);
        }

        Ok(is_field)
    }

    fn scan_operator(&mut self) -> Result<bool> {
        let begin_pos = self.pos;
        let mut cur_pos = begin_pos;
        let mut end_char = self.at_char(cur_pos);

        if self.cc == Some(&'a') && self.is_and_keywords() {
            return Ok(false);
        }

        while end_char != Some(&')') && !end_char.is_white_space() {
            cur_pos += 1;
            end_char = self.at_char(cur_pos);
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
            self.scan_at(cur_pos - 1);
        }

        Ok(is_operator)
    }

    fn tokenize_or(&mut self) -> Result<bool> {
        if self.at_char(self.pos + 1) == Some(&'r') && self.at_char(self.pos + 2).is_white_space() {
            self.scan_at(self.pos + 1);
            self.push_token(Token::Or)?;

            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn tokenize_and(&mut self) -> Result<bool> {
        if self.is_and_keywords() {
            self.scan_at(self.pos + 2);
            self.push_token(Token::And)?;

            Ok(true)
        } else {
            Ok(false)
        }
    }

    // 当前位置是否是 `and` 关键字。
    fn is_and_keywords(&self) -> bool {
        self.at_char(self.pos + 1) == Some(&'n')
            && self.at_char(self.pos + 2) == Some(&'d')
            && self.at_char(self.pos + 3).is_white_space()
    }

    // 扫描下一个字符并自增指针位置。
    fn scan(&mut self) -> Option<&char> {
        self.pos += 1;
        self.cc = self.input.get(self.pos);

        self.cc
    }

    // 回退一个字符并自减指针位置。
    fn back(&mut self) -> Option<&char> {
        self.pos -= 1;
        self.cc = self.input.get(self.pos);

        self.cc
    }

    // 访问指定位置的字符。此方法不移动位置。
    fn at_char(&self, pos: usize) -> Option<&char> {
        self.input.get(pos)
    }

    // 扫描指定位置的字符。此方法将改变指针位置。
    fn scan_at(&mut self, pos: usize) -> Option<&char> {
        self.pos = pos;
        self.cc = self.input.get(self.pos);

        self.cc
    }

    // 跳过空白字符。返回跳过的字符数量。
    fn skip_white_space(&mut self) -> usize {
        let mut begin_pos = self.pos;
        while self.cc.is_white_space() {
            self.scan();
            begin_pos += 1;
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

trait IsInteger {
    fn is_integer(self) -> bool;
}

impl IsWhiteSpace for Option<&char> {
    fn is_white_space(self) -> bool {
        if let Some(c) = self {
            c == &' ' || c == &'\n'
        } else {
            false
        }
    }
}

impl IsInteger for Option<&char> {
    fn is_integer(self) -> bool {
        if let Some(c) = self {
            c.is_digit(10)
        } else {
            false
        }
    }
}
