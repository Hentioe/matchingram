//! 消息匹配实现。

use lazy_static::lazy_static;
use maplit::hashmap;
use std::collections::HashMap;
use std::str::FromStr;
use strum_macros::{EnumString, ToString};

use super::error::Error;
use super::falsey::UnwrapOrFalseyHosting;
use super::models::Message;
use super::ope::{prelude::*, Operator};
use super::result::Result;
use super::truthy::IsTruthy;

pub type Groups = Vec<Vec<Cont>>;

lazy_static! {
    static ref FIELD_OPERATORS: HashMap<&'static Field, &'static [Operator]> = {
        use Field::*;
        use Operator::*;

        hashmap! {
            &MessageText                => &[Eq, In, Any, All][..],
            &MessageTextSize            => &[Eq, Gt, Ge, Le][..],
            &MessageFromFirstName       => &[Eq, In, Any, All, Hd][..],
            &MessageFromIsBot           => &[],
        }
    };
}

/// 匹配器。一般作为表达式的编译目标。
///
/// 匹配器可表达与字符串规则完全对应的结构化的条件关系。
/// 每个匹配器对象都具备一个“条件组”序列。
/// ```
/// use matchingram::models::Message;
/// use matchingram::matcher::*;
/// use matchingram::ope::Operator;
///
/// // 手动创建一个匹配器对象：
/// let groups = vec![
///     vec![
///         Cont {
///             is_negative: false,
///             field: Field::MessageText,
///             operator: Some(Operator::Any),
///             value: Some(vec![Value::from_str("柬埔寨"), Value::from_str("东南亚")]),
///         },
///         Cont {
///             is_negative: false,
///             field: Field::MessageText,
///             operator: Some(Operator::Any),
///             value: Some(vec![Value::from_str("菠菜"), Value::from_str("博彩")]),
///         },
///     ],
///     vec![Cont {
///         is_negative: false,
///         field: Field::MessageText,
///         operator: Some(Operator::All),
///         value: Some(vec![Value::from_str("承接"), Value::from_str("广告")]),
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
/// (message.text any {"柬埔寨" "东南亚"} and message.text any {"菠菜" "博彩"}) or (message.text all {"承接" "广告"})
/// ```
/// **注意**：匹配器中的所有条件之间都没有显式的关系存在，因为匹配器中每一个独立的组之间一定是 `or` 关系，组内的条件之间一定是 `and` 关系。即：已存在隐式的关系表达。
#[derive(Debug, Default)]
pub struct Matcher {
    /// 条件组序列。
    pub groups: Groups,
    // 上个组的匹配结果。
    is_last_match: bool,
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
            is_last_match: true,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Decimal(i64),
    Letter(String),
}

/// 单个条件。
#[derive(Debug)]
pub struct Cont {
    /// 是否取反。
    pub is_negative: bool,
    /// 字段。
    pub field: Field,
    /// 运算符。
    pub operator: Option<Operator>,
    /// 值。
    pub value: Option<Vec<Value>>,
}

