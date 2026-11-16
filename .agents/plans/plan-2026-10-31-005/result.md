Documented Map invariants; Error and every variant; every public and private Map method. Documentation describes validation order, too-few/too-many input handling, boundary behavior, corner order and radius-zero behavior, iterator ordering, helper preconditions, and panic conditions. No executable code changed.

Validation:
- cargo test -p ap-2026-common-code-geometry --offline: passed (8 existing doctests; no unit tests). Existing unused-variable and naming warnings remain.
- RUSTDOCFLAGS='-D warnings' cargo doc -p ap-2026-common-code-geometry --no-deps --offline: passed.
- Comment-stripped comparison with the pre-edit file: identical.
- rustfmt --check --edition 2024 src/geometry/src/map.rs: failed on pre-existing import order and code layout. Formatting was left unchanged to preserve the documentation-only scope and existing user edits.

No commit was made: rustfmt does not pass on the pre-existing code, and map.rs, the geometry manifest and Cargo.lock already contained uncommitted user changes. The required formatting-only commit was not made because isolating it from those existing edits would expand the task scope.

Conversation questions and answers: none. The protected .agents directory required an escalated tool invocation to create the required records; it succeeded.
