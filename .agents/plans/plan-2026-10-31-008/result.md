# Result: implement and document `TurnToken`

## Authorship

The English documentation was initially written by the user and then finalized with assistance from ChatGPT (OpenAI, GPT-6).

ChatGPT also provided the two `compile_fail` doctests, assisted with moving the tests to the integration test directory, and provided the additional `u64` test.

## Changes

- `TurnToken`: implemented as a struct containing a private `nonce: u64`.
- `TurnToken::issue()`: creates a token from the provided nonce.
- `TurnToken::matches()`: compares the stored nonce with the provided value without consuming the token.
- `Debug`: implemented manually to redact the nonce.
- No `Clone` or `Copy` implementations were added.
- Documentation: described the public API guarantees and the requirement for deterministic nonce generation.
- Integration tests: moved `test_matches` and `test_debug_hides_nonce` from `src/shared/src/turntoken.rs` to `src/shared/tests/turntoken.rs`, following the team's testing conventions.
- Additional integration test: added `test_nonce_u64` to verify that the API supports `u64::MAX` and correctly distinguishes it from other nonce values.
- Doctests: retained two `compile_fail` examples verifying that the token cannot be cloned or copied.

The implementation follows the nonce-only design discussed in protocol PR #49, which was still awaiting final approval when reviewed.

## Validation

- `cargo test -p ap-2026-common-code-shared` passes: 1 existing unit test (`ShipId`), 3 `TurnToken` integration tests, and 2 `compile_fail` doctests.
- `cargo clippy -p ap-2026-common-code-shared --all-targets -- -D warnings` passes without warnings.
- `cargo fmt --all -- --check` passes.
- `git diff --check` passes.
- `cargo doc -p ap-2026-common-code-shared --no-deps` successfully generated documentation before the latest test relocation.