/// 条件字段。
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, EnumString, ToString)]
pub enum Field {
    /// 消息来源 ID。
    #[strum(serialize = "message.from.id")]
    MessageFromId,
    /// 消息来源是否为 bot。
    #[strum(serialize = "message.from.is_bot")]
    MessageFromIsBot,
    /// 消息来源的姓。
    #[strum(serialize = "message.from.first_name")]
    MessageFromFirstName,
    /// 消息来源的全名。
    #[strum(serialize = "message.from.full_name")]
    MessageFromFullName,
    /// 消息来源的语言代码。
    #[strum(serialize = "message.from.language_code")]
    MessageFromLanguageCode,
    /// 转发的源头。
    #[strum(serialize = "message.forward_from_chat")]
    MessageForwardFromChat,
    /// 转发源头的 ID。
    #[strum(serialize = "message.forward_from_chat.id")]
    MessageForwardFromChatId,
    /// 转发源头的类型。
    #[strum(serialize = "message.forward_from_chat.type")]
    MessageForwardFromChatType,
    /// 转发源头的标题。
    #[strum(serialize = "message.forward_from_chat.title")]
    MessageForwardFromChatTitle,
    /// 回复的目标消息。
    #[strum(serialize = "message.reply_to_message")]
    MessageReplyToMessage,
    /// 消息的文本。
    #[strum(serialize = "message.text")]
    MessageText,
    /// 消息文本大小。
    #[strum(serialize = "message.text.size")]
    MessageTextSize,
    /// 消息的动画。
    #[strum(serialize = "message.animation")]
    MessageAnimation,
    /// 消息动画的时长。
    #[strum(serialize = "message.animation.duration")]
    MessageAnimationDuration,
    /// 消息动画的文件名。
    #[strum(serialize = "message.animation.file_name")]
    MessageAnimationFileName,
    /// 消息动画的媒体类型。
    #[strum(serialize = "message.animation.mime_type")]
    MessageAnimationMimeType,
    /// 消息动画的文件大小。
    #[strum(serialize = "message.animation.file_size")]
    MessageAnimationFileSize,
    /// 消息的音频。
    #[strum(serialize = "message.audio")]
    MessageAudio,
    /// 消息音频的时长。
    #[strum(serialize = "message.audio.duration")]
    MessageAudioDuration,
    /// 消息音频的表演者。
    #[strum(serialize = "message.audio.performer")]
    MessageAudioPerformer,
    /// 消息音频的媒体类型。
    #[strum(serialize = "message.audio.mime_type")]
    MessageAudioMimeType,
    /// 消息音频的文件大小。
    #[strum(serialize = "message.audio.file_size")]
    MessageAudioFileSize,
    /// 消息的文档。
    #[strum(serialize = "message.document")]
    MessageDocument,
    /// 消息文档的文件名。
    #[strum(serialize = "message.document.file_name")]
    MessageDocumentFileName,
    /// 消息文档的媒体类型。
    #[strum(serialize = "message.document.mime_type")]
    MessageDocumentMimeType,
    /// 消息文档的文件大小。
    #[strum(serialize = "message.document.file_size")]
    MessageDocumentFileSize,
    /// 消息的图片。
    #[strum(serialize = "message.photo")]
    MessagePhoto,
    /// 消息的贴纸。
    #[strum(serialize = "message.sticker")]
    MessageSticker,
    /// 消息贴纸是否包含动画。
    #[strum(serialize = "message.sticker.is_animated")]
    MessageStickerIsAnimated,
    /// 消息贴纸的 emoji 名称。
    #[strum(serialize = "message.sticker.emoji")]
    MessageStickerEmoji,
    /// 消息贴纸的集合名称。
    #[strum(serialize = "message.sticker.set_name")]
    MessageStickerSetName,
    /// 消息的视频。
    #[strum(serialize = "message.video")]
    MessageVideo,
    /// 消息视频的时长。
    #[strum(serialize = "message.video.duration")]
    MessageVideoDuration,
    /// 消息视频的媒体类型。
    #[strum(serialize = "message.video.mime_type")]
    MessageVideoMimeType,
    /// 消息视频的文件大小。
    #[strum(serialize = "message.video.file_size")]
    MessageVideoFileSize,
    /// 消息的语音。
    #[strum(serialize = "message.voice")]
    MessageVoice,
    /// 消息语音的时长。
    #[strum(serialize = "message.voice.duration")]
    MessageVoiceDuration,
    /// 消息语音的媒体类型。
    #[strum(serialize = "message.voice.mime_type")]
    MessageVoiceMimeType,
    /// 消息语音的文件大小。
    #[strum(serialize = "message.voice.file_size")]
    MessageVoiceFileSize,
    /// 附件（动画、音频、文档、照片、视频）的说明文字。
    #[strum(serialize = "message.caption")]
    MessageCaption,
    // 附件说明文字的长度。
    #[strum(serialize = "message.caption.len")]
    MessageCaptionLen,
    // 消息是一个骰子。
    #[strum(serialize = "message.dice")]
    MessageDice,
    // 消息骰子的 emoji。
    #[strum(serialize = "message.dice.emoji")]
    MessageDiceEmoji,
    // 消息是一个投票。
    #[strum(serialize = "message.poll")]
    MessagePoll,
    // 消息投票的类型。
    #[strum(serialize = "message.poll.type")]
    MessagePollType,
    // 消息是一个场地。
    #[strum(serialize = "message.venue")]
    MessageVenue,
    // 消息场地的标题。
    #[strum(serialize = "message.venue.title")]
    MessageVenueTitle,
    // 消息场地的地址。
    #[strum(serialize = "message.venue.address")]
    MessageVenueAddress,
    // 消息是一个共享位置。
    #[strum(serialize = "message.location")]
    MessageLocation,
    // 消息位置维度。
    #[strum(serialize = "message.location.longitude")]
    MessageLocationLongitude,
    // 消息位置的经度。
    #[strum(serialize = "message.location.latitude")]
    MessageLocationLatitude,
    // 消息中包含的新成员列表。
    #[strum(serialize = "message.new_chat_members")]
    MessageNewChatMembers,
    // 消息中包含的新 chat 标题。
    #[strum(serialize = "message.new_chat_title")]
    MessageNewChatTitle,
    // 消息中包含的新 chat 图片。
    #[strum(serialize = "message.new_chat_photo")]
    MessageNewChatPhoto,
    // 消息中被置顶的消息。
    #[strum(serialize = "message.pinned_message")]
    MessagePinnedMessage,
    // 消息是否为服务消息。
    #[strum(serialize = "message.is_service_message")]
    MessageIsServiceMessage,
    // 消息是否为命令。
    #[strum(serialize = "message.is_command")]
    MessageIsCommand,
}

