use std::fmt;

/// Token issued by the Admiralty for a game turn and
/// included in a Ship's submission.
///
/// The nonce is private and the token cannot be cloned or copied.
/// Its `Debug` representation never reveals the nonce.
///
/// The Admiralty is responsible for issuing tokens using
/// nonces derived from the game seed, never from the clock.
///
/// This example does not compile because `TurnToken` is not `Clone`:
///
/// ```compile_fail
/// use ap_2026_common_code_shared::TurnToken;
///
/// let token = TurnToken::issue(123);
/// let cloned = token.clone();
/// ```
///
/// This example does not compile because `TurnToken` is not `Copy`:
///
/// ```compile_fail
/// use ap_2026_common_code_shared::TurnToken;
///
/// let token = TurnToken::issue(123);
/// let moved = token;
/// assert!(moved.matches(123));
/// assert!(token.matches(123));
/// ```
pub struct TurnToken {
    nonce: u64,
}

impl TurnToken {
    /// Creates a token containing the provided nonce.
    ///
    /// The caller must provide a nonce derived from the
    /// deterministic random stream of the game.
    #[must_use]
    pub fn issue(nonce: u64) -> Self {
        Self { nonce }
    }

    /// Returns `true` if the provided nonce matches the stored
    /// nonce, and `false` otherwise.
    ///
    /// Does not consume or modify the token.
    #[must_use]
    pub fn matches(&self, nonce: u64) -> bool {
        self.nonce == nonce
    }
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
}
