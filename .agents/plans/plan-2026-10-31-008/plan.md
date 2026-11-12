# Plan: implement and document `TurnToken`

Target: `src/shared/src/turntoken.rs`

## Goal

Replace the placeholder `TurnToken` implementation with the API proposed in protocol issue #62 and PR #49.

Document its guarantees and ensure that the token cannot be cloned, copied or accidentally exposed through `Debug`.

## Steps

1. `TurnToken` (struct):
   - store a private `nonce: u64` field;
   - do not derive `Clone` or `Copy`;
   - document that tokens are issued by the Admiralty and used to authenticate submissions for a round.

2. `TurnToken::issue(nonce: u64)`:
   - create a new token containing the provided nonce;
   - document that the nonce must come from a deterministic random stream derived from the game seed.

3. `TurnToken::matches(&self, nonce: u64)`:
   - return `true` when the provided nonce matches the stored one;
   - return `false` otherwise;
   - do not consume the token.

4. `Debug`:
   - implement `Debug` manually;
   - ensure that the stored nonce is never exposed.

5. Tests:
   - verify that `matches()` accepts the correct nonce;
   - verify that `matches()` rejects an incorrect nonce;
   - verify that `Debug` does not reveal the nonce;
   - add a `compile_fail` doctest verifying that `TurnToken` cannot be cloned.

6. Validation:
   - run unit tests and doctests;
   - check formatting with `cargo fmt`;
   - run Clippy Pedantic;
   - generate the Rust documentation.

## Guidelines applied

- Document public API guarantees, not obvious implementation details.
- Do not introduce unnecessary public methods.
- Do not use `unsafe` code.
- Avoid undocumented panics.
- Follow `AGENTS.md` and the course AI Policy.
- Keep changes limited to the `TurnToken` implementation and its tests.

## Open points

- Protocol PR #49 is still open and labelled `TO BE DISCUSSED`.
- `ShipId` and `Round` are omitted because the Admiralty already tracks them separately.
- The nonce must be supplied by the caller from a deterministic stream derived from the game seed. `TurnToken` does not generate randomness itself.
- Only the Admiralty should issue tokens; this restriction is a protocol responsibility rather than a Rust access restriction.
