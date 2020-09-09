use matchingram::lexer::Lexer;
use matchingram::models::Message;
use matchingram::parser::Parser;

#[test]
fn test_parser() {
    let rule = r#"(not message.text contains_one {"say:" "说："})"#;
    let input = rule.chars().collect::<Vec<_>>();
    let mut lexer = Lexer::new(&input);

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
