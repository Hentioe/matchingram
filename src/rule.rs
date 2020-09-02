//! Implementation of the rules.
//!
//! Cloudflare 防火墙规则分析：每一个条件由“字段” + “运算符” + “值” 构成。条件和条件之间可具备 `and` 或 `or` 的关系，不能嵌套。
//! 字段通过点（`.`）进行分类。运算符使用 snake_case 风格命名。多值使用大括号（`{}`）包裹以及空格分隔，单值使用引号（`""`）包裹。
//! 相邻的具有 `and` 关系的条件会被归纳到同一个括号中，但相邻的 `or` 关系的条件之间彼此独立。
//! 一个具体的例子：
//! ```text
//! (ip.src in {1.1.1.1 192.168.1.1} and http.request.uri.query contains "page") or (ip.geoip.country eq "AF") or (http.request.method eq "POST")
//! ```
//! 本项目的规则的风格将与之完全一致。

use super::error::Error;
use super::models::Message;
use super::result::Result;

/// 结构化的规则内容。
///
/// 负责解析字符串表达式并表达多个结构化的条件关系。
/// 每个规则对象都是一系列有顺序的条件组集合。
/// ```
/// use matchingram::models::Message;
/// use matchingram::rule::*;
///
/// // 手动创建一个规则对象：
/// let groups = vec![
///     vec![
///         Cont {
///             field: Field::MessageText,
///             operator: Operator::ContainsOne,
///             value: vec!["柬埔寨".to_owned(), "东南亚".to_owned()],
///         },
///         Cont {
///             field: Field::MessageText,
///             operator: Operator::ContainsOne,
///             value: vec!["菠菜".to_owned(), "博彩".to_owned()],
///         },
///     ],
///     vec![Cont {
///         field: Field::MessageText,
///         operator: Operator::ContainsAll,
///         value: vec!["承接".to_owned(), "广告".to_owned()],
///     }],
/// ];
/// let mut rule = Rule::new(groups).unwrap();
/// // 两条典型的东南亚博彩招人消息
/// let message_text1 = format!("柬埔寨菠菜需要的来");
/// let message_text2 = format!("东南亚博彩招聘");
/// // 一条业务宣传消息
/// let message_text3 = format!("承接博彩广告业务");
///
/// let message1 = Message {
///     text: Some(message_text1),
///     ..Default::default()
/// };
/// let message2 = Message {
///     text: Some(message_text2),
///     ..Default::default()
/// };
/// let message3 = Message {
///     text: Some(message_text3),
///     ..Default::default()
/// };
///
/// assert!(matches!(rule.match_message(&message1), Ok(true)));
/// assert!(matches!(rule.match_message(&message2), Ok(true)));
/// assert!(matches!(rule.match_message(&message3), Ok(true)));
/// ```
/// 它对应的字符串表达式为：
/// ```text
/// (message.text contains_one {柬埔寨 东南亚} and message.text contains_one {菠菜 博彩}) or (message.text contains_all {承接 广告})
/// ```
/// **注意**：结构化的规则中没有“关系”存在，因为规则中每一个独立的组之间一定是 `or` 关系，组内的条件之间一定是 `and` 关系。即：已存在隐式的关系表达。
#[derive(Debug, Default)]
pub struct Rule {
    /// 条件组集合。
    pub groups: Vec<Vec<Cont>>,
    // 上一组的匹配结果
    last_is_matching: bool,
}

impl Rule {
    /// 解析字符串表达式创建规则对象，字符串将被扩展为具有特定的结构的规则对象。
    /// 规则对象匹配将具有更快的速度，因为不需要再次对表达式进行扩展。
    pub fn prase<S: Into<String>>(_expression: S) -> Result<Self> {
        let rule = Rule {
            groups: vec![],
            last_is_matching: true,
        };

        Ok(rule)
    }

    /// 使用条件组创建规则对象。
    pub fn new(groups: Vec<Vec<Cont>>) -> Result<Self> {
        let rule = Rule {
            groups: groups,
            last_is_matching: true,
        };

        Ok(rule)
    }
}

/// 单个条件。
#[derive(Debug)]
pub struct Cont {
    /// 字段。
    pub field: Field,
    /// 运算符。
    pub operator: Operator,
    /// 值。
    pub value: Vec<String>,
}

/// 条件字段。
#[derive(Debug)]
pub enum Field {
    /// 消息文本
    MessageText,
}

/// 条件操作符。
#[derive(Debug)]
pub enum Operator {
    // 等于。
    Eq,
    /// 包含其一。
    ContainsOne,
    /// 包含全部。
    ContainsAll,
}

impl Rule {
    pub fn match_message(&mut self, message: &Message) -> Result<bool> {
        self.loop_match(message, 0)
    }

    fn loop_match(&mut self, message: &Message, position: usize) -> Result<bool> {
        if position > 0 && self.last_is_matching {
            return Ok(true);
        }
        if position > (self.groups.len() - 1) {
            return Ok(self.last_is_matching);
        }

        let conts = unsafe { self.groups.get_unchecked(position) };

        let mut result = true;
        for cont in conts {
            if !cont.match_message(message)? {
                result = false;
                break;
            }
        }
        self.last_is_matching = result;
        self.loop_match(message, position + 1)
    }
}

impl Cont {
    pub fn match_message(&self, message: &Message) -> Result<bool> {
        match self.field {
            Field::MessageText => {
                if let Some(text) = message.text.as_ref() {
                    match self.operator {
                        Operator::ContainsOne => {
                            let mut result = false;
                            for v in &self.value {
                                if text.contains(v) {
                                    result = true;
                                    break;
                                }
                            }

                            Ok(result)
                        }
                        Operator::ContainsAll => {
                            let mut result = true;
                            for v in &self.value {
                                if !text.contains(v) {
                                    result = false;
                                    break;
                                }
                            }

                            Ok(result)
                        }
                        _ => Err(Error::UnsupportedOperator {
                            field: self.field.to_string(),
                            operator: self.operator.to_string(),
                        }),
                    }
                } else {
                    Ok(false)
                }
            }
        }
    }
}

impl ToString for Field {
    fn to_string(&self) -> String {
        match self {
            Field::MessageText => format!("message.text"),
        }
    }
}

impl ToString for Operator {
    fn to_string(&self) -> String {
        match self {
            Operator::Eq => format!("eq"),
            Operator::ContainsAll => format!("contains_all"),
            Operator::ContainsOne => format!("contains_one"),
        }
    }
}
