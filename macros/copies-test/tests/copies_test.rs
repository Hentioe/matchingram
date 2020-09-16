use copies_test::{it_works, ModelA, ModelB};

#[test]
fn test_it_works() {
    assert!(it_works());

    let b = ModelB { id: 1 };
    let a = ModelA::from(&b);

    assert_eq!(&b.id, a.id);
}
