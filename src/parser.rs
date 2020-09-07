//! 规则表达式的文法分析实现。
//!
//! 产生式：
//! ```text
//! 规则 -> 条件组 可选条件组列表 <EOF>
//! 条件组 -> <(> 条件 可选条件列表 <)>
//! 条件 -> 未取反条件 | <not> 未取反条件
//! 未取反条件 -> <字段> <运算符> 值表示
//! 值表示 -> <{> <值> <}> | <"> <值> <">
//! 可选条件列表 -> <and> 条件 可选条件列表 | <空>
//! 可选条件组列表 -> <or> 条件组 可选条件组列表 | <空>
//! ```
//!
//! 当前的实现基于递归下降算法，语法制导直接生成 [`Matcher`](../matcher/struct.Matcher.html) 对象。
//!
//! 一个使用案例：
//! ```
//! use matchingram::models::Message;
//! use matchingram::lexer::Lexer;
//! use matchingram::parser::Parser;
//!
//! // 准备规则，用于匹配东南亚博彩或广告业务宣传。
//! let rule = "(message.text contains_one {柬埔寨 东南亚} and message.text contains_one {菠菜 博彩}) or (message.text contains_all {承接 广告})";
//! // 解析规则并得到 matcher 对象。
//! let input = rule.chars().collect::<Vec<_>>();
//! let mut lexer = Lexer::new(&input);
//! let parser = Parser::new(&mut lexer)?;
//! let mut matcher = parser.parse()?;
//! // 两条典型的东南亚博彩招人消息。
//! let message_text1 = format!("柬埔寨菠菜需要的来");
//! let message_text2 = format!("东南亚博彩招聘");
//! // 一条业务宣传消息。
//! let message_text3 = format!("承接博彩广告业务");
//!
//! let message1 = Message {
//!     text: Some(message_text1),
//!     ..Default::default()
//! };
//! let message2 = Message {
//!     text: Some(message_text2),
//!     ..Default::default()
//! };
//! let message3 = Message {
//!     text: Some(message_text3),
//!     ..Default::default()
//! };
//!
//! assert!(matcher.match_message(&message1)?);
//! assert!(matcher.match_message(&message2)?);
//! assert!(matcher.match_message(&message3)?);
//! # Ok::<(), matchingram::Error>(())
//! ```

use super::error::Error;
use super::lexer::{Lexer, Position, Token};
use super::matcher::{Cont, Groups as ContGroups, Matcher};
use super::result::Result;

type Input = Vec<Token>;

/// 文法分析器。
/// 解析 [`Lexer::Token`](../lexer/enum.Token.html) 序列生成 [`Matcher`](../matcher/struct.Matcher.html) 对象。
#[derive(Debug)]
pub struct Parser<'a> {
    /// 输入（token 序列）。
    pub input: &'a Input,
    /// 数据序列。
    pub data: Vec<&'a [char]>,
    /// 位置序列。
    positions: &'a Vec<Position>,
    // 当前指针。
    pos: usize,
    // 当前 token。
    pub current_token: Option<&'a Token>,
}

impl<'a> Parser<'a> {
    /// 从词法分析器创建一个解析器。
    pub fn new(lexer: &'a mut Lexer<'a>) -> Result<Self> {
        // 确保已完成分析。
        if !lexer.is_end() {
            lexer.tokenize()?;
        }
        let input = lexer.output();

        Ok(Parser {
            input,
            data: lexer.data(),
            positions: lexer.positions(),
            pos: 0,
            current_token: input.get(0),
        })
    }

    /// 解析并得到匹配器对象。
    pub fn parse(mut self) -> Result<Matcher> {
        let mut groups: ContGroups = vec![];

        groups.push(self.parse_group()?);

        self.scan();
        let mut optinal_groups = self.parse_optinal_group_list(vec![])?;
        if optinal_groups.len() > 0 {
            groups.append(&mut optinal_groups);
            self.scan();
        }

        if self.current_token != Some(&Token::EOF) {
            let position = self.current_position()?;
            return Err(Error::ShouldEndHere {
                column: position.begin,
            });
        }

        Ok(Matcher::new(groups))
    }

    fn parse_group(&mut self) -> Result<Vec<Cont>> {
        if self.current_token != Some(&Token::OpenParenthesis) {
            let position = self.current_position()?;
            return Err(Error::ShouldOpenParenthesisHere {
                column: position.begin,
            });
        }

        let mut conts = vec![];

        self.scan();
        conts.push(self.parse_cont()?);

        self.scan();

        let mut optinal_conts = self.parse_optinal_cont_list(vec![])?;
        if optinal_conts.len() > 0 {
            self.scan();
            conts.append(&mut optinal_conts);
        }

        if self.current_token != Some(&Token::CloseParenthesis) {
            let position = self.current_position()?;
            return Err(Error::ShouldCloseParenthesisHere {
                column: position.begin,
            });
        }

        Ok(conts)
    }

