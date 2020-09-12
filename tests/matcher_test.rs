use matchingram::rule_match_json;

#[test]
fn test_matcher() {
    let json_data = r#"
        {
            "text": "我是五个字",
            "from": {
                "first_name": "Rust",
                "is_bot": true
            }
        }
    "#;

    let rule = r#"(message.text.size eq 5)"#;
    assert!(rule_match_json(rule, json_data).unwrap());

    let rule = r#"(message.text.size gt 4)"#;
    assert!(rule_match_json(rule, json_data).unwrap());

    let rule = r#"(message.text.size gt 5)"#;
    assert!(!rule_match_json(rule, json_data).unwrap());

    let rule = r#"(not message.text.size gt 5)"#;
    assert!(rule_match_json(rule, json_data).unwrap());

    let rule = r#"(message.text.size ge 5)"#;
    assert!(rule_match_json(rule, json_data).unwrap());

    let rule = r#"(message.text.size ge 6)"#;
    assert!(!rule_match_json(rule, json_data).unwrap());

    let rule = r#"(message.text.size le 5)"#;
    assert!(rule_match_json(rule, json_data).unwrap());

    let rule = r#"(message.text.size le 4)"#;
    assert!(!rule_match_json(rule, json_data).unwrap());
}
