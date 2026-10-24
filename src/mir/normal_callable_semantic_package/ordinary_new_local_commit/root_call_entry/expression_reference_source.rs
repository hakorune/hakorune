//! Original source and entry correspondence for recorded expression reads.
use super::*;

pub(super) fn prepare_scalar_binding_producer(
    binding: crate::mir::resolved_semantics::BindingRefV1,
    source: &Producer,
    read: &BindingValueObservationV2,
    context: CallValidationContextV1<'_>,
    function: &MirFunction,
) -> Result<ReferenceProducerV1, String> {
    if read.binding_value().0 != binding {
        return Err(freeze("expression/scalar-binding-observation"));
    }
    let producer = match source {
        Producer::BranchNormal(_) | Producer::LoopHeader(_) | Producer::LoopAfter { .. } => {
            ReferenceProducerV1::Phi(std::rc::Rc::clone(
                read.loan_scalar_join_v1(source)?.relation(),
            ))
        }
        Producer::Initializer(frame) => {
            let value = initializer_value(source, read, context)?;
            if function.params.contains(&value) {
                let formal = initializer_signed_entry_v1(frame).ok_or_else(|| {
                        if std::env::var_os("NYASH_DEBUG_LOCAL_CALLS").is_some() {
                            eprintln!("[local-call-bindings] entry-alias source={:?} nodes={} refs={:?}",
                                frame.source().nodes().first().map(|node| node.source()), frame.source().nodes().len(),
                                frame.evaluation().references().iter().map(|(&node, reference)| (node, reference.binding(),
                                    match reference {
                                        Reference::Scalar(crate::mir::resolved_semantics::home_new_prefix::CheckedScalarReferenceV1::Original(read)) =>
                                            match read.producer() {
                                                Producer::SignedEntry(formal) => format!("SignedEntry({formal:?})"),
                                                Producer::Initializer(_) => "Initializer".to_owned(),
                                                Producer::Integer { value, .. } => format!("Integer({value})"),
                                                Producer::Call(_) => "Call".to_owned(),
                                                Producer::BindingRebind(_) => "Rebind".to_owned(),
                                                Producer::BranchNormal(_) => "BranchNormal".to_owned(),
                                                Producer::LoopHeader(_) => "LoopHeader".to_owned(),
                                                Producer::LoopAfter { .. } => "LoopAfter".to_owned(),
                                            },
                                        Reference::Scalar(crate::mir::resolved_semantics::home_new_prefix::CheckedScalarReferenceV1::Obligation { kind, .. }) =>
                                            format!("Obligation({kind:?})"),
                                        _ => "other".to_owned(),
                                    })).collect::<Vec<_>>());
                        }
                        freeze("expression/scalar-entry-ordinal-value")
                    })?;
                if matches!(
                    frame.evaluation().references().get(&0),
                    Some(Reference::FiniteEntry { .. })
                ) {
                    read.loan_forwarded_copies(formal, value)?;
                } else {
                    signed_entry_layout(binding, formal, context)?;
                    read.loan_forwarded_copies(formal, value)?;
                }
                ReferenceProducerV1::Entry(value)
            } else {
                definition_for_initializer(value, function)?
            }
        }
        Producer::BindingRebind(frame) => context.with_original_rebind_v1(frame, |packet| {
            let value = read.loan_checked_rebind_v1(frame)?.0;
            if packet.value() != value {
                return Err(freeze("expression/rebind-assigned-value"));
            }
            Ok(packet.root_producer().clone())
        })?,
        Producer::SignedEntry(formal) => {
            let (ordinal, _, _, _) = signed_entry_layout(binding, *formal, context)?;
            let value = *function
                .params
                .get(ordinal)
                .ok_or_else(|| freeze("expression/scalar-entry-param"))?;
            ReferenceProducerV1::Entry(value)
        }
        _ => {
            let value = initializer_value(source, read, context)?;
            definition_for_initializer(value, function)?
        }
    };
    validate_scalar_binding_observation_v1(
        binding, source, read, &producer, context, function, None,
    )?;
    Ok(producer)
}
pub(super) fn definition_for_initializer(
    value: ValueId,
    function: &MirFunction,
) -> Result<ReferenceProducerV1, String> {
    let mut rows = function
        .blocks
        .iter()
        .flat_map(|(block, body)| body.all_instructions().map(move |row| (*block, row)))
        .filter(|(_, row)| row.dst_value() == Some(value));
    let (block, row) = rows
        .next()
        .ok_or_else(|| freeze("expression/scalar-producer-missing"))?;
    if rows.next().is_some() || function.params.contains(&value) {
        return Err(freeze("expression/scalar-producer-unique"));
    }
    Ok(ReferenceProducerV1::Definition((block, row.clone())))
}