    fn parse_optinal_cont_list(&mut self, mut conts: Vec<Cont>) -> Result<Vec<Cont>> {
        if self.current_token != Some(&Token::And) {
            return Ok(conts);
        }

        self.scan();
        conts.push(self.parse_cont()?);

        self.parse_optinal_cont_list(conts)
    }

    fn parse_optinal_group_list(&mut self, mut groups: ContGroups) -> Result<ContGroups> {
        if self.current_token != Some(&Token::Or) {
            return Ok(groups);
        }

        self.scan();
        groups.push(self.parse_group()?);

        self.parse_optinal_group_list(groups)
    }

    fn parse_cont(&mut self) -> Result<Cont> {
        let is_negate = if self.current_token == Some(&Token::Not) {
            self.scan();

            true
        } else {
            false
        };

        if self.current_token != Some(&Token::Field) {
            let position = self.current_position()?;
            return Err(Error::MissingField {
                column: position.begin,
            });
        }
        let field = self.current_data()?.iter().collect();

        self.scan();
        if self.current_token != Some(&Token::Operator) {
            let position = self.current_position()?;
            return Err(Error::MissingOperator {
                column: position.begin,
            });
        }
        let operator = self.current_data()?.iter().collect();

        self.scan();
        let value = self.parse_value()?;

        Ok(Cont::new(is_negate, field, operator, value)?)
    }

    fn parse_value(&mut self) -> Result<String> {
        match self.current_token {
            Some(Token::Quote) => {
                self.scan();
                if self.current_token != Some(&Token::Value) {
                    let position = self.current_position()?;
                    return Err(Error::MissingValue {
                        column: position.begin,
                    });
                }
                self.scan();
                if self.current_token != Some(&Token::Quote) {
                    let position = self.current_position()?;
                    return Err(Error::ShouldQuoteHere {
                        column: position.begin,
                    });
                }
            }

            Some(Token::OpenBrace) => {
                self.scan();
                if self.current_token != Some(&Token::Value) {
                    let position = self.current_position()?;
                    return Err(Error::MissingValue {
                        column: position.begin,
                    });
                }
                self.scan();
                if self.current_token != Some(&Token::CloseBrace) {
                    let position = self.current_position()?;
                    return Err(Error::ShouldCloseBraceHere {
                        column: position.begin,
                    });
                }
            }

            _ => {
                let position = self.current_position()?;
                return Err(Error::ShouldOpenBraceOrQuote {
                    column: position.begin,
                });
            }
        }

        let value = self.at_data(self.pos - 1)?.iter().collect();

        Ok(value)
    }

    // 当前位置的 token 数据引用。
    fn current_data(&self) -> Result<&'a [char]> {
        self.at_data(self.pos)
    }

    // 指定位置的 token 数据引用。
    fn at_data(&self, pos: usize) -> Result<&'a [char]> {
        self.data
            .get(pos)
            .cloned()
            .ok_or(Error::MissingTokenData { index: self.pos })
    }

    // 扫描下一个 token。此方法会将指针向后移动一位。
    fn scan(&mut self) -> Option<&Token> {
        self.pos += 1;
        self.current_token = self.input.get(self.pos);

        self.current_token
    }

    // 当前 token 的位置信息。
    fn current_position(&self) -> Result<&Position> {
        if let Some(position) = self.positions.get(self.pos) {
            Ok(position)
        } else {
            Err(Error::MissingPosition { index: self.pos })
        }
    }
}

#[test]
fn test_not_cont() {
    use super::models::Message;

    let rule = "(not message.text contains_one {say: 说：})";
    let input = rule.chars().collect::<Vec<_>>();

    let mut lexer = Lexer::new(&input);
    let parser = Parser::new(&mut lexer).unwrap();
    let mut matcher = parser.parse().unwrap();

    let text1 = format!("Jay say: Hello!");
    let text2 = format!("小明说：你好！");
    let text3 = format!("怎么发消息还得遵循格式啊？");

    let message1 = Message {
        text: Some(text1),
        ..Default::default()
    };
    let message2 = Message {
        text: Some(text2),
        ..Default::default()
    };
    let message3 = Message {
        text: Some(text3),
        ..Default::default()
    };

    assert!(!matcher.match_message(&message1).unwrap());
    assert!(!matcher.match_message(&message2).unwrap());
    assert!(matcher.match_message(&message3).unwrap());
}
