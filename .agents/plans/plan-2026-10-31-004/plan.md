# Plan: fix docs of `Cell::neighbours`, `Cell::disc`, `Cell::ring`

Scope: doc comments and inline comments only, in `src/geometry/src/cell.rs`. No code changes.

1. `neighbours`
   - Fix spelling ("neighbors" -> "neighbours") and wording.
   - Document the order of the returned cells (same as `Direction` variants).
   - Add a `# Panics` section (delegates to `neighbour`).
   - Add a doctest example with the six neighbours of the origin.
2. `disc`
   - Rewrite summary: cells *within* distance `radius`, including `self`.
   - Fix "Cells are Ord by ascending" -> "yielded in ascending order (by `q`, then by `r`)".
   - Document the number of cells (`3r(r+1)+1`).
   - Add `# Panics` (radius > i32::MAX; coordinate overflow during iteration).
   - Fix inner comment ("circle" -> hexagon, "higher" -> upper).
   - Remove blank line between doc and `fn`.
   - Add doctest example.
3. `ring`
   - Rewrite summary: cells at distance *exactly* `radius`; `ring(0)` yields `self`.
   - Same ordering fix, document count (`6r`).
   - Add `# Panics` (radius > i32::MAX; unchecked `+` overflow in debug builds).
   - Rewrite the two inner comments about `step`.
   - Add doctest example.
4. Run `cargo fmt --check`; run doctests (the `neighbours` one is expected to fail
   while `Direction::offset` is `todo!()`).
