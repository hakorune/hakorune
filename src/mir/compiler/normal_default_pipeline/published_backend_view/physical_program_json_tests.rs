use super::*;
use crate::mir::compiler::normal_default_pipeline::{MirCompiler, NormalCompileRequestV1};
use crate::parser::NyashParser;
use std::collections::HashMap;

fn request(source: &str) -> NormalCompileRequestV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        crate::parser::ParserBuildConfig::default(),
    )
    .expect("exact callable parse");
    let transformed = crate::r#macro::transform_normal_callable_program_v1(parsed)
        .expect("exact callable transform");
    let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed
    else { panic!("source identity must remain intact") };
    NormalCompileRequestV1::for_mir_mode_callable_source(source, None, HashMap::new())
}

#[test]
fn unannotated_pair_retains_contract_but_cannot_issue_physical_input() {
    use crate::mir::normal_callable_semantic_package::BirthFormalPhysicalDispositionV1;
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let mut compiler = MirCompiler::with_options(false);
        compiler.compile_normal_with_published(
            request(include_str!("../../../../../apps/typed-object-birth-min/main.hako")),
            |view, verification| -> Result<(), String> {
                assert!(verification.is_ok(), "{verification:?}");
                let entry = view.issue_lifecycle_compiled_entry_contract()?;
                let formals = entry.births()[0].formals();
                assert_eq!(formals.len(), 3);
                assert_eq!(formals[0].disposition(), None);
                for formal in &formals[1..] {
                    assert_eq!(formal.disposition(), Some(
                        BirthFormalPhysicalDispositionV1::UnavailableTaggedOrCheckedRepresentation));
                }
                assert!(matches!(view.issue_lifecycle_physical_abi_input(),
                    Err(error) if error.contains("formal-representation-unavailable")));
                Ok(())
            },
        ).unwrap();
    });
}

#[test]
fn serializer_rejects_nonissued_instruction_vocabulary() {
    let instruction = MirInstruction::Const {
        dst: ValueId::new(0),
        value: ConstValue::Bool(true),
    };
    assert!(matches!(
        encode_instruction(&instruction, &BTreeMap::new(), None, false),
        Err(error) if error.contains("instruction-unsupported"),
    ));
}

#[test]
fn direct_physical_input_rejects_unit_root_before_c() {
    let source = r#"
box Pair { left: i64 right: i64
  birth(left, right) { me.left = left me.right = right }
}
static box Main { main() { local pair = new Pair(10, 20) return } }
"#;
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let mut compiler = MirCompiler::with_options(false);
        let result = compiler.compile_normal_with_published(request(source), |view, _| {
            match view.issue_lifecycle_physical_abi_input() {
                Err(error) if error.contains("root-result-unavailable") => Ok(()),
                Err(error) => Err(format!("unexpected direct-input rejection: {error}")),
                Ok(_) => Err("Unit root unexpectedly entered direct physical input".into()),
            }
        });
        if let Err(error) = result { panic!("{error}"); }
    });
}
