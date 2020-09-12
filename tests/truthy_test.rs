use matchingram::truthy::IsTruthy;

// 由于 `IsTruthy` 相关实现使用了不稳定的 `min_specialization` 功能，需要保证测试通过。
#[test]
fn test_is_truthy() {
    assert!(Some(0).is_truthy());
    assert!(!None.is_truthy());
    assert!(Some(true).is_truthy());
    assert!(!Some(false).is_truthy());
    assert!(true.is_truthy());
    assert!(!false.is_truthy());
}
