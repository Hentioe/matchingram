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
