# Plan: document `Cell` and its coordinates

Target: `src/geometry/src/cell.rs`

## Goal

Replace the `TODO` doc comments on `Cell` and its fields with short docs that
state what each item guarantees, as required by `AGENTS.md`.

## Steps

1. `Cell` (struct):
   - say it is a hexagonal cell identified by axial coordinates `(q, r)`;
   - say the third cube coordinate is not stored, is always `s = -q - r`, and
     can be obtained with [`Cell::s`];
   - say any pair of `i32` is a valid `Cell`, and that membership in a map is
     decided by `Map::contains`.
2. `Cell::q`: axial column coordinate; increases going `Direction::East`.
3. `Cell::r`: axial row coordinate; increases going `Direction::SouthEast`.
4. Use intra-doc links (`[...]`) to related items so `cargo doc` renders them
   as links.

## Guidelines applied

- Document the convention (which axis, which direction it grows), not the obvious.
- Describe guarantees, not implementation details.
- Do not repeat the item name (`/// The q coordinate.` adds nothing).
- One line is enough for fields and variants.

## Open points

- The direction offsets assume the standard axial "pointy-top" layout;
  `Direction::offset` is still `todo!()`, so they must be checked against the
  project protocol.
