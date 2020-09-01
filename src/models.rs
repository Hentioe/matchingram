//! All types used in a Bot API message.

use std::rc::Rc;

/// This object represents a message.
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

/// This object represents a Telegram user or bot.
#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
}

/// This object represents a chat.
#[derive(Debug)]
pub struct Chat {
    pub id: i64,
    pub type_: String,
    pub title: Option<String>,
}

/// This object represents one special entity in a text message.
/// For example, hashtags, usernames, URLs, etc.
#[derive(Debug)]
pub struct MessageEntity {
    pub type_: String,
    pub offset: i32,
    pub length: i32,
    pub url: Option<String>,
    pub user: Option<User>,
    pub language: Option<String>,
}

/// This object represents an animation file (GIF or H.264/MPEG-4 AVC video without sound).
#[derive(Debug)]
pub struct Animation {
    pub duration: i32,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i32>,
}

/// This object represents an audio file to be treated as music by the Telegram clients.
#[derive(Debug)]
pub struct Audio {
    pub duration: i32,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i32>,
}

/// This object represents a general file (as opposed to photos, voice messages and audio files).
#[derive(Debug)]
pub struct Document {
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i32>,
}

/// This object represents one size of a photo or a file / sticker thumbnail.
#[derive(Debug)]
pub struct PhotoSize {
    pub width: i32,
    pub height: i32,
    pub file_size: Option<i32>,
}

/// This object represents a sticker.
#[derive(Debug)]
pub struct Sticker {
    /// True, if the sticker is animated.
    pub is_animated: bool,
    /// Emoji associated with the sticker.
    pub emoji: Option<String>,
    /// Name of the sticker set to which the sticker belongs.
    pub set_name: Option<String>,
}

/// This object represents a video file.
#[derive(Debug)]
pub struct Video {
    pub duration: i32,
    pub mime_type: Option<String>,
    pub file_size: Option<i32>,
}

/// This object represents a video message (available in Telegram apps as of v.4.0).
#[derive(Debug)]
pub struct VideoNote {
    pub duration: i32,
    pub file_size: Option<i32>,
}

/// This object represents a voice note.
#[derive(Debug)]
pub struct Voice {
    pub duration: i32,
    pub mime_type: Option<String>,
    pub file_size: Option<i32>,
}

/// This object represents an animated emoji that displays a random value.
#[derive(Debug)]
pub struct Dice {
    /// Emoji on which the dice throw animation is based.
    pub emoji: String,
}

/// This object contains information about a poll.
#[derive(Debug)]
pub struct Poll {
    /// Poll type, currently can be “regular” or “quiz”.
    pub type_: String,
}

/// This object represents a venue.
#[derive(Debug)]
pub struct Venue {
    pub location: Location,
    pub title: String,
    pub address: String,
}

/// This object represents a point on the map.
#[derive(Debug)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
}
