use ap_2026_common_code_geometry::{test_vectors::test_visible, visible};

#[test]
fn test_public_visibility_vectors() {
    test_visible(visible);
}
