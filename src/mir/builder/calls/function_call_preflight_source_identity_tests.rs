use super::PreparedRawExplicitExternCallV1;

#[test]
fn exact_resolver_symbol_is_required_before_argument_lowering() {
    assert!(PreparedRawExplicitExternCallV1::prepare(
        "env.get".to_owned(),
        Box::<str>::from("env.set"),
        Vec::new(),
    )
    .is_err());
    assert!(PreparedRawExplicitExternCallV1::prepare(
        "env.get".to_owned(),
        Box::<str>::from("env.get"),
        Vec::new(),
    )
    .is_ok());
}
