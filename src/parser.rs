//! 规则表达式的文法分析实现。
//!
//! 产生式：
//! ```text
//! 规则 -> 条件组 可选条件组列表 <EOF>
//! 条件组 -> <(> 条件 可选条件列表 <)>
//! 条件 -> 未取反条件 | <not> 未取反条件
//! 未取反条件 -> <字段> <运算符> 值表示
//! 值表示 -> 单值表示 | 多值表示
//! 多值表示 -> <{> 单值表示 单值表示 ... <}>
//! 单值表示 -> <"> <letter> <"> | <decimal>
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
//! let rule = r#"(message.text contains_one {"柬埔寨" "东南亚"} and message.text contains_one {"菠菜" "博彩"}) or (message.text contains_all {"承接" "广告"})"#;
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
use super::matcher::{Cont, Groups as ContGroups, Matcher, Value};
use super::result::Result;

use derivative::Derivative;

type Input = Vec<Token>;

/// 文法分析器。
/// 解析 [`Lexer::Token`](../lexer/enum.Token.html) 序列生成 [`Matcher`](../matcher/struct.Matcher.html) 对象。
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Parser<'a> {
    /// 输入（token 序列）。
    pub input: &'a Input,
    /// 数据序列。
    pub data: Vec<&'a [char]>,
    /// 位置序列。
    #[derivative(Debug = "ignore")]
    positions: &'a Vec<Position>,
    // 当前的指针位置。
    pos: usize,
    // 当前的 token（current token）。
    pub ct: Option<&'a Token>,
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
            ct: input.get(0),
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
        }

        self.scan();
        if self.ct != Some(&Token::EOF) {
            let position = self.current_position()?;
            return Err(Error::ShouldEndHere {
                column: position.begin,
            });
        }

        Ok(Matcher::new(groups))
    }

    fn parse_group(&mut self) -> Result<Vec<Cont>> {
        if self.ct != Some(&Token::OpenParenthesis) {
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
            conts.append(&mut optinal_conts);
        }
        self.scan();

        if self.ct != Some(&Token::CloseParenthesis) {
            let position = self.current_position()?;

            return Err(Error::ShouldCloseParenthesisHere {
                column: position.begin,
            });
        }

        Ok(conts)
    }

    fn parse_optinal_cont_list(&mut self, mut conts: Vec<Cont>) -> Result<Vec<Cont>> {
        if conts.len() > 0 {
            self.scan();
        }

        if self.ct != Some(&Token::And) {
            self.back();
            return Ok(conts);
        }

        self.scan();
        conts.push(self.parse_cont()?);

        self.parse_optinal_cont_list(conts)
    }

    fn parse_optinal_group_list(&mut self, mut groups: ContGroups) -> Result<ContGroups> {
        if groups.len() > 0 {
            self.scan();
        }

        if self.ct != Some(&Token::Or) {
            self.back();
            return Ok(groups);
        }

        self.scan();
        groups.push(self.parse_group()?);

        self.parse_optinal_group_list(groups)
    }

    fn parse_cont(&mut self) -> Result<Cont> {
        let is_negative = if self.ct == Some(&Token::Not) {
            self.scan();

            true
        } else {
            false
        };

        if self.ct != Some(&Token::Field) {
            let position = self.current_position()?;
            return Err(Error::MissingField {
                column: position.begin,
            });
        }
        let field = self.current_data()?.iter().collect();

        self.scan();
        if self.ct != Some(&Token::Operator) {
            let position = self.current_position()?;
            return Err(Error::MissingOperator {
                column: position.begin,
            });
        }
        let operator = self.current_data()?.iter().collect();

        self.scan();
        let value = self.parse_value()?;

        Ok(Cont::new(is_negative, field, operator, value)?)
    }

    fn parse_value(&mut self) -> Result<Vec<Value>> {
        // 匹配多值
        if self.ct == Some(&Token::OpenBrace) {
            let mut value = vec![];

            self.scan();
            while self.ct != Some(&Token::CloseBrace) {
                value.push(self.prase_single_value()?);
                self.scan();
            }

            Ok(value)
        } else {
            let value = self.prase_single_value()?;

            Ok(vec![value])
        }
    }

    fn prase_single_value(&mut self) -> Result<Value> {
        let position = self.current_position()?;

        if self.ct == Some(&Token::Decimal) {
            let value_data = self.at_data(self.pos)?;
            let value_decimal =
                i64::from_str_radix(value_data.iter().collect::<String>().as_str(), 10).map_err(
                    |_| Error::DecimalParseFailed {
                        column: position.begin,
                    },
                )?;

            self.scan();

            return Ok(Value::Decimal(value_decimal));
        }

        if self.ct == Some(&Token::Quote)
            && self.input.get(self.pos + 1) == Some(&Token::Letter)
            && self.input.get(self.pos + 2) == Some(&Token::Quote)
        {
            let value_data = self.at_data(self.pos + 1)?;

            self.scan_at(self.pos + 2);

            return Ok(Value::Letter(value_data.iter().collect()));
        }

        return Err(Error::ShouldValueHere {
            column: position.begin,
        });
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

    // 扫描下一个 token，并向后移动一个指针位置。
    fn scan(&mut self) -> Option<&Token> {
        self.pos += 1;
        self.ct = self.input.get(self.pos);

        self.ct
    }

    // 回到上一个 token。
    fn back(&mut self) -> Option<&Token> {
        self.pos -= 1;
        self.ct = self.input.get(self.pos);

        self.ct
    }

    // 扫描指定位置的 token，并移动指针到该位置。
    fn scan_at(&mut self, pos: usize) -> Option<&Token> {
        self.pos = pos;
        self.ct = self.input.get(self.pos);

        self.ct
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
