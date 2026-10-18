//! Cached-site drift and duplicate rejection must not commit an earlier row.
use super::*;
use crate::mir::normal_callable_semantic_package::LocalFieldReadSourceV1;
use crate::mir::resolved_semantics::{
    BodyExpressionShapeV1, ResolvedLexicalRefV1, SourcePathSegmentV1, SourcePathV1,
};

fn rows() -> Vec<(OwnedExprSiteV1, field_reads::LocalFieldRead)> {
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        "box Pool { first: i64 = 0 second: i64 = 0 birth() { } }
         static box Main { main() { local pool = new Pool()
            local a = pool.first local b = pool.second return a } }",
    ).expect("exact field read fixture");
    let mut rows = Vec::new();
    for declaration in package.batch().declarations() {
        package
            .batch()
            .with_lowering_input(declaration.batch_slot(), |input| {
                for shape in input.body_shape().expect("body shape").expressions() {
                    let BodyExpressionShapeV1::FieldAccess { site, object, .. } = shape else {
                        continue;
                    };
                    let Some(ResolvedLexicalRefV1::Local(receiver)) =
                        input.function().variable_ref(object)
                    else {
                        panic!("receiver");
                    };
                    let owned = OwnedExprSiteV1::new(declaration.owner(), site.clone());
                    let source = package
                        .ordinary_new_claim_ledger
                        .staged_local_field_source(&owned)
                        .expect("staged read");
                    rows.push((
                        owned,
                        field_reads::LocalFieldRead { local: None,
                            receiver_site: object.clone(),
                            receiver,
                            home: receiver,
                            source,
                            numeric_source: None,
                            progress: field_reads::LocalReadProgress::Pending,
                        },
                    ));
                }
            })
            .expect("source loan");
    }
    assert_eq!(rows.len(), 2);
    rows
}

