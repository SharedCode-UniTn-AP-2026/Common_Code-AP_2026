# Result: fix the English of the `messages` doc comments

## Changes

`src/messages/src/lib.rs`, doc comments only (no code changed):

| Item | Before | After |
|---|---|---|
| `Action` | Every action that a ship can do. | Every action that a ship can perform. |
| `Move` | A ship that move to an adjacent cell | A ship that moves to an adjacent cell. |
| `Move::to` | The cell where the ship want to go | The cell where the ship wants to go. |
| `Moor` | A ship that moor to an island | A ship that moors at an island. |
| `Moor::island` | (missing period) | The id of the island where the ship will moor. |
| `AskClue` | A ship that ask a Clue to the island where is moored. | A ship that asks the island where it is moored for a clue. |
| `AskClue` | It works only if the ship is moored to an island. | It works only if the ship is moored at an island. |
| `Bombard` | A ship that bombard another ship. | A ship that bombards another ship. |
| `Bombard` | It works only if the `ShipId` is in the sight. | It works only if the target is in sight. |
| `Bombard::target` | (missing period) | The `ShipId` of the target. |
| `ProposeAlliance` | A ship that propose an alliance to another ship. | A ship that proposes an alliance to another ship. |
| `ProposeAlliance::to` | The `ShipId` of the other ship to make an alliance. | The `ShipId` of the ship to make an alliance with. |
| `AcceptAlliance` | A ship that accept an alliance. | A ship that accepts an alliance. |
| `AcceptAlliance::proposer` | The `ShipId` that proposes the alliance. | The `ShipId` of the ship that proposed the alliance. |
| `BreakAlliance` | A ship that break an alliance. | A ship that breaks its alliance. |
| `BreakAlliance` | It works only if had already an alliance. | It works only if the ship is already in an alliance. |
| `Declare` | A ship that declare the treasure in his cell. | A ship that declares the treasure in its cell. |
| `Hold` | A ship that do nothing | A ship that does nothing. |
| `EndReason` | The possibles ending of the game for a ship | The possible endings of the game for a ship. |
| `RoundCap` | The round are finished | The rounds are over. |

Notes on the meaning-related fixes:

- `Bombard`: an id cannot be "in sight"; the target ship is.
- `AcceptAlliance::proposer`: the ship proposes the alliance, not its id.
- `AskClue`: "Clue" lowercased because there is no `Clue` type in the codebase.

## Validation

- `cargo fmt --all -- --check` passed.
- `cargo clippy -p ap-2026-common-code-messages -- -D warnings` passed.
- `RUSTDOCFLAGS="-D warnings" cargo doc -p ap-2026-common-code-messages --no-deps` passed.

## Questions and answers

None.
