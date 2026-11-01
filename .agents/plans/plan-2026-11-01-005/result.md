# Result: implement and document `Params`

## Authorship

OpenAI Codex was used to review the implementation plan and suggest documentation improvements.

ChatGPT assisted with the preparation of `plan.md` and `result.md`.

## Changes

- Replaced the placeholder in `src/shared/src/params.rs` with the documented `Params` struct.
- Added all ten public configuration fields, including `bombard_damage`.
- Added `#[non_exhaustive]` and the required derived traits.
- Implemented `Default` using the values specified in Main §10.2.
- Implemented ten `with_*` methods, each marked with `#[must_use]`.
- Implemented `rock_damage()` using `hull_strength.div_ceil(2)`.
- Documented field meanings, units and relevant limitations.
- Added three integration tests in `src/shared/tests/params.rs`, covering default values, builder methods and rock damage.

## Validation

- `cargo fmt --all -- --check` — passed.
- `cargo test -p ap-2026-common-code-shared` — passed: 1 unit test and 3 integration tests.
- `cargo clippy -p ap-2026-common-code-shared --all-targets -- -D warnings` — passed without warnings.
- `cargo doc -p ap-2026-common-code-shared --no-deps` — passed.
- `git diff --check` — passed.

## Questions and answers

- **Which branch was used?** `geometry-feat-params`, created from `geometry-feat-first-impl`.
- **Why does `Params` use `#[non_exhaustive]`?** To allow future fields without permitting external crates to construct the struct directly using a struct literal.
- **Why implement `Default`?** To provide a standard configuration and an entry point for constructing `Params` from other crates.
- **Why provide `with_*` methods?** To allow callers to customize individual parameters and chain configuration changes.
- **Why is rock damage not stored as a field?** It is derived from `hull_strength` using ceiling division by two.
- **Are parameter combinations automatically validated?** No. The struct stores configuration values without enforcing combinations of game constraints.
- **Where should the tests be located?** According to the team's guidance, unit tests belong in their implementation file, while tests using only the public API belong in `tests/`.
- **Why were the tests moved?** All three `Params` tests use only the public API, so they were moved to `src/shared/tests/params.rs`.
- **Why did Cargo initially run the same tests twice?** The original unit-test module had not yet been removed after creating the integration tests. The duplicate tests were subsequently deleted.
- **Were Git changes from other team members checked?** Yes. Remote branches were fetched and inspected without merging changes into the current branch.
