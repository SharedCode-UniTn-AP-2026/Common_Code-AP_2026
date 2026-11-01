# Result: implement and document `TurnToken`

## Authorship

The English documentation was firstly written by the user then finalized by ChatGPT (OpenAI, GPT-6).

ChatGPT also provided a solution with the two `compile_fail` doctests.

## Changes

- `TurnToken`: now is a struct containing a private `nonce: u64`.
- `TurnToken::issue()`: creates a token from the provided nonce.
- `TurnToken::matches()`: compares the stored nonce with the provided value without consuming the token.
- `Debug`: implemented manually to redact the nonce.
- No `Clone` or `Copy` implementations were added.
- Documentation: described the public API guarantees and the requirement for deterministic nonce generation.
- Unit tests: added tests for matching and non-matching nonces, and for redaction in `Debug`.
- Doctests: added two `compile_fail` examples verifying that the token cannot be cloned or copied.

The implementation follows the nonce-only design discussed in protocol PR #49, which was still awaiting final approval when reviewed.

## Validation

- `cargo test -p ap-2026-common-code-shared` passes: 3 unit tests and 2 doctests.
- `cargo clippy -p ap-2026-common-code-shared --all-targets -- -D warnings` passes without warnings.
- `cargo fmt --all -- --check` passes.
- `cargo doc -p ap-2026-common-code-shared --no-deps` successfully generated documentation before the final documentation edits.