/// A local declaration may preserve the exact physical parameter ValueId.
/// Admit that entry only when this same initializer frame is a single direct
/// lexical read whose original scalar producer is the matching signed formal.
pub(super) fn initializer_signed_entry_v1(
    frame: &crate::mir::resolved_semantics::home_new_prefix::CheckedInitializerExpressionFrameV1,
) -> Option<crate::mir::resolved_semantics::BindingRefV1> {
    use crate::mir::resolved_semantics::ResolvedLexicalRefV1;
    use crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1;
    let [node] = frame.source().nodes() else {
        return None;
    };
    let ExpressionNodeSourceV1::Reference(ResolvedLexicalRefV1::Local(binding)) = node.source()
    else {
        return None;
    };
    if frame.evaluation().references().len() != 1 {
        return None;
    }
    match frame.evaluation().references().get(&0)? {
        Reference::Scalar(
            crate::mir::resolved_semantics::home_new_prefix::CheckedScalarReferenceV1::Original(
                read,
            ),
        ) if read.binding() == *binding
            && matches!(read.producer(), Producer::SignedEntry(formal) if formal == binding) =>
        {
            Some(*binding)
        }
        Reference::Scalar(
            crate::mir::resolved_semantics::home_new_prefix::CheckedScalarReferenceV1::Obligation {
                binding: source_binding,
                kind: crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Integer,
            },
        ) if *source_binding == *binding => Some(*binding),
        Reference::FiniteEntry { read, .. }
            if read.signed_only()
                && read.requirement().binding() == *binding
                && read.requirement().formal().1 == *binding =>
        {
            Some(*binding)
        }
        _ => None,
    }
}

