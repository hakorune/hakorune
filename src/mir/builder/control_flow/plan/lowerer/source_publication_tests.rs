//! Original field-index Set through the source-aware Plan port. Full-body
//! guard/finished-consumer validation remains in publication regressions.
use super::*;
use crate::mir::builder::normal_callable_binding_materialization_port::PreparedCallableEntryValuesV1;
use crate::mir::builder::normal_callable_loop_source_port::CallableLoopSourceExpressionPortV1;
use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use crate::mir::builder::raw_invocation_source_transport::{RawInvocationRootLineageV1, RawInvocationSourceContextV1};
use crate::mir::builder::control_flow::plan::normalizer::{PlanNormalizer, loop_body_lowering_associated_input};
use crate::mir::builder::{CompilationContext, SelectedNormalCallableKeyV1};
use crate::mir::normal_callable_semantic_package::{issue_source_package_fixture_v1, STORED_BORROWED_LOCAL_SOURCE};
use crate::mir::resolved_semantics::OwnedExprSiteV1;
use crate::mir::{ArrayElementWriteKind, MirInstruction};
use std::collections::BTreeMap;

#[test]
fn source_publication_set_preserves_original_reads_and_affine_completion() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    // Reuse the existing stored-nullable field-index source, not a new profile.
    let text = STORED_BORROWED_LOCAL_SOURCE.replace("maybe(p: Item)", "maybe(p: Item, n)")
        .replace("if p == null { return null } return p",
            "if p == null { return null } if n <= 0 { return null } me.slots.set(p.value, n) p.value = n return p")
        .replace("me.child.maybe(p)", "me.child.maybe(p, 7)")
        .replace("box Leaf { flag: i64", "box Leaf { slots: ArrayBox flag: i64")
        .replace("me.flag = 0", "me.flag = 0 me.slots = new ArrayBox()");
    for (value_position, refusal) in [(false, None), (true, None), (false, Some("call-shape")),
        (false, Some("destination-already-defined")), (false, Some("receiver-value-mismatch"))] {
        let package = issue_source_package_fixture_v1(&text).expect("existing original source");
        let mut context = CompilationContext::new();
        let installed = package.prepare_install(&mut context).unwrap().commit();
        let ledger = installed.ordinary_new_claim_ledger();
        let key = crate::mir::builder::CanonicalSameModuleCallableKeyV1::instance_box_method("Leaf", "maybe", 2);
        let (state, owned, node) = installed.begin_lowering(&context).unwrap()
            .with_selected_lowering_input(&SelectedNormalCallableKeyV1::Cataloged(key.clone()), |input| {
                let source = input.source();
                let rows = ledger.borrowed_ordinary_entry_source_v1(&input)
                    .and_then(|row| {
                        if let Some(entry) = &row { ledger.stage_borrowed_entry_field_reads_v1(entry, &input)?; }
                        Ok(row.map(|row| row.formals().into()))
                    });
                let mut state = CallableSemanticLoweringState::from_exact_source(source).unwrap();
                state.lend_ordinary_new_claim_ledger(Rc::clone(&ledger));
                state.stage_borrowed_entry_formals(source.owner(), rows).unwrap();
                let (site, _) = source.function().method_calls().find(|(_, call)| call.selector() == "set").unwrap();
                let owned = OwnedExprSiteV1::new(source.owner(), site.clone());
                let node = source.source().expr_at(&owned).unwrap().node().clone();
                (state, owned, node)
            }).unwrap();
        let state = Rc::new(RefCell::new(state));
        let ast = crate::parser::NyashParser::parse_from_string(&text).unwrap();
        crate::mir::builder::program_declaration_facts::PreparedNormalProgramDeclarationFactsV1::collect(&ast)
            .unwrap().install_into(&mut context);
        let mut builder = MirBuilder::new();
        builder.comp_ctx = context;
        let names = vec!["p".to_owned(), "n".to_owned()];
        builder.create_method_skeleton("Leaf.maybe/2".into(), "Leaf", &names, &[]).unwrap();
        builder.setup_method_params("Leaf", &names).unwrap();
        let [receiver, parameter, value] = builder.function_state.current_function.as_ref().unwrap().params.as_slice()
            else { panic!("original instance entry") };
        let (receiver, parameter, value) = (*receiver, *parameter, *value);
        let entry = PreparedCallableEntryValuesV1::instance_method(&builder, 2).unwrap();
        state.borrow_mut().install_entry_values(&entry).unwrap();
        let source = RawInvocationSourceContextV1::Located {
            root: RawInvocationRootLineageV1::Cataloged(key), site: owned.site().node().clone(), body_kind: None,
        };
        let port = CallableLoopSourceExpressionPortV1::new(&state, None);
        let input = port.expr(&node, &source).unwrap();
        let mut plans = if value_position {
            PlanNormalizer::lower_value_input(&port, input, &mut builder, &BTreeMap::new()).unwrap().1
        } else {
            loop_body_lowering_associated_input::lower_method_call_statement_input(
                &port, input, &mut builder, &BTreeMap::new(), "test").unwrap()
        };
        if let Some(refusal) = refusal {
            match refusal {
                "call-shape" => if let CoreEffectPlan::MethodCall { method, .. } = plans.last_mut().unwrap() {
                    *method = "get".into();
                },
                "destination-already-defined" => if let CoreEffectPlan::FieldGet { dst, .. } = &mut plans[0] {
                    *dst = receiver;
                },
                "receiver-value-mismatch" => if let CoreEffectPlan::FieldGet { base, .. } = &mut plans[0] {
                    *base = parameter;
                },
                _ => unreachable!(),
            }
            let selected = if refusal == "call-shape" { plans.last().unwrap() } else { &plans[0] };
            let mut emission = CorePlanEffectEmissionPortV1::source_publication(Rc::clone(&state));
            let error = emission.emit_effect(&mut builder, selected).unwrap_err();
            assert!(error.contains(refusal), "{refusal}: {error}");
            assert!(builder.function_state.current_function.as_ref().unwrap().blocks.values()
                .all(|block| block.all_instructions().next().is_none()));
            continue;
        }
        // The original receiver field must precede the original index field.
        let [CoreEffectPlan::FieldGet { source: Some(receiver_site), dst: receiver_dst, .. },
            CoreEffectPlan::FieldGet { source: Some(index_site), dst: index_dst, .. },
            CoreEffectPlan::MethodCall { object, args, .. }] = plans.as_slice() else {
                panic!("source-selected field reads and Set: {plans:?}")
            };
        let loan = ledger.borrowed_array_set_source_at_v1(owned.owner(), &owned).unwrap().unwrap();
        assert_eq!(receiver_site, loan.receiver());
        assert_eq!(index_site.site(), loan.argument_sites()[0]);
        assert_eq!((*object, args.as_slice()), (*receiver_dst, &[*index_dst, value][..]));
        let mut emission = CorePlanEffectEmissionPortV1::source_publication(Rc::clone(&state));
        for plan in &plans { emission.emit_effect(&mut builder, plan).unwrap(); }
        emission.finish().unwrap();
        let originals = ledger.original_borrowed_array_set_bindings_v1(owned.owner()).unwrap();
        assert!(originals.iter().any(|(_, instruction)| matches!(instruction,
            MirInstruction::ArrayElementWrite { kind: ArrayElementWriteKind::Set, .. })));
        assert!(originals.iter().any(|(_, instruction)| instruction.dst_value() == Some(*receiver_dst)));
        assert!(originals.iter().any(|(_, instruction)| instruction.dst_value() == Some(*index_dst)));
        // Same affine row cannot be replayed into the raw writer.
        let snapshot = |builder: &MirBuilder| builder.function_state.current_function.as_ref().unwrap().blocks.iter()
            .map(|(id, block)| (*id, block.all_instructions().cloned().collect::<Vec<_>>()))
            .collect::<BTreeMap<_, _>>();
        let before = snapshot(&builder);
        let mut emission = CorePlanEffectEmissionPortV1::source_publication(Rc::clone(&state));
        let error = emission.emit_effect(&mut builder, &plans[0]).unwrap_err();
        assert!(error.contains("already-taken"), "{error}");
        assert_eq!(snapshot(&builder), before);
    }
}

