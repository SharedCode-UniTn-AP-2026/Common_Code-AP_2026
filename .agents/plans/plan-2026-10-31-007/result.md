Changes: Documented the exact true-returning variants of Terrain::is_land and Terrain::is_water using intra-doc links. Behavior and existing user edits are unchanged.

Validation: Checked the patch against both method bodies. `cargo fmt --all -- --check` reports the existing missing space in `pub enum Terrain{`. No tests were needed for this documentation-only change. No commit was created; the file contains existing user changes and formatting does not pass.

Questions and answers: The user requested documentation for is_land and is_water. No clarification questions were asked.
