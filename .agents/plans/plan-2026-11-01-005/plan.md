# Plan: implement and document `Params`

**Target:** `src/shared/src/params.rs`

## Goal

Replace the placeholder `Params` implementation with a documented configuration struct for the Archipelago game.

Provide default values, methods for changing individual parameters, and a derived rock damage calculation.

## Steps

1. `Params`:
   - define `#[non_exhaustive] #[derive(Clone, Copy, Debug, PartialEq, Eq)] pub struct Params`;
   - implement the ten public fields with the specified types;
   - document the purpose and guarantees of the struct and each field;
   - include `bombard_damage`, as required by the Main document.

2. `Default`:
   - implement `Default` with the recommended values from Main §10.2;
   - set `archipelago_radius` to 12, `island_count` to 8, `cliff_count` to 6 and `rock_count` to 12;
   - set `vision_range` to 6, `bombard_range` to 2 and `bombard_damage` to 1;
   - set `hull_strength` to 3, `round_cap` to 200 and `round_deadline_ms` to 500;
   - ensure that the default configuration satisfies `archipelago_radius > vision_range` and `bombard_range <= vision_range`.

3. `with_*` methods:
   - add one method for each of the ten fields;
   - make every method accept the corresponding field type and return an updated `Self`;
   - mark each method with `#[must_use]`;
   - document that each method updates the corresponding parameter;
   - allow external crates to customize `Params` starting from `Params::default()`.

4. `rock_damage()`:
   - implement `pub fn rock_damage(&self) -> u32`;
   - calculate the damage using `self.hull_strength.div_ceil(2)`;
   - document the rule from Main §2.2;
   - do not store rock damage as a separate field.

5. Documentation:
   - explain the meaning and units of every public field;
   - document the `u16` limit of 65,535 for `cliff_count` and `rock_count`;
   - explain that the number of Ships is fixed at six and is not stored in `Params`;
   - explain the purpose of `#[non_exhaustive]`;
   - avoid undocumented panics.

6. Tests:
   - verify that all ten default values match the specification;
   - verify that the default configuration satisfies the radius and range constraints;
   - verify that every `with_*` method updates the intended field;
   - verify that builder methods can be chained;
   - verify that `rock_damage()` returns 2 for `hull_strength` 3 and behaves correctly for even values;
   - run `cargo test -p ap-2026-common-code-shared`, `cargo fmt --all -- --check`, `cargo clippy -p ap-2026-common-code-shared --all-targets -- -D warnings`, `cargo doc -p ap-2026-common-code-shared --no-deps` and `git diff --check`.

## Guidelines applied

- Follow `AGENTS.md` and the team's contribution guidelines.
- Document every public item and its guarantees.
- Avoid unnecessary dependencies and unrelated changes.
- Keep the implementation restricted to the `Params` task.
- Create a conventional Git commit after successful validation.

## Open points

- `bombard_damage` is required by Main §§5.5 and 10.2 but omitted from the Protocol's original field list. Follow the team's task distribution, which requires including it.
- The protocol uses `u16` for `cliff_count` and `rock_count`; preserve these types and document their limits.
- The task specifies setters but does not require runtime validation of arbitrary parameter combinations. Do not introduce additional validation behavior without an agreed requirement.
- Verify any remaining field-specific documentation against the applicable Main and Protocol sections before finalizing.
