use crate::ast::LiteralValue;
use crate::config::env::BuilderMethodizeCompatibilityV1;
use crate::mir::{ConstValue, MirBuilder, MirInstruction, MirType};

#[test]
fn mirbuilder_minimal_literal_integer_path_smoke() {
    let mut builder = MirBuilder::new();
    builder.prepare_module().expect("module shell");
    let literal = builder
        .build_literal(LiteralValue::Integer(0))
        .expect("literal integer");
    let module = builder.finalize_module(literal).expect("final module");
    let main = module
        .get_function("main")
        .expect("minimal literal path should create main");

    assert_eq!(main.signature.return_type, MirType::Integer);
    assert!(module.get_function("condition_fn").is_none());
    assert!(main.blocks.values().any(|block| {
        block.instructions.iter().any(|instruction| {
            matches!(
                instruction,
                MirInstruction::Const {
                    dst,
                    value: ConstValue::Integer(0),
                } if *dst == literal
            )
        }) && matches!(
            &block.terminator,
            Some(MirInstruction::Return { value: Some(value) }) if *value == literal
        )
    }));
}

#[test]
fn module_ingress_snapshots_explicit_methodize_policy_before_lowering() {
    let mut builder = MirBuilder::new();
    crate::test_support::with_env_var("HAKO_MIR_BUILDER_METHODIZE", "1", || {
        builder.prepare_module().expect("module shell");
    });

    // The lowering session owns the snapshot; once ingress has returned, the
    // restored process environment cannot alter the prepared Builder policy.
    assert_eq!(
        builder.comp_ctx.builder_methodize_compatibility,
        BuilderMethodizeCompatibilityV1::ExplicitLegacyCompatibility
    );
}

#[test]
fn normal_default_ingress_snapshots_explicit_methodize_policy() {
    crate::test_support::with_env_var("HAKO_MIR_BUILDER_METHODIZE", "1", || {
        let mut builder = MirBuilder::new();
        builder
            .prepare_normal_default_module(false)
            .expect("normal module shell");
        assert_eq!(
            builder.comp_ctx.builder_methodize_compatibility,
            BuilderMethodizeCompatibilityV1::ExplicitLegacyCompatibility
        );
    });
}

#[test]
fn module_ingress_snapshots_canonical_policy_for_unset_and_zero() {
    for value in [None, Some("0")] {
        crate::test_support::with_env_vars(&[("HAKO_MIR_BUILDER_METHODIZE", value)], || {
            let mut builder = MirBuilder::new();
            builder
                .prepare_normal_default_module(false)
                .expect("canonical methodize selector should prepare");
            assert_eq!(
                builder.comp_ctx.builder_methodize_compatibility,
                BuilderMethodizeCompatibilityV1::Canonical
            );
        });
    }
}

#[test]
fn invalid_methodize_selector_rejects_before_normal_module_mutation() {
    crate::test_support::with_env_var("HAKO_MIR_BUILDER_METHODIZE", "garbage", || {
        let mut builder = MirBuilder::new();
        let error = builder
            .prepare_normal_default_module(false)
            .expect_err("invalid methodize selector must fail at ingress");
        assert!(error.contains("mir/methodize/ingress"));
        assert!(builder.current_module.is_none());
        assert_eq!(
            builder.comp_ctx.builder_methodize_compatibility,
            BuilderMethodizeCompatibilityV1::Canonical
        );
    });
}
