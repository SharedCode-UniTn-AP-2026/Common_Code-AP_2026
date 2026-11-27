use ap_2026_common_code_conformance::chart::{RefChart, generate};

#[test]
fn conformance() {
    generate::<RefChart, RefChart>();
}
