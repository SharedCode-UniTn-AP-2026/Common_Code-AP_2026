# Plan: fix the English of the `messages` doc comments

## Goal

Correct grammar and wording in the doc comments of `Action` and `EndReason`
in `src/messages/src/lib.rs`, keeping the original style and length.

## Steps

1. Fix subject-verb agreement in the `Action` variant docs
   ("A ship that move" -> "A ship that moves", etc.).
2. Fix wrong or unclear phrasing:
   - `AskClue`: "ask a Clue to the island where is moored";
   - `Bombard`: "the `ShipId` is in the sight" (an id is not in sight, the
     target is);
   - `AcceptAlliance::proposer`: "The `ShipId` that proposes" (the ship
     proposes, not the id);
   - `BreakAlliance`: "if had already an alliance" (missing subject);
   - `EndReason`: "The possibles ending", "The round are finished".
3. Add missing final periods for consistency.
4. Do not touch code, attributes or the "Do not change these" header.

## Validation

- `cargo fmt --all -- --check`
- `cargo clippy -p <messages crate> -- -D warnings`
- `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps` for the crate.
