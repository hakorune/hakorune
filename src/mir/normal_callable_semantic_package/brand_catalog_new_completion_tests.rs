//! Source-issued New/Home progress and finishing coverage, including multiple Homes.
use super::issue_with_brand_catalog;

#[test]
fn ordinary_new_home_prefix_retains_order_and_requires_prior_installation() {
    use super::super::ordinary_new_coseal::OrdinaryNewClaimTakeErrorV1;
    use crate::mir::resolved_semantics::SourceBindingSiteV1;
    use crate::mir::ValueId;
    let package = issue_with_brand_catalog(
        "box Page { birth() { } } static box Main { main() { local first = new Page() local alias = first local second = new Page() local third = new Page() return 0 } }"
    ).unwrap();
    let claim_rows = package.ordinary_new_claim_ledger.pending_claims_for_test();
    let claims: Vec<_> = claim_rows.values().collect();
    assert_eq!(claims.len(), 3);
    let prefixes: Vec<_> = claims
        .iter()
        .map(|claim| claim.home_prefix().unwrap())
        .collect();
    assert!(prefixes[0].prior_homes().is_empty());
    assert_eq!(prefixes[1].prior_homes(), &[prefixes[0].destination()]);
    assert_eq!(
        prefixes[2].prior_homes(),
        &[prefixes[1].destination(), prefixes[0].destination()]
    );
    assert_eq!(prefixes[2].covered_statements().len(), 4);
    let sites: Vec<_> = claims.iter().map(|claim| claim.site().clone()).collect();
    let destinations: Vec<_> = prefixes.iter().map(|prefix| prefix.destination()).collect();
    let declarations: Vec<_> = sites
        .iter()
        .map(|site| {
            let owner = package
                .batch()
                .declarations()
                .find(|row| row.owner() == site.owner())
                .unwrap();
            package
                .batch()
                .with_lowering_input(owner.batch_slot(), |input| {
                    input
                        .function()
                        .expression_source()
                        .initializers()
                        .find(|row| row.initializer_site() == Some(site.site()))
                        .unwrap()
                        .declaration_site()
                        .clone()
                })
                .unwrap()
        })
        .collect();
    drop(claims);
    drop(claim_rows);
    let ledger = package.ordinary_new_claim_ledger;
    let mut physical = crate::mir::MirFunction::new(
        crate::mir::FunctionSignature {
            name: "binding_witness".into(),
            params: vec![],
            return_type: crate::mir::MirType::Void,
            effects: crate::mir::EffectMask::CONTROL,
        },
        crate::mir::BasicBlockId::new(0),
    );
    ledger.register_new_root(sites[0].owner()).unwrap();
    assert!(ledger.register_new_root(sites[0].owner()).is_err());
    assert_eq!(
        ledger.try_take(&sites[1], "Page", 0),
        Err(OrdinaryNewClaimTakeErrorV1::Mismatch)
    );
    for (index, site) in sites.iter().enumerate() {
        let claim = ledger.try_take(site, "Page", 0).unwrap().unwrap();
        assert!(ledger.prepare_new_emission(&claim).unwrap());
        let (prior, reclaim) = ledger.begin_new_emission(site).unwrap();
        let reclaim = reclaim.expect("Birth construction retains reclaim origin");
        assert_eq!(
            prior.iter().map(|operation| match operation {
                crate::mir::instruction::InvokeOperation::HomeRelease { value, .. } => *value,
                _ => panic!("ordinary Home end operation changed"),
            }).collect::<Vec<_>>(),
            (0..index)
                .rev()
                .map(|i| ValueId(i as u32 * 2 + 1))
                .collect::<Vec<_>>()
        );
        let initializer = ValueId(index as u32 * 2);
        let local = ValueId(index as u32 * 2 + 1);
        // This unit tests binding validation, not full Invoke CFG acceptance.
        let block_id = crate::mir::BasicBlockId::new(index as u32);
        let binding = crate::mir::MirInstruction::InvokeNormalResult {
            invoke_block: block_id,
            dst: initializer,
        };
        let frame = crate::mir::MirInstruction::FaultFrameEnter {
            dst: ValueId(100),
            mode: crate::mir::instruction::FaultFrameMode::RootOwned,
        };
        let mut block = crate::mir::BasicBlock::new(block_id);
        if index == 0 {
            block.add_instruction(frame.clone());
        }
        block.add_instruction(binding.clone());
        block.add_instruction(crate::mir::MirInstruction::Copy {
            dst: local,
            src: initializer,
        });
        physical.add_block(block);
        let reclaim_block = crate::mir::BasicBlockId::new(100 + index as u32);
        let reclaim_instruction = crate::mir::MirInstruction::Invoke {
            operation: crate::mir::instruction::InvokeOperation::ReclaimUnpublished {
                object: reclaim.object(),
                value: initializer,
            },
            fault_frame: ValueId(100),
            normal_landing: block_id,
            fault_landing: block_id,
        };
        let mut reclaim_physical = crate::mir::BasicBlock::new(reclaim_block);
        reclaim_physical.add_instruction(reclaim_instruction.clone());
        physical.add_block(reclaim_physical);
        let super::super::OrdinaryNewConstructorDispositionV1::Birth(recipe) = claim.constructor()
        else {
            panic!("source-issued Birth")
        };
        let crate::mir::MirInstruction::Call(call) = crate::mir::MirInstruction::call(
            None,
            crate::mir::Callee::BirthConstructor {
                key: recipe.target_ref().clone(),
                receiver: initializer,
            },
            vec![],
            recipe.physical_effect_mask(),
        ) else {
            unreachable!()
        };
        let birth_id = crate::mir::BasicBlockId(200 + index as u32);
        let birth = crate::mir::MirInstruction::Invoke {
            operation: crate::mir::instruction::InvokeOperation::Call(call),
            fault_frame: ValueId(100),
            normal_landing: block_id,
            fault_landing: reclaim_block,
        };
        let mut birth_block = crate::mir::BasicBlock::new(birth_id);
        birth_block.set_terminator(birth.clone());
        physical.add_block(birth_block);
        ledger
            .record_new_emission(
                site,
                initializer,
                Vec::new(),
                Some((reclaim, reclaim_block, reclaim_instruction.clone())),
                vec![
                    (crate::mir::BasicBlockId::new(0), frame),
                    (block_id, binding),
                    (reclaim_block, reclaim_instruction),
                    (birth_id, birth),
                ],
            )
            .unwrap();
        ledger
            .complete_new_expression(site, "Page", initializer)
            .unwrap();
        let SourceBindingSiteV1::Local { statement, ordinal } = &declarations[index] else {
            panic!("local")
        };
        ledger
            .complete_local_installation(
                site.owner(),
                statement.node(),
                &[(destinations[index], *ordinal, initializer, local)],
            )
            .unwrap();
    }
    assert!(
        !ledger.is_empty(),
        "local installation alone is not physical completion"
    );
    let mut reclaim_drift = physical.clone();
    if let Some(crate::mir::MirInstruction::Invoke {
        operation: crate::mir::instruction::InvokeOperation::ReclaimUnpublished { value, .. },
        ..
    }) = reclaim_drift
        .blocks
        .get_mut(&crate::mir::BasicBlockId::new(100))
        .unwrap()
        .terminator
        .as_mut()
    {
        *value = ValueId(999);
    }
    assert!(ledger
        .validate_new_emissions(sites[0].owner(), &reclaim_drift)
        .unwrap_err()
        .contains("reclaim-origin-operation-drift"));
    ledger
        .complete_new_emissions(sites[0].owner(), &physical)
        .unwrap();
    assert!(
        !ledger.is_empty(),
        "normal-exit obligation remains after New completion"
    );
    let exit_site = ledger
        .root_completion_for_test()
        .explicit_site()
        .unwrap()
        .node()
        .clone();
    assert!(ledger
        .prepare_root_home_exit(sites[0].owner(), &exit_site)
        .unwrap());
    assert!(ledger
        .prepare_root_home_exit(sites[0].owner(), &exit_site)
        .is_err());
    let origins = ledger.begin_root_home_exit().unwrap();
    assert_eq!(
        origins
            .iter()
            .map(|origin| match origin.operation() {
                crate::mir::instruction::InvokeOperation::HomeRelease { value, .. } => *value,
                _ => panic!("ordinary Home end operation changed"),
            })
            .collect::<Vec<_>>(),
        vec![ValueId(5), ValueId(3), ValueId(1)]
    );
    assert!(origins
        .iter()
        .all(|origin| origin.exit().node() == &exit_site));
    let exit_id = crate::mir::BasicBlockId::new(50);
    let result = ValueId(300);
    let literal = ledger
        .prepare_terminal_integer_literal_return(sites[0].owner(), &exit_site)
        .unwrap()
        .expect("source return literal");
    let exit = crate::mir::MirInstruction::Return {
        value: Some(result),
    };
    let mut exit_block = crate::mir::BasicBlock::new(exit_id);
    exit_block.add_instruction(crate::mir::MirInstruction::Const {
        dst: result,
        value: crate::mir::ConstValue::Integer(literal),
    });
    ledger
        .record_terminal_integer_literal_return(result)
        .unwrap();
    exit_block.set_terminator(exit.clone());
    physical.add_block(exit_block);
    // Match the production emitter: N clean nodes plus N-1 pending-Fault nodes.
    let fault_id = crate::mir::BasicBlockId(51);
    let fault_return = crate::mir::MirInstruction::ReturnFault { fault_frame: ValueId(100) };
    let mut fault_block = crate::mir::BasicBlock::new(fault_id);
    fault_block.set_terminator(fault_return.clone());
    physical.add_block(fault_block);
    let mut origin_bindings = Vec::new();
    let mut bindings = vec![(exit_id, exit), (fault_id, fault_return)];
    for (index, origin) in origins.into_iter().enumerate() {
        let block_id = crate::mir::BasicBlockId::new(60 + index as u32);
        let normal = if index == 2 { exit_id } else { crate::mir::BasicBlockId(61 + index as u32) };
        let fault = if index == 2 { fault_id } else { crate::mir::BasicBlockId(71 + index as u32) };
        let instruction = crate::mir::MirInstruction::Invoke {
            operation: origin.operation().clone(),
            fault_frame: ValueId(100),
            normal_landing: normal,
            fault_landing: fault,
        };
        let mut block = crate::mir::BasicBlock::new(block_id);
        block.set_terminator(instruction.clone());
        physical.add_block(block);
        bindings.push((block_id, instruction.clone()));
        if index > 0 {
            let pending_id = crate::mir::BasicBlockId(70 + index as u32);
            let pending = crate::mir::MirInstruction::Invoke {
                operation: origin.operation().clone(),
                fault_frame: ValueId(100),
                normal_landing: fault,
                fault_landing: fault,
            };
            let mut block = crate::mir::BasicBlock::new(pending_id);
            block.set_terminator(pending.clone());
            physical.add_block(block);
            bindings.push((pending_id, pending));
        }
        origin_bindings.push((origin, block_id, instruction));
    }
    let entry_id = crate::mir::BasicBlockId(90);
    let entry_jump = crate::mir::MirInstruction::Jump {
        target: crate::mir::BasicBlockId(60), edge_args: None,
    };
    let mut entry = crate::mir::BasicBlock::new(entry_id);
    entry.instructions = std::mem::take(&mut physical.blocks.get_mut(&exit_id).unwrap().instructions);
    entry.set_terminator(entry_jump.clone());
    physical.add_block(entry);
    physical.blocks.get_mut(&crate::mir::BasicBlockId(2)).unwrap().set_terminator(
        crate::mir::MirInstruction::Jump { target: entry_id, edge_args: None });
    bindings.push((entry_id, entry_jump));
    ledger
        .record_root_home_exit(origin_bindings, bindings)
        .unwrap();
    assert!(ledger.is_empty());
    let mut changed_home = physical.clone();
    if let Some(crate::mir::MirInstruction::Invoke {
        operation: crate::mir::instruction::InvokeOperation::HomeRelease { value, .. },
        ..
    }) = changed_home
        .blocks
        .get_mut(&crate::mir::BasicBlockId::new(60))
        .unwrap()
        .terminator
        .as_mut()
    {
        *value = ValueId(999);
    }
    assert!(ledger
        .validate_finalized_new_root(&changed_home)
        .unwrap_err()
        .contains("root-exit-operation-drift"));
    let mut changed_frame = physical.clone();
    for instruction in &mut changed_frame
        .blocks
        .get_mut(&crate::mir::BasicBlockId::new(0))
        .unwrap()
        .instructions
    {
        if let crate::mir::MirInstruction::FaultFrameEnter { mode, .. } = instruction {
            *mode = crate::mir::instruction::FaultFrameMode::Borrowed;
        }
    }
    assert!(ledger
        .validate_finalized_new_root(&changed_frame)
        .unwrap_err()
        .contains("emission-binding-drift"));
    let mut changed_copy = physical.clone();
    for block in changed_copy.blocks.values_mut() {
        for instruction in &mut block.instructions {
            if let crate::mir::MirInstruction::Copy { src, .. } = instruction {
                *src = ValueId(99);
            }
        }
    }
    assert!(ledger
        .validate_finalized_new_root(&changed_copy)
        .unwrap_err()
        .contains("emission-local-copy-drift"));
    let mut drifted = physical.clone();
    drifted.blocks.clear();
    assert!(ledger.validate_finalized_new_root(&drifted).is_err());
    assert_eq!(
        ledger.validate_finalized_new_root(&physical).unwrap(),
        crate::mir::function::RootOrdinaryNewObservation::SourceCompleteAtFinalization
    );
    assert!(ledger
        .validate_finalized_new_root(&physical)
        .unwrap_err()
        .contains("duplicate-root-validation"));
    physical
        .install_root_ordinary_new_observation(
            crate::mir::function::RootOrdinaryNewObservation::SourceCompleteAtFinalization,
        )
        .unwrap();
    assert!(ledger
        .validate_after_compiler_finishing(&changed_frame)
        .unwrap_err()
        .contains("emission-binding-drift"));
    let mut changed_exit = physical.clone();
    changed_exit.blocks.remove(&exit_id);
    assert!(ledger
        .validate_after_compiler_finishing(&changed_exit)
        .unwrap_err()
        .contains("root-cleanup-graph/residual-node"));
    for (extra_terminal, expected_error) in [
        (crate::mir::MirInstruction::ReturnFault {
            fault_frame: ValueId(100),
        }, "artifact-unowned-lifecycle-site"),
        (physical.blocks[&crate::mir::BasicBlockId(60)]
            .terminator
            .clone()
            .unwrap(), "root-cleanup-graph/internal-incoming"),
    ] {
        let mut extra_lifecycle = physical.clone();
        let mut extra_block = crate::mir::BasicBlock::new(crate::mir::BasicBlockId::new(99));
        extra_block.set_terminator(extra_terminal);
        extra_lifecycle.add_block(extra_block);
        let error = ledger.validate_artifact_after_compiler_finishing(&extra_lifecycle)
            .unwrap_err();
        assert!(error.contains(expected_error), "{error}");
    }
    ledger
        .validate_artifact_after_compiler_finishing(&physical)
        .unwrap();
    assert!(ledger
        .validate_after_compiler_finishing(&physical)
        .unwrap_err()
        .contains("duplicate-finishing-validation"));
}