pub(super) fn validate_initializer_finite_entry_v1(
    binding: crate::mir::resolved_semantics::BindingRefV1,
    frame: &std::rc::Rc<
        crate::mir::resolved_semantics::home_new_prefix::CheckedInitializerExpressionFrameV1,
    >,
    read: &BindingValueObservationV2,
    entry: ValueId,
    context: CallValidationContextV1<'_>,
    function: &MirFunction,
) -> Result<(), String> {
    let Some(Reference::FiniteEntry { read: finite, .. }) = frame.evaluation().references().get(&0)
    else {
        return Err(freeze("expression/scalar-entry-ordinal-value"));
    };
    let formal = initializer_signed_entry_v1(frame)
        .ok_or_else(|| freeze("expression/scalar-entry-ordinal-value"))?;
    if !finite.signed_only()
        || finite.requirement().binding() != formal
        || finite.requirement().formal().1 != formal
    {
        return Err(freeze("expression/scalar-entry-contract"));
    }
    context.with_original_initializer_v1(frame, |packet| {
        let original_read = packet
            .reference_read_v1(0)
            .ok_or_else(|| freeze("initializer-expression/source-or-result"))?;
        let (formal_value, current_value, domain) =
            context.ledger().validate_prepared_finite_entry_read_v1(
                finite.requirement(),
                function,
                original_read.exact_read_v1(),
            )?;
        if formal_value != entry
            || current_value != entry
            || packet.value() != entry
            || !finite.corroborates_finite_domain_v1(&domain)
        {
            return Err(freeze("initializer-expression/source-or-result"));
        }
        read.loan_forwarded_initializer_v1(Some(formal_value))?;
        read.loan_forwarded_copies(formal, formal_value)?;
        Ok(())
    })
}
pub(super) fn original_reference(
    frame: &CheckedExpressionEvaluationV1,
    node: usize,
) -> Result<&Reference, String> {
    let reference = frame
        .references()
        .get(&node)
        .ok_or_else(|| freeze("expression/reference-source-missing"))?;
    if !reference.matches_node(frame.source(), node) {
        return Err(freeze("expression/reference-source-node"));
    }
    Ok(reference)
}
pub(super) fn loop_text_reference_relation_v1(
    frame: &CheckedExpressionEvaluationV1,
    node: usize,
    source: &crate::mir::resolved_semantics::home_new_prefix::LoopTextReadV1,
    read: &ExactLexicalReadV1,
    context: CallValidationContextV1<'_>,
) -> Result<std::rc::Rc<crate::mir::builder::LocalSsaPhysicalRelationV1>, String> {
    if !source.matches_source_node(frame.source(), node) {
        return Err(freeze("source-loop/text-point-binding"));
    }
    let producer = source.producer().clone();
    if let crate::mir::resolved_semantics::home_new_prefix::SourceLoopTextProducerV1::BranchNormal(branch) = &producer {
        context.ledger().require_branch_text_source_v1(branch)?;
        let point = read.binding_observation_v2().loan_text_branch_v1(&producer)?;
        if point.source().binding() != source.binding() || point.relation().values().0 != read.source_value().2 {
            return Err(freeze("source-loop/text-point-binding"));
        }
        return Ok(std::rc::Rc::clone(point.relation()));
    }
    let product = context.source_loop_text_point_product_v2(&producer)?;
    if matches!(
        producer,
        crate::mir::resolved_semantics::home_new_prefix::SourceLoopTextProducerV1::Header(_)
    ) && !product.owns_expression_v2(frame.source())
    {
        return Err(freeze("source-loop/text-point-binding"));
    }
    let point = read.binding_observation_v2().loan_text_join_v1(&producer)?;
    if point.binding() != source.binding() || point.value() != read.source_value().2 {
        return Err(freeze("source-loop/text-point-binding"));
    }
    Ok(std::rc::Rc::clone(point.relation()))
}
pub(super) fn loop_dynamic_reference_relation_v1(
    plan: &std::rc::Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>,
    node: usize,
    source: &crate::mir::resolved_semantics::home_new_prefix::LoopDynamicReadV1,
    read: &ExactLexicalReadV1,
    context: CallValidationContextV1<'_>,
    function: &MirFunction,
) -> Result<std::rc::Rc<crate::mir::builder::LocalSsaPhysicalRelationV1>, String> {
    if !source.matches_source_node(plan, node) {
        return Err(freeze("source-loop/dynamic-point-binding"));
    }
    ExpressionReferenceReadV1::validate_dynamic_binding_v2(
        source.binding(),
        source.producer(),
        read.binding_observation_v2(),
        context,
        function,
    )?;
    let point = read
        .binding_observation_v2()
        .loan_dynamic_join_v1(source.producer())?;
    if point.binding() != source.binding() || point.value() != read.source_value().2 {
        return Err(freeze("source-loop/dynamic-point-binding"));
    }
    Ok(std::rc::Rc::clone(point.relation()))
}
pub(super) fn reference_operand(
    frame: &CheckedExpressionEvaluationV1,
    node: usize,
    read: &ExactLexicalReadV1,
    producer: &ReferenceProducerV1,
    context: CallValidationContextV1<'_>,
) -> Result<Operand, String> {
    let reference = original_reference(frame, node)?;
    let site = &frame.source().nodes()[node];
    read.value_for(
        frame.source().owner(),
        site.site().node(),
        reference.binding(),
    )
    .map_err(|error| error.to_string())?;
    match reference {
        Reference::StoredMapGet(_) => Err(freeze("expression/reference-source-node")),
        Reference::DynamicInvocationResult(initializer) => {
            let (value, binding) = dynamic_initializer_result_producer_v1(
                initializer,
                frame.source().owner(),
                site.site().node(),
                read,
                context,
            )?;
            if !initializer.is_dynamic_value()
                || !matches!(producer, ReferenceProducerV1::Definition(original) if original == &binding)
            {
                return Err(freeze("expression/dynamic-initializer-source"));
            }
            Ok(Operand::Dynamic(value))
        }
        Reference::StaticText(original) => {
            let value = read
                .loan_local_initializer_v1(original.definition_binding())?
                .0;
            if !matches!(producer, ReferenceProducerV1::Definition((_, MirInstruction::Const {
                dst, value: ConstValue::String(text) })) if *dst == value && text == original.utf8())
            {
                return Err(freeze("expression/reference-static-value"));
            }
            Ok(Operand::StaticText {
                utf8: original.utf8().to_owned(),
            })
        }
        Reference::Scalar(source) => Ok(Operand::NormalInteger(validate_source(
            frame.source(),
            source
                .original_read()
                .ok_or_else(|| freeze("expression/reference-scalar-obligation"))?,
            read,
            producer,
            context,
        )?)),
        Reference::Residence(original) => {
            if frame.prior_homes().iter().filter(|home| matches!(home, crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1::Text(acquisition) if acquisition.same_source(original.acquisition()))).count() != 1
                || original.root() != original.acquisition().binding() {
                return Err(freeze("expression/residence-current-home"));
            }
            context.with_text_residence_v1(original.acquisition(), read, |value, original_binding| {
                if !matches!(producer, ReferenceProducerV1::Definition(binding) if binding == original_binding) {
                    return Err(freeze("expression/residence-original-pair"));
                }
                // Borrow the original pair: the alias observation proves its
                // correspondence but creates no new acquisition or cleanup.
                Ok(Operand::OwnedString(value))
            })
        }
        Reference::LoopText(original) => {
            let relation = loop_text_reference_relation_v1(frame, node, original, read, context)?;
            if !matches!(producer, ReferenceProducerV1::Phi(point) if std::rc::Rc::ptr_eq(point, &relation))
            {
                return Err(freeze("source-loop/text-point-binding"));
            }
            Ok(Operand::TextOperand(relation.values().0))
        }
        Reference::LoopDynamic(original) => {
            if !original.matches_source_node(frame.source(), node) {
                return Err(freeze("source-loop/dynamic-point-binding"));
            }
            let ReferenceProducerV1::Phi(relation) = producer else {
                return Err(freeze("source-loop/dynamic-point-binding"));
            };
            let value = read
                .value_for(
                    frame.source().owner(),
                    site.site().node(),
                    original.binding(),
                )
                .map_err(|error| error.to_string())?;
            if relation.values().0 != value {
                return Err(freeze("source-loop/dynamic-point-value"));
            }
            Ok(Operand::Dynamic(value))
        }
        Reference::ArgumentEntry(required) => text_entry_operand(required, read, producer, context),
        Reference::FiniteEntry {
            read: original,
            exclusion,
        } => {
            if original.null_compare_v1().is_some() {
                return Err(freeze(
                    "expression/reference-execution-pending/text-entry-domain",
                ));
            }
            let value = entry_value(original.requirement(), read, producer, context)?;
            let leaf = original.leaf_class(exclusion.as_deref());
            if original.signed_only() { Ok(Operand::NormalInteger(value)) }
            else if leaf == Some(crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1::Value(
                crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::String)) { Ok(Operand::BorrowedText(value)) }
            else if original.tagged_string_integer_only_v1(exclusion.as_deref()) {
                Ok(Operand::Dynamic(value))
            }
            else { Err(freeze("expression/reference-execution-pending/text-entry-domain")) }
        }
        Reference::DynamicEntry(original) => {
            let value = dynamic_entry_value(original.requirement(), read, producer)?;
            Ok(Operand::Dynamic(value))
        }
    }
}
pub(super) fn prepare_entry(
    required: &crate::mir::normal_callable_semantic_package::BorrowedTextEntryReadRequirementV1,
    initializer: Option<&crate::mir::normal_callable_semantic_package::FiniteEntryReadV1>,
    read: &ExactLexicalReadV1,
    context: CallValidationContextV1<'_>,
    function: &MirFunction,
) -> Result<ValueId, String> {
    let (entry, _, domain) = context
        .ledger()
        .validate_prepared_finite_entry_read_v1(required, function, read)?;
    if initializer.map_or(!domain.text_only(), |original| {
        !original.corroborates_finite_domain_v1(&domain)
            || !(original.null_compare_v1().is_some()
                || domain.text_only()
                || domain.signed_only()
                || domain.bool_only()
                || original.tagged_string_integer_only_v1(None))
    }) {
        return Err(freeze(
            "expression/reference-execution-pending/text-entry-domain",
        ));
    }
    let (index, count, receivers, symbol) = context.ledger().original_entry_layout_v1(
        required.binding(),
        required.formal().1,
        |kind| {
            matches!(kind,
            crate::mir::callable_parameter_contract::CallableParameterContractKindV1::OpaqueHandle)
        },
    )?;
    if function.signature.name != symbol
        || function.params.len() != count
        || function.params.get(index) != Some(&entry)
        || function.metadata.declared_param_decls.len() != count
        || function
            .metadata
            .declared_param_decls
            .iter()
            .enumerate()
            .any(|(index, row)| row.implicit_receiver != (receivers == 1 && index == 0))
    {
        return Err(freeze("expression/reference-text-entry-layout"));
    }
    Ok(entry)
}
pub(super) fn prepare_dynamic_entry(
    source: &crate::mir::normal_callable_semantic_package::DynamicEntryReadV1,
    read: &ExactLexicalReadV1,
    context: CallValidationContextV1<'_>,
    function: &MirFunction,
) -> Result<ValueId, String> {
    if !source.corroborates_original_read_v1() {
        return Err(freeze("expression/dynamic-entry-source-drift"));
    }
    let (entry, _) = context.ledger().validate_prepared_dynamic_entry_read_v1(
        source.requirement(),
        function,
        read,
    )?;
    Ok(entry)
}
pub(super) fn text_entry_operand(
    required: &crate::mir::normal_callable_semantic_package::BorrowedTextEntryReadRequirementV1,
    read: &ExactLexicalReadV1,
    producer: &ReferenceProducerV1,
    context: CallValidationContextV1<'_>,
) -> Result<Operand, String> {
    entry_value(required, read, producer, context).map(Operand::BorrowedText)
}
pub(super) fn entry_value(
    required: &crate::mir::normal_callable_semantic_package::BorrowedTextEntryReadRequirementV1,
    read: &ExactLexicalReadV1,
    producer: &ReferenceProducerV1,
    context: CallValidationContextV1<'_>,
) -> Result<ValueId, String> {
    let entries = context
        .ledger()
        .borrowed_ordinary_entry_values_v1(required.binding().owner())?;
    let mut matching = entries
        .iter()
        .filter(|(ordinal, formal, _)| (*ordinal, *formal) == required.formal());
    let entry = matching
        .next()
        .ok_or_else(|| freeze("expression/reference-text-entry-value"))?
        .2;
    if matching.next().is_some()
        || !matches!(producer, ReferenceProducerV1::Entry(value) if *value == entry)
    {
        return Err(freeze("expression/reference-text-entry-producer"));
    }
    read.loan_forwarded_copies(required.formal().1, entry)?;
    read.value_for(
        required.binding().owner(),
        required.read().node(),
        required.binding(),
    )
    .map_err(|error| error.to_string())
}
pub(super) fn dynamic_entry_value(
    required: &crate::mir::normal_callable_semantic_package::BorrowedFormalEntryReadRequirementV1,
    read: &ExactLexicalReadV1,
    producer: &ReferenceProducerV1,
) -> Result<ValueId, String> {
    let ReferenceProducerV1::Entry(entry) = producer else {
        return Err(freeze("expression/reference-dynamic-entry-producer"));
    };
    read.loan_forwarded_copies(required.formal().1, *entry)?;
    let value = read
        .value_for(
            required.binding().owner(),
            required.read().node(),
            required.binding(),
        )
        .map_err(|error| error.to_string())?;
    Ok(value)
}
pub(super) fn signed_entry_layout(
    binding: crate::mir::resolved_semantics::BindingRefV1,
    formal: crate::mir::resolved_semantics::BindingRefV1,
    context: CallValidationContextV1<'_>,
) -> Result<(usize, usize, usize, String), String> {
    context.ledger().original_entry_layout_v1(binding, formal, |kind| matches!(kind,
        crate::mir::callable_parameter_contract::CallableParameterContractKindV1::ExactTrivial(abi) if abi.is_i64()))
}

