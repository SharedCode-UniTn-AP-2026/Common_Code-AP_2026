use ap_2026_common_code_conformance::chart::{deterministic_generation, RefChart};

#[test]
fn equal() {
    deterministic_generation::<RefChart>();
}
