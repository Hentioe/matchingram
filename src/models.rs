//! Types of Telegram bot API.

use std::rc::Rc;

/// A Telegram message.
#[derive(Debug)]
pub struct Message {
    /// Unique message identifier inside this chat.
    pub message_id: i64,
    /// Sender, empty for messages sent to channels.
    pub from: Option<User>,
    /// For forwarded messages, sender of the original message.
    pub forward_from: Option<User>,
    /// For messages forwarded from channels, information about the original channel.
    pub forward_from_chat: Option<Chat>,
    /// For replies, the original message.
    /// Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    pub reply_to_message: Option<Rc<Message>>,
    /// Bot through which the message was sent.
    pub via_bot: Option<User>,
    /// For text messages, the actual UTF-8 text of the message, 0-4096 characters.
    pub text: Option<String>,
    /// For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text.
    pub entities: Option<Vec<MessageEntity>>,
    /// Message is an animation, information about the animation. For backward compatibility,
    /// when this field is set, the document field will also be set.
    pub animation: Option<Animation>,
    /// Message is an audio file, information about the file.
    pub audio: Option<Audio>,
    /// Message is a general file, information about the file.
    pub document: Option<Document>,
    /// Message is a photo, available sizes of the photo.
    pub photo: Option<Vec<PhotoSize>>,
    /// Message is a sticker, information about the sticker.
    pub sticker: Option<Sticker>,
    /// Message is a video, information about the video.
    pub video: Option<Video>,
    /// Message is a video note, information about the video message.
    pub video_note: Option<VideoNote>,
    /// Message is a voice message, information about the file.
    pub voice: Option<Voice>,
    /// Caption for the animation, audio, document, photo, video or voice, 0-1024 characters.
    pub caption: Option<String>,
    /// For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption.
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Message is a dice with random value from 1 to 6.
    pub dice: Option<Dice>,
    /// Message is a native poll, information about the poll.
    pub poll: Option<Poll>,
    /// Message is a venue, information about the venue. For backward compatibility,
    /// when this field is set, the location field will also be set.
    pub venue: Option<Venue>,
    /// Message is a shared location, information about the location.
    pub location: Option<Location>,
    /// New members that were added to the group or supergroup and information about them
    /// (the bot itself may be one of these members).
    pub new_chat_members: Option<Vec<User>>,
    /// A member was removed from the group, information about them (this member may be the bot itself).
    pub left_chat_member: Option<User>,
    /// A chat title was changed to this value.
    pub new_chat_title: Option<String>,
    /// A chat photo was change to this value.
    pub new_chat_photo: Option<Vec<PhotoSize>>,
    /// Specified message was pinned. Note that the Message object in this field will
    /// not contain further `reply_to_message` fields even if it is itself a reply.
    pub pinned_message: Option<Rc<Message>>,
}

#[derive(Debug)]
pub struct Animation {}

#[derive(Debug)]
pub struct Audio {}

#[derive(Debug)]
pub struct MessageEntity {}

#[derive(Debug)]
pub struct Chat {}

#[derive(Debug)]
pub struct Contact {}

#[derive(Debug)]
pub struct Dice {}

#[derive(Debug)]
pub struct Document {}

#[derive(Debug)]
pub struct User {}

#[derive(Debug)]
pub struct Game {}

#[derive(Debug)]
pub struct Invoice {}

#[derive(Debug)]
pub struct Location {}
#[derive(Debug)]
pub struct PhotoSize {}
#[derive(Debug)]
pub struct PassportData {}
#[derive(Debug)]
pub struct Poll {}
#[derive(Debug)]
pub struct InlineKeyboardMarkup {}

#[derive(Debug)]
pub struct Sticker {}
#[derive(Debug)]
pub struct SuccessfulPayment {}

#[derive(Debug)]
pub struct Venue {}

#[derive(Debug)]
pub struct Video {}

#[derive(Debug)]
pub struct VideoNote {}
#[derive(Debug)]
pub struct Voice {}
