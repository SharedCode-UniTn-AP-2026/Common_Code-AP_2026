# Plan: implement and document shared small types

Target: `src/shared/src/lib.rs`

## Goal

Complete the documentation of `ShipId`, `IslandId`, `Round` and `Seed`, and add `ShipId::ALL` as required by the Geometry team's task distribution.

Preserve the existing type definitions and public interfaces.

## Steps

1. `ShipId`:
   - document that the game has six ships, identified by IDs from 0 to 5;
   - explain that Ship `i` starts at map corner `i`;
   - add `pub const ALL: [ShipId; 6]` containing all six identifiers in ascending order;
   - document the ordering guarantee of `ALL`.

2. `IslandId`:
   - document that it identifies an island in the archipelago;
   - preserve the existing `u8` newtype definition.

3. `Round`:
   - document that game rounds are numbered starting from 1;
   - preserve the existing `u32` type alias.

4. `Seed`:
   - document that it is used for reproducible randomness;
   - specify that randomness must derive from the game seed, not from the system clock;
   - preserve the existing `u64` type alias.

5. Tests:
   - verify that `ShipId::ALL` contains exactly six elements;
   - verify that the identifiers are ordered from 0 to 5;
   - run `cargo test`, `cargo fmt --all -- --check` and `cargo clippy --all-targets -- -W clippy::pedantic`.

## Guidelines applied

- Document guarantees, not obvious implementation details.
- Keep documentation concise and precise.
- Preserve the type definitions specified by the protocol.
- Do not introduce unnecessary public APIs.
- Do not modify `Cell`, `Direction`, `Map`, `Terrain` or Line of Sight.
- Follow `AGENTS.md` and the course AI Policy.

## Open points

- `ShipId(pub u8)` allows constructing values outside the range 0–5. The documentation must distinguish valid game identifiers from values accepted by the Rust type.
- `Round` is a type alias and does not prevent the value 0, even though game rounds start at 1.
- The existing module contains the instruction `Do not change these`; the underlying type definitions will remain unchanged, with only the required documentation and associated constant added.
