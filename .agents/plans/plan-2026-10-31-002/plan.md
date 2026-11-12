# Plan

1. Replace the `TODO` doc comments on `Direction` and its six variants in
   `src/geometry/src/cell.rs` with real docs: what the enum is (the six
   neighbour directions of a hex cell in axial coordinates), the
   counter-clockwise ordering starting at East, and the axial offset of each variant.
2. Fill the empty doctest of `Direction::offset` with one example.
3. Fill the empty doctest of `Direction::relative_angle` with one example and
   clarify that the result counts 60° steps.
4. Validate with `cargo fmt --check` and `cargo test --doc`.
