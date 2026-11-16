Changes: Replaced the Terrain enum's TODO with documentation describing map-cell terrain and its water and land classifications. Preserved existing user changes.

Validation: Reviewed the diff. `cargo fmt --all -- --check` reports existing spacing in `pub enum Terrain{`; no formatting or behavior changes were made. No commit was created because formatting does not pass and the file contains pre-existing user edits.

Questions and answers: The user requested documentation for the Terrain enum. No clarification questions were asked.
