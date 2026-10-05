//! Source-issued direct field results retain their canonical finishing positions.
use super::*;

#[test]
fn root_and_borrowed_child_field_returns_survive_canonical_finishing() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for (shape, main) in [
            ("root", "local item = new Item() return item.value"),
            (
                "object",
                "local recv = new Transport() local item = new Item() return recv.read(item)",
            ),
            (
                "null",
                "local recv = new Transport() return recv.read(null)",
            ),
        ] {
            let reader = if shape == "root" {
                ""
            } else {
                "box Transport { flag: i64 birth() { me.flag = 0 }
                    read(p: Item): i64 { if p == null { return 7 } return p.value } }"
            };
            let text = format!(
                "box Item {{ value: i64 birth() {{ me.value = 5 }} }}
                 {reader} static box Main {{ main() {{ {main} }} }}"
            );
            for optimize in [false, true] {
                MirCompiler::with_options(optimize).compile_normal_with_published(
                    request(&text), |view, verification| -> Result<(), String> {
                        classify_pretransform_report(verification);
                        let input = view.issue_lifecycle_physical_abi_input()?;
                        let wire = super::super::super::physical_program_json::emit_lifecycle_physical_abi_json(&input)?;
                        let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
                        let target = if shape == "root" { "main" } else { "Transport.read/1" };
                        let reader = json["functions"].as_array().unwrap().iter()
                            .find(|f| f["name"] == target).unwrap();
                        let reads = reader["blocks"].as_array().unwrap().iter()
                            .flat_map(|block| block["instructions"].as_array().unwrap())
                            .filter(|row| row["instruction"]["op"] == "object_field_get")
                            .count();
                        assert_eq!(reads, 1, "{shape}/opt{optimize}: exact source read");
                        std::fs::write(std::env::temp_dir().join(format!(
                            "hako-issued-field-return-projection-{shape}-opt{optimize}.json"
                        )), wire).unwrap();
                        Ok(())
                    },
                ).unwrap_or_else(|error| panic!("{shape}/opt{optimize}: {error}"));
            }
        }
    });
}

/// Branch-arm meaning belongs to the existing final borrowed-use verifier;
/// retain the original source handoff while forging only the physical guard.
#[test]
fn field_return_publication_rejects_swapped_null_guard_with_original_handoff() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let text = "box Item { value: i64 birth() { me.value = 5 } }
            box Transport { flag: i64 birth() { me.flag = 0 }
                read(p: Item): i64 { if p == null { return 7 } return p.value } }
            static box Main { main() { local recv = new Transport() return recv.read(null) } }";
        for optimize in [false, true] {
            MirCompiler::with_options(optimize).compile_normal_with_published(
                request(text), |view, verification| -> Result<(), String> {
                    classify_pretransform_report(verification);
                    let healthy = view.issue_lifecycle_physical_abi_input()?;
                    super::super::super::physical_program_json::emit_lifecycle_physical_abi_json(&healthy)?;
                    let handoff = view.retained_handoff.expect("same source-issued handoff");
                    let profile = view.lifecycle_storage_profile.expect("original finalized profile");
                    let control = PublishedMirBackendView::try_new(view.module())
                        .map_err(|error| error.to_string())?
                        .bind_finalized_root_handoff(Some(handoff))?;
                    let control = super::super::super::super::lifecycle_admission::admit_lifecycle(control, profile)?;
                    control.issue_lifecycle_physical_abi_input()?;
                    let mut changed = view.module().clone();
                    let reader = changed.functions.get_mut("Transport.read/1").unwrap();
                    let (then_bb, else_bb) = reader.blocks.values_mut().find_map(|block| {
                        match block.terminator.as_mut() {
                            Some(MirInstruction::Branch { then_bb, else_bb, .. }) => Some((then_bb, else_bb)),
                            _ => None,
                        }
                    }).expect("original null guard");
                    assert_ne!(then_bb, else_bb);
                    std::mem::swap(then_bb, else_bb);
                    let forged = PublishedMirBackendView::try_new(&changed)
                        .map_err(|error| error.to_string())?
                        .bind_finalized_root_handoff(Some(handoff))?;
                    let forged = super::super::super::super::lifecycle_admission::admit_lifecycle(forged, profile)?;
                    let error = forged.issue_lifecycle_physical_abi_input().unwrap_err();
                    assert!(error.contains("borrowed-use/undominated-view"), "opt{optimize}: {error}");
                    Ok(())
                },
            ).unwrap_or_else(|error| panic!("opt{optimize}: {error}"));
        }
    });
}
