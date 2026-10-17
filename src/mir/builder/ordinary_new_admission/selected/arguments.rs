//! Physical materialization of one prepared claim's argument rows. This is
//! the sole argument-value owner inside the selected `new` admission: it
//! validates the row/source relation, then emits the physical operand for
//! each kind — never a second issuer of argument meaning.
use super::freeze;
use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use crate::mir::normal_callable_semantic_package::{
    OrdinaryNewClaimLedgerV1, OrdinaryNewTrivialArgumentKindV1, OrdinaryNewTrivialArgumentV1,
};
use crate::mir::{MirBuilder, MirInstruction, MirType, ValueId};

pub(super) fn materialize_arguments(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    site: &crate::mir::resolved_semantics::OwnedExprSiteV1,
    arity: usize,
    rows: &[OrdinaryNewTrivialArgumentV1],
) -> Result<Vec<ValueId>, String> {
    validate_argument_rows_v1(site.owner(), site, arity, rows)?;
    rows.iter()
        .map(|row| match row.kind() {
            OrdinaryNewTrivialArgumentKindV1::Integer(value) => {
                crate::mir::builder::emission::constant::emit_integer(builder, *value)
            }
            OrdinaryNewTrivialArgumentKindV1::Bool(value) => {
                crate::mir::builder::emission::constant::emit_bool(builder, *value)
            }
            OrdinaryNewTrivialArgumentKindV1::Null => {
                crate::mir::builder::emission::constant::emit_null(builder)
            }
            OrdinaryNewTrivialArgumentKindV1::Local { binding }
            | OrdinaryNewTrivialArgumentKindV1::Handle { binding }
            | OrdinaryNewTrivialArgumentKindV1::BoundValue { binding } => {
                let value = state
                    .value_for_exact_binding(site.owner(), *binding)
                    .map_err(|_| freeze("argument-binding-unavailable"))?;
                state
                    .observe_variable_site(row.site().node(), *binding, value)
                    .map_err(|_| freeze("argument-local-observation"))?;
                Ok(value)
            }
            // `QualifiedStaticCall` is a provider-`new`-only row sealed by
            // the Birth construction plan; the selected `local x = new`
            // lane never admits it. A row reaching this owner is issuer
            // drift, not a supported actual.
            OrdinaryNewTrivialArgumentKindV1::QualifiedStaticCall { .. } => {
                return Err(freeze("argument-kind-provider-only"))
            }
            OrdinaryNewTrivialArgumentKindV1::I64Field { object } => {
                // The read site is this row's exact `FieldAccess`
                // expression site; the staged issuer proof under it is
                // taken once and must resolve to the receiver's live value.
                let read_site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                    site.owner(),
                    row.site().clone(),
                );
                let (base, field) = ledger.take_argument_field_read(
                    &read_site,
                    *object,
                    |binding, receiver_site| {
                        let value = state
                            .read_variable(receiver_site)
                            .map_err(|_| freeze("argument-field-observation"))?;
                        let expected = state
                            .value_for_exact_binding(site.owner(), binding)
                            .map_err(|_| freeze("argument-binding-unavailable"))?;
                        if value != expected {
                            return Err(freeze("argument-field-binding-drift"));
                        }
                        Ok(value)
                    },
                )?;
                let block = builder
                    .function_state
                    .current_block
                    .ok_or_else(|| freeze("no-block"))?;
                let dst = builder.next_value_id();
                builder.emit_instruction(MirInstruction::ObjectFieldGet { dst, base, field })?;
                builder
                    .function_state
                    .type_ctx
                    .value_types
                    .insert(dst, MirType::Integer);
                ledger.record_argument_field_read(&read_site, block, dst, base, field)?;
                Ok(dst)
            }
        })
        .collect()
}

