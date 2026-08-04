//! P4-S1: observe the real resolved DirectAccum candidate before publication.
//!
//! This child consumes the compiler's prepared unpublished module and reuses
//! the immutable alpha observer.  It owns no source admission, CFG/SSA/PHI
//! writer, route selection, or publication path.

#![cfg(test)]

use super::physical_digest_test_support::{
    observe_mir, AlphaFinalBindingWitnessV1, AlphaFunctionResultWitnessV1,
    AlphaPhysicalMirDigestV1, MirRoleWitnessV1,
};
use super::semantic_digest_test_support::{semantic_digest, AlphaPhysicalMirDigestV2};
use crate::mir::compiler::{MirCompiler, VerifiedResolvedSourceUnitV1};
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, MirType, ValueId};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy)]
struct Standard5Roles {
    preheader: BasicBlockId,
    header: BasicBlockId,
    body: BasicBlockId,
    step: BasicBlockId,
    after: BasicBlockId,
}

fn source() -> VerifiedResolvedSourceUnitV1 {
    VerifiedResolvedSourceUnitV1::resolve_function(
        crate::mir::compiler::direct_accum_projection::direct_accum_function_for_test(),
    )
    .expect("DirectAccum fixture resolves")
}

fn label(labels: &BTreeMap<ValueId, String>, value: ValueId) -> Result<String, String> {
    labels
        .get(&value)
        .cloned()
        .ok_or_else(|| format!("resolved snapshot has uncredited value {value:?}"))
}

fn standard5_roles(function: &MirFunction) -> Result<Standard5Roles, String> {
    crate::mir::builder::resolved_lowering::canonical_cfg::verify_terminator_edges_for_test(
        function,
    )
    .map_err(|error| format!("canonical MIR edge witness rejected: {error}"))?;
    if function.blocks.len() != 5 {
        return Err(format!(
            "resolved DirectAccum requires exactly five blocks, got {}",
            function.blocks.len()
        ));
    }
    let preheader = function.entry_block;
    let jump_target = |block: BasicBlockId, name: &str| -> Result<BasicBlockId, String> {
        match function
            .get_block(block)
            .ok_or_else(|| format!("missing {name} block {block:?}"))?
            .terminator
            .as_ref()
        {
            Some(MirInstruction::Jump { target, .. }) => Ok(*target),
            other => Err(format!(
                "{name} must have one Jump terminator, got {other:?}"
            )),
        }
    };
    let header = jump_target(preheader, "preheader")?;
    let (body, after) = match function
        .get_block(header)
        .ok_or_else(|| format!("missing header block {header:?}"))?
        .terminator
        .as_ref()
    {
        Some(MirInstruction::Branch {
            then_bb, else_bb, ..
        }) => (*then_bb, *else_bb),
        other => {
            return Err(format!(
                "header must have one Branch terminator, got {other:?}"
            ))
        }
    };
    let step = jump_target(body, "body")?;
    if jump_target(step, "step")? != header {
        return Err("step must jump back to header".to_owned());
    }
    if !matches!(
        function
            .get_block(after)
            .ok_or_else(|| format!("missing after block {after:?}"))?
            .terminator,
        Some(MirInstruction::Return { .. })
    ) {
        return Err("after must have one Return terminator".to_owned());
    }
    let roles = Standard5Roles {
        preheader,
        header,
        body,
        step,
        after,
    };
    let witness = MirRoleWitnessV1::standard5(
        roles.preheader,
        roles.header,
        roles.body,
        roles.step,
        roles.after,
    )?;
    let covered = witness
        .rows
        .iter()
        .map(|(_, block)| *block)
        .collect::<std::collections::BTreeSet<_>>();
    if covered.len() != function.blocks.len()
        || function
            .block_ids()
            .into_iter()
            .any(|block| !covered.contains(&block))
    {
        return Err("resolved DirectAccum role witness does not cover all blocks".to_owned());
    }
    Ok(roles)
}

