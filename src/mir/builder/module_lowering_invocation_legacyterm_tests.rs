//! LEGACYTERM0-P0: failure and body-carrier proofs for the disconnected port.
//!
//! These fixtures call the real `ModuleLoweringPortV1::complete_legacy_child`
//! seam. They do not wire raw BoxDeclaration production lowering.

use std::panic::{catch_unwind, AssertUnwindSafe};

use crate::ast::ASTNode;
use crate::mir::builder::calls::CanonicalFunctionSessionErrorV1;
use crate::mir::region::RegionId;
use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirBuilder, MirFunction, MirType};
use crate::parser::NyashParser;

use super::module_lowering_invocation::{
    LegacyChildDraftAdmissionV1, ModuleLoweringInvocationV1, ModuleLoweringPortChildErrorV1,
};

fn draft(symbol: &str) -> MirFunction {
    MirFunction::new(
        FunctionSignature {
            name: symbol.to_owned(),
            params: Vec::new(),
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    )
}

fn seeded_invocation<'builder>(
    builder: &'builder mut MirBuilder,
) -> ModuleLoweringInvocationV1<'builder> {
    builder.enter_function_for_test("legacy_parent/0".to_owned());
    ModuleLoweringInvocationV1::open(builder)
}

fn assert_parent_restored(builder: &MirBuilder) {
    assert_eq!(
        builder
            .function_state
            .current_function
            .as_ref()
            .expect("restored parent function")
            .signature
            .name,
        "legacy_parent/0"
    );
    assert_eq!(builder.recursion_depth, 0);
}

fn exact_box_method_body(source: &str, box_name: &str, is_static: bool) -> Vec<ASTNode> {
    let ASTNode::Program { statements, .. } = NyashParser::parse_from_string(source).unwrap()
    else {
        panic!("expected source Program");
    };
    let ASTNode::BoxDeclaration {
        is_static: actual_static,
        methods,
        ..
    } = statements
        .iter()
        .find(|node| matches!(node, ASTNode::BoxDeclaration { name, .. } if name == box_name))
        .expect("expected exact BoxDeclaration")
    else {
        panic!("expected BoxDeclaration");
    };
    assert_eq!(*actual_static, is_static);
    let ASTNode::FunctionDeclaration { body, .. } =
        methods.get("run").expect("expected exact Box method")
    else {
        panic!("expected FunctionDeclaration");
    };
    body.clone()
}

#[test]
fn legacy_child_primary_and_during_cleanup_restore_without_collection() {
    let mut builder = MirBuilder::new();
    let mut invocation = seeded_invocation(&mut builder);
    let primary = invocation.with_module_port(|builder, port| {
        port.complete_legacy_child(
            builder,
            Vec::new(),
            LegacyChildDraftAdmissionV1::legacy_symbol("Legacy.primary/0".into(), 0),
            |_| Err("legacy primary".to_owned()),
        )
    });
    assert!(matches!(
        primary,
        Err(ModuleLoweringPortChildErrorV1::Session(
            CanonicalFunctionSessionErrorV1::Primary(_)
        ))
    ));
    invocation.with_header_port(|builder, headers| {
        assert_eq!(headers.symbol_count(), 0);
        assert_parent_restored(builder);
    });

    let during_cleanup = invocation.with_module_port(|builder, port| {
        port.complete_legacy_child(
            builder,
            Vec::new(),
            LegacyChildDraftAdmissionV1::legacy_symbol("Legacy.during/0".into(), 0),
            |builder| {
                builder.metadata_ctx.push_region(RegionId(811));
                builder.metadata_ctx.push_region(RegionId(812));
                Err("legacy primary plus cleanup".to_owned())
            },
        )
    });
    assert!(matches!(
        during_cleanup,
        Err(ModuleLoweringPortChildErrorV1::Session(
            CanonicalFunctionSessionErrorV1::DuringCleanup { .. }
        ))
    ));
    invocation.with_header_port(|builder, headers| {
        assert_eq!(headers.symbol_count(), 0);
        assert_parent_restored(builder);
    });
}

#[test]
fn legacy_child_success_cleanup_failure_restores_without_collection() {
    let mut builder = MirBuilder::new();
    let mut invocation = seeded_invocation(&mut builder);

    let result = invocation.with_module_port(|builder, port| {
        port.complete_legacy_child(
            builder,
            Vec::new(),
            LegacyChildDraftAdmissionV1::legacy_symbol("Legacy.cleanup/0".into(), 0),
            |builder| {
                builder.recursion_depth = 1;
                Ok(draft("Legacy.cleanup/0"))
            },
        )
    });
    assert!(matches!(
        result,
        Err(ModuleLoweringPortChildErrorV1::Session(
            CanonicalFunctionSessionErrorV1::Cleanup(_)
        ))
    ));
    invocation.with_header_port(|builder, headers| {
        assert_eq!(headers.symbol_count(), 0);
        assert_parent_restored(builder);
    });
}

#[test]
fn legacy_child_unwind_restores_without_collection() {
    let mut builder = MirBuilder::new();
    let mut invocation = seeded_invocation(&mut builder);

    let unwind = catch_unwind(AssertUnwindSafe(|| {
        let _ = invocation.with_module_port(|builder, port| {
            port.complete_legacy_child(
                builder,
                Vec::new(),
                LegacyChildDraftAdmissionV1::legacy_symbol("Legacy.panic/0".into(), 0),
                |_| -> Result<MirFunction, String> { panic!("legacy child panic") },
            )
        });
    }));
    assert!(unwind.is_err());
    invocation.with_header_port(|builder, headers| {
        assert_eq!(headers.symbol_count(), 0);
        assert_parent_restored(builder);
    });
}

#[test]
fn legacy_child_port_receives_exact_static_and_instance_box_bodies() {
    let fixtures = [
        (
            "static box RawStatic { run() { return 7 } }",
            "RawStatic",
            true,
            "RawStatic.run/0",
        ),
        (
            "box RawInstance { run() { return 8 } }",
            "RawInstance",
            false,
            "RawInstance.run/0",
        ),
    ];
    let mut builder = MirBuilder::new();
    let mut invocation = seeded_invocation(&mut builder);

    for (source, box_name, is_static, symbol) in fixtures {
        let body = exact_box_method_body(source, box_name, is_static);
        invocation
            .with_module_port(|builder, port| {
                port.complete_legacy_child(
                    builder,
                    body.clone(),
                    LegacyChildDraftAdmissionV1::legacy_symbol(symbol.into(), 0),
                    |builder| {
                        assert_eq!(
                            builder.function_state.compilation.fn_body_ast.as_deref(),
                            Some(body.as_slice())
                        );
                        Ok(draft(symbol))
                    },
                )
            })
            .unwrap();
    }

    invocation.with_header_port(|builder, headers| {
        assert_eq!(headers.symbol_count(), 2);
        assert!(headers.contains_symbol("RawStatic.run/0"));
        assert!(headers.contains_symbol("RawInstance.run/0"));
        assert_parent_restored(builder);
    });
}
