# Plan: property-based tests for `Cell` and `Direction`

## Goal

Add an integration test suite for the public API of `src/geometry/src/cell.rs`
(`Cell` and `Direction`), checking both its properties on ordinary inputs and its
documented behavior at the `i32` limits.

## Steps

1. **Set up property-based testing**
   - Add `proptest` as a dev-dependency of the geometry crate.
   - Define input strategies: any cell in a small range where no method can
     overflow, and any cell that a `Map` could hold (`|q|`, `|r|`, `|s|` at most
     `i32::MAX / 2`).

2. **Write the property tests** (by hand)
   - Distance: symmetry, agreement with an independent formula computed in
     `i64`, one step changes the distance by at most 1.
   - Neighbours: all at distance 1, all distinct, at least one is closer to any
     other cell.
   - Ring and disc: number of cells, distance from the center, strictly
     ascending order (which also rules out duplicates).
   - Limits: no overflow and correct results for cells inside any `Map`.

3. **Write the `Direction` tests** (by hand)
   - `relative_angle` is 3 exactly for opposite offsets and 1 exactly for
     adjacent neighbours.

4. **Write the documented panics tests** (AI-assisted)
   - One `#[should_panic]` test per `# Panics` section in `cell.rs`, matching
     the panic message.
   - One test right at the boundary that must not panic, to check the limit.

5. **Review** (AI-assisted)
   - Proofread comments and add doc comments to the test helpers.
   - Review the usefulness of each test and remove redundant ones.

6. **Validation**
   - Run the tests, `rustfmt` and Clippy.
