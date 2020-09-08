//! 规则表达式的词法分析实现。
//!
//! 词法分析器并不保证语法上的正确性，但仍然具备一定的错误识别能力。例如通过前后关系才能确定的”字段“和”操作符“两个 token 类型。
//! 一个例子：
//! ```
//! use matchingram::lexer::Lexer;
//! use matchingram::lexer::Token::*;
//!
//! let rule = r#"(message.text contains_all "bye" and message.text contains_one {"parent" "world"}) or (message.text contains_one {"see" "you"})"#;
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
//!     (Letter, String::from("bye")),
//!     (Quote, String::from("\"")),
//!     (And, String::from("and")),
//!     (Field, String::from("message.text")),
//!     (Operator, String::from("contains_one")),
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
//!     (Operator, String::from("contains_one")),
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
    /// 十进制数字值。
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
    // 当前指针。
    pos: usize,
    // 当前字符。
    current_char: Option<&'a char>,
    // 全部的 Token。
    tokens: Vec<Token>,
    // 位置序列。
    positions: Vec<Position>,
    // 是否处在引号内部。
    is_inside_quote: bool,
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
            current_char: input.get(0),
            tokens: vec![],
            positions: vec![],
            is_inside_quote: false,
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
        while self.current_char.is_some() {
            self.skip_white_space();
            if let Some(cc) = self.current_char {
                match cc {
                    '(' => {
                        self.push_token(Token::OpenParenthesis)?;
                        if self.skip_white_space() == 0 {
                            self.scan();
                        }
                        if !self.scan_field()? {
                            return Err(Error::MissingField {
                                column: self.pos + 1,
                            });
                        }
                        self.scan();
                        if self.skip_white_space() == 0 {
                            self.scan();
                        }
                        if !self.scan_operator()? {
                            return Err(Error::MissingOperator {
                                column: self.pos + 1,
                            });
                        }
                    }
                    ')' => self.push_token(Token::CloseParenthesis)?,
                    '{' => self.push_token(Token::OpenBrace)?,
                    '}' => self.push_token(Token::CloseBrace)?,
                    '"' => {
                        self.is_inside_quote = !self.is_inside_quote;
                        self.push_token(Token::Quote)?;
                        if self.is_inside_quote {
                            if self.skip_white_space() == 0 {
                                self.scan();
                            }
                            if !self.scan_letter()? {
                                return Err(Error::MissingQuote {
                                    column: self.pos + 1,
                                });
                            }
                        }
                    }
                    _ => {
                        if !self.scan_keywords()? && !self.scan_decimal()? {
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
        if let Some(cc) = self.current_char {
            match cc {
                'a' => {
                    if self.tokenize_and()? {
                        self.scan();
                        if self.skip_white_space() == 0 {
                            self.scan();
                        }
                        if !self.scan_field()? {
                            return Err(Error::MissingField {
                                column: self.pos + 1,
                            });
                        }
                        self.scan();
                        if self.skip_white_space() == 0 {
                            self.scan();
                        }
                        if !self.scan_operator()? {
                            return Err(Error::MissingOperator {
                                column: self.pos + 1,
                            });
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

    // 扫描数字。
    fn scan_decimal(&mut self) -> Result<bool> {
        let begin_pos = self.pos;
        let mut end_pos = begin_pos;

        while self.at_char(end_pos).is_decimal() {
            end_pos += 1;
        }

        let end_char = self.at_char(end_pos);
        let is_decimal = end_pos > begin_pos
            // 检查是否合法结束
            && match end_char {
                Some(&' ') => true,
                Some(&'\n') => true,
                Some(&'}') => true,
                Some(&')') => true,
                _ => false,
            };

        if is_decimal {
            self.scan_at(end_pos - 1);
            self.push_token_position(
                Token::Decimal,
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

    // 扫描字面值（字符串）
    fn scan_letter(&mut self) -> Result<bool> {
        // 如果不在引号内部，则不扫描。
        if !self.is_inside_quote {
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
        if self.current_char == Some(&'n') && self.tokenize_not()? {
            self.scan();
            if self.skip_white_space() == 0 {
                self.scan();
            }
        }
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
            self.scan_at(cur_pos - 1);
        }

        Ok(is_field)
    }

    fn scan_operator(&mut self) -> Result<bool> {
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

trait IsDecimal {
    fn is_decimal(self) -> bool;
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

impl IsDecimal for Option<&char> {
    fn is_decimal(self) -> bool {
        if let Some(c) = self {
            c.is_digit(10)
        } else {
            false
        }
    }
}

#[test]
fn test_lexer() {
    use Token::*;

    let rule = r#"(not message.text contains_one {"say:" "说："} and message.text.size gt 1234567890) or (message.text eq "/say")"#;
    let input = rule.chars().collect::<Vec<_>>();

    let mut lexer = Lexer::new(&input);
    lexer.tokenize().unwrap();

    let truthy = [
        (OpenParenthesis, String::from("(")),
        (Not, String::from("not")),
        (Field, String::from("message.text")),
        (Operator, String::from("contains_one")),
        (OpenBrace, String::from("{")),
        (Quote, String::from("\"")),
        (Letter, String::from("say:")),
        (Quote, String::from("\"")),
        (Quote, String::from("\"")),
        (Letter, String::from("说：")),
        (Quote, String::from("\"")),
        (CloseBrace, String::from("}")),
        (And, String::from("and")),
        (Field, String::from("message.text.size")),
        (Operator, String::from("gt")),
        (Decimal, String::from("1234567890")),
        (CloseParenthesis, String::from(")")),
        (Or, String::from("or")),
        (OpenParenthesis, String::from("(")),
        (Field, String::from("message.text")),
        (Operator, String::from("eq")),
        (Quote, String::from("\"")),
        (Letter, String::from("/say")),
        (Quote, String::from("\"")),
        (CloseParenthesis, String::from(")")),
        (EOF, String::from("")),
    ];

    assert_eq!(truthy.len(), lexer.output().len());
    for (i, mapping) in lexer.token_data_owner().unwrap().into_iter().enumerate() {
        assert_eq!(truthy[i], mapping);
    }
}
