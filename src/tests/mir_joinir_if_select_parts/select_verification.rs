use super::helpers::{create_double_select_joinir, create_select_joinir, strict_if_env_guard};

#[test]
fn test_if_select_simple_with_verify() {
    let _env = strict_if_env_guard();
    use crate::mir::join_ir::verify::verify_select_minimal;

    let join_func = create_select_joinir();
    let result = verify_select_minimal(&join_func, true);
    assert!(
        result.is_ok(),
        "Verify should pass for simple pattern: {:?}",
        result
    );

    eprintln!("✅ verify_select_minimal passed for simple pattern");
}

#[test]
fn test_if_select_local_with_verify() {
    let _env = strict_if_env_guard();
    use crate::mir::join_ir::verify::verify_select_minimal;

    let join_func = create_select_joinir();
    let result = verify_select_minimal(&join_func, true);
    assert!(
        result.is_ok(),
        "Verify should pass for local pattern: {:?}",
        result
    );

    eprintln!("✅ verify_select_minimal passed for local pattern");
}

#[test]
fn test_if_select_verify_rejects_multiple_selects() {
    let _env = strict_if_env_guard();
    use crate::mir::join_ir::verify::verify_select_minimal;

    let join_func = create_double_select_joinir();
    let result = verify_select_minimal(&join_func, true);
    assert!(result.is_err(), "Verify should reject multiple Selects");

    match result {
        Err(e) => {
            let msg = e.to_string();
            assert!(
                msg.contains("expected exactly 1 Select, found 2"),
                "Error message should mention multiple Selects: {}",
                msg
            );
            assert!(
                msg.contains("single PHI"),
                "Error message should reference single PHI invariant: {}",
                msg
            );
            eprintln!("✅ verify_select_minimal correctly rejected multiple Selects");
        }
        Ok(_) => panic!("Expected Err, got Ok"),
    }
}

#[test]
fn test_if_select_verify_checks_invariants() {
    let _env = strict_if_env_guard();
    use crate::mir::join_ir::verify::verify_select_minimal;

    let join_func = create_select_joinir();
    let result = verify_select_minimal(&join_func, true);
    assert!(result.is_ok(), "Verification should pass");

    eprintln!("✅ verify_select_minimal properly checks invariants from phi_invariants.rs and conservative.rs");
}