#[test]
fn source_loop_completion_requires_final_incoming_and_unique_physical_definitions() {
    use super::super::loop_cond_after_role::LoopCondPhysicalCompletionV1;
    use crate::mir::builder::control_flow::edgecfg::api::Frag;
    use crate::mir::builder::control_flow::plan::{CoreLoopPlan, CorePhiInfo, LoopStepMode};
    use crate::mir::resolved_semantics::{FunctionOwnerIssuerV1, SourcePathV1};
    use crate::mir::{BasicBlock, BasicBlockId as B, ConstValue, EffectMask, FunctionSignature, MirFunction, MirType, ValueId as V};
    let owner = FunctionOwnerIssuerV1::new_for_compilation().unwrap().issue().unwrap();
    let site = SourcePathV1::root_body(0).stmt();
    let plan = CoreLoopPlan {
        preheader_bb: B(0), preheader_is_fresh: false, header_bb: B(1), body_bb: B(2),
        step_bb: B(2), continue_target: B(1), after_bb: B(3), found_bb: B(3),
        body: vec![], cond_loop: V(9), cond_match: V(9), block_effects: vec![],
        phis: vec![
            CorePhiInfo { block: B(1), dst: V(2), inputs: vec![(B(0), V(1)), (B(2), V(3))], tag: "header".into() },
            CorePhiInfo { block: B(3), dst: V(4), inputs: vec![(B(1), V(2))], tag: "after".into() },
        ],
        frag: Frag { entry: B(1), block_params: BTreeMap::new(), exits: BTreeMap::new(), wires: vec![], branches: vec![] },
        final_values: vec![("value".into(), V(4))].into(),
        step_mode: LoopStepMode::InlineInBody, has_explicit_step: false,
    };
    let mut function = MirFunction::new(FunctionSignature {
        name: "loop-completion".into(), params: vec![], return_type: MirType::Integer, effects: EffectMask::CONTROL,
    }, B(0));
    for id in 0..=3 { function.add_block(BasicBlock::new(B(id))); }
    for (dst, value) in [(V(1), ConstValue::Integer(3)), (V(8), ConstValue::Integer(8)), (V(9), ConstValue::Bool(true))] {
        function.blocks.get_mut(&B(0)).unwrap().add_instruction(MirInstruction::Const { dst, value });
    }
    function.blocks.get_mut(&B(0)).unwrap().set_terminator(MirInstruction::Jump { target: B(1), edge_args: None });
    let mut completed = plan.clone();
    // Actual Break incoming is added by the same lowerer AFTER preparation.
    completed.phis[1].inputs.push((B(2), V(3)));
    for phi in &completed.phis {
        function.blocks.get_mut(&phi.block).unwrap().add_instruction(MirInstruction::Phi {
            dst: phi.dst, inputs: phi.inputs.clone(), type_hint: Some(MirType::Integer),
        });
    }
    function.blocks.get_mut(&B(1)).unwrap().set_terminator(MirInstruction::Branch {
        condition: V(9), then_bb: B(2), else_bb: B(3), then_edge_args: None, else_edge_args: None,
    });
    function.blocks.get_mut(&B(2)).unwrap().add_instruction(MirInstruction::Const { dst: V(3), value: ConstValue::Integer(7) });
    function.blocks.get_mut(&B(2)).unwrap().set_terminator(MirInstruction::Branch {
        condition: V(9), then_bb: B(1), else_bb: B(3), then_edge_args: None, else_edge_args: None,
    });
    function.blocks.get_mut(&B(3)).unwrap().set_terminator(MirInstruction::Return { value: Some(V(4)) });
    // The SAME physical fixture checks early definitions independently of the
    // final completion census. No extra accepted syntax or proof app is needed.
    let draft_builder = || {
        let mut draft = function.clone();
        for body in draft.blocks.values_mut() {
            for index in (0..body.instructions.len()).rev() {
                if matches!(body.instructions[index], MirInstruction::Phi { .. }) {
                    body.instructions.remove(index);
                    body.instruction_spans.remove(index);
                }
            }
        }
        let mut builder = crate::mir::builder::MirBuilder::new();
        builder.function_state.current_function = Some(draft);
        builder
    };
    use crate::mir::builder::normal_callable_loop_source_facts::SourceLoopPhiSessionV1;
    for mutation in ["none", "missing-header", "missing-after", "extra-pending", "definition-drift", "wrong-scope"] {
        let mut builder = draft_builder();
        let session = SourceLoopPhiSessionV1::default();
        let headers = if mutation == "extra-pending" { vec![V(2), V(10)] } else { vec![V(2)] };
        let afters = if mutation == "extra-pending" { vec![V(4), V(11)] } else { vec![V(4)] };
        session.prepare(&mut builder, B(1), B(3), headers.into_iter(), afters.into_iter()).unwrap();
        for (block, dst) in [(B(1), V(2)), (B(3), V(4))] {
            let definitions: Vec<_> = builder.function_state.current_function.as_ref().unwrap().blocks.iter()
                .flat_map(|(id, body)| body.all_instructions().filter(move |row| row.dst_value() == Some(dst))
                    .map(move |row| (*id, row))).collect();
            assert!(matches!(definitions.as_slice(), [(id, MirInstruction::Phi { inputs, .. })]
                if *id == block && inputs.is_empty()), "defined before exposure");
        }
        let mut selected = completed.clone();
        match mutation {
            "missing-header" => { selected.phis.remove(0); }
            "missing-after" => { selected.phis.pop(); }
            "wrong-scope" => selected.header_bb = B(20),
            "definition-drift" => {
                let body = builder.function_state.current_function.as_mut().unwrap().blocks.get_mut(&B(1)).unwrap();
                let MirInstruction::Phi { inputs, .. } = &mut body.instructions[0] else { unreachable!() };
                inputs.push((B(0), V(1)));
            }
            _ => {}
        }
        let adoption = session.adopt_and_define(&mut builder, &selected);
        if mutation == "none" {
            adoption.unwrap();
            assert!(session.adopt_and_define(&mut builder, &selected).is_err(), "single adoption");
            session.with_transaction(|txn| super::super::phi_processing::patch_all_phis_with_transaction(
                &mut builder, &selected, Some(txn))).unwrap();
            session.require_patched().unwrap();
            let completed_function = builder.function_state.current_function.as_ref().unwrap();
            let completion = LoopCondPhysicalCompletionV1::new(owner, site.clone(), &plan).unwrap();
            completion.observe_completed_loop(&selected, completed_function).unwrap();
            session.commit(&mut builder).unwrap();
            assert!(session.commit(&mut builder).is_err(), "single transaction completion");
        } else {
            assert!(adoption.is_err(), "{mutation}");
            let error = session.abort_error(&mut builder, "adoption failure".into());
            assert_eq!(session.abort_error(&mut builder, error.clone()), error,
                "outer abort preserves the first primary/cleanup error");
        }
    }
    // A source failure before adoption must roll back BOTH early definitions.
    let mut builder = draft_builder();
    let session = SourceLoopPhiSessionV1::default();
    session.prepare(&mut builder, B(1), B(3), [V(2)].into_iter(), [V(4)].into_iter()).unwrap();
    let error = session.abort_error(&mut builder, "source condition/body failure".into());
    assert!(error.contains("source condition/body failure"));
    assert!(!builder.function_state.current_function.as_ref().unwrap().blocks.values()
        .flat_map(|body| body.all_instructions()).any(|row| matches!(row, MirInstruction::Phi { .. })));
    // Partial preparation is owned too: V8 already has its original Const.
    let mut builder = draft_builder();
    let session = SourceLoopPhiSessionV1::default();
    let error = session.prepare(&mut builder, B(1), B(3), [V(2)].into_iter(), [V(8)].into_iter()).unwrap_err();
    session.abort_error(&mut builder, error);
    assert!(!builder.function_state.current_function.as_ref().unwrap().blocks.values()
        .flat_map(|body| body.all_instructions()).any(|row| matches!(row, MirInstruction::Phi { .. })));
    for mutation in ["none", "old-plan", "wrong-value", "wrong-block", "duplicate-definition",
        "parameter", "missing-input", "extra-predecessor", "duplicate-destination", "extra-phi", "empty-input"] {
        let mut changed = function.clone();
        let mut final_plan = completed.clone();
        match mutation {
            "old-plan" => final_plan = plan.clone(),
            "wrong-value" => if let MirInstruction::Phi { inputs, .. } = &mut changed.blocks.get_mut(&B(3)).unwrap().instructions[0] { inputs[1].1 = V(8); },
            "wrong-block" => final_plan.phis[1].block = B(2),
            "duplicate-definition" => changed.blocks.get_mut(&B(2)).unwrap().add_instruction(MirInstruction::Const { dst: V(4), value: ConstValue::Integer(7) }),
            "parameter" => { changed.params.push(V(4)); changed.signature.params.push(MirType::Integer); },
            "missing-input" => if let MirInstruction::Phi { inputs, .. } = &mut changed.blocks.get_mut(&B(3)).unwrap().instructions[0] { inputs.pop(); },
            "extra-predecessor" => { let mut extra = BasicBlock::new(B(5)); extra.set_terminator(MirInstruction::Jump { target: B(3), edge_args: None }); changed.add_block(extra); },
            "duplicate-destination" => final_plan.phis.push(final_plan.phis[0].clone()),
            "extra-phi" => changed.blocks.get_mut(&B(3)).unwrap().instructions.insert(0, MirInstruction::Phi { dst: V(10), inputs: vec![(B(1), V(2)), (B(2), V(3))], type_hint: None }),
            "empty-input" => final_plan.phis[1].inputs.clear(),
            "none" => {}, _ => unreachable!(),
        }
        let completion = LoopCondPhysicalCompletionV1::new(owner, site.clone(), &plan).unwrap();
        assert!(completion.require_finished().is_err());
        let result = completion.observe_completed_loop(&final_plan, &changed);
        assert_eq!(result.is_ok(), mutation == "none", "{mutation}: {result:?}");
        if mutation == "none" {
            completion.require_finished().unwrap();
            assert!(completion.observe_completed_loop(&final_plan, &changed).is_err(), "one affine completion");
            let mut nested = completed.clone(); nested.header_bb = B(20); nested.after_bb = B(21);
            completion.observe_completed_loop(&nested, &changed).unwrap();
        } else { assert!(completion.require_finished().is_err()); }
    }
}
