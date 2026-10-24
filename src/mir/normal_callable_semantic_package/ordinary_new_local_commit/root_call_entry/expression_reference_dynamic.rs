//! Dynamic carrier reads retain the original initializer or Loop producer.
use super::*;

impl ExpressionReferenceReadV1 {
    pub(in crate::mir) fn validate_dynamic_binding_v2(
        binding: crate::mir::resolved_semantics::BindingRefV1,
        source: &crate::mir::resolved_semantics::home_new_prefix::SourceLoopDynamicProducerV1,
        read: &BindingValueObservationV2,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
    ) -> Result<Vec<Binding>, String> {
        use crate::mir::resolved_semantics::home_new_prefix::SourceLoopDynamicProducerV1 as Source;
        if read.binding_value().0 != binding {
            return Err(freeze("source-loop/dynamic-binding"));
        }
        let mut dependencies = Vec::new();
        match source {
            Source::Initializer(frame) => {
                if frame.binding() != binding || !frame.is_dynamic_value() {
                    return Err(freeze("source-loop/dynamic-entry-source"));
                }
                let (initialized, copies) = read.loan_local_initializer_v1(binding)?;
                context.with_original_initializer_v1(frame, |packet| {
                    if !std::rc::Rc::ptr_eq(packet.frame(), frame)
                        || !(packet.has_dynamic_carrier() || packet.has_dynamic_alias())
                        || packet.value() != initialized || packet.local_value() != read.binding_value().1 {
                        return Err(freeze("expression/dynamic-initializer-source"));
                    }
                    dependencies.push(packet.dynamic_read_binding_v1()?.clone());
                    Ok(())
                })?;
                dependencies.extend_from_slice(copies);
            }
            Source::BindingRebind(frame) => {
                if frame.target() != binding || !frame.is_dynamic_loop_carrier_v1() {
                    return Err(freeze("source-loop/dynamic-rebind-producer"));
                }
                context.with_original_rebind_v1(frame, |packet| {
                    let (assigned, copies) = read.loan_checked_rebind_v1(frame)?;
                    if !std::rc::Rc::ptr_eq(packet.frame(), frame)
                        || assigned != packet.value() || packet.value() != read.binding_value().1 {
                        return Err(freeze("source-loop/dynamic-rebind-producer"));
                    }
                    dependencies.extend_from_slice(copies);
                    Ok(())
                })?;
            }
            Source::Source(point) => {
                if point.binding() != binding
                    || !point.proves_source_value_v1(binding, point.value())
                {
                    return Err(freeze("source-loop/dynamic-entry-source"));
                }
                let rebind = point
                    .expression_site_v1()
                    .map(|site| read.source_dynamic_rebind_v1(binding, site))
                    .transpose()?
                    .flatten();
                if let Some(frame) = rebind {
                    context.with_original_rebind_v1(&frame, |packet| {
                        let (assigned, copies) = read.loan_checked_rebind_v1(&frame)?;
                        if !std::rc::Rc::ptr_eq(packet.frame(), &frame)
                            || assigned != packet.value()
                            || packet.value() != read.binding_value().1
                        {
                            return Err(freeze("source-loop/dynamic-rebind-producer"));
                        }
                        dependencies.extend_from_slice(copies);
                        Ok(())
                    })?;
                } else {
                    let (_, copies) = read.loan_local_initializer_v1(binding)?;
                    dependencies.extend_from_slice(copies);
                }
            }
            Source::Header(_) | Source::After { .. } => {
                let product = context.source_loop_dynamic_point_product_v2(source)?;
                if product.source.headers.get(&binding).is_none_or(|basis|
                    !matches!(basis.entry_producer(), crate::mir::resolved_semantics::home_new_prefix::SourceLoopValueProducerV1::Dynamic(_))) {
                    return Err(freeze("source-loop/dynamic-point-binding"));
                }
                let point = read.loan_dynamic_join_v1(source)?;
                point.relation().verify(function)?;
                dependencies.push(point.relation().identity_phi_binding_v1()?.clone());
            }
        }
        dependencies.extend_from_slice(read.original_copy_dependencies_v1()?);
        Ok(dependencies)
    }
}
pub(super) fn dynamic_initializer_result_producer_v1(
    initializer: &std::rc::Rc<
        crate::mir::resolved_semantics::home_new_prefix::CheckedInitializerExpressionFrameV1,
    >,
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    site: &crate::mir::resolved_semantics::SourceNodeSiteV1,
    read: &ExactLexicalReadV1,
    context: CallValidationContextV1<'_>,
) -> Result<(ValueId, Binding), String> {
    if !initializer.is_dynamic_value() || initializer.source().owner() != owner {
        return Err(freeze("expression/dynamic-initializer-source"));
    }
    let binding = initializer.binding();
    let value = read
        .value_for(owner, site, binding)
        .map_err(|error| error.to_string())?;
    let (initialized, _) = read.loan_local_initializer_v1(binding)?;
    context.with_original_initializer_v1(initializer, |packet| {
        let same_frame = std::rc::Rc::ptr_eq(packet.frame(), initializer);
        if !same_frame
            || !(packet.has_dynamic_carrier() || packet.has_dynamic_alias())
            || packet.local_value() != value
            || packet.value() != initialized
        {
            return Err(freeze("expression/dynamic-initializer-source"));
        }
        Ok((value, packet.dynamic_read_binding_v1()?.clone()))
    })
}
