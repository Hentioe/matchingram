//! 规则表达式的文法分析实现。
//!

// use super::error::Error;
use super::lexer::{Lexer, Position, Token};
use super::matcher::Matcher;
use super::result::Result;

type Input = Vec<Token>;

/// 文法分析器。
/// 解析 [`Lexer::Token`](../lexer/enum.Token.html) 序列生成 [`Matcher`](../matcher/struct.Matcher.html)。
#[derive(Debug)]
pub struct Parser<'a> {
    /// 输入。
    pub input: &'a Input,
    /// 数据序列。
    pub data: Vec<&'a [char]>,
    /// 位置信息
    positions: &'a Vec<Position>,
    // 指针位置。
    pos: usize,
    // 当前 token。
    pub current_token: Option<&'a Token>,
}

impl<'a> Parser<'a> {
    /// 创建一个解析器。
    pub fn from_lexer(lexer: &'a Lexer<'a>) -> Result<Self> {
        Ok(Parser {
            input: &lexer.tokens,
            data: lexer
                .token_data()?
                .into_iter()
                .map(|(_, data)| data)
                .collect(),
            positions: &lexer.positions,
            pos: 0,
            current_token: lexer.tokens.get(0),
        })
    }

    /// 解析并得到匹配器。
    pub fn parse(&mut self) -> Result<Matcher> {
        let groups = vec![];

        Ok(Matcher::new(groups))
    }

    // // 扫描下一个 token。此方法会将指针向后移动一位。
    // fn scan(&mut self) -> Option<&Token> {
    //     self.pos += 1;
    //     self.current_token = self.input.get(self.pos);

    //     self.current_token
    // }

    // fn get_current_position(&self) -> &Position {
    //     if let Some(position) = self.positions.get(self.pos) {
    //         position
    //     } else {
    //         self.positions.last().unwrap()
    //     }
    // }

    // // 访问指定位置的 token。此方法不移动指针位置。
    // fn at_token(&self, pos: usize) -> Option<&Token> {
    //     self.input.get(pos)
    // }

    // // 扫描指定位置的指针。此方法会将指针移动到指定位置。
    // fn scan_at(&mut self, pos: usize) -> Option<&Token> {
    //     self.pos = pos;

    //     self.input.get(self.pos)
    // }

    // // 回退到上一个 token。此方法会将指针向前移动一位。
    // fn back(&mut self) -> Option<&Token> {
    //     self.pos -= 1;

    //     self.input.get(self.pos)
    // }
}

// #[test]
// fn test_parse() {
//     use super::lexer::Lexer;

//     let rule = "(message.text contains_all \"bye\" and message.text contains_one {parent world}) or (message.text contains_one {see you})";
//     let input = rule.chars().collect::<Vec<_>>();
//     let mut lexer = Lexer::new(&input);
//     lexer.tokenize().unwrap();
//     let mut parser = Parser::from_lexer(&lexer).unwrap();

//     parser.parse().unwrap();
// }