fn role_rows(roles: Standard5Roles) -> [(&'static str, BasicBlockId); 5] {
    [
        ("P", roles.preheader),
        ("H", roles.header),
        ("B", roles.body),
        ("S", roles.step),
        ("A", roles.after),
    ]
}

fn assign_labels(
    function: &MirFunction,
    roles: Standard5Roles,
) -> Result<BTreeMap<ValueId, String>, String> {
    let roles = role_rows(roles);
    let mut labels = BTreeMap::new();
    let mut preheader_constants = 0;
    for (role, block_id) in roles {
        let block = function
            .get_block(block_id)
            .ok_or_else(|| format!("resolved snapshot missing role {role}"))?;
        let mut phi_index = 0;
        for instruction in &block.instructions {
            match instruction {
                MirInstruction::Phi { dst, .. } => {
                    if role != "H" {
                        return Err(format!(
                            "unexpected resolved DirectAccum PHI in role {role}"
                        ));
                    }
                    let name = match phi_index {
                        0 => "i",
                        1 => "sum",
                        _ => return Err(format!("unexpected {role} PHI index {phi_index}")),
                    };
                    labels.insert(*dst, format!("phi:carrier:{name}"));
                    phi_index += 1;
                }
                MirInstruction::Const { dst, value } => {
                    if role == "P" && preheader_constants < 2 {
                        labels.insert(
                            *dst,
                            if preheader_constants == 0 {
                                "binding:i".to_owned()
                            } else {
                                "binding:sum".to_owned()
                            },
                        );
                        preheader_constants += 1;
                    } else {
                        labels.insert(*dst, format!("const:{value:?}"));
                    }
                }
                MirInstruction::Copy { dst, src } => {
                    labels.insert(*dst, label(&labels, *src)?);
                }
                MirInstruction::BinOp { dst, op, lhs, rhs } => {
                    labels.insert(
                        *dst,
                        format!(
                            "bin:{op:?}:{}:{}",
                            label(&labels, *lhs)?,
                            label(&labels, *rhs)?
                        ),
                    );
                }
                MirInstruction::Compare { dst, op, lhs, rhs } => {
                    labels.insert(
                        *dst,
                        format!(
                            "compare:{op:?}:{}:{}",
                            label(&labels, *lhs)?,
                            label(&labels, *rhs)?
                        ),
                    );
                }
                MirInstruction::KeepAlive { values } => {
                    for value in values {
                        let _ = label(&labels, *value)?;
                    }
                }
                MirInstruction::Branch { .. }
                | MirInstruction::Jump { .. }
                | MirInstruction::Return { .. } => {}
                other => {
                    return Err(format!(
                        "unexpected resolved snapshot instruction: {other:?}"
                    ))
                }
            }
        }
    }
    if preheader_constants != 2 {
        return Err(format!(
            "resolved snapshot preheader constants={preheader_constants}, expected 2"
        ));
    }
    Ok(labels)
}

fn alpha_digest(function: &MirFunction) -> Result<AlphaPhysicalMirDigestV1, String> {
    let roles = standard5_roles(function)?;
    let labels = assign_labels(function, roles)?;
    let header = function
        .get_block(roles.header)
        .ok_or_else(|| "resolved snapshot header block missing".to_owned())?;
    let final_bindings = header
        .instructions
        .iter()
        .filter_map(|instruction| match instruction {
            MirInstruction::Phi { dst, inputs, .. } => Some((*dst, inputs)),
            _ => None,
        })
        .collect::<Vec<_>>();
    if final_bindings.len() != 2 {
        return Err(format!(
            "resolved snapshot carrier PHIs={}, expected 2",
            final_bindings.len()
        ));
    }
    let final_bindings = final_bindings
        .into_iter()
        .enumerate()
        .map(|(index, (value, inputs))| {
            let name = match index {
                0 => "i",
                1 => "sum",
                _ => unreachable!(),
            };
            let provenance_value = inputs
                .first()
                .map(|(_, value)| *value)
                .ok_or_else(|| format!("resolved snapshot final PHI {name} has no input"))?;
            Ok(AlphaFinalBindingWitnessV1 {
                name: name.to_owned(),
                value,
                provenance: label(&labels, provenance_value)?,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    observe_mir(
        function,
        &MirRoleWitnessV1::standard5(
            roles.preheader,
            roles.header,
            roles.body,
            roles.step,
            roles.after,
        )?,
        &labels,
        &final_bindings,
        &AlphaFunctionResultWitnessV1 {
            value: None,
            provenance: "unit".to_owned(),
            expected_type: MirType::Void,
        },
        &function.metadata.value_types,
    )
}

fn semantic_candidate(compiler: &mut MirCompiler) -> Result<AlphaPhysicalMirDigestV2, String> {
    let unit = source();
    let prepared = compiler
        .prepare_direct_accum_candidate_for_snapshot_test(
            unit.lowering_input(),
            Some("direct_accum_snapshot.hako"),
        )
        .map_err(|error| error.to_string())?;
    let function = prepared
        .module_for_snapshot_test()
        .functions
        .get("accum/0")
        .ok_or_else(|| "resolved snapshot function accum/0 missing".to_owned())?;
    let alpha = alpha_digest(function)?;
    semantic_digest(
        &alpha,
        &[
            "final:i:carrier:i:Integer",
            "final:sum:carrier:sum:Integer",
            "result:unit:Void",
        ],
    )
}

#[test]
fn resolved_candidate_semantic_core_matches_legacy_observer() {
    let mut compiler = MirCompiler::with_options(false);
    let candidate = semantic_candidate(&mut compiler).expect("resolved candidate snapshot");
    let legacy = super::physical_parity_tests::direct_legacy_semantic_digest();
    assert_eq!(candidate.semantic, legacy.semantic);
    super::semantic_digest_test_support::DirectAccumLegacyAuxPolicyV1
        .validate(&candidate.legacy_aux)
        .expect("candidate auxiliary rows use the explicit policy");
}

#[test]
fn resolved_candidate_sealed_after_has_no_synthetic_final_phis() {
    let mut compiler = MirCompiler::with_options(false);
    let unit = source();
    let prepared = compiler
        .prepare_direct_accum_candidate_for_snapshot_test(
            unit.lowering_input(),
            Some("direct_accum_final_carrier.hako"),
        )
        .expect("resolved candidate prepared");
    let function = prepared
        .module_for_snapshot_test()
        .functions
        .get("accum/0")
        .expect("resolved snapshot function accum/0");
    let roles = standard5_roles(function).expect("standard5 roles");
    let after = function.get_block(roles.after).expect("after block");
    assert!(after
        .instructions
        .iter()
        .all(|instruction| !matches!(instruction, MirInstruction::Phi { .. })));
    let header = function.get_block(roles.header).expect("header block");
    assert_eq!(
        header
            .instructions
            .iter()
            .filter(|instruction| matches!(instruction, MirInstruction::Phi { .. }))
            .count(),
        2
    );
}

#[test]
fn resolved_candidate_snapshot_is_unpublished_and_fresh_reuse_is_stable() {
    let mut compiler = MirCompiler::with_options(false);
    let before = compiler.builder_test_fingerprint_for_snapshot();
    let baseline = semantic_candidate(&mut compiler).expect("baseline snapshot");
    assert_eq!(compiler.builder_test_fingerprint_for_snapshot(), before);

    let error = compiler
        .compile_direct_accum_candidate_with_prepared_failure_for_test(
            source().lowering_input(),
            Some("direct_accum_snapshot_failure.hako"),
        )
        .expect_err("late prepared failure must abort");
    assert!(error
        .to_string()
        .contains("test_injected_prepared_commit_failure"));
    assert_eq!(compiler.builder_test_fingerprint_for_snapshot(), before);

    let fresh = semantic_candidate(&mut compiler).expect("fresh snapshot");
    assert_eq!(fresh, baseline);
    assert_eq!(compiler.builder_test_fingerprint_for_snapshot(), before);
}
