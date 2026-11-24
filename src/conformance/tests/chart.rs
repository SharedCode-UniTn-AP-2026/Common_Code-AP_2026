use ap_2026_common_code_conformance::chart::{RefChart, deterministic_generation};

#[test]
fn equal() {
    deterministic_generation::<RefChart>();
}
