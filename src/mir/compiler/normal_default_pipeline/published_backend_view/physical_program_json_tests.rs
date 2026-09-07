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
fn unannotated_pair_issues_tagged_input_from_retained_contract() {
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
                let input = view.issue_lifecycle_physical_abi_input()?;
                let json = emit_lifecycle_physical_abi_json(&input)?;
                let decoded: serde_json::Value = serde_json::from_str(&json).unwrap();
                assert_eq!(decoded["schema"], "hako.published-lifecycle-physical-program.v2");
                assert_eq!(decoded["functions"][1]["params"].as_array().unwrap().len(), 2);
                assert_eq!(decoded["functions"][1]["params"][0]["representation"], "kind_payload_v1");
                // Capture this exact issued input for the companion C protocol test.
                std::fs::write(std::env::temp_dir().join("hako-issued-physical-v2.json"), json).unwrap();
                Ok(())
            },
        ).unwrap();
    });
}

#[test]
fn serializer_rejects_nonissued_instruction_vocabulary() {
    let instruction = MirInstruction::Const {
        dst: ValueId::new(0),
        value: ConstValue::Float(1.0),
    };
    assert!(matches!(
        encode_instruction(&instruction, &BTreeMap::new(), None, None),
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

#[test]
fn bool_actuals_keep_kind_and_do_not_specialize_the_birth_definition() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let original = include_str!("../../../../../apps/typed-object-birth-min/main.hako");
        let mut birth = None;
        for (ordinal, arguments) in ["true, 20", "10, false"].iter().enumerate() {
            let source = original.replace("new Pair(10, 20)", &format!("new Pair({arguments})"));
            MirCompiler::with_options(false).compile_normal_with_published(request(&source), |view, verification| {
                assert!(verification.is_ok(), "{verification:?}");
                let input = view.issue_lifecycle_physical_abi_input()?;
                let text = emit_lifecycle_physical_abi_json(&input)?;
                let json: Value = serde_json::from_str(&text).unwrap();
                if let Some(expected) = &birth { assert_eq!(expected, &json["functions"][1]); }
                else { birth = Some(json["functions"][1].clone()); }
                let instructions: Vec<_> = json["functions"][0]["blocks"].as_array().unwrap().iter()
                    .flat_map(|block| block["instructions"].as_array().unwrap()).map(|row| &row["instruction"]).collect();
                let constant = instructions.iter().find(|row| row["op"] == "const_bool").unwrap();
                assert_eq!(constant["value"], ordinal == 0);
                let args = input.entry().birth_calls()[0].actual().arguments();
                assert_eq!(super::super::physical_abi::scalar_actual_kind(args[ordinal].source().kind())?, 2);
                std::fs::write(std::env::temp_dir().join(format!("hako-issued-physical-v2-bool-{ordinal}.json")), text).unwrap();
                Ok::<(), String>(())
            }).unwrap();
        }
    });
}

#[test]
fn exact_annotation_remains_unavailable_despite_integer_actual() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let source = include_str!("../../../../../apps/typed-object-birth-min/main.hako")
            .replace("birth(left, right)", "birth(left: i64, right)");
        MirCompiler::with_options(false).compile_normal_with_published(request(&source), |view, _| {
            assert!(view.issue_lifecycle_physical_abi_input().unwrap_err().contains("formal-declaration-unavailable"));
            Ok::<(), String>(())
        }).unwrap();
    });
}
