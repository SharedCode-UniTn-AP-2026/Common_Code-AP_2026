use std::fmt;

/// Token issued by the Admiralty for a game turn and consumed by a Ship's submission.
///
/// Its `Debug` representation never reveals the nonce.
///
/// See Protocol §7.1, "The exchange".
#[derive(PartialEq, Eq)]
pub struct TurnToken {
    nonce: u64,
}

impl fmt::Debug for TurnToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TurnToken").field("nonce", &"<redacted>").finish()
    }
}

#[cfg(test)]
mod tests {
    use super::TurnToken;

    #[test]
    fn test_debug_hides_nonce() {
        let nonce = 987_654_321_123_456_789;
        let token = TurnToken { nonce };

        let output = format!("{token:?}");

        assert!(output.contains("TurnToken"));
        assert!(output.contains("<redacted>"));
        assert!(!output.contains(&nonce.to_string()));
    }
}
