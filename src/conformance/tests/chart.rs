use ap_2026_common_code_conformance::chart::{
    RefChart, deterministic_generation, generate_fail_island, generate_fail_params,
};

#[test]
fn equal() {
    deterministic_generation::<RefChart>();
}

#[test]
fn generate() {
    generate_fail_params::<RefChart>();
    generate_fail_island::<RefChart>();
}
