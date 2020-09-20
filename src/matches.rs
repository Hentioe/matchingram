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

pub type ContGroups = Vec<Vec<Cont>>;
pub type Values = Vec<Value>;

lazy_static! {
    static ref FIELD_OPERATORS: HashMap<&'static Field, &'static [Operator]> = {
        use Field::*;
        use Operator::*;

        hashmap! {
            &MessageFromId                  => &[Eq, Gt, Ge, Le][..],
            &MessageFromIsBot               => &[][..],
            &MessageFromFirstName           => &[Eq, In, Any, All, Hd][..],
            &MessageFromFullName            => &[Eq, In, Any, All, Hd][..],
            &MessageFromLanguageCode        => &[Eq, In, Hd][..],
            &MessageForwardFromChat         => &[][..],
            &MessageForwardFromChatId       => &[Eq, Gt, Ge, Le][..],
            &MessageForwardFromChatType     => &[Eq, In][..],
            &MessageForwardFromChatTitle    => &[Eq, Any, All, Hd][..],
            &MessageReplyToMessage          => &[][..],
            &MessageText                    => &[Eq, In, Any, All][..],
            &MessageTextLen                 => &[Eq, Gt, Ge, Le][..],
            &MessageAnimation               => &[][..],
            &MessageAnimationDuration       => &[Eq, Gt, Ge, Le][..],
            &MessageAnimationFileName       => &[Eq, Any, All, Hd][..],
            &MessageAnimationMimeType       => &[Eq, In, Hd][..],
            &MessageAnimationFileSize       => &[Eq, Gt, Ge, Le][..],
            &MessageAudio                   => &[][..],
            &MessageAudioDuration           => &[Eq, Gt, Ge, Le][..],
            &MessageAudioPerformer          => &[Eq, All, Any, Hd][..],
            &MessageAudioMimeType           => &[Eq, In, Hd][..],
            &MessageAudioFileSize           => &[Eq, Gt, Ge, Le][..],
            &MessageDocument                => &[][..],
            &MessageDocumentFileName        => &[Eq, All, Any, Hd][..],
            &MessageDocumentMimeType        => &[Eq, In, Hd][..],
            &MessageDocumentFileSize        => &[Eq, Gt, Ge, Le][..],
            &MessagePhoto                   => &[][..],
            &MessageSticker                 => &[][..],
            &MessageStickerIsAnimated       => &[][..],
            &MessageStickerEmoji            => &[Eq, In][..],
            &MessageStickerSetName          => &[Eq, All, Any, Hd][..],
            &MessageVideo                   => &[][..],
            &MessageVideoDuration           => &[Eq, Gt, Ge, Le][..],
            &MessageVideoMimeType           => &[Eq, In, Hd][..],
            &MessageVideoFileSize           => &[Eq, Gt, Ge, Le][..],
            &MessageVoice                   => &[][..],
            &MessageVoiceDuration           => &[Eq, Gt, Ge, Le][..],
            &MessageVoiceMimeType           => &[Eq, In, Hd][..],
            &MessageVoiceFileSize           => &[Eq, Gt, Ge, Le][..],
            &MessageCaption                 => &[Eq, All, Any, Hd][..],
            &MessageCaptionLen              => &[Eq, Gt, Ge, Le][..],
            &MessageDice                    => &[][..],
            &MessageDiceEmoji               => &[Eq, In][..],
            &MessagePoll                    => &[][..],
            &MessagePollType                => &[Eq, In][..],
            &MessageVenue                   => &[][..],
            &MessageVenueTitle              => &[Eq, All, Any, Hd][..],
            &MessageVenueAddress            => &[Eq, All, Any, Hd][..],
            &MessageLocation                => &[][..],
            &MessageLocationLongitude       => &[Eq, Gt, Ge, Le][..],
            &MessageLocationLatitude        => &[Eq, Gt, Ge, Le][..],
            &MessageNewChatMembers          => &[][..],
            &MessageLeftChatMember          => &[][..],
            &MessageNewChatTitle            => &[][..],
            &MessageNewChatPhoto            => &[][..],
            &MessagePinnedMessage           => &[][..],
            &MessageIsServiceMessage        => &[][..],
            &MessageIsCommand               => &[][..],
        }
    };
}

