# Plan: port legacy Line of Sight implementation

## Goal

Port the previously developed Line of Sight implementation into the current Geometry crate as a starting point for Michele's subsequent work.

## Steps

1. **Review current Geometry APIs**
   - Inspect `Cell`, `Map` and `Terrain`.
   - Identify differences from the previous implementation.

2. **Port the existing implementation**
   - Replace the `todo!()` placeholder in `src/geometry/src/los.rs`.
   - Preserve `visible()` and its existing helper functions.
   - Apply only the changes necessary for compatibility with the current crate.

3. **Port existing tests**
   - Preserve the previous tests where possible.
   - Adapt map construction to the current API.
   - Keep tests of private helper functions in the implementation file.
   - Avoid introducing the separate public test-vector feature.

4. **Document limitations**
   - Document the public API and its current guarantees.
   - Identify known limitations, including potential integer overflow, assertions and out-of-map behavior.
   - Leave algorithmic robustness improvements to Michele.

5. **Validation**
   - Run Cargo tests.
   - Run Rustfmt and Clippy.
   - Check for unintended changes.

6. **Finalize**
   - Document the changes, validation results and AI assistance in `result.md`.
   - Commit and open a pull request linked to Issue #37.
