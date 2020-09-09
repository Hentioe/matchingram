use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn rule_match(rule: &str) -> matchingram::result::Result<bool> {
    let message = matchingram::models::Message {
        text: Some(MESSAGE_TEST.to_owned()),
        ..Default::default()
    };

    matchingram::rule_match(rule, &message)
}

fn compile_rule(rule: &str) -> matchingram::result::Result<matchingram::Matcher> {
    matchingram::compile_rule(rule)
}

fn load_data_file(fname: &str) -> Vec<u8> {
    use std::env;
    use std::fs::read;

    let mut fpath = env::current_dir().unwrap();

    fpath.push("data");
    fpath.push(fname);

    match read(&fpath) {
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                eprintln!(
                    "the data file `{}` failed to load, try to run `cargo make data.gen` command?",
                    &fpath.to_str().unwrap()
                );
            }

            panic!(e.to_string());
        }
        Ok(bytes) => bytes,
    }
}

static MESSAGE_TEST: &'static str = r#"
太平洋是地球上五大洋中面积最大的洋，面积1.813亿平方公里，它从北冰洋一直延伸至南冰洋，其西面为亚洲、大洋洲，东面为美洲，覆盖着地球约46%的水面及约32%的总面积，比地球上所有陆地面积的总和还要大。赤道将太平洋分为北太平洋及南太平洋。北面连接白令海峡，南面则以南纬60度为界。

位于北太平洋西侧的马里亚纳海沟是地球表面最深的位置。海沟最大深度为海平面下 10,911米（35,797英尺）。

太平洋之名称起源自拉丁文“Mare Pacificum”，意为“平静的海洋”，由航海家麦哲伦命名。受雇于西班牙的葡萄牙航海家麦哲伦于1520年10月，率领5艘船从大西洋找到了一个西南出口（麦哲伦海峡）向西航行，经过38天的惊涛骇浪后到达一个平静的洋面，他因称之为太平洋。
"#;

fn criterion_benchmark(c: &mut Criterion) {
    let regular_rule = r#"(message.text contains_all {"太平洋" "年" "月"})"#;
    let regular_negate_rule = r#"(not message.text contains_all {"太平洋" "年" "月"})"#;
    let long_rule = r#"(
        message.text contains_one {"太"} and
        message.text contains_one {"平"} and
        message.text contains_one {"洋"} and
        message.text contains_one {"年"} and
        message.text contains_one {"月"}
    )"#;
    let longer_rule = r#"(
        message.text contains_one {"太"} and
        message.text contains_one {"平"} and
        message.text contains_one {"洋"} and
        message.text contains_one {"年"} and
        message.text contains_one {"月"} and
        message.text contains_all {"太"} and
        message.text contains_all {"平"} and
        message.text contains_all {"洋"} and
        message.text contains_all {"年"} and
        message.text contains_all {"月"} and
        message.text contains_all {"太" "平" "洋" "年" "月"}
    )"#;

    let mb_rule_data = load_data_file("1mb-rule.txt");
    let mb_rule = std::str::from_utf8(&mb_rule_data).unwrap();

    assert!(matches!(rule_match(regular_rule), Ok(true)));
    assert!(matches!(rule_match(regular_negate_rule), Ok(false)));
    assert!(matches!(rule_match(long_rule), Ok(true)));
    assert!(matches!(rule_match(longer_rule), Ok(true)));
    assert!(matches!(compile_rule(mb_rule), Ok(_)));

    c.bench_function("rule_match regular-rule", |b| {
        b.iter(|| rule_match(black_box(regular_rule)))
    });
    c.bench_function("rule_match regular-netate-rule", |b| {
        b.iter(|| rule_match(black_box(regular_negate_rule)))
    });
    c.bench_function("rule_match long-rule", |b| {
        b.iter(|| rule_match(black_box(long_rule)))
    });
    c.bench_function("rule_match longer-rule", |b| {
        b.iter(|| rule_match(black_box(long_rule)))
    });
    c.bench_function("compile_rule 1mb-rule", |b| {
        b.iter(|| compile_rule(black_box(mb_rule)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
