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
/// 每个规则对象都是一系列有顺序的单元集合。
///
/// 手动创建一个规则对象：
/// ```
/// use matchingram::rule::*;
///
/// let groups = vec![
///     vec![Cont {
///         field: Field::MessageText,
///         operator: Operator::ContainsOne,
///         value: Value::Multi(vec!["hello".to_owned(), "world".to_owned()]),
///     }],
///     vec![Cont {
///         field: Field::MessageText,
///         operator: Operator::ContainsAll,
///         value: Value::Multi(vec!["hello".to_owned(), "bye".to_owned()]),
///     }],
/// ];
/// let rule = Rule::new(groups);
/// ```
/// 它对应的字符串表达式为：
/// ```text
/// (message.text contains_one {hello world}) or (message.text contains_all {hello bye})
/// ```
/// **注意**：结构化的规则中没有“关系”存在，因为规则中每一个独立的组之间一定是 `or` 关系，组内的条件之间一定是 `and` 关系。即：已存在隐式的关系表达。
#[derive(Debug, Default)]
pub struct Rule {
    /// 单元集合
    pub groups: Vec<Vec<Cont>>,
    // 上一组的匹配结果
    pub last_is_matching: bool,
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
    pub value: Value,
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

/// 条件值。
#[derive(Debug, Clone)]
pub enum Value {
    /// 单值（字符串）
    Single(String),
    /// 多值（字符串）
    Multi(Vec<String>),
}

impl Rule {
    pub fn match_message(&mut self, message: &Message) -> Result<bool> {
        self.loop_match(message, 0)
    }

    fn loop_match(&mut self, message: &Message, position: usize) -> Result<bool> {
        if position > 0 && self.last_is_matching {
            return Ok(true);
        }
        if position >= (self.groups.len() - 1) {
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
                    let values = match self.value.clone() {
                        Value::Single(value) => vec![value],
                        Value::Multi(values) => values,
                    };
                    match self.operator {
                        Operator::ContainsOne => {
                            let mut result = false;
                            for v in values {
                                if text.contains(&v) {
                                    result = true;
                                    break;
                                }
                            }

                            Ok(result)
                        }
                        Operator::ContainsAll => {
                            let mut result = true;
                            for v in values {
                                if !text.contains(&v) {
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
