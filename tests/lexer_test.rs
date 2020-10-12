use matchingram::lexer::{Lexer, Token::*};

#[test]
fn test_lexer() {
    let rule = r#"(not message.from.is_bot) or (message.text eq "/say")"#;
    let input = rule.chars().collect::<Vec<_>>();

    let mut lexer = Lexer::new(&input);
    lexer.tokenize().unwrap();

    let truthy = [
        (OpenParenthesis, String::from("(")),
        (Not, String::from("not")),
        (Field, String::from("message.from.is_bot")),
        (CloseParenthesis, String::from(")")),
        (Or, String::from("or")),
        (OpenParenthesis, String::from("(")),
        (Field, String::from("message.text")),
        (Operator, String::from("eq")),
        (Quote, String::from("\"")),
        (Letter, String::from("/say")),
        (Quote, String::from("\"")),
        (CloseParenthesis, String::from(")")),
        (EOF, String::from("")),
    ];

    assert_eq!(truthy.len(), lexer.output().len());
    for (i, mapping) in lexer.token_data_owner().unwrap().into_iter().enumerate() {
        assert_eq!(truthy[i], mapping);
    }
}

#[test]
fn test_lex_number() {
    let rule = r#"(message.longitude gt 1920.1080) or (message.text.len ge 120)"#;
    let input = rule.chars().collect::<Vec<_>>();

    let mut lexer = Lexer::new(&input);
    lexer.tokenize().unwrap();

    let truthy = [
        (OpenParenthesis, String::from("(")),
        (Field, String::from("message.longitude")),
        (Operator, String::from("gt")),
        (Decimal, String::from("1920.1080")),
        (CloseParenthesis, String::from(")")),
        (Or, String::from("or")),
        (OpenParenthesis, String::from("(")),
        (Field, String::from("message.text.len")),
        (Operator, String::from("ge")),
        (Integer, String::from("120")),
        (CloseParenthesis, String::from(")")),
        (EOF, String::from("")),
    ];

    assert_eq!(truthy.len(), lexer.output().len());
    for (i, mapping) in lexer.token_data_owner().unwrap().into_iter().enumerate() {
        assert_eq!(truthy[i], mapping);
    }

    // 测试带符号的数字解析。
    let rule = r#"(message.longitude gt -1920.1080) or (message.from.id gt -100001)"#;
    let input = rule.chars().collect::<Vec<_>>();
    let mut lexer = Lexer::new(&input);

    lexer.tokenize().unwrap();

    assert_eq!(
        Some(&(Decimal, "-1920.1080".to_owned())),
        lexer.token_data_owner().unwrap().get(3)
    );
    assert_eq!(
        Some(&(Integer, "-100001".to_owned())),
        lexer.token_data_owner().unwrap().get(9)
    );

    // TODO: 下面的错误 assertions 还需要保证字段的值。
    let rule = r#"(message.longitude gt .1920.1080)"#;
    let input = rule.chars().collect::<Vec<_>>();
    let mut lexer = Lexer::new(&input);

    assert!(lexer.tokenize().is_err());

    let rule = r#"(message.longitude gt 19201080.)"#;
    let input = rule.chars().collect::<Vec<_>>();
    let mut lexer = Lexer::new(&input);

    assert!(lexer.tokenize().is_err());

    let rule = r#"(message.longitude gt 1920.108.0)"#;
    let input = rule.chars().collect::<Vec<_>>();
    let mut lexer = Lexer::new(&input);

    assert!(lexer.tokenize().is_err());
}
