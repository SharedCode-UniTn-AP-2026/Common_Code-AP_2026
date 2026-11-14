# Result: document the `Map` trait

## Changes

- `src/geometry/src/map.rs`
  - Documented the `Map` trait: a complete hexagon centered on the origin,
    built from the two required methods, with `DefaultMap` as the provided
    implementation.
  - Documented `Map::radius`, including the implementor contract
    `radius() <= i32::MAX / 2` (the bound already enforced by
    `DefaultMap::try_new`).
  - Documented `Map::terrain`, including the contract that it returns `Some`
    exactly for the cells accepted by `contains`.
  - Added `# Panics` sections to the default methods whose new bodies can panic
    when the radius contract is broken:
    - `contains`: `Cell::s` overflows when `radius() > i32::MAX / 2`;
    - `corners`: negation overflows (with overflow checks) when
      `radius() > i32::MAX`;
    - `water_cells` / `clear_cells`: `Cell::disc` panics when
      `radius() > i32::MAX`.
  - Replaced every stale `[`Map::try_new`]` link (no longer resolvable since
    `Map` became a trait) with `[`DefaultMap::try_new`]`: in the `Error` doc and
    in the private helpers `start`, `check_out_of_map` and `cell_index`.
- `src/geometry/tests/los.rs`
  - Updated the `test_map` doc comment from `Map::try_new` to
    `DefaultMap::try_new`.

No code was changed; only doc comments.

## Not changed

- `cell.rs` links to `crate::Map` and `crate::Map::contains` still resolve (now
  to the trait). Its claims such as "never happens for a cell inside a `Map`"
  remain true given the new `radius() <= i32::MAX / 2` contract.
- The doc comments on the `impl Map for DefaultMap` methods were left as they
  were; they hold for `DefaultMap`, whose radius is always bounded.

## Validation

- `RUSTDOCFLAGS="-D warnings" cargo doc -p ap-2026-common-code-geometry --no-deps`
  passed, also with `--document-private-items` (no broken intra-doc links).
- `cargo test -p ap-2026-common-code-geometry` passed (4 unit, 12 integration,
  8 doctests).
- `cargo fmt --all -- --check` passed.
- `cargo clippy -p ap-2026-common-code-geometry --all-targets -- -D warnings`
  passed.

## Questions and answers

None asked during the conversation.