/// 匹配器。一般作为表达式的编译目标。
///
/// 匹配器可表达与字符串规则完全对应的结构化的条件关系。
/// 每个匹配器对象都具备一个“条件组”序列。
/// ```
/// use matchingram::models::Message;
/// use matchingram::matches::*;
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
    pub groups: ContGroups,
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
    pub fn new(groups: ContGroups) -> Self {
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
    pub value: Option<Values>,
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
    /// 消息来自转发。
    #[strum(serialize = "message.forward_from_chat")]
    MessageForwardFromChat,
    /// 消息的转发源头 ID。
    #[strum(serialize = "message.forward_from_chat.id")]
    MessageForwardFromChatId,
    /// 消息的转发源头类型。
    #[strum(serialize = "message.forward_from_chat.type")]
    MessageForwardFromChatType,
    /// 消息的转发源头标题。
    #[strum(serialize = "message.forward_from_chat.title")]
    MessageForwardFromChatTitle,
    /// 消息是对其它消息的回复。
    #[strum(serialize = "message.reply_to_message")]
    MessageReplyToMessage,
    /// 消息中包含文本。
    #[strum(serialize = "message.text")]
    MessageText,
    /// 消息中包含的文本大小。
    #[strum(serialize = "message.text.len")]
    MessageTextLen,
    /// 消息中包含动画。
    #[strum(serialize = "message.animation")]
    MessageAnimation,
    /// 消息中的动画时长。
    #[strum(serialize = "message.animation.duration")]
    MessageAnimationDuration,
    /// 消息中的动画的文件名。
    #[strum(serialize = "message.animation.file_name")]
    MessageAnimationFileName,
    /// 消息中的动画的媒体类型。
    #[strum(serialize = "message.animation.mime_type")]
    MessageAnimationMimeType,
    /// 消息中的动画的文件大小。
    #[strum(serialize = "message.animation.file_size")]
    MessageAnimationFileSize,
    /// 消息中包含音频。
    #[strum(serialize = "message.audio")]
    MessageAudio,
    /// 消息中的音频的时长。
    #[strum(serialize = "message.audio.duration")]
    MessageAudioDuration,
    /// 消息中的音频的表演者。
    #[strum(serialize = "message.audio.performer")]
    MessageAudioPerformer,
    /// 消息中音频的媒体类型。
    #[strum(serialize = "message.audio.mime_type")]
    MessageAudioMimeType,
    /// 消息中音频的文件大小。
    #[strum(serialize = "message.audio.file_size")]
    MessageAudioFileSize,
    /// 消息中包含文档。
    #[strum(serialize = "message.document")]
    MessageDocument,
    /// 消息中文档的文件名。
    #[strum(serialize = "message.document.file_name")]
    MessageDocumentFileName,
    /// 消息中文档的媒体类型。
    #[strum(serialize = "message.document.mime_type")]
    MessageDocumentMimeType,
    /// 消息中文档的文件大小。
    #[strum(serialize = "message.document.file_size")]
    MessageDocumentFileSize,
    /// 消息中包含图片。
    #[strum(serialize = "message.photo")]
    MessagePhoto,
    /// 消息中包含贴纸。
    #[strum(serialize = "message.sticker")]
    MessageSticker,
    /// 消息中的贴纸是否为动画。
    #[strum(serialize = "message.sticker.is_animated")]
    MessageStickerIsAnimated,
    /// 消息中的贴纸的 emoji 名称。
    #[strum(serialize = "message.sticker.emoji")]
    MessageStickerEmoji,
    /// 消息中的贴纸的集合名称。
    #[strum(serialize = "message.sticker.set_name")]
    MessageStickerSetName,
    /// 消息中包含视频。
    #[strum(serialize = "message.video")]
    MessageVideo,
    /// 消息中的视频的时长。
    #[strum(serialize = "message.video.duration")]
    MessageVideoDuration,
    /// 消息中的视频的媒体类型。
    #[strum(serialize = "message.video.mime_type")]
    MessageVideoMimeType,
    /// 消息中的视频的文件大小。
    #[strum(serialize = "message.video.file_size")]
    MessageVideoFileSize,
    /// 消息中包含语音。
    #[strum(serialize = "message.voice")]
    MessageVoice,
    /// 消息中的语音的时长。
    #[strum(serialize = "message.voice.duration")]
    MessageVoiceDuration,
    /// 消息中的语音的媒体类型。
    #[strum(serialize = "message.voice.mime_type")]
    MessageVoiceMimeType,
    /// 消息中的语音的文件大小。
    #[strum(serialize = "message.voice.file_size")]
    MessageVoiceFileSize,
    /// 消息中包含附件（动画、音频、文档、照片、视频）的说明文字。
    #[strum(serialize = "message.caption")]
    MessageCaption,
    // 消息中的附件的说明文字的长度。
    #[strum(serialize = "message.caption.len")]
    MessageCaptionLen,
    // 消息中包含骰子。
    #[strum(serialize = "message.dice")]
    MessageDice,
    // 消息中的骰子的 emoji。
    #[strum(serialize = "message.dice.emoji")]
    MessageDiceEmoji,
    // 消息中包含投票。
    #[strum(serialize = "message.poll")]
    MessagePoll,
    // 消息中的投票的类型。
    #[strum(serialize = "message.poll.type")]
    MessagePollType,
    // 消息包含场地。
    #[strum(serialize = "message.venue")]
    MessageVenue,
    // 消息中的场地的标题。
    #[strum(serialize = "message.venue.title")]
    MessageVenueTitle,
    // 消息中的场地的地址。
    #[strum(serialize = "message.venue.address")]
    MessageVenueAddress,
    // 消息包含共享位置。
    #[strum(serialize = "message.location")]
    MessageLocation,
    // 消息中的位置的维度。
    #[strum(serialize = "message.location.longitude")]
    MessageLocationLongitude,
    // 消息中的位置的经度。
    #[strum(serialize = "message.location.latitude")]
    MessageLocationLatitude,
    // 消息中包含新成员。
    #[strum(serialize = "message.new_chat_members")]
    MessageNewChatMembers,
    // 消息中包含已退出（包括被移除）的成员。
    #[strum(serialize = "message.left_chat_member")]
    MessageLeftChatMember,
    // 消息中包含新群组标题。
    #[strum(serialize = "message.new_chat_title")]
    MessageNewChatTitle,
    // 消息中包含新群组图片。
    #[strum(serialize = "message.new_chat_photo")]
    MessageNewChatPhoto,
    // 消息中包含被置顶的消息。
    #[strum(serialize = "message.pinned_message")]
    MessagePinnedMessage,
    // 消息是否为服务消息。
    #[strum(serialize = "message.is_service_message")]
    MessageIsServiceMessage,
    // 消息是否为命令。
    #[strum(serialize = "message.is_command")]
    MessageIsCommand,
}

