use nyash_rust::mir::instruction_introspection;

// MIR14: ensure instruction roster remains stable
#[test]
fn mir14_shape_is_fixed() {
    let impl_names = instruction_introspection::mir14_instruction_names();
    assert_eq!(
        impl_names.len(),
        13,
        "MIR14 roster must match the current canonical 13-op profile"
    );
    assert!(
        impl_names.contains(&"UnaryOp"),
        "MIR14 must include UnaryOp"
    );
}
