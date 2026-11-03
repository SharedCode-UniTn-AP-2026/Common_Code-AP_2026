use ap_2026_common_code_shared::TurnToken;

#[test]
fn test_matches() {
    let token = TurnToken::issue(12345);

    assert!(token.matches(12345));
    assert!(!token.matches(54321));
}

#[test]
fn test_debug_hides_nonce() {
    let nonce = 987_654_321_123_456_789;
    let token = TurnToken::issue(nonce);

    let output = format!("{token:?}");

    assert!(output.contains("TurnToken"));
    assert!(output.contains("<redacted>"));
    assert!(!output.contains(&nonce.to_string()));
}

#[test]
fn test_nonce_u64() {
    let nonce: u64 = u64::MAX;
    let token = TurnToken::issue(nonce);

    assert!(token.matches(nonce));
    assert!(!token.matches(u64::from(u32::MAX)));
    assert!(!token.matches(0));
}