pub trait RefSinleValue {
    fn ref_a_str(&self) -> Result<&str>;
    fn ref_a_decimal(&self) -> Result<&i64>;
}
pub trait RefADecimal {
    fn ref_a_decimal(&self) -> Result<&i64>;
}

impl ToString for Value {
    fn to_string(&self) -> String {
        use Value::*;

        match self {
            Letter(v) => v.to_owned(),
            Decimal(v) => v.to_string(),
        }
    }
}

impl RefSinleValue for Value {
    fn ref_a_str(&self) -> Result<&str> {
        use Value::*;

        match self {
            Letter(v) => Ok(v),
            Decimal(_) => Err(Error::NotAString {
                value: self.clone(),
            }),
        }
    }

    fn ref_a_decimal(&self) -> Result<&i64> {
        use Value::*;

        match self {
            Letter(_) => Err(Error::NotADecimal {
                value: self.clone(),
            }),
            Decimal(v) => Ok(v),
        }
    }
}

impl RefSinleValue for Vec<Value> {
    fn ref_a_str(&self) -> Result<&str> {
        if let Some(first) = self.first() {
            first.ref_a_str()
        } else {
            Err(Error::RefValueInEmptyList)
        }
    }

    fn ref_a_decimal(&self) -> Result<&i64> {
        if let Some(first) = self.first() {
            first.ref_a_decimal()
        } else {
            Err(Error::RefValueInEmptyList)
        }
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
        is_negative: bool,
        field_str: String,
        operator_str: String,
        value: Vec<Value>,
    ) -> Result<Self> {
        let operator =
            Operator::from_str(operator_str.as_str()).map_err(|_| Error::UnknownOperator {
                operator: operator_str.to_owned(),
            })?;

        let field = Field::from_str(field_str.as_str()).map_err(|_| Error::UnknownField {
            field: field_str.to_owned(),
        })?;

        let operators = FIELD_OPERATORS
            .get(&field)
            .copied()
            // 没有注册运算符列表，表示字段未启用。
            .ok_or(Error::FieldNotEndabled { field })?;

        // 检查运算符是否支持。
        if !operators.contains(&operator) {
            return Err(Error::UnsupportedOperator { field, operator });
        }

        Ok(Cont {
            is_negative,
            field,
            operator: Some(operator),
            value: Some(value),
        })
    }

