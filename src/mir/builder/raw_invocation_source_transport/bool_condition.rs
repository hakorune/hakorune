//! Bind the SAME source-scoped recursive descent to whole-condition evidence.
use super::*;
use crate::mir::builder::ordinary_new_admission::selected::{bool_condition as physical, terminal_call};
use crate::mir::normal_callable_semantic_package::{BoolConditionNodeProjectionV1, BoolConditionValueProofV1 as Proof};
use crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1 as Node;
use crate::mir::resolved_semantics::{home_new_prefix::CheckedBoolConditionFrameV1, OwnedExprSiteV1, SourceExprSiteV1};

impl RawInvocationChildPortV1<'_, '_> {
    pub(super) fn lower_whole_bool_condition_v1(&mut self, builder: &mut MirBuilder,
        frame: Rc<CheckedBoolConditionFrameV1>, condition: ASTNode,
    ) -> Result<physical::LoweredBoolConditionV1, String> {
        if self.bool_condition_emission.is_some() || self.field_write_rhs_emission.is_some() || self.callable_owner_v1() != Some(frame.source().owner())
            || self.current_source_site_v1().as_ref() != Some(frame.source().nodes()[0].site().node()) {
            return Err("[freeze:contract][bool-condition/source-scope]".into());
        }
        let ledger = self.ordinary_new_claim_ledger.clone().ok_or("[freeze:contract][bool-condition/no-ledger]")?;
        ledger.validate_bool_condition_source_v1(&frame)?;
        let emission = Rc::new(RefCell::new(physical::BoolConditionEmissionV1::begin(builder, frame)?));
        self.bool_condition_emission = Some(Rc::clone(&emission));
        let value = self.lower_expression(builder, condition);
        self.bool_condition_emission = None;
        let value = value?;
        Rc::try_unwrap(emission).map_err(|_| "[freeze:contract][bool-condition/escaped-emission-scope]".to_owned())?
            .into_inner().finish(value, ledger)
    }

