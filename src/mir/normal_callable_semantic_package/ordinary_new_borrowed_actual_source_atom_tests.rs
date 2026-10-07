//! Passive source atoms do not infer flow, ownership or transport capability.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::borrowed_actual_source_atom_v1;

#[test]
fn borrowed_source_atoms_preserve_literal_tags_signed_site_and_unsupported_spelling() {
    let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
        "box Token {} box Transport { probe(a,b,c,d,e,f,g): i64 { return 0 } } static box Main { main() { local s = new Transport() local out = s.probe(7,true,null,-7,\"no\",7+1,new Token()) return 0 } }", crate::parser::ParserBuildConfig::default(),
    ).unwrap();
    let transformed = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap()
    });
    let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed else {
        panic!("original source");
    };
    let mut resolver =
        crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1::new(71).unwrap();
    let batch = crate::mir::callable_semantic_batch::issue_resolved_callable_semantic_batch_v1(
        &mut resolver,
        source,
    )
    .unwrap();
    let mut checked = 0;
    for declaration in batch.declarations() {
        batch.with_lowering_input(declaration.batch_slot(), |input| {
            for (_, call) in input.function().method_calls() {
                if call.selector() != "probe" { continue; }
                checked += 1;
                let atoms: Vec<_> = call.arguments().iter().map(|argument| borrowed_actual_source_atom_v1(input, argument.site())).collect();
                assert_eq!(atoms, vec![Some(BorrowedCallActualValueV1::Integer(7)), Some(BorrowedCallActualValueV1::Bool(true)), Some(BorrowedCallActualValueV1::Null), Some(BorrowedCallActualValueV1::Integer(-7)), None, None, None]);
                assert!(borrowed_actual_source_atom_v1(input, call.receiver_site()).is_some_and(|atom| matches!(atom, BorrowedCallActualValueV1::Binding(binding) if binding.owner() == input.owner())));
            }
        }).unwrap();
    }
    assert_eq!(checked, 1);
}

#[test]
fn borrowed_source_atoms_keep_parameter_identity_without_home_or_scalar_grant() {
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        "box Transport { probe(p): i64 { return 0 } forward(p) { local out = me.probe(p) return 0 } } static box Main { main() { local s = new Transport() local out = s.forward(7) return 0 } }"
    ).unwrap();
    let mut checked = 0;
    for contract in &package.parameter_contracts {
        package
            .batch()
            .with_lowering_input(contract.batch_slot, |input| {
                for (_, call) in input.function().method_calls() {
                    if call.selector() != "probe" {
                        continue;
                    }
                    let original = input
                        .function()
                        .variable_ref(call.arguments()[0].site())
                        .unwrap();
                    let ResolvedLexicalRefV1::Local(binding) = original else {
                        panic!("original local parameter");
                    };
                    assert_eq!(
                        borrowed_actual_source_atom_v1(input, call.arguments()[0].site()),
                        Some(BorrowedCallActualValueV1::Binding(binding))
                    );
                    checked += 1;
                }
            })
            .unwrap();
    }
    assert_eq!(checked, 1);
}