#[test]
fn scalar_expression_batch_cached_drift_rejects_before_new_row_commit() {
    for metadata_drift in [false, true] {
        let mut rows = rows();
        let (site, old) = rows.pop().unwrap();
        let mut drift = field_reads::LocalFieldRead { local: None,
            receiver_site: if metadata_drift { old.receiver_site.clone() } else {
                SourcePathV1::from_node(old.receiver_site.node()).child(SourcePathSegmentV1::Lhs).expr()
            },
            receiver: old.receiver,
            home: old.home,
            source: if metadata_drift {
                LocalFieldReadSourceV1::from_declaration(old.source.field(), old.source.result().clone(), Some("usize".into()))
            } else { Rc::clone(&old.source) },
            numeric_source: None,
            progress: field_reads::LocalReadProgress::Pending,
        };
        let original_site = old.receiver_site.clone();
        let mut staged = BTreeMap::from([(site.clone(), old)]);
        let fresh = rows[0].0.clone();
        rows.push((site.clone(), drift));
        assert!(matches!(
            stage_local_field_read_batch(&mut staged, rows),
            Err(OrdinaryNewCoSealIssueV1::DuplicateSite { .. })
        ));
        assert_eq!(staged.len(), 1);
        assert!(!staged.contains_key(&fresh));
        assert_eq!(staged[&site].receiver_site, original_site);
        let original_source = Rc::clone(&staged[&site].source);
        staged.get_mut(&site).unwrap().progress = field_reads::LocalReadProgress::Taken {
            base: crate::mir::ValueId(42), source: Rc::clone(&original_source),
        };
        // Equal descriptors preserve the old row (including its affine progress).
        drift = field_reads::LocalFieldRead { local: None,
            receiver_site: original_site,
            receiver: staged[&site].receiver,
            home: staged[&site].home,
            source: Rc::new((*staged[&site].source).clone()),
            numeric_source: None,
            progress: field_reads::LocalReadProgress::Pending,
        };
        assert_eq!(
            stage_local_field_read_batch(&mut staged, vec![(site, drift)])
                .unwrap()
                .len(),
            1
        );
        assert_eq!(staged.len(), 1);
        assert!(Rc::ptr_eq(&staged.values().next().unwrap().source, &original_source));
        assert!(matches!(staged.values().next().unwrap().progress,
            field_reads::LocalReadProgress::Taken { base: crate::mir::ValueId(42), .. }));
    }
    // A real whole initializer supplies the SAME source plan and qualified
    // receiver declaration. This is a batch/physical-state test, not a claim
    // that Print/String or whole-app execution is connected.
    let mut package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        "box Counter { n: i64 other: i64 birth() { me.n = 7 me.other = 0 } }
         static box Main { main() { local c = new Counter()
            local value = c.n - c.other return 0 } }",
    ).unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let (qualified, completion, foreign_loan) = package.batch().declarations().find_map(|declaration| {
        package.batch().with_lowering_input(declaration.batch_slot(), |input| {
            let initializer = input.function().expression_source().initializers()
                .find(|row| row.initializer_site().is_some_and(|site|
                    input.function().expression_source().binary(site).is_some()))?;
            let source_ledger = input.forest().callable_source_ledger(input.owner()).unwrap();
            let mut flow = ledger.completion_for_owner(input.owner()).unwrap().cleanup().root_flow().unwrap().clone();
            let original_frame = flow.expression_plans().iter().find(|frame|
                frame.source().root().initializer() == Some(initializer)).unwrap().clone();
            let plan = Rc::clone(original_frame.source());
            let requests: Vec<_> = input.body_shape().unwrap().expressions().iter().filter_map(|shape| {
                let BodyExpressionShapeV1::FieldAccess { site, object, field, .. } = shape else { return None };
                let owned = OwnedExprSiteV1::new(input.owner(), site.clone());
                let rows = ledger.local_field_reads.borrow();
                let row = rows.get(&owned)?;
                Some(crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadRequestV1 {
                    site: owned, receiver_site: object.clone(), receiver: row.receiver,
                    home: row.home, alias_class: None, nullable: false, formal: false, received: None, path: None, alias_path: None, field: field.clone(), owned_ready: None,
                })
            }).collect();
            assert_eq!(requests.len(), 2);
            assert!(original_frame.clone().with_source_field_requirements(&requests[..1]).is_none());
            assert!(original_frame.clone().with_source_field_requirements(&[requests[0].clone(), requests[0].clone()]).is_none());
            let mut foreign_requests = requests.clone();
            foreign_requests[0].receiver_site = SourcePathV1::from_node(foreign_requests[0].receiver_site.node())
                .child(SourcePathSegmentV1::Lhs).expr();
            assert!(original_frame.clone().with_source_field_requirements(&foreign_requests).is_none());
            let refined = original_frame.with_source_field_requirements(&requests).unwrap();
            assert!(flow.refine_expression_plan(refined));
            assert_eq!(flow.source_expression_fields().count(), 2);
            let mut completion = crate::mir::resolved_control_flow::verify_function_completion_v1(input).unwrap();
            completion.attach_root_home_flow(flow);
            let candidates = collect_local_candidates_v1(package.batch(), package.instance_constructors(), input).unwrap();
            let prove = |plan| prove_local_field_read_batch(
                package.instance_constructors(), &candidates, package.batch().ordinary_box_coverage(),
                None, None, None, &requests,
                crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadDemandV1::SourceExpression { plan },
                &ledger.borrowed_i64_results, ledger.borrowed_formal_source.as_ref().unwrap(),
                &mut |_| None, &mut |_| Ok(None), None,
            ).unwrap().unwrap();
            let rows = prove(Rc::clone(&plan));
            let repeated = prove(Rc::clone(&plan));
            let foreign_plan = crate::mir::resolved_value_profile::expression_plan::local_expression_source_plan_v1(
                &source_ledger, input.body_shape(), initializer).unwrap();
            let foreign = prove(foreign_plan);
            let foreign_loan = foreign[0].1.numeric_source.clone().unwrap();
            // Qualification changes and foreign equal-source plan identities
            // reject BEFORE inserting the fresh earlier row.
            for mode in 0..3 {
                let mut incoming = prove(Rc::clone(&plan));
                let (last_site, mut old) = incoming.pop().unwrap();
                if mode == 0 { old.numeric_source = None; }
                if mode == 1 { incoming = prove(Rc::clone(&plan)); incoming.last_mut().unwrap().1.numeric_source = None; }
                else { incoming.push((last_site.clone(), prove(Rc::clone(&plan)).pop().unwrap().1)); }
                if mode == 2 { incoming = prove(crate::mir::resolved_value_profile::expression_plan::local_expression_source_plan_v1(
                    &source_ledger, input.body_shape(), initializer).unwrap()); }
                let fresh = incoming[0].0.clone();
                let mut staged = BTreeMap::from([(last_site.clone(), old)]);
                assert!(stage_local_field_read_batch(&mut staged, incoming).is_err());
                assert_eq!(staged.len(), 1);
                assert!(!staged.contains_key(&fresh));
            }
            let mut staged = BTreeMap::new();
            stage_local_field_read_batch(&mut staged, rows).unwrap();
            stage_local_field_read_batch(&mut staged, repeated).unwrap();
            assert_eq!(staged.len(), 2);
            assert!(stage_local_field_read_batch(&mut staged, foreign).is_err());
            Some((staged, completion, foreign_loan))
        }).unwrap()
    }).expect("whole initializer source");
    let site = qualified.keys().next().unwrap().clone();
    let other = qualified.keys().nth(1).unwrap().clone();
    // The state fixture keeps the genuinely verified whole root and SAME
    // original operation loans; only its strict batch refines field demands.
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let original_completion = ledger.completion_index.insert(site.owner(), Ok(Rc::new(completion))).unwrap();
    *ledger.local_field_reads.borrow_mut() = qualified;
    let original_loan = ledger.local_field_reads.borrow_mut().get_mut(&site).unwrap().numeric_source.take().unwrap();
    assert!(ledger.take_local_field_read(&site, |_| panic!("missing mandatory loan before base lookup"))
        .unwrap_err().contains("required-source-loan-missing"));
    ledger.local_field_reads.borrow_mut().get_mut(&site).unwrap().numeric_source = Some(original_loan.clone());
    let descriptor = Rc::clone(&ledger.local_field_reads.borrow()[&site].source);
    ledger.local_field_reads.borrow_mut().get_mut(&site).unwrap().source =
        LocalFieldReadSourceV1::from_declaration(descriptor.field(), descriptor.result().clone(), Some("usize".into()));
    assert!(ledger.take_local_field_read(&site, |_| panic!("source refused before base lookup"))
        .unwrap_err().contains("receiver-declaration"));
    ledger.local_field_reads.borrow_mut().get_mut(&site).unwrap().source = Rc::clone(&descriptor);
    let (base, descriptor) = ledger.take_local_field_read(&site, |_| Ok(crate::mir::ValueId(10))).unwrap().unwrap();
    let field = descriptor.field();
    let wrong = LocalFieldReadSourceV1::from_declaration(field, descriptor.result().clone(), Some("usize".into()));
    {
        let mut reads = ledger.local_field_reads.borrow_mut();
        let row = reads.get_mut(&site).unwrap();
        row.source = Rc::clone(&wrong);
        let field_reads::LocalReadProgress::Taken { source, .. } = &mut row.progress else { panic!("taken") };
        *source = Rc::clone(&wrong);
    }
    assert!(ledger.record_local_field_read(&site, crate::mir::BasicBlockId(0), crate::mir::ValueId(11), base, field, &wrong)
        .unwrap_err().contains("receiver-declaration"));
    {
        let mut reads = ledger.local_field_reads.borrow_mut();
        let row = reads.get_mut(&site).unwrap();
        row.source = Rc::clone(&descriptor);
        let field_reads::LocalReadProgress::Taken { source, .. } = &mut row.progress else { panic!("taken") };
        *source = Rc::clone(&descriptor);
    }
    ledger.record_local_field_read(&site, crate::mir::BasicBlockId(0), crate::mir::ValueId(11), base, field, &descriptor).unwrap();
    let mut function = crate::mir::MirFunction::new(crate::mir::FunctionSignature {
        name: "qualified_field_state".into(), params: vec![], return_type: crate::mir::MirType::Void,
        effects: crate::mir::EffectMask::CONTROL,
    }, crate::mir::BasicBlockId(0));
    function.blocks.get_mut(&crate::mir::BasicBlockId(0)).unwrap().add_instruction(
        crate::mir::MirInstruction::ObjectFieldGet { dst: crate::mir::ValueId(11), base, field });
    let (other_base, other_source) = ledger.take_local_field_read(&other, |_| Ok(crate::mir::ValueId(10))).unwrap().unwrap();
    ledger.record_local_field_read(&other, crate::mir::BasicBlockId(0), crate::mir::ValueId(12), other_base,
        other_source.field(), &other_source).unwrap();
    function.blocks.get_mut(&crate::mir::BasicBlockId(0)).unwrap().add_instruction(
        crate::mir::MirInstruction::ObjectFieldGet { dst: crate::mir::ValueId(12), base: other_base, field: other_source.field() });
    ledger.validate_field_reads(site.owner(), &function).unwrap();
    // The immutable frame survives deletion of both the row and its physical
    // read. An erased optional loan or equal-source foreign plan also refuses.
    let removed = ledger.local_field_reads.borrow_mut().remove(&other).unwrap();
    let removed_instruction = function.blocks.get_mut(&crate::mir::BasicBlockId(0)).unwrap().instructions.pop().unwrap();
    assert!(ledger.take_local_field_read(&other, |_| panic!("missing required read"))
        .unwrap_err().contains("required-source-read-missing"));
    assert!(ledger.validate_field_reads(site.owner(), &function).unwrap_err().contains("required-source-read-missing"));
    ledger.local_field_reads.borrow_mut().insert(other.clone(), removed);
    function.blocks.get_mut(&crate::mir::BasicBlockId(0)).unwrap().instructions.push(removed_instruction);
    ledger.local_field_reads.borrow_mut().get_mut(&site).unwrap().numeric_source = None;
    assert!(ledger.validate_field_reads(site.owner(), &function).unwrap_err().contains("required-source-loan-missing"));
    ledger.local_field_reads.borrow_mut().get_mut(&site).unwrap().numeric_source = Some(foreign_loan);
    assert!(ledger.validate_field_reads(site.owner(), &function).unwrap_err().contains("required-source-read-drift"));
    ledger.local_field_reads.borrow_mut().get_mut(&site).unwrap().numeric_source = Some(original_loan);
    let refined_completion = ledger.completion_index.insert(site.owner(), original_completion).unwrap();
    assert!(ledger.validate_field_reads(site.owner(), &function).unwrap_err().contains("orphan-source-read"));
    ledger.completion_index.insert(site.owner(), refined_completion);
    ledger.validate_field_reads(site.owner(), &function).unwrap();
    // Consistent row/progress substitution must retain BOTH raw declaration
    // and Scalar result. Keeping "i64" while forging Alias is still refused.
    for replacement in [
        LocalFieldReadSourceV1::from_declaration(field, descriptor.result().clone(), Some("usize".into())),
        LocalFieldReadSourceV1::from_declaration(field,
            crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::Alias("Counter".into()), Some("i64".into())),
    ] {
        {
            let mut reads = ledger.local_field_reads.borrow_mut();
            let row = reads.get_mut(&site).unwrap();
            row.source = Rc::clone(&replacement);
            let field_reads::LocalReadProgress::Emitted { source, .. } = &mut row.progress else { panic!("emitted") };
            *source = replacement;
        }
        assert!(ledger.validate_field_reads(site.owner(), &function).unwrap_err().contains("receiver-declaration"));
    }

}

