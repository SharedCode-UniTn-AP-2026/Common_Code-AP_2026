Updated src/geometry/src/map.rs documentation only:
- Map::start documents column-length conversion failures and arithmetic overflow.
- Map::cell_index documents radius, column and offset conversion failures, column bounds, and arithmetic overflow.
- Both explain why validated inputs avoid these panic conditions.

Validation:
- cargo doc -p ap-2026-common-code-geometry --no-deps --document-private-items --offline passed.
- git diff --check passed.
- cargo fmt --all -- --check failed before and after the documentation edit on existing expression formatting in map.rs.

No commit was created: map.rs already contains substantial uncommitted implementation absent from HEAD, so committing its formatting or documentation would include existing user work. Existing implementation and dependency edits were preserved.

Workflow deviation: the sandbox denied the initial plan-directory write; the patch applied before the plan write was retried successfully with elevated access.

Conversation questions and answers: none.