    pub fn single_field(is_negative: bool, field_str: String) -> Result<Self> {
        let field = Field::from_str(field_str.as_str()).map_err(|_| Error::UnknownField {
            field: field_str.to_owned(),
        })?;

        let _operators = FIELD_OPERATORS
            .get(&field)
            .copied()
            // 没有注册运算符列表，表示字段未启用。
            .ok_or(Error::FieldNotEndabled { field })?;

        Ok(Cont {
            is_negative,
            field,
            operator: None,
            value: None,
        })
    }

    fn operator(&self) -> Result<&Operator> {
        if let Some(operator) = &self.operator {
            Ok(operator)
        } else {
            Err(Error::FieldRequireOperator { field: self.field })
        }
    }

    fn value(&self) -> Result<&Vec<Value>> {
        if let Some(value) = &self.value {
            Ok(value)
        } else {
            Err(Error::FieldRequireValue { field: self.field })
        }
    }
}

impl Matcher {
    pub fn match_message(&mut self, message: &Message) -> Result<bool> {
        self.loop_match(message, 0)
    }

    fn loop_match(&mut self, message: &Message, position: usize) -> Result<bool> {
        if position > 0 && self.is_last_match {
            return Ok(true);
        }
        if position > (self.groups.len() - 1) {
            return Ok(self.is_last_match);
        }

        let conts = unsafe { self.groups.get_unchecked(position) };

        let mut result = true;
        for cont in conts {
            if !cont.match_message(message)? {
                result = false;
                break;
            }
        }
        self.is_last_match = result;
        self.loop_match(message, position + 1)
    }
}

// 检查子字段是否为存在或为真。
//
// 第一个参数为 `Option<T>` 类型。如果为 `None` 则返回 `false`，否则进一步判断。
// 如果第一个参数存在值，将通过值访问子级字段并获取 `is_truthy` 方法的调用结果。
macro_rules! child_is_truthy {
    ($optinal:expr, $child:tt) => {
        if let Some(parent) = $optinal {
            parent.$child.is_truthy()
        } else {
            false
        }
    };
}

// **u**nwrap_**o**r_**f**alsey_**h**osting
macro_rules! uofh {
    ($optinal:expr) => {
        $optinal.unwrap_or_falsey_hosting()?
    };
}

impl Cont {
    pub fn match_message(&self, message: &Message) -> Result<bool> {
        let unsupported_operator_err = || -> Result<Error> {
            Ok(Error::UnsupportedOperator {
                field: self.field,
                operator: *self.operator()?,
            })
        };

        let r = match self.field {
            Field::MessageText => match self.operator()? {
                Operator::Eq => uofh!(message.text).eq_ope(self.value()?),
                Operator::In => uofh!(message.text).in_ope(self.value()?),
                Operator::Any => uofh!(message.text).any_ope(self.value()?),
                Operator::All => uofh!(message.text).all_ope(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessageTextSize => match self.operator()? {
                Operator::Eq => uofh!(message.text).eq_ope_for_content_len(self.value()?),
                Operator::Gt => uofh!(message.text).gt_ope_for_content_len(self.value()?),
                Operator::Ge => uofh!(message.text).ge_ope_for_content_len(self.value()?),
                Operator::Le => uofh!(message.text).le_ope_for_content_len(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessageFromIsBot => Ok(child_is_truthy!(&message.from, is_bot)),
            Field::MessageFromFirstName => match self.operator()? {
                Operator::In => uofh!(message.from).first_name.in_ope(self.value()?),
                Operator::Hd => uofh!(message.from).first_name.hd_ope(self.value()?),

                _ => Err(unsupported_operator_err()?),
            },

            field => Err(Error::FieldNotEndabled { field }),
        };

        match r {
            Ok(no_negative) => {
                if self.is_negative {
                    Ok(!no_negative)
                } else {
                    Ok(no_negative)
                }
            }
            Err(Error::FalsyValueHosting) => {
                if self.is_negative {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            e => e,
        }
    }
}