fn validate_argument_rows_v1(
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    new_site: &crate::mir::resolved_semantics::OwnedExprSiteV1,
    arity: usize,
    rows: &[OrdinaryNewTrivialArgumentV1],
) -> Result<(), String> {
    if rows.len() != arity {
        return Err(freeze("argument-row-count"));
    }
    let mut sites = std::collections::BTreeSet::new();
    for (index, row) in rows.iter().enumerate() {
        let ordinal = u32::try_from(index).map_err(|_| freeze("argument-ordinal-overflow"))?;
        if row.owner() != owner
            || row.new_site() != new_site
            || row.ordinal() != ordinal
            || !sites.insert(row.site().clone())
        {
            return Err(freeze("argument-row-drift"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::resolved_semantics::{
        FunctionOwnerIssuerV1, SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1,
    };

    fn site(segments: Vec<SourcePathSegmentV1>) -> crate::mir::resolved_semantics::OwnedExprSiteV1 {
        let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().expect("owner issuer");
        crate::mir::resolved_semantics::OwnedExprSiteV1::new(
            issuer.issue().expect("owner"),
            SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(segments)),
        )
    }

    fn integer_row(
        new_site: &crate::mir::resolved_semantics::OwnedExprSiteV1,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        ordinal: u32,
        argument: SourceExprSiteV1,
    ) -> OrdinaryNewTrivialArgumentV1 {
        OrdinaryNewTrivialArgumentV1::new(
            owner,
            new_site.clone(),
            ordinal,
            argument,
            OrdinaryNewTrivialArgumentKindV1::Integer(1),
        )
    }

    #[test]
    fn selected_new_argument_rows_reject_malformed_source_relations() {
        let new_site = site(vec![SourcePathSegmentV1::Body(0)]);
        let argument = SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::Argument(0),
        ]));
        let valid = integer_row(&new_site, new_site.owner(), 0, argument.clone());
        assert!(
            validate_argument_rows_v1(new_site.owner(), &new_site, 1, &[])
                .unwrap_err()
                .contains("argument-row-count")
        );
        assert!(validate_argument_rows_v1(
            new_site.owner(),
            &new_site,
            2,
            &[
                valid.clone(),
                integer_row(&new_site, new_site.owner(), 1, argument)
            ],
        )
        .unwrap_err()
        .contains("argument-row-drift"));
        let foreign_site = site(vec![SourcePathSegmentV1::Body(1)]);
        assert!(validate_argument_rows_v1(
            new_site.owner(),
            &new_site,
            1,
            &[integer_row(
                &new_site,
                foreign_site.owner(),
                0,
                valid.site().clone()
            )],
        )
        .unwrap_err()
        .contains("argument-row-drift"));
        assert!(validate_argument_rows_v1(
            new_site.owner(),
            &new_site,
            1,
            &[integer_row(
                &new_site,
                new_site.owner(),
                1,
                valid.site().clone()
            )],
        )
        .unwrap_err()
        .contains("argument-row-drift"));
    }

    impl MirBuilder {
        /// Exercise the sole writer from a genuinely qualified original frame.
        /// No fake Print purpose or executable ordinary input is issued here.
        pub(in crate::mir) fn assert_checked_expression_core_for_test(
            input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
            ledger: std::rc::Rc<OrdinaryNewClaimLedgerV1>,
            frame: &crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1,
        ) {
            use std::rc::Rc;
            use crate::mir::BasicBlockId;
            use crate::mir::instruction::{InvokeOperation, StringInvokeOperand as Operand, StringInvokeOperation as Operation};
            use crate::mir::normal_callable_semantic_package::{CallValidationContextV1, ExpressionPhysicalProjectionRefV1};
            let mut state = CallableSemanticLoweringState::from_exact_source(input).unwrap();
            let mut builder = MirBuilder::new();
            if frame.text_reads().is_empty() {
                builder.enter_function_for_test("core_argument".into());
            } else {
                let mut names: Vec<_> = input.function().declaration_sites().filter_map(|site| match site {
                    crate::mir::resolved_semantics::SourceBindingSiteV1::Parameter { index } => {
                        let binding = input.function().declaration_binding(site).unwrap();
                        Some((*index, input.function().binding(binding).unwrap().diagnostic_name().to_owned()))
                    }
                    _ => None,
                }).collect();
                names.sort_by_key(|row| row.0);
                let names: Vec<_> = names.into_iter().map(|row| row.1).collect();
                builder.create_function_skeleton("core_argument".into(), &names, &[]).unwrap();
                builder.setup_function_params(&names).unwrap();
                // This unit shell predates the canonical compiler skeleton;
                // install its ordinary physical column before entry projection.
                use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1 as Carrier;
                builder.function_state.current_function.as_mut().unwrap().metadata.physical_param_carriers =
                    Some(vec![Carrier::ExistingCallableI64; names.len()].into_boxed_slice());
            }
            // Reference-free evaluation needs no entry values. In particular
            // this unit cannot manufacture the unfinished caller's Text entry.
            state.lend_ordinary_new_claim_ledger(Rc::clone(&ledger));
            if !frame.text_reads().is_empty() {
                let formals: std::collections::BTreeSet<_> = frame.text_reads().iter().map(|read| read.formal()).collect();
                state.stage_borrowed_entry_formals(input.owner(), Ok(Some(formals.into_iter().collect()))).unwrap();
                let entry = crate::mir::builder::normal_callable_binding_materialization_port::CallableEntryShapeV1::Static {
                    parameter_count: builder.function_state.current_function.as_ref().unwrap().params.len(),
                }.prepare_values(&builder).unwrap();
                let (carriers, projected) = state.prepare_borrowed_entry_carriers(&entry, &builder).unwrap().unwrap();
                assert!(projected.is_empty(), "opaque Text entry carries no declared object view");
                state.install_entry_values(&entry).unwrap();
                builder.function_state.current_function.as_mut().unwrap().metadata.physical_param_carriers = Some(carriers);
            }

            crate::mir::builder::vars::lexical_scope::try_with_lexical_scope(
                &mut builder, |mut builder| -> Result<(), String> {
                    if !frame.static_reads().is_empty() {
                        // This fixture is an original zeroarg source caller. Install its
                        // actual empty header, then use genuine local writer receipts.
                        let entry = crate::mir::builder::normal_callable_binding_materialization_port::CallableEntryShapeV1::Static { parameter_count: 0 }
                            .prepare_values(&builder).unwrap();
                        state.install_entry_values(&entry).unwrap();
                        let mut originals: Vec<_> = input.function().expression_source().initializers().collect();
                        originals.sort_by_key(|row| row.declaration_site().clone());
                        for original in originals {
                            let crate::mir::resolved_semantics::SourceBindingSiteV1::Local { statement, ordinal } = original.declaration_site() else { continue };
                            if statement == frame.source().root().statement() { break; }
                            assert_eq!(*ordinal, 0);
                            let site = original.initializer_site().unwrap();
                            let value = match input.function().expression_source().literal(site) {
                                Some(crate::mir::resolved_semantics::ResolvedLiteralSourceV1::String(text)) =>
                                    crate::mir::builder::emission::constant::emit_string(&mut builder, text.to_string()).unwrap(),
                                _ => state.read_variable(site.node()).unwrap(),
                            };
                            let mut completed = Vec::new();
                            let value = crate::mir::builder::stmts::variable_stmt::build_local_statement_from_values_with_types_and_preclaims_with_receipt_v1(
                                &mut builder, vec![input.function().binding(original.binding()).unwrap().diagnostic_name().to_owned()],
                                vec![value], vec![None], vec![None], &mut completed,
                                &[crate::mir::builder::stmts::variable_stmt::LocalValuePlacement::Copy]).unwrap();
                            state.record_completed_local(statement.node(), &crate::mir::builder::stmts::CompletedLocalStatementV1::from_parts(value, completed)).unwrap();
                        }
                    }
                    if let Some(source) = frame.static_reads().last() {
                        let snapshot = builder.function_state.current_function.as_ref().unwrap().clone();
                        let pending = state.prepare_exact_lexical_read(input.owner(), source.read().node(), source.binding()).unwrap();
                        let (initializer, copies) = pending.loan_local_initializer_v1(source.definition_binding()).unwrap();
                        let first_copy = copies.first().unwrap().clone();
                        let physical_rows = |function: &crate::mir::MirFunction| function.blocks.iter()
                            .map(|(id, block)| (*id, block.all_instructions().cloned().collect::<Vec<_>>()))
                            .collect::<std::collections::BTreeMap<_, _>>();
                        for mutation in 0..3 {
                            let function = builder.function_state.current_function.as_mut().unwrap();
                            match mutation {
                                0 => function.blocks.get_mut(&first_copy.0).unwrap().add_instruction(first_copy.1.clone()),
                                1 => function.params.push(initializer),
                                2 => {
                                    let row = function.blocks.values_mut().flat_map(|block| block.instructions.iter_mut())
                                        .find(|row| row.dst_value() == Some(initializer)).unwrap();
                                    let MirInstruction::Const { value, .. } = row else { unreachable!() };
                                    *value = crate::mir::ConstValue::String("wrong".into());
                                }
                                _ => unreachable!(),
                            }
                            let expected = physical_rows(builder.function_state.current_function.as_ref().unwrap());
                            let rejected = super::super::expression_value::emit(&mut builder, &mut state, &ledger, frame.evaluation(),
                                &mut |_, _, _, _, _| Err("unexpected ordinary Call".to_owned())).err().unwrap();
                            assert!(rejected.contains("static-"), "{rejected}");
                            assert_eq!(physical_rows(builder.function_state.current_function.as_ref().unwrap()), expected);
                            for original in frame.static_reads() {
                                state.prepare_exact_lexical_read(input.owner(), original.read().node(), original.binding()).unwrap();
                            }
                            *builder.function_state.current_function.as_mut().unwrap() = snapshot.clone();
                        }
                    }
                    for requirement in frame.text_reads() {
                        let read = state.prepare_exact_lexical_read(input.owner(), requirement.read().node(), requirement.binding()).unwrap();
                        let function = builder.function_state.current_function.as_ref().unwrap();
                        match ledger.validate_prepared_text_entry_read_v1(requirement, function, read.observation_v1()) {
                            Ok(value) => {
                                assert_eq!(value, read.value_for(input.owner(), requirement.read().node(), requirement.binding()).unwrap());
                                for mutation in 0..4 {
                                    use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1 as Carrier;
                                    let mut changed = function.clone();
                                    let ordinal = requirement.formal().0 as usize;
                                    match mutation {
                                        0 => changed.metadata.physical_param_carriers.as_mut().unwrap()[ordinal] = Carrier::ExistingCallableI64,
                                        1 => changed.params[ordinal] = crate::mir::ValueId(999_998),
                                        2 => changed.blocks.get_mut(&changed.entry_block).unwrap().add_instruction(MirInstruction::Const {
                                            dst: changed.params[ordinal], value: crate::mir::ConstValue::Integer(0),
                                        }),
                                        3 => {
                                            let carriers = changed.metadata.physical_param_carriers.as_mut().unwrap();
                                            let other = carriers.iter_mut().find(|carrier| **carrier != Carrier::BorrowedTaggedValue)
                                                .expect("source fixture has a non-borrowed i64 formal");
                                            *other = Carrier::BorrowedTaggedValue;
                                        }
                                        _ => unreachable!(),
                                    }
                                    assert!(ledger.validate_prepared_text_entry_read_v1(requirement, &changed, read.observation_v1()).is_err());
                                    state.prepare_exact_lexical_read(input.owner(), requirement.read().node(), requirement.binding()).unwrap();
                                }
                            }
                            Err(error) => assert!(error.contains("domain-not-text-only"), "{error}"),
                        }
                    }
                    let before = builder.function_state.current_function.as_ref().unwrap().blocks.iter()
                        .flat_map(|(id, block)| block.all_instructions().map(move |row| (*id, row.clone()))).collect::<Vec<_>>();
                    let result = super::super::expression_value::emit(&mut builder, &mut state, &ledger, frame.evaluation(),
                        &mut |_, _, _, _, _| Err("unexpected ordinary Call".to_owned()));
                    if !frame.residence_reads().is_empty() || !frame.text_reads().is_empty() {
                        let error = result.err().unwrap();
                        assert!(error.contains("reference-execution-pending") || error.contains("domain-not-text-only"), "{error}");
                        for requirement in frame.text_reads() {
                            state.prepare_exact_lexical_read(input.owner(), requirement.read().node(), requirement.binding()).unwrap();
                        }
                        let after = builder.function_state.current_function.as_ref().unwrap().blocks.iter()
                            .flat_map(|(id, block)| block.all_instructions().map(move |row| (*id, row.clone()))).collect::<Vec<_>>();
                        assert_eq!(after, before, "unconnected reference refuses before FaultFrame or MIR effects");
                        return Ok(());
                    }
                    let emitted = result.unwrap();
                    assert!(matches!(emitted.value, Operand::OwnedString(_)));
                    assert_eq!(emitted.acquired.len(), 1, "only substring acquires, scalar Core readers do not");
                    let reads: Vec<_> = emitted.path_reads.into_values().collect();
                    let view = ExpressionPhysicalProjectionRefV1::new(frame.evaluation(), &emitted.values, &emitted.calls, &reads, &emitted.reference_reads, &emitted.bindings);
                    let context = CallValidationContextV1::Lowering(&ledger);
                    view.validate_source(context).unwrap();
                    view.validate_original_reference_dependencies(context, builder.function_state.current_function.as_ref().unwrap()).unwrap();
                    view.assert_reference_finishing_for_test(builder.function_state.current_function.as_ref().unwrap());
                    if !emitted.reference_reads.is_empty() {
                        assert_eq!(emitted.reference_reads.len(), frame.static_reads().len());
                        // Dependency producers precede the expression entry. Mutation
                        // or removal must fail the shared physical check independently.
                        let producer = &emitted.reference_reads.values().next().unwrap().1;
                        for remove in [true, false] {
                            let mut changed = builder.function_state.current_function.as_ref().unwrap().clone();
                            let body = changed.blocks.get_mut(&producer.0).unwrap();
                            if remove { body.instructions.retain(|row| row != &producer.1); }
                            else {
                                let row = body.instructions.iter_mut().find(|row| **row == producer.1).unwrap();
                                let MirInstruction::Const { value, .. } = row else { unreachable!() };
                                *value = crate::mir::ConstValue::String("different".to_owned());
                            }
                            assert!(view.validate_original_reference_dependencies(context, &changed).is_err());
                        }
                    }
                    let ordered = view.ordered_bindings_in_context(context).unwrap();
                    let core_count = frame.evaluation().cores().len();
                    assert_eq!(ordered.iter().filter(|(_, row)| matches!(row, MirInstruction::Invoke { operation: InvokeOperation::String(_), .. })).count(), core_count);
                    assert_eq!(ordered.iter().filter(|(_, row)| matches!(row, MirInstruction::InvokeNormalResult { .. })).count(), core_count);
                    for mutation in 0..3 {
                        let mut changed = emitted.bindings.clone();
                        let index = changed.iter().position(|(_, row)| matches!(row, MirInstruction::Invoke { operation: InvokeOperation::String(Operation::Substring { .. }), .. })).unwrap();
                        match mutation {
                            0 => { changed.remove(index); }
                            1 => {
                                let MirInstruction::Invoke { operation: InvokeOperation::String(operation), .. } = &mut changed[index].1 else { unreachable!() };
                                let Operation::Substring { value, .. } = operation else { unreachable!() };
                                *operation = Operation::Length { value: value.clone() };
                            }
                            2 => {
                                let projection = changed.iter_mut().find(|(_, row)| matches!(row, MirInstruction::InvokeNormalResult { dst, .. } if Some(*dst) == emitted.value.value())).unwrap();
                                let MirInstruction::InvokeNormalResult { invoke_block, .. } = &mut projection.1 else { unreachable!() };
                                *invoke_block = BasicBlockId(999_999);
                            }
                            _ => unreachable!(),
                        }
                        let changed = ExpressionPhysicalProjectionRefV1::new(frame.evaluation(), &emitted.values, &emitted.calls, &reads, &emitted.reference_reads, &changed);
                        assert!(changed.validate_source(context).is_err(), "Core producer/projection mutation {mutation}");
                    }
                    Ok(())
                },
            ).unwrap();
        }
    }
}
