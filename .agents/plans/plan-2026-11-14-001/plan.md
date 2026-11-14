# Plan: document the `Map` trait

## Goal

Document the new `Map` trait and its required methods, and fix documentation
made stale by moving the map API from the `Map` struct to a trait with default
methods and a `DefaultMap` implementation.

## Steps

1. **Trait and required methods**
   - Replace the `TODO` doc comments on `Map`, `Map::radius` and `Map::terrain`.
   - State the contract implementors must uphold: a complete hexagon centered on
     the origin, `radius() <= i32::MAX / 2`, and `terrain` returning `Some`
     exactly for the cells accepted by `contains`.

2. **Default methods**
   - Add `# Panics` sections where the default bodies can panic when the
     radius contract is broken (`contains` via `Cell::s`, `corners` via
     negation, `water_cells`/`clear_cells` via `Cell::disc`).
   - Point out that `is_land`/`is_water` and the iterators rely on `terrain`.

3. **Stale links**
   - Replace `Map::try_new` links (no longer existing) with
     `DefaultMap::try_new` in `map.rs` and in `tests/los.rs`.

4. **Validation**
   - `cargo doc` (public and private items) with broken-link warnings denied.
   - `cargo test`, `cargo fmt --check`, `cargo clippy`.

5. **Finalize**
   - Write `result.md`.
