//! 消息匹配实现。

use super::error::Error;
use super::models::Message;
use super::result::Result;

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
/// assert!(matches!(matcher.match_message(&message1), Ok(true)));
/// assert!(matches!(matcher.match_message(&message2), Ok(true)));
/// assert!(matches!(matcher.match_message(&message3), Ok(true)));
/// # Ok::<(), matchingram::Error>(())
/// ```
/// 它对应的字符串表达式为：
/// ```text
/// (message.text contains_one {柬埔寨 东南亚} and message.text contains_one {菠菜 博彩}) or (message.text contains_all {承接 广告})
/// ```
/// **注意**：匹配器中的所有条件之间都没有显式的关系存在，因为匹配器中每一个独立的组之间一定是 `or` 关系，组内的条件之间一定是 `and` 关系。即：已存在隐式的关系表达。
#[derive(Debug, Default)]
pub struct Matcher {
    /// 条件组序列。
    pub groups: Vec<Vec<Cont>>,
    // 上个组的匹配结果。
    last_is_matching: bool,
}

impl Matcher {
    /// 将规则表达式解析为匹配器对象。
    /// 相比规则表达式匹配器对象具有更高的性能，因为不用再经历编译过程。为了提升性能，可将规则预编译为匹配器对象再执行匹配动作。
    pub fn prase<S: Into<String>>(_rule: S) -> Result<Self> {
        panic!("This function has not been implemented yet!")
    }

    /// 使用条件组创建匹配器对象。
    pub fn new(groups: Vec<Vec<Cont>>) -> Self {
        Matcher {
            groups: groups,
            last_is_matching: true,
        }
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
