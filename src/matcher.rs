//! 消息匹配实现。

use phf::phf_map;
use std::str::FromStr;
use strum_macros::{EnumString, ToString};

use super::error::Error;
use super::models::Message;
use super::result::Result;

pub type Groups = Vec<Vec<Cont>>;

pub static FIELD_OPERATORS: phf::Map<&'static str, &'static [Operator]> = phf_map! {
    "message.text" =>  &[Operator::Eq, Operator::ContainsOne, Operator::ContainsAll]
};

/// 匹配器。一般作为表达式的编译目标。
///
/// 匹配器可表达与字符串规则完全对应的结构化的条件关系。
/// 每个匹配器对象都具备一个“条件组”序列。
/// ```
/// use matchingram::models::Message;
/// use matchingram::matcher::*;
///
/// // 手动创建一个匹配器对象：
/// let groups = vec![
///     vec![
///         Cont {
///             is_negate: false,
///             field: Field::MessageText,
///             operator: Operator::ContainsOne,
///             value: vec![Value::from_str("柬埔寨"), Value::from_str("东南亚")],
///         },
///         Cont {
///             is_negate: false,
///             field: Field::MessageText,
///             operator: Operator::ContainsOne,
///             value: vec![Value::from_str("菠菜"), Value::from_str("博彩")],
///         },
///     ],
///     vec![Cont {
///         is_negate: false,
///         field: Field::MessageText,
///         operator: Operator::ContainsAll,
///         value: vec![Value::from_str("承接"), Value::from_str("广告")],
///     }],
/// ];
/// let mut matcher = Matcher::new(groups);
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
/// assert!(matcher.match_message(&message1)?);
/// assert!(matcher.match_message(&message2)?);
/// assert!(matcher.match_message(&message3)?);
/// # Ok::<(), matchingram::Error>(())
/// ```
/// 它对应的字符串表达式为：
/// ```text
/// (message.text contains_one {"柬埔寨" "东南亚"} and message.text contains_one {"菠菜" "博彩"}) or (message.text contains_all {"承接" "广告"})
/// ```
/// **注意**：匹配器中的所有条件之间都没有显式的关系存在，因为匹配器中每一个独立的组之间一定是 `or` 关系，组内的条件之间一定是 `and` 关系。即：已存在隐式的关系表达。
#[derive(Debug, Default)]
pub struct Matcher {
    /// 条件组序列。
    pub groups: Groups,
    // 上个组的匹配结果。
    last_is_matching: bool,
}

impl Matcher {
    /// 解析规则表达式创建匹配器对象。
    /// 相比规则表达式匹配器对象具有更高的性能，因为不用再经历编译过程。为了提升性能，可将规则预编译为匹配器对象再执行匹配动作。
    pub fn from_rule<S: Into<String>>(rule: S) -> Result<Self> {
        use super::lexer::Lexer;
        use super::parser::Parser;

        let input = rule.into().chars().collect::<Vec<_>>();
        let mut lexer = Lexer::new(&input);
        let parser = Parser::new(&mut lexer)?;
        let matcher = parser.parse()?;

        Ok(matcher)
    }

    /// 使用条件组创建匹配器对象。
    pub fn new(groups: Groups) -> Self {
        Matcher {
            groups: groups,
            last_is_matching: true,
        }
    }
}

#[derive(Debug)]
pub enum Value {
    Decimal(i64),
    Letter(String),
}

/// 单个条件。
#[derive(Debug)]
pub struct Cont {
    /// 是否取反。
    pub is_negate: bool,
    /// 字段。
    pub field: Field,
    /// 运算符。
    pub operator: Operator,
    /// 值。
    pub value: Vec<Value>,
}

/// 条件字段。
#[derive(Debug, Copy, Clone, EnumString, ToString)]
pub enum Field {
    /// 消息文本
    #[strum(serialize = "message.text")]
    MessageText,
    /// 消息文本大小
    #[strum(serialize = "message.text.size")]
    MessageTextSize,
}

/// 条件操作符。
#[derive(Debug, Eq, PartialEq, Copy, Clone, EnumString, ToString)]
#[strum(serialize_all = "snake_case")]
pub enum Operator {
    // 等于。
    Eq,
    // 大于
    Gt,
    // 小于
    Lt,
    // 大于或等于
    Ge,
    // 小于或等于
    Le,
    /// 包含其一。
    ContainsOne,
    /// 包含全部。
    ContainsAll,
}

trait AsString {
    fn as_string(&self) -> String;
}

impl AsString for Value {
    fn as_string(&self) -> String {
        use Value::*;

        match self {
            Letter(v) => v.clone(),
            Decimal(v) => v.to_string(),
        }
    }
}

impl AsString for Vec<Value> {
    fn as_string(&self) -> String {
        self.into_iter()
            .map(|v| v.as_string())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl Value {
    pub fn from_str(value_s: &str) -> Self {
        Value::Letter(value_s.to_owned())
    }
}

impl Cont {
    /// 从字符串数据中构建条件。
    pub fn new(
        is_negate: bool,
        field_s: String,
        operator_s: String,
        value: Vec<Value>,
    ) -> Result<Self> {
        let operator =
            Operator::from_str(operator_s.as_str()).map_err(|_| Error::UnknownOperator {
                operator: operator_s.clone(),
            })?;

        let field = Field::from_str(field_s.as_str()).map_err(|_| Error::UnknownField {
            field: field_s.clone(),
        })?;

        let empty_operators = &vec![];
        let operators = FIELD_OPERATORS
            .get(field_s.as_str())
            .copied()
            .unwrap_or(&empty_operators);

        // 检查运算符是否支持。
        if !operators.contains(&operator) {
            return Err(Error::UnsupportedOperator {
                field: field_s,
                operator: operator_s,
            });
        }

        Ok(Cont {
            is_negate,
            field,
            operator,
            value,
        })
    }
}

impl Matcher {
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

macro_rules! negating_wrap {
    ($cont:ident, $result:ident) => {
        if $cont.is_negate {
            !$result
        } else {
            $result
        }
    };
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
                                if text.contains(&v.as_string()) {
                                    result = true;
                                    break;
                                }
                            }

                            Ok(negating_wrap!(self, result))
                        }
                        Operator::ContainsAll => {
                            let mut result = true;
                            for v in &self.value {
                                if !text.contains(&v.as_string()) {
                                    result = false;
                                    break;
                                }
                            }

                            Ok(negating_wrap!(self, result))
                        }
                        Operator::Eq => {
                            let result = text.eq(&self.value.as_string());

                            Ok(negating_wrap!(self, result))
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
            Field::MessageTextSize => {
                // TODO：有待实现。
                Ok(false)
            }
        }
    }
}