pub(super) fn initializer_value(
    source: &Producer,
    read: &BindingValueObservationV2,
    context: CallValidationContextV1<'_>,
) -> Result<ValueId, String> {
    let expected = match source {
        Producer::Integer { .. } => None,
        Producer::Call(call) => {
            if context.ledger().lexical_i64_call_source(call.site()) != Some(call) {
                return Err(freeze("expression/scalar-call-source"));
            }
            call.local_binding()
                .ok_or_else(|| freeze("expression/scalar-call-local"))?;
            let value = context.original_scalar_call_result_v1(call)?;
            Some(value)
        }
        Producer::Initializer(frame) => {
            Some(context.with_original_initializer_v1(frame, |packet| Ok(packet.value()))?)
        }
        Producer::SignedEntry(_) => return Err(freeze("expression/scalar-entry-not-initializer")),
        Producer::BranchNormal(_) => return Err(freeze("expression/scalar-producer-kind")),
        Producer::BindingRebind(_) => {
            return Err(freeze("expression/scalar-rebind-not-initializer"))
        }
        Producer::LoopHeader(_) => return Err(freeze("expression/loop-header-physical-pending")),
        Producer::LoopAfter { .. } => return Err(freeze("expression/loop-after-physical-pending")),
    };
    read.loan_forwarded_initializer_v1(expected)
}
pub(super) fn validate_source(
    plan: &std::rc::Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>,
    source: &CurrentScalarReadV1,
    read: &ExactLexicalReadV1,
    producer: &ReferenceProducerV1,
    context: CallValidationContextV1<'_>,
) -> Result<ValueId, String> {
    read.value_for(
        source.binding().owner(),
        source.read().node(),
        source.binding(),
    )
    .map_err(|error| error.to_string())?;
    if plan.owner() != source.binding().owner() || !plan.nodes().iter().any(|node| node.site() == source.read()
        && matches!(node.source(), crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::Reference(
            crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding)) if *binding == source.binding())) {
        return Err(freeze("expression/reference-source-node"));
    }
    if matches!(source.producer(), Producer::LoopHeader(_)) {
        let product = context.source_loop_point_product_v2(source.producer())?;
        if !product.owns_expression_v2(plan) {
            return Err(freeze("expression/loop-header-source-scope"));
        }
    }
    validate_scalar_origin(
        source.binding(),
        source.producer(),
        read.binding_observation_v2(),
        producer,
        context,
    )
}
pub(super) fn validate_scalar_origin(
    binding: crate::mir::resolved_semantics::BindingRefV1,
    source: &Producer,
    read: &BindingValueObservationV2,
    producer: &ReferenceProducerV1,
    context: CallValidationContextV1<'_>,
) -> Result<ValueId, String> {
    let value = read
        .value_for(binding.owner(), binding)
        .map_err(|error| error.to_string())?;
    if let Producer::BranchNormal(basis) = source {
        context.ledger().require_branch_scalar_source_v1(basis)?;
        let point = read.loan_scalar_join_v1(source)?;
        let ReferenceProducerV1::Phi(relation) = producer else {
            return Err(freeze("expression/scalar-producer-kind"));
        };
        if !std::rc::Rc::ptr_eq(point.relation(), relation) || point.branch().is_none() {
            return Err(freeze("expression/scalar-producer-kind"));
        }
        relation.require_completed_v1()?;
        return Ok(value);
    }
    if matches!(source, Producer::LoopHeader(_) | Producer::LoopAfter { .. }) {
        let ReferenceProducerV1::Phi(point) = producer else {
            return Err(freeze("expression/loop-point-producer-kind"));
        };
        if !std::rc::Rc::ptr_eq(read.loan_scalar_join_v1(source)?.relation(), point) {
            return Err(freeze("expression/loop-point-original-purpose"));
        }
        context.source_loop_point_product_v2(source)?;
        if matches!(source, Producer::LoopAfter { .. })
            || matches!(context, CallValidationContextV1::Finalized(_))
        {
            point.require_completed_v1()?;
        }
        return Ok(value);
    }
    if let Producer::BindingRebind(frame) = source {
        return context.with_original_rebind_v1(frame, |packet| {
            let assigned = read.loan_checked_rebind_v1(frame)?.0;
            if assigned != packet.value() || producer != packet.root_producer() {
                return Err(freeze("expression/rebind-original-producer"));
            }
            Ok(value)
        });
    }
    match (source, producer) {
        (Producer::Initializer(frame), ReferenceProducerV1::Entry(entry)) => {
            let formal = initializer_signed_entry_v1(frame)
                .ok_or_else(|| freeze("expression/scalar-entry-ordinal-value"))?;
            if !matches!(
                frame.evaluation().references().get(&0),
                Some(Reference::FiniteEntry { .. })
            ) {
                signed_entry_layout(binding, formal, context)?;
            }
            if initializer_value(source, read, context)? != *entry {
                return Err(freeze("expression/scalar-entry-ordinal-value"));
            }
            read.loan_forwarded_copies(formal, *entry)?;
        }
        (Producer::SignedEntry(formal), ReferenceProducerV1::Entry(entry)) => {
            signed_entry_layout(binding, *formal, context)?;
            read.loan_forwarded_copies(*formal, *entry)?;
        }
        (Producer::SignedEntry(_), _) | (_, ReferenceProducerV1::Entry(_)) => {
            return Err(freeze("expression/scalar-producer-kind"))
        }
        (_, ReferenceProducerV1::Phi(_)) => {
            return Err(freeze("expression/loop-point-source-kind"))
        }
        (original, ReferenceProducerV1::Definition(binding)) => {
            if binding.1.dst_value() != Some(initializer_value(source, read, context)?) {
                return Err(freeze("expression/scalar-producer-value"));
            }
            if let Producer::Integer {
                value: expected, ..
            } = original
            {
                if !matches!(binding.1, MirInstruction::Const { value: ConstValue::Integer(actual), .. } if actual == *expected)
                {
                    return Err(freeze("expression/scalar-literal-producer"));
                }
            }
        }
    }
    Ok(value)
}
pub(super) fn validate_dependencies(
    read: &BindingValueObservationV2,
    producer: &ReferenceProducerV1,
    function: &MirFunction,
) -> Result<(), String> {
    let definition = match producer {
        ReferenceProducerV1::Definition(binding) => Some(binding),
        ReferenceProducerV1::Entry(entry) => {
            validate_dependencies_for_entry(*entry, function)?;
            None
        }
        ReferenceProducerV1::Phi(point) => {
            point.identity_phi_binding_v1()?;
            point.verify(function)?;
            None
        }
    };
    for binding in definition
        .into_iter()
        .chain(read.original_copy_dependencies_v1()?)
    {
        let dst = binding
            .1
            .dst_value()
            .ok_or_else(|| freeze("expression/reference-dependency-dst"))?;
        let mut rows = function
            .blocks
            .iter()
            .flat_map(|(block, body)| body.all_instructions().map(move |row| (*block, row)))
            .filter(|(_, row)| row.dst_value() == Some(dst));
        if function.params.contains(&dst)
            || rows.next() != Some((binding.0, &binding.1))
            || rows.next().is_some()
        {
            return Err(freeze("expression/reference-dependency-identity"));
        }
    }
    Ok(())
}

