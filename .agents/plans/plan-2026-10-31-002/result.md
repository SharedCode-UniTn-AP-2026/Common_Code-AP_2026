# Result

## Changes (`src/geometry/src/cell.rs`)

- `Direction`: replaced the `TODO` doc comments with documentation of the enum
  (six neighbour directions, counter-clockwise from East, 60° apart,
  discriminants 0–5) and of each variant (its axial offset).
- `Direction::offset`: reworded the doc and added the example
  `Direction::NorthEast.offset() == (1, -1)`.
- `Direction::relative_angle`: documented that the result is in 60° steps,
  in `0..=3`, symmetric; added the example
  `Direction::East.relative_angle(Direction::SouthWest) == 2`.

## Validation

- `cargo fmt --check`: passes.
- `cargo test --doc`: 5 doctests pass, including the two new ones.

## Questions and answers

None.
