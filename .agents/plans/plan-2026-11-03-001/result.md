# Result: port legacy Line of Sight implementation

## Authorship

The original Line of Sight implementation and tests were previously developed by the user.

Codex (OpenAI) assisted with adapting the original tests to the current Geometry API since the task was very redundant.

## Changes

- Ported the previous Line of Sight implementation into `src/geometry/src/los.rs`.
- Preserved `visible()` and its three helper functions.
- Updated the `Cell::distance()` call to match the current API.
- Added documentation describing the existing behavior and limitations.
- Preserved 7 unit tests for the private helper functions.
- Moved and adapted 12 public API tests to `src/geometry/tests/los.rs`.
- Replaced direct access to private `Map` fields with `Map::try_new()` in the integration tests.
- Added module-level documentation for `los.rs`.

The implementation is intended as a baseline for @alterax05 subsequent improvements.

## Validation

- `cargo check -p ap-2026-common-code-geometry` passed.
- `cargo test -p ap-2026-common-code-geometry` passed:
  - 7 unit tests.
  - 12 integration tests.
  - 8 doctests.
- `cargo fmt --all -- --check` passed.
- `git diff --check` reported only a line-ending conversion warning.
- `cargo clippy -p ap-2026-common-code-geometry --all-targets -- -D warnings` failed due to two existing `must_use_candidate` errors in `src/geometry/src/terrain.rs`, outside the scope of this task.
