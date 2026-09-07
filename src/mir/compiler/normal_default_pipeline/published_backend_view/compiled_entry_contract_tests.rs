use super::*;
use crate::mir::{MirCompiler, NormalCompileRequestV1};

#[test]
fn per_new_actuals_survive_definition_dedup_and_are_consumed_once() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        // Existing two-destination cohort; Local is retained, never promoted to i64.
        let text = "box Page { left: i64\nright: i64\nbirth(a, b) { me.left = a\nme.right = b } } static box Main { main() { local a = 7\nlocal b = 9\nlocal first = new Page(a, b)\nlocal second = new Page(b, a)\nreturn 0 } }";
        let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
            text, crate::parser::ParserBuildConfig::default()).unwrap();
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
            crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap()
            else { panic!("source identity lost") };
        let request = NormalCompileRequestV1::for_mir_mode_callable_source(source, None, Default::default());
        MirCompiler::with_options(false).compile_normal_with_published(request, |view, verification| {
            assert!(verification.is_ok(), "{verification:?}");
            let contract = view.issue_lifecycle_compiled_entry_contract()?;
            assert_eq!(contract.births().len(), 1);
            assert_eq!(contract.birth_calls().len(), 2);
            let actuals = view.retained_root_source().unwrap().birth_actuals();
            assert_eq!(actuals.len(), 2);
            assert_ne!(actuals[0].site(), actuals[1].site());
            assert_ne!(actuals[0].receiver(), actuals[1].receiver());
            for call in contract.birth_calls() {
                assert_eq!(call.function_index(), 1);
                assert!(call.actual().arguments().iter().all(|argument| matches!(argument.source().kind(),
                    crate::mir::normal_callable_semantic_package::OrdinaryNewTrivialArgumentKindV1::Local { .. })));
            }
            let [root, births @ ..] = contract.program().functions() else { panic!("root missing") };
            let PublishedLifecyclePhysicalFunctionRoleV1::BirthUnit { abi } = births[0].role()
                else { panic!("Birth ABI missing") };
            let formals = contract.births()[0].formals();
            assert!(formals[0].contract().is_none());
            assert_eq!(formals.len(), abi.formal_contracts().len() + 1);
            for (formal, source) in formals[1..].iter().zip(abi.formal_contracts()) {
                assert_eq!(formal.contract(), Some(source),
                    "declaration, binding, ordinal and exact use sites must survive together");
                assert_eq!(formal.source_ordinal(), Some(source.ordinal()));
                assert_eq!(formal.disposition(), Some(source.disposition()));
            }
            let mut reordered = actuals.to_vec();
            reordered.reverse();
            assert_eq!(issue_birth_calls(root, births, &reordered)?, contract.birth_calls());
            assert!(issue_birth_calls(root, births, &actuals[..1]).unwrap_err().contains("actual-mismatch"));
            let duplicated = vec![actuals[0].clone(), actuals[0].clone()];
            assert!(issue_birth_calls(root, births, &duplicated).unwrap_err().contains("actual-membership"));
            assert!(view.issue_lifecycle_physical_abi_input().unwrap_err().contains("actual-kind-unavailable"));
            Ok::<(), String>(())
        }).unwrap();
    });
}
