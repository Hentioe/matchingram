use auto_from_test::{it_works, ModelA, ModelB};

#[test]
fn test_it_works() {
    assert!(it_works());

    let b = ModelB {
        _id: 1,
        text: String::from("文本"),
    };
    let a = ModelA::from(&b);

    assert_eq!(&b._id, a.id);
}