pub(super) fn validate_entry_layout(
    binding: crate::mir::resolved_semantics::BindingRefV1,
    source: &Producer,
    producer: &ReferenceProducerV1,
    context: CallValidationContextV1<'_>,
    function: &MirFunction,
) -> Result<(), String> {
    let (Producer::SignedEntry(formal), ReferenceProducerV1::Entry(entry)) = (source, producer)
    else {
        return Ok(());
    };
    let (ordinal, count, receivers, symbol) = signed_entry_layout(binding, *formal, context)?;
    let declarations = &function.metadata.declared_param_decls;
    if function.signature.name != symbol || count != function.params.len()
        || function.params.get(ordinal) != Some(entry) || declarations.len() != count
        || declarations.iter().enumerate().any(|(index, row)| row.implicit_receiver != (receivers == 1 && index == 0))
        || receivers == 1 && function.metadata.physical_param_carriers.as_deref().and_then(|rows| rows.first())
            != Some(&crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1::ExistingCallableI64) {
        return Err(freeze("expression/scalar-entry-ordinal-value"));
    }
    // Reuse the complete physical entry and exact Copy checks, independently
    // of optional projection permission for ordinary instruction definitions.
    validate_dependencies_for_entry(*entry, function)
}
pub(super) fn validate_dependencies_for_entry(entry: ValueId, function: &MirFunction) -> Result<(), String> {
    use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1 as Carrier;
    let carriers = function
        .metadata
        .physical_param_carriers
        .as_deref()
        .ok_or_else(|| freeze("expression/scalar-entry-carriers"))?;
    if function.params.len() != function.signature.params.len()
        || carriers.len() != function.params.len()
        || function
            .params
            .iter()
            .filter(|value| **value == entry)
            .count()
            != 1
        || function
            .params
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            != function.params.len()
        || function
            .blocks
            .values()
            .flat_map(|block| block.all_instructions())
            .any(|row| {
                row.dst_value()
                    .is_some_and(|dst| function.params.contains(&dst))
            })
    {
        return Err(freeze("expression/scalar-entry-physical"));
    }
    let index = function
        .params
        .iter()
        .position(|value| *value == entry)
        .unwrap();
    if function
        .metadata
        .declared_param_decls
        .get(index)
        .is_none_or(|row| row.implicit_receiver)
        || carriers[index] != Carrier::ExistingCallableI64
        || function.signature.params[index] != crate::mir::MirType::Integer
    {
        return Err(freeze("expression/scalar-entry-carrier"));
    }
    Ok(())
}
