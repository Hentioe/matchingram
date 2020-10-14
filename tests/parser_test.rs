use matchingram::lexer::Lexer;
use matchingram::models::{Location, Message};
use matchingram::parser::Parser;

#[test]
fn test_parser() {
    let rule = r#"(not message.text any {"say:" "说："} and not message.from.is_bot)"#;
    let input = rule.chars().collect::<Vec<_>>();
    let mut lexer = Lexer::new(&input);

    lexer.tokenize().unwrap();

    let parser = Parser::new(&mut lexer).unwrap();
    let mut matcher = parser.parse().unwrap();

    let text1 = format!("Jay say: Hello!");
    let text2 = format!("小明说：你好！");
    let text3 = format!("怎么发消息还得遵循格式啊？");

    let message1 = Message {
        text: Some(text1),
        ..Default::default()
    };
    let message2 = Message {
        text: Some(text2),
        ..Default::default()
    };
    let message3 = Message {
        text: Some(text3),
        ..Default::default()
    };

    assert!(!matcher.match_message(&message1).unwrap());
    assert!(!matcher.match_message(&message2).unwrap());
    assert!(matcher.match_message(&message3).unwrap());
}

#[test]
fn test_parse_number() {
    let rule = r#"(message.text.len gt -5) or (message.location.latitude gt -0.2)"#;
    let input = rule.chars().collect::<Vec<_>>();
    let mut lexer = Lexer::new(&input);
    let parser = Parser::new(&mut lexer).unwrap();
    let mut matcher = parser.parse().unwrap();

    // TODO: 以下的 assertions 应该以测试 Matcher 结构的字段内容为主，而不是测试匹配结果。

    let message1 = Message {
        text: Some(String::from("我有六个字。")),
        ..Default::default()
    };
    let message2 = Message {
        location: Some(Location {
            latitude: -0.1,
            longitude: 0.0,
        }),
        ..Default::default()
    };

    assert!(matcher.match_message(&message1).unwrap());
    assert!(matcher.match_message(&message2).unwrap());
}
