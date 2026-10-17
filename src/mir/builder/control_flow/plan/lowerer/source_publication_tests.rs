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
