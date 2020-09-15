use matchingram::rule_match_json;

#[test]
fn test_matcher() {
    let json_data = r#"
        {
            "text": "我是五个字",
            "from": {
                "id": 1000012,
                "first_name": "Rust",
                "is_bot": true
            }
        }
    "#;

    let rule = r#"(message.text.len eq 5)"#;
    assert!(rule_match_json(rule, json_data).unwrap());

    let rule = r#"(message.text.len gt 4)"#;
    assert!(rule_match_json(rule, json_data).unwrap());

    let rule = r#"(message.text.len gt 5)"#;
    assert!(!rule_match_json(rule, json_data).unwrap());

    let rule = r#"(not message.text.len gt 5)"#;
    assert!(rule_match_json(rule, json_data).unwrap());

    let rule = r#"(message.text.len ge 5)"#;
    assert!(rule_match_json(rule, json_data).unwrap());

    let rule = r#"(message.text.len ge 6)"#;
    assert!(!rule_match_json(rule, json_data).unwrap());

    let rule = r#"(message.text.len le 5)"#;
    assert!(rule_match_json(rule, json_data).unwrap());

    let rule = r#"(message.text.len le 4)"#;
    assert!(!rule_match_json(rule, json_data).unwrap());

    let rule = r#"(message.from.first_name in {"Java" "Rust"})"#;
    assert!(rule_match_json(rule, json_data).unwrap());

    let rule = r#"(message.from.first_name in {"Java" "Golang"})"#;
    assert!(!rule_match_json(rule, json_data).unwrap());

    let rule = r#"(message.from.first_name hd "Rus")"#;
    assert!(rule_match_json(rule, json_data).unwrap());
}