#[test]
fn scalar_expression_batch_duplicate_rejects_without_commit() {
    let mut rows = rows();
    let (site, row) = rows.pop().unwrap();
    let duplicate = field_reads::LocalFieldRead { local: None,
        receiver_site: row.receiver_site.clone(),
        receiver: row.receiver,
        home: row.home,
        source: Rc::clone(&row.source),
        numeric_source: None,
        progress: field_reads::LocalReadProgress::Pending,
    };
    rows.push((site.clone(), row));
    rows.push((site, duplicate));
    let mut staged = BTreeMap::new();
    assert!(matches!(
        stage_local_field_read_batch(&mut staged, rows),
        Err(OrdinaryNewCoSealIssueV1::DuplicateSite { .. })
    ));
    assert!(staged.is_empty());

    // Whole Print producer, using real source rows/field closure/result loans.
    // The candidate is not a claim that checked native Print already exists.
    for expression in [
        "\"hello\"", "\"\"", "7", "\"sum=\" + (7 + 9)", "\"x\" + \"y\"", "\"n=\" + c.n",
        "(\"n=\" + c.n) + (\" m=\" + c.other)", "\"n=\" + c.read()", "\"bad=\" + c.echo(9)", "\"p=\" + c.run(h)",
        "\"bad=\" + c", "\"bad=\" + true", "\"bad=\" + null", "c.n + \"bad\"", "(\"bad=\" + c.n) + c", "(\"bad=\" + c.read()) + true", "\"bad=\" + c.plain(9)", "\"bad-store=\" + c.other + c.n",
    ] {
        let wrong_store = expression.contains("bad-store");
        let text = if wrong_store {
            format!("box Counter {{ n: i64 other: i64 birth() {{ me.other = 0 me.n = true }} }}
                static box Main {{ main() {{ local c = new Counter() print({expression}) return 0 }} }}")
        } else if expression.contains("run(h)") {
            format!("box Handle {{ block_id: i64 birth() {{ me.block_id = 0 }} }}
                box Leaf {{ probe(p: Handle) {{ if p == null {{ return 0 }} return 7 }} }}
                box Parent {{ child: Leaf birth() {{ me.child = new Leaf() }} run(p: Handle) {{
                    if p == null {{ return 0 }} return me.child.probe(p) }} }}
                static box Main {{ main() {{ local c = new Parent() local h = new Handle()
                    print({expression}) return 0 }} }}")
        } else {
            format!("box Counter {{ n: i64 other: i64 birth() {{ me.n = 7 me.other = 0 }} read() {{ return me.n }} echo(p) {{ return me.read() }} plain(p) {{ return p }} }}
            static box Main {{ main() {{ local c = new Counter() print({expression}) return 0 }} }}")
        };
        let issued = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&text);
        if wrong_store {
            // The production Print walk now asks this strict batch during
            // package issuance. The invalid Birth refuses the entire package,
            // before any caller can obtain a partially staged expression.
            let issue = issued.expect_err("declared i64 cannot authorize a Bool Birth store");
            assert!(format!("{issue:?}").contains("[ordinary-new/integer-field/source-missing]"),
                "{expression}: {issue:?}");
            continue;
        }
        let package = issued.unwrap_or_else(|issue| panic!("{expression}: {issue:?}"));
        let ledger = &package.ordinary_new_claim_ledger;
        let expected = !expression.contains("bad");
        package.batch().declarations().for_each(|declaration| {
            package.batch().with_lowering_input(declaration.batch_slot(), |input| {
                let shape = input.body_shape().unwrap();
                let Some(statement) = shape.statements().iter().find(|row| matches!(row,
                    crate::mir::resolved_semantics::BodyStatementShapeV1::Print { .. })) else { return };
                let source_ledger = input.forest().callable_source_ledger(input.owner()).unwrap();
                let source = crate::mir::resolved_value_profile::expression_plan::statement_expression_source_plan_v1(
                    &source_ledger, shape, statement.site()).unwrap().unwrap();
                let candidates = collect_local_candidates_v1(package.batch(), package.instance_constructors(), input).unwrap();
                let homes: Vec<_> = candidates.iter().map(|row| row.destination).collect();
                assert_eq!(homes.len(), if expression.contains("run(h)") { 2 } else { 1 }, "real pre-Print fresh local Homes");
                let prepared: super::super::super::lexical_instance_call::PreparedLexicalInstanceCallSourceTargetsV1 =
                    Ok(ledger.lexical_instance_calls.borrow().values().map(|slot| {
                        use super::super::super::lexical_instance_call::LexicalInstanceCallDispositionSlotV1 as Slot;
                        match slot {
                            Slot::Ready(row) => Ok(Some(row.source_target().clone())),
                            Slot::SourcePending(source) => Ok(Some(source.clone())),
                            Slot::Taken => panic!("fresh source fixture has no physical consumer"),
                        }
                    }).collect());
                let mut staged = BTreeMap::new();
                let mut batches = 0;
                let observe = |frame_homes: &[crate::mir::resolved_semantics::BindingRefV1], can_install: bool, staged: &mut BTreeMap<_, _>, batches: &mut usize| {
                    crate::mir::resolved_semantics::home_new_prefix::qualify_print_expression_v1(
                        Rc::clone(&source), frame_homes,
                &mut |_| None, &mut |_| None, &mut |_| None,
                        &mut |site| {
                            let Some(BodyExpressionShapeV1::FieldAccess { object, field, .. }) = shape.expression_shape(site) else { return None };
                            let Some(ResolvedLexicalRefV1::Local(receiver)) = input.function().variable_ref(object) else { return None };
                            Some(crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadRequestV1 {
                                site: OwnedExprSiteV1::new(input.owner(), site.clone()), receiver_site: object.clone(),
                                receiver, home: receiver, alias_class: None, nullable: false, formal: false, received: None, path: None, alias_path: None, field: field.clone(), owned_ready: None,
                            })
                        },
                        &mut |site| crate::mir::resolved_semantics::home_new_prefix::issue_source_scalar_expression_call(
                            input, statement.site(), site, &homes, &mut |_| None,
                            &mut |site, request| {
                                assert!(matches!(request, crate::mir::resolved_semantics::home_new_prefix::BorrowedCallActualRequestV1::SourceScalarResultArguments));
                                super::super::super::lexical_instance_call::source_scalar_result_arguments_v1(
                                    &prepared, &package.parameter_contracts,
                                    ledger.borrowed_formal_source.as_ref().unwrap(), &ledger.borrowed_formal_actuals,
                                    &ledger.borrowed_i64_results, site, &ledger.callable_result_classes)
                            },
                        ).map(|call| call.map(crate::mir::resolved_semantics::home_new_prefix::ExpressionCallSourceV1::Ordinary)),
                        &mut |_| can_install,
                        &mut |requests, demand| {
                            *batches += 1;
                            let Some(rows) = prove_local_field_read_batch(package.instance_constructors(), &candidates,
                                package.batch().ordinary_box_coverage(), None, None, None, requests, demand,
                                &ledger.borrowed_i64_results, ledger.borrowed_formal_source.as_ref().unwrap(),
                                &mut |_| None, &mut |_| Ok(None), None).map_err(|issue| format!("{issue:?}"))? else { return Ok(false) };
                            stage_local_field_read_batch(staged, rows).map_err(|issue| format!("{issue:?}"))?;
                            Ok(true)
                        },
                    )
                };
                let result = observe(&homes, true, &mut staged, &mut batches);
                let frame = result.unwrap();
                assert_eq!(frame.is_some(), expected, "{expression}");
                assert_eq!(batches, usize::from(expected), "reject complete root before staging: {expression}");
                if let Some(plan) = frame {
                    assert!(Rc::ptr_eq(plan.source(), &source));
                    let print = plan.print_frame().unwrap();
                    assert!(Rc::ptr_eq(print.source(), &source));
                    assert_eq!(print.prior_homes(), homes.as_slice());
                    let mut duplicate_homes = homes.clone(); duplicate_homes.push(homes[0]);
                    let mut refused = BTreeMap::new(); let mut refused_batches = 0;
                    assert!(observe(&duplicate_homes, true, &mut refused, &mut refused_batches).unwrap().is_none());
                    assert!(refused.is_empty()); assert_eq!(refused_batches, 0);
                    if !print.calls().is_empty() {
                        assert!(observe(&[], true, &mut refused, &mut refused_batches).unwrap().is_none());
                        assert!(refused.is_empty()); assert_eq!(refused_batches, 0);
                    }
                    assert_eq!(print.owned_result_nodes().len(), if expression.contains("7 + 9") { 1 } else { expression.matches('+').count() });
                    if expression.contains(" m=") {
                        let root = source.nodes()[0].site();
                        let path = SourcePathV1::from_node(root.node());
                        let owned: Vec<_> = print.owned_result_nodes().iter().map(|index| source.nodes()[*index].site().clone()).collect();
                        assert_eq!(owned, vec![path.child(SourcePathSegmentV1::Lhs).expr(),
                            path.child(SourcePathSegmentV1::Rhs).expr(), root.clone()], "LHS acquisition, RHS acquisition, outer acquisition");
                    }
                    if expression.contains("7 + 9") { assert_eq!(print.owned_result_nodes(), &[0]); }
                    assert_eq!(plan.source_fields().len(), staged.len());
                    assert_eq!(print.calls().len(), usize::from(expression.contains("read()") || expression.contains("run(h)")));
                    for call in print.calls() {
                        assert!(call.is_expression_value());
                        assert_eq!(call.prior_homes(), homes.as_slice());
                        assert_eq!(call.arguments().len(), call.borrowed_scalar().unwrap().source().argument_sites().len());
                    }
                    for op in print.operations() { assert!(Rc::ptr_eq(op.source(), &source)); }
                    let mut rejected = BTreeMap::new(); let mut rejected_batches = 0;
                    assert!(observe(&homes, false, &mut rejected, &mut rejected_batches).unwrap().is_none());
                    assert!(rejected.is_empty()); assert_eq!(rejected_batches, 0);
                } else { assert!(staged.is_empty()); }
            }).unwrap();
        });
    }
}