    pub(super) fn observe_bool_binary_v1(&mut self, completed: &crate::mir::builder::ops::CompletedOrdinaryBinaryV1) -> Result<(), String> {
        let Some(emission) = &self.bool_condition_emission else { return Ok(()) };
        let site = self.current_source_site_v1().ok_or("[freeze:contract][bool-condition/binary-site]")?;
        let binding = completed.comparison_original().or_else(|| completed.arithmetic_original())
            .ok_or("[freeze:contract][bool-condition/binary-writer]")?.clone();
        if emission.borrow_mut().binary.insert(SourceExprSiteV1::from_node(site), binding).is_some() {
            return Err("[freeze:contract][bool-condition/binary-emitted-twice]".into());
        }
        Ok(())
    }
    pub(super) fn observe_bool_logical_v1(&mut self, completed: &crate::mir::builder::ops::CompletedLogicalShortCircuitV1) -> Result<(), String> {
        let Some(emission) = &self.bool_condition_emission else { return Ok(()) };
        let site = self.current_source_site_v1().ok_or("[freeze:contract][bool-condition/logical-site]")?;
        if emission.borrow_mut().logical.insert(SourceExprSiteV1::from_node(site), completed.clone()).is_some() {
            return Err("[freeze:contract][bool-condition/logical-emitted-twice]".into());
        }
        Ok(())
    }
    pub(super) fn lower_with_bool_condition_v1(&mut self, builder: &mut MirBuilder, input: ASTNode) -> Result<ValueId, String> {
        let Some(emission) = self.bool_condition_emission.clone() else {
            return self.lower_expression_unobserved_v1(builder, input);
        };
        let site = SourceExprSiteV1::from_node(self.current_source_site_v1().ok_or("[freeze:contract][bool-condition/node-site]")?);
        let frame = Rc::clone(&emission.borrow().frame);
        let source = frame.source();
        let index = source.nodes().iter().position(|node| node.site() == &site)
            .ok_or("[freeze:contract][bool-condition/foreign-source-node]")?;
        if frame.transfers_node(index) { return Err("[freeze:contract][bool-condition/transferred-node-emitted]".into()); }
        let entry = physical::cursor(builder)?;
        let state = self.callable_ledger.clone().ok_or("[freeze:contract][bool-condition/no-state]")?;
        let ledger = self.ordinary_new_claim_ledger.clone().ok_or("[freeze:contract][bool-condition/no-ledger]")?;
        let (value, proof) = match source.nodes()[index].source() {
            Node::Binary(_) if frame.text_comparison_children_v1(index).is_some() => {
                physical::emit_scalar_condition_v1(builder, &mut state.borrow_mut(), &ledger, &frame, index)?
            },
            Node::Reference(_) => {
                use crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionReferenceV1 as Reference;
                let reference = frame.references().get(&index).ok_or("[freeze:contract][bool-condition/reference-kind]")?;
                if matches!(reference, Reference::Scalar(read) if read.original_read().is_some())
                    || matches!(reference, Reference::FiniteEntry { .. } | Reference::StoredMapGet(_)) {
                    use crate::mir::normal_callable_semantic_package::{ExpressionReferenceReadV1, CallValidationContextV1};
                    let mut state = state.borrow_mut();
                    let prepared = state.prepare_exact_lexical_read(source.owner(), site.node(), reference.binding())
                        .map_err(|error| error.to_string())?;
                    let function = builder.function_state.current_function.as_ref()
                        .ok_or("[freeze:contract][bool-condition/no-function]")?;
                    let producer = ExpressionReferenceReadV1::prepare(frame.evaluation(), index,
                        prepared.observation_v1(), CallValidationContextV1::Lowering(&ledger), function)?;
                    let read = state.commit_exact_lexical_reads(vec![prepared]).map_err(|error| error.to_string())?
                        .pop().ok_or("[freeze:contract][bool-condition/reference-commit]")?;
                    let observed = producer.finish(read);
                    let value = observed.source_reference_value_v1(source, index, reference,
                        CallValidationContextV1::Lowering(&ledger), function)?;
                    (value, Proof::OriginalReference(observed))
                } else {
                    let Reference::Scalar(reference) = reference
                        else { return Err("[freeze:contract][bool-condition/reference-kind]".into()); };
                    let binding = reference.binding();
                    let read = state.borrow_mut().take_exact_lexical_read(source.owner(), site.node(), binding)
                        .map_err(|error| error.to_string())?;
                    let value = read.value_for(source.owner(), site.node(), binding).map_err(|error| error.to_string())?;
                    let function = builder.function_state.current_function.as_ref().ok_or("[freeze:contract][bool-condition/no-function]")?;
                    (value, Proof::Reference { read, producer: physical::definition(function, value)? })
                }
            },
            Node::Shape(crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { object, .. }) => {
                let owned = OwnedExprSiteV1::new(source.owner(), site.clone());
                let (base, read) = ledger.take_local_field_read(&owned, |binding| {
                    state.borrow_mut().take_exact_lexical_read(source.owner(), object.node(), binding)
                        .and_then(|read| read.value_for(source.owner(), object.node(), binding)).map_err(|error| error.to_string())
                })?.ok_or("[freeze:contract][bool-condition/field-source]")?;
                let (value, binding) = builder.complete_exact_field_read_v1(base, read.field(), &owned, &ledger,
                    crate::mir::builder::fields::PreparedExactFieldReadClaimV1::Local(read))?;
                (value, Proof::Field(binding))
            },
            Node::MethodCall if frame.evaluation().cores().get(&index).is_some_and(|core| core.field_map().is_some()) => {
                physical::emit_scalar_condition_v1(builder, &mut state.borrow_mut(), &ledger, &frame, index)?
            },
            Node::MethodCall => {
                let observation = frame.calls().iter().find(|call| call.site().site() == &site)
                    .ok_or("[freeze:contract][bool-condition/call-source]")?;
                let mut bindings = Vec::new();
                let consume_receiver = self.lexical_self_receiver_locator_consumer_v1()?;
                let packet = Rc::new(terminal_call::emit_bool_condition_call(builder, &mut state.borrow_mut(), &ledger,
                    &frame, observation, &mut bindings,
                    &consume_receiver)?);
                let value = packet.value_with_ledger(source.owner(), observation.arguments(), &ledger)?;
                (value, Proof::Call { packet, bindings: bindings.into_boxed_slice() })
            },
            Node::Literal(_) => {
                let value = self.lower_expression_unobserved_v1(builder, input)?;
                let function = builder.function_state.current_function.as_ref().ok_or("[freeze:contract][bool-condition/no-function]")?;
                (value, Proof::Literal(physical::definition(function, value)?.ok_or("[freeze:contract][bool-condition/literal-writer]")?))
            },
            Node::Binary(binary) => {
                let value = self.lower_expression_unobserved_v1(builder, input)?;
                if matches!(binary.operator(), crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::And
                    | crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::Or) {
                    let completed = emission.borrow_mut().logical.remove(&site).ok_or("[freeze:contract][bool-condition/logical-observation-missing]")?;
                    let function = builder.function_state.current_function.as_ref().ok_or("[freeze:contract][bool-condition/no-function]")?;
                    let relations = &mut builder.function_state.local_ssa_correspondence;
                    let conditions = completed.conditions();
                    let left = relations.loan(function, completed.pinned_lhs(), conditions.0)
                        .ok_or("[freeze:contract][bool-condition/logical-left-correspondence]")?;
                    let right = relations.loan(function, completed.operands().1, conditions.1)
                        .ok_or("[freeze:contract][bool-condition/logical-right-correspondence]")?;
                    (value, Proof::Logical { completed, conditions: [left, right] })
                } else {
                    let append = emission.borrow_mut().binary.remove(&site).ok_or("[freeze:contract][bool-condition/binary-observation-missing]")?;
                    let (lhs, rhs) = match append.1 {
                        crate::mir::MirInstruction::Compare { lhs, rhs, .. } | crate::mir::MirInstruction::BinOp { lhs, rhs, .. } => (lhs, rhs),
                        _ => return Err("[freeze:contract][bool-condition/binary-append-kind]".into()),
                    };
                    let inputs = (emission.borrow().value_at(binary.lhs())?, emission.borrow().value_at(binary.rhs())?);
                    let function = builder.function_state.current_function.as_ref().ok_or("[freeze:contract][bool-condition/no-function]")?;
                    let relations = &mut builder.function_state.local_ssa_correspondence;
                    let left = relations.loan(function, inputs.0, lhs).ok_or("[freeze:contract][bool-condition/left-correspondence]")?;
                    let right = relations.loan(function, inputs.1, rhs).ok_or("[freeze:contract][bool-condition/right-correspondence]")?;
                    (value, Proof::Binary { append, operands: [left, right] })
                }
            },
            _ => return Err("[freeze:contract][bool-condition/node-consumer-missing]".into()),
        };
        let exit = physical::cursor(builder)?;
        emission.borrow_mut().nodes.push(BoolConditionNodeProjectionV1 { node: index, entry, exit, value, proof });
        Ok(value)
    }

}