pub trait RefSingleValue {
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

impl RefSingleValue for Value {
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

impl RefSingleValue for Values {
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
        value: Values,
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

    fn value(&self) -> Result<&Values> {
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

// unwrap_or_falsey_hosting
macro_rules! ufh {
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
            Field::MessageFromId => match self.operator()? {
                Operator::Eq => ufh!(message.from).id.eq_ope(self.value()?),
                Operator::Gt => ufh!(message.from).id.gt_ope(self.value()?),
                Operator::Ge => ufh!(message.from).id.ge_ope(self.value()?),
                Operator::Le => ufh!(message.from).id.le_ope(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessageFromIsBot => Ok(child_is_truthy!(&message.from, is_bot)),
            Field::MessageFromFirstName => match self.operator()? {
                Operator::Eq => ufh!(message.from).first_name.eq_ope(self.value()?),
                Operator::In => ufh!(message.from).first_name.in_ope(self.value()?),
                Operator::Any => ufh!(message.from).first_name.any_ope(self.value()?),
                Operator::All => ufh!(message.from).first_name.all_ope(self.value()?),
                Operator::Hd => ufh!(message.from).first_name.hd_ope(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessageFromFullName => match self.operator()? {
                Operator::Eq => ufh!(message.from).full_name().eq_ope(self.value()?),
                Operator::In => ufh!(message.from).full_name().in_ope(self.value()?),
                Operator::Any => ufh!(message.from).full_name().any_ope(self.value()?),
                Operator::All => ufh!(message.from).full_name().all_ope(self.value()?),
                Operator::Hd => ufh!(message.from).full_name().hd_ope(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessageFromLanguageCode => match self.operator()? {
                Operator::Eq => ufh!(message.from).language_code.eq_ope(self.value()?),
                Operator::In => ufh!(message.from).language_code.in_ope(self.value()?),
                Operator::Hd => ufh!(message.from).language_code.hd_ope(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessageForwardFromChat => Ok(message.forward_from_chat.is_truthy()),
            Field::MessageForwardFromChatId => match self.operator()? {
                Operator::Eq => ufh!(message.forward_from_chat).id.eq_ope(self.value()?),
                Operator::Gt => ufh!(message.forward_from_chat).id.gt_ope(self.value()?),
                Operator::Ge => ufh!(message.forward_from_chat).id.ge_ope(self.value()?),
                Operator::Le => ufh!(message.forward_from_chat).id.le_ope(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessageForwardFromChatType => match self.operator()? {
                Operator::Eq => ufh!(message.forward_from_chat).type_.eq_ope(self.value()?),
                Operator::In => ufh!(message.forward_from_chat).type_.in_ope(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessageForwardFromChatTitle => match self.operator()? {
                Operator::Eq => ufh!(message.forward_from_chat).title.eq_ope(self.value()?),
                Operator::Any => ufh!(message.forward_from_chat).title.any_ope(self.value()?),
                Operator::All => ufh!(message.forward_from_chat).title.all_ope(self.value()?),
                Operator::Hd => ufh!(message.forward_from_chat).title.hd_ope(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessageReplyToMessage => Ok(message.reply_to_message.is_truthy()),
            Field::MessageText => match self.operator()? {
                Operator::Eq => message.text.eq_ope(self.value()?),
                Operator::In => message.text.in_ope(self.value()?),
                Operator::Any => message.text.any_ope(self.value()?),
                Operator::All => message.text.all_ope(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessageTextLen => match self.operator()? {
                Operator::Eq => message.text.eq_ope_for_content_len(self.value()?),
                Operator::Gt => message.text.gt_ope_for_content_len(self.value()?),
                Operator::Ge => message.text.ge_ope_for_content_len(self.value()?),
                Operator::Le => message.text.le_ope_for_content_len(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessageAnimation => Ok(message.animation.is_truthy()),
            Field::MessageAnimationDuration => match self.operator()? {
                Operator::Eq => ufh!(message.animation).duration.eq_ope(self.value()?),
                Operator::Gt => ufh!(message.animation).duration.gt_ope(self.value()?),
                Operator::Ge => ufh!(message.animation).duration.ge_ope(self.value()?),
                Operator::Le => ufh!(message.animation).duration.le_ope(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessageAnimationFileName => match self.operator()? {
                Operator::Eq => ufh!(message.animation).file_name.eq_ope(self.value()?),
                Operator::Any => ufh!(message.animation).file_name.any_ope(self.value()?),
                Operator::All => ufh!(message.animation).file_name.all_ope(self.value()?),
                Operator::Hd => ufh!(message.animation).file_name.hd_ope(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessageAnimationMimeType => match self.operator()? {
                Operator::Eq => ufh!(message.animation).mime_type.eq_ope(self.value()?),
                Operator::In => ufh!(message.animation).mime_type.in_ope(self.value()?),
                Operator::Hd => ufh!(message.animation).mime_type.hd_ope(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessageAnimationFileSize => match self.operator()? {
                Operator::Eq => ufh!(message.animation).file_size.eq_ope(self.value()?),
                Operator::Gt => ufh!(message.animation).file_size.gt_ope(self.value()?),
                Operator::Ge => ufh!(message.animation).file_size.ge_ope(self.value()?),
                Operator::Le => ufh!(message.animation).file_size.le_ope(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessageAudio => Ok(message.audio.is_truthy()),
            Field::MessageAudioDuration => match self.operator()? {
                Operator::Eq => ufh!(message.audio).duration.eq_ope(self.value()?),
                Operator::Gt => ufh!(message.audio).duration.gt_ope(self.value()?),
                Operator::Ge => ufh!(message.audio).duration.ge_ope(self.value()?),
                Operator::Le => ufh!(message.audio).duration.le_ope(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessageAudioPerformer => match self.operator()? {
                Operator::Eq => ufh!(message.audio).performer.eq_ope(self.value()?),
                Operator::Any => ufh!(message.audio).performer.any_ope(self.value()?),
                Operator::All => ufh!(message.audio).performer.all_ope(self.value()?),
                Operator::Hd => ufh!(message.audio).performer.hd_ope(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessageAudioMimeType => match self.operator()? {
                Operator::Eq => ufh!(message.audio).mime_type.eq_ope(self.value()?),
                Operator::In => ufh!(message.audio).mime_type.in_ope(self.value()?),
                Operator::Hd => ufh!(message.audio).mime_type.hd_ope(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessageAudioFileSize => match self.operator()? {
                Operator::Eq => ufh!(message.audio).file_size.eq_ope(self.value()?),
                Operator::Gt => ufh!(message.audio).file_size.gt_ope(self.value()?),
                Operator::Ge => ufh!(message.audio).file_size.ge_ope(self.value()?),
                Operator::Le => ufh!(message.audio).file_size.le_ope(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessageDocument => Ok(message.document.is_truthy()),
            Field::MessageDocumentFileName => match self.operator()? {
                Operator::Eq => ufh!(message.document).file_name.eq_ope(self.value()?),
                Operator::Any => ufh!(message.document).file_name.any_ope(self.value()?),
                Operator::All => ufh!(message.document).file_name.all_ope(self.value()?),
                Operator::Hd => ufh!(message.document).file_name.hd_ope(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessageDocumentMimeType => match self.operator()? {
                Operator::Eq => ufh!(message.document).mime_type.eq_ope(self.value()?),
                Operator::In => ufh!(message.document).mime_type.in_ope(self.value()?),
                Operator::Hd => ufh!(message.document).mime_type.hd_ope(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessageDocumentFileSize => match self.operator()? {
                Operator::Eq => ufh!(message.document).file_size.eq_ope(self.value()?),
                Operator::Gt => ufh!(message.document).file_size.gt_ope(self.value()?),
                Operator::Ge => ufh!(message.document).file_size.ge_ope(self.value()?),
                Operator::Le => ufh!(message.document).file_size.le_ope(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessagePhoto => Ok(message.photo.is_truthy()),
            Field::MessageSticker => Ok(message.sticker.is_truthy()),
            Field::MessageStickerIsAnimated => {
                Ok(child_is_truthy!(&message.sticker, is_animated).is_truthy())
            }
            Field::MessageStickerEmoji => match self.operator()? {
                Operator::Eq => ufh!(message.sticker).emoji.eq_ope(self.value()?),
                Operator::In => ufh!(message.sticker).emoji.in_ope(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessageStickerSetName => match self.operator()? {
                Operator::Eq => ufh!(message.sticker).set_name.eq_ope(self.value()?),
                Operator::Any => ufh!(message.sticker).set_name.any_ope(self.value()?),
                Operator::All => ufh!(message.sticker).set_name.all_ope(self.value()?),
                Operator::Hd => ufh!(message.sticker).set_name.hd_ope(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessageVideo => Ok(message.video.is_truthy()),
            Field::MessageVideoDuration => match self.operator()? {
                Operator::Eq => ufh!(message.video).duration.eq_ope(self.value()?),
                Operator::Gt => ufh!(message.video).duration.gt_ope(self.value()?),
                Operator::Ge => ufh!(message.video).duration.ge_ope(self.value()?),
                Operator::Le => ufh!(message.video).duration.le_ope(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessageVideoMimeType => match self.operator()? {
                Operator::Eq => ufh!(message.video).mime_type.eq_ope(self.value()?),
                Operator::In => ufh!(message.video).mime_type.in_ope(self.value()?),
                Operator::Hd => ufh!(message.video).mime_type.hd_ope(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessageVideoFileSize => match self.operator()? {
                Operator::Eq => ufh!(message.video).file_size.eq_ope(self.value()?),
                Operator::Gt => ufh!(message.video).file_size.gt_ope(self.value()?),
                Operator::Ge => ufh!(message.video).file_size.ge_ope(self.value()?),
                Operator::Le => ufh!(message.video).file_size.le_ope(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessageVoice => Ok(message.voice.is_truthy()),
            Field::MessageVoiceDuration => match self.operator()? {
                Operator::Eq => ufh!(message.voice).duration.eq_ope(self.value()?),
                Operator::Gt => ufh!(message.voice).duration.gt_ope(self.value()?),
                Operator::Ge => ufh!(message.voice).duration.ge_ope(self.value()?),
                Operator::Le => ufh!(message.voice).duration.le_ope(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessageVoiceMimeType => match self.operator()? {
                Operator::Eq => ufh!(message.voice).mime_type.eq_ope(self.value()?),
                Operator::In => ufh!(message.voice).mime_type.in_ope(self.value()?),
                Operator::Hd => ufh!(message.voice).mime_type.hd_ope(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessageVoiceFileSize => match self.operator()? {
                Operator::Eq => ufh!(message.voice).file_size.eq_ope(self.value()?),
                Operator::Gt => ufh!(message.voice).file_size.gt_ope(self.value()?),
                Operator::Ge => ufh!(message.voice).file_size.ge_ope(self.value()?),
                Operator::Le => ufh!(message.voice).file_size.le_ope(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessageCaption => match self.operator()? {
                Operator::Eq => message.caption.eq_ope(self.value()?),
                Operator::In => message.caption.in_ope(self.value()?),
                Operator::Any => message.caption.any_ope(self.value()?),
                Operator::All => message.caption.all_ope(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessageCaptionLen => match self.operator()? {
                Operator::Eq => message.caption.eq_ope_for_content_len(self.value()?),
                Operator::Gt => message.caption.gt_ope_for_content_len(self.value()?),
                Operator::Ge => message.caption.ge_ope_for_content_len(self.value()?),
                Operator::Le => message.caption.le_ope_for_content_len(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessageDice => Ok(message.dice.is_truthy()),
            Field::MessageDiceEmoji => match self.operator()? {
                Operator::Eq => ufh!(message.dice).emoji.eq_ope(self.value()?),
                Operator::In => ufh!(message.dice).emoji.in_ope(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessagePoll => Ok(message.poll.is_truthy()),
            Field::MessagePollType => match self.operator()? {
                Operator::Eq => ufh!(message.poll).type_.eq_ope(self.value()?),
                Operator::In => ufh!(message.poll).type_.in_ope(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessageVenue => Ok(message.venue.is_truthy()),
            Field::MessageVenueTitle => match self.operator()? {
                Operator::Eq => ufh!(message.venue).title.eq_ope(self.value()?),
                Operator::Any => ufh!(message.venue).title.any_ope(self.value()?),
                Operator::All => ufh!(message.venue).title.all_ope(self.value()?),
                Operator::Hd => ufh!(message.venue).title.hd_ope(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessageVenueAddress => match self.operator()? {
                Operator::Eq => ufh!(message.venue).address.eq_ope(self.value()?),
                Operator::Any => ufh!(message.venue).address.any_ope(self.value()?),
                Operator::All => ufh!(message.venue).address.all_ope(self.value()?),
                Operator::Hd => ufh!(message.venue).address.hd_ope(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessageLocation => Ok(message.location.is_truthy()),
            Field::MessageLocationLongitude => match self.operator()? {
                Operator::Eq => ufh!(message.location).longitude.eq_ope(self.value()?),
                Operator::Gt => ufh!(message.location).longitude.gt_ope(self.value()?),
                Operator::Ge => ufh!(message.location).longitude.ge_ope(self.value()?),
                Operator::Le => ufh!(message.location).longitude.le_ope(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessageLocationLatitude => match self.operator()? {
                Operator::Eq => ufh!(message.location).latitude.eq_ope(self.value()?),
                Operator::Gt => ufh!(message.location).latitude.gt_ope(self.value()?),
                Operator::Ge => ufh!(message.location).latitude.ge_ope(self.value()?),
                Operator::Le => ufh!(message.location).latitude.le_ope(self.value()?),
                _ => Err(unsupported_operator_err()?),
            },
            Field::MessageNewChatMembers => Ok(message.new_chat_members.is_truthy()),
            Field::MessageLeftChatMember => Ok(message.left_chat_member.is_truthy()),
            Field::MessageNewChatTitle => Ok(message.new_chat_title.is_truthy()),
            Field::MessageNewChatPhoto => Ok(message.new_chat_photo.is_truthy()),
            Field::MessagePinnedMessage => Ok(message.pinned_message.is_truthy()),
            Field::MessageIsServiceMessage => Ok(
                message.new_chat_members.is_truthy() || // 加入成员
                message.left_chat_member.is_truthy() || // 离开成员
                message.new_chat_title.is_truthy() || // 新标题
                message.new_chat_photo.is_truthy() || // 新头像
                message.pinned_message.is_truthy() // 置顶消息
            ),
            Field::MessageIsCommand => {
                // TODO: 待实现。
                Err(Error::FieldNotEndabled {
                    field: Field::MessageIsCommand,
                })
            }
            //
            // field => Err(Error::FieldNotEndabled { field }),
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
