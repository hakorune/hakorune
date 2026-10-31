//! One prepared transaction publishes the original prefix and installed initializer Home.
//! No source reclassification, second packet pool or fallible work after insertion.
use super::*;

pub(super) fn commit(prepared: PreparedLifecyclePrefixBindingGroupV1<'_>) -> Result<(), String> {
    let ledger = prepared.ledger;
    let expected = prepared.destination.expected(ledger, prepared.owner)?;
    if expected
        .get(prepared.preceding_sites.len())
        .is_none_or(|source| !prepared.group.matches_source(source))
    {
        return Err(freeze("local-call-binding-preflight-stale"));
    }
    let home = prepared.group.text_residence_v1();
    let dynamic_carrier = prepared.group.dynamic_carrier_residence_v1();
    let shared = prepared.group.shared_initializer_v1();
    let mut homes = if home.is_some() || dynamic_carrier.is_some() || shared.is_some() {
        Some(ledger.local_commits.borrow_mut())
    } else {
        None
    };
    if let (Some(packet), Some(rows)) = (&home, &homes) {
        if !packet.has_text_home()
            || packet.owner() != prepared.owner
            || rows.contains_key(prepared.group.site())
            || rows.values().any(|row| {
                row.binding() == Some(packet.binding())
                    && !matches!(row, LocalCommitV1::TextResidence(_))
            })
        {
            return Err(freeze("initializer-expression/home-publication-conflict"));
        }
    }
    if let (Some(packet), Some(rows)) = (&dynamic_carrier, &homes) {
        if !packet.has_dynamic_carrier()
            || packet.owner() != prepared.owner
            || rows.contains_key(prepared.group.site())
            || rows.values().any(|row| row.binding() == Some(packet.binding()))
        {
            return Err(freeze("initializer-expression/dynamic-home-publication-conflict"));
        }
    }
    if let (Some(packet), Some(rows)) = (&shared, &homes) {
        if packet.binding.owner() != prepared.owner || rows.contains_key(prepared.group.site())
            || rows.values().any(|row| row.binding() == Some(packet.binding)) {
            return Err(freeze("shared-initializer/home-publication-conflict"));
        }
    }
    let mut rows = ledger.root_local_call_bindings.borrow_mut();
    let mut children = match &prepared.destination {
        source_loop_projection::LifecyclePrefixDestinationV2::Root => None,
        source_loop_projection::LifecyclePrefixDestinationV2::Loop(packet) => {
            // The destination Loop is open exactly once in this owner's tree,
            // as a root group or as a child Loop group of its parent.
            fn occurrences(groups: &[RootLifecyclePrefixBindingGroupV1],
                packet: &std::rc::Rc<source_loop_projection::EmittedSourceLoopProjectionV2>) -> usize {
                groups.iter().filter_map(|group| group.source_loop_rc())
                    .map(|original| usize::from(std::rc::Rc::ptr_eq(original, packet)) + occurrences(&original.snapshot(), packet))
                    .sum()
            }
            if rows.get(&prepared.owner).map_or(0, |groups| occurrences(groups, packet)) != 1
            {
                return Err(freeze("source-loop/publication-parent-stale"));
            }
            Some(packet.children.borrow_mut())
        }
    };
    let groups = match &children {
        Some(groups) => groups.as_slice(),
        None => rows.get(&prepared.owner).map(Vec::as_slice).unwrap_or(&[]),
    };
    let repeated_site = groups
        .iter()
        .any(|recorded| recorded.site() == prepared.group.site());
    if groups.len() != prepared.preceding_sites.len()
        || groups
            .iter()
            .map(|recorded| recorded.site())
            .ne(prepared.preceding_sites.iter())
        || (repeated_site
            && !prepared.destination.allows_same_site_neighbor_v2(
                &expected,
                groups,
                &prepared.group,
            ))
    {
        return Err(freeze("local-call-binding-preflight-stale"));
    }
    // All checks precede both mutations, with both destination maps borrowed.
    if let (Some(packet), Some(homes)) = (home, &mut homes) {
        homes.insert(
            prepared.group.site().clone(),
            LocalCommitV1::TextResidence(packet),
        );
    }
    if let (Some(packet), Some(homes)) = (dynamic_carrier, &mut homes) {
        homes.insert(
            prepared.group.site().clone(),
            LocalCommitV1::DynamicCarrierResidence(packet),
        );
    }
    if let (Some(packet), Some(homes)) = (shared, &mut homes) {
        homes.insert(prepared.group.site().clone(), LocalCommitV1::Shared(packet));
    }
    match &mut children {
        Some(groups) => groups.push(prepared.group),
        None => rows.entry(prepared.owner).or_default().push(prepared.group),
    }
    Ok(())
}

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn text_rebind_displaced_end_v1(
        &self,
        frame: &crate::mir::resolved_semantics::home_new_prefix::CheckedBindingRebindFrameV1,
    ) -> Result<Option<InvokeOperation>, String> {
        let Some(previous) = frame.previous_text() else {
            return Ok(None);
        };
        let operations = self.prior_home_unwind_for(&[
            crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1::Text(previous.clone()),
        ])?;
        if operations.len() != 1
            || !matches!(
                operations[0],
                InvokeOperation::String(crate::mir::instruction::StringInvokeOperation::End { .. })
            )
        {
            return Err(freeze("rebind-expression/old-text-end"));
        }
        Ok(operations.into_iter().next())
    }
    pub(in crate::mir::normal_callable_semantic_package) fn validate_text_residence_home_v1(
        &self,
        packet: &TextResidencePacketV1,
        context: CallValidationContextV1<'_>,
    ) -> Result<(), String> {
        context.require_ledger(self)?;
        let site = packet.site();
        let rows = self.local_commits.borrow();
        if !packet.has_text_home()
            || !matches!(rows.get(&site), Some(LocalCommitV1::TextResidence(original))
            if original.same_packet(packet))
        {
            return Err(freeze("text-residence/published-home-identity"));
        }
        context.with_local_call_group_tree_v2(site.owner(), |groups| {
            if groups
                .iter()
                .filter_map(|group| group.text_residence_v1())
                .filter(|original| original.same_packet(packet))
                .count()
                != 1
            {
                return Err(freeze("text-residence/published-prefix-identity"));
            }
            Ok(())
        })?;
        Ok(())
    }

    pub(in crate::mir::normal_callable_semantic_package) fn validate_dynamic_carrier_residence_home_v1(
        &self,
        packet: &DynamicCarrierResidencePacketV1,
        context: CallValidationContextV1<'_>,
    ) -> Result<(), String> {
        context.require_ledger(self)?;
        if !packet.has_dynamic_carrier() {
            return Err(freeze("dynamic-carrier-residence/source-unavailable"));
        }
        let rows = self.local_commits.borrow();
        if !matches!(rows.get(&packet.site()), Some(LocalCommitV1::DynamicCarrierResidence(original))
            if original.same_packet(packet))
        {
            return Err(freeze("dynamic-carrier-residence/published-home-identity"));
        }
        context.with_local_call_group_tree_v2(packet.owner(), |groups| {
            if groups
                .iter()
                .filter_map(|group| group.dynamic_carrier_residence_v1())
                .filter(|original| original.same_packet(packet))
                .count()
                != 1
            {
                return Err(freeze("dynamic-carrier-residence/published-prefix-identity"));
            }
            Ok(())
        })?;
        Ok(())
    }
}

impl OrdinaryNewClaimLedgerV1 {
    /// The retained source transition, not the descriptor tag, grants the old
    /// residency's End. Ordinary Homes keep their existing unwind authority.
    pub(in crate::mir) fn prepare_loop_text_rebind_unwind_v2(
        &self,
        frame: &Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedBindingRebindFrameV1>,
        product: &Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>,
        read: &crate::mir::builder::BindingValueObservationV2,
        function: &MirFunction,
    ) -> Result<(InvokeOperation, Vec<InvokeOperation>), String> {
        self.loop_text_rebind_unwind_v2(frame, product, read,
            crate::mir::normal_callable_semantic_package::CallValidationContextV1::Lowering(self), function, None)
    }

    pub(super) fn loop_text_rebind_unwind_v2(
        &self,
        frame: &Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedBindingRebindFrameV1>,
        product: &Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>,
        read: &crate::mir::builder::BindingValueObservationV2,
        context: crate::mir::normal_callable_semantic_package::CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
    ) -> Result<(InvokeOperation, Vec<InvokeOperation>), String> {
        use crate::mir::resolved_semantics::home_new_prefix::{SourceLoopTextProducerV1 as Producer,
            SourceLoopValueProducerV1 as ValueProducer};
        use crate::mir::instruction::StringInvokeOperation as Op;
        let basis = frame.text_loop_carrier_v1().ok_or_else(|| freeze("rebind-expression/old-text-end"))?;
        let retained = self.retained_source_loop_recipe_v2(frame.source().owner(), basis.source().site())?
            .ok_or_else(|| freeze("rebind-expression/old-text-end"))?;
        if !Rc::ptr_eq(&retained, product) {
            return Err(freeze("rebind-expression/old-text-end"));
        }
        let before = product.text_rebind_before_v2(frame)?
            .ok_or_else(|| freeze("rebind-expression/old-text-end"))?;
        let unwind = self.loop_expression_text_unwind_v2(frame.evaluation(), product,
            std::slice::from_ref(read), context, function, finishing)?;
        let old = match before {
            Producer::Header(original) if Rc::ptr_eq(original, basis) => {
                if !matches!(basis.entry_producer(), ValueProducer::Text(Producer::Static { .. })) {
                    return Err(freeze("rebind-expression/old-text-end"));
                }
                InvokeOperation::String(Op::EndIfOwned { value: read.loan_text_join_v1(before)?.value() })
            }
            Producer::Residence(acquisition) => {
                let operations = self.prior_home_unwind_for(&[
                    crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1::Text(acquisition.clone())])?;
                let [old] = operations.as_slice() else { return Err(freeze("rebind-expression/old-text-end")); };
                if !matches!(old, InvokeOperation::String(Op::End { .. })) {
                    return Err(freeze("rebind-expression/old-text-end"));
                }
                old.clone()
            }
            Producer::BranchNormal(source) if source.is_owned_residence_tree_v1()
                || source.is_loop_header_or_owned_residence_tree_v1(basis) => {
                let value = read.loan_text_branch_v1(before)?.relation().values().0;
                InvokeOperation::String(Op::EndIfOwned { value })
            }
            _ => return Err(freeze("rebind-expression/old-text-end")),
        };
        if unwind.iter().filter(|operation| **operation == old).count() != 1 {
            return Err(freeze("rebind-expression/old-home-missing"));
        }
        Ok((old, unwind))
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir) fn prepare_return_text_after_unwind_v2(
        &self,
        frame: &Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1>,
        read: &crate::mir::builder::BindingValueObservationV2,
        function: &MirFunction,
    ) -> Result<(InvokeOperation, Vec<InvokeOperation>), String> {
        self.return_text_after_unwind_v2(frame, read, CallValidationContextV1::Lowering(self), function, None)
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn return_text_after_unwind_v2(
        &self,
        frame: &Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1>,
        read: &crate::mir::builder::BindingValueObservationV2,
        context: CallValidationContextV1<'_>, function: &MirFunction,
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
    ) -> Result<(InvokeOperation, Vec<InvokeOperation>), String> {
        use crate::mir::resolved_semantics::home_new_prefix::{SourceLoopTextProducerV1 as Producer,
            SourceLoopValueProducerV1 as ValueProducer};
        context.require_ledger(self)?;
        let (_, source) = frame.return_text_after_read_v2().map_err(freeze)?
            .ok_or_else(|| freeze("source-loop/text-point-binding"))?;
        let Producer::After { product, binding } = source.producer() else {
            return Err(freeze("source-loop/text-point-binding"));
        };
        if self.return_expression_evaluation_v1(frame.source().owner(), frame.source().root().statement())?
            .is_none_or(|original| !Rc::ptr_eq(&original, frame))
            || frame.prior_homes() != product.source.condition.prior_homes()
            || product.source.headers.get(binding).is_none_or(|basis|
                !matches!(basis.entry_producer(), ValueProducer::Text(Producer::Static { .. }))) {
            return Err(freeze("expression/residence-original-pair"));
        }
        super::expression_projection::ExpressionReferenceReadV1::validate_text_binding_with_finishing_v2(
            *binding, source.producer(), read, context, function, finishing)?;
        let value = read.loan_text_join_v1(source.producer())?.value();
        let old = InvokeOperation::String(crate::mir::instruction::StringInvokeOperation::EndIfOwned { value });
        // The loop generation was acquired after the retained entry Homes.
        // Equality above excludes younger ordinary Homes needing earlier End.
        let mut unwind = vec![old.clone()];
        unwind.extend(self.prior_home_unwind_for(frame.prior_homes())?);
        Ok((old, unwind))
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir) fn prepare_loop_expression_text_unwind_v2(
        &self,
        frame: &crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1,
        product: &Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>,
        reads: &[crate::mir::builder::BindingValueObservationV2], function: &MirFunction,
    ) -> Result<Vec<InvokeOperation>, String> {
        self.loop_expression_text_unwind_v2(frame, product, reads,
            CallValidationContextV1::Lowering(self), function, None)
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn loop_expression_text_unwind_v2(
        &self,
        frame: &crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1,
        product: &Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>,
        reads: &[crate::mir::builder::BindingValueObservationV2],
        context: CallValidationContextV1<'_>, function: &MirFunction,
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
    ) -> Result<Vec<InvokeOperation>, String> {
        use crate::mir::resolved_semantics::home_new_prefix::{LiveHomeV1,
            SourceLoopTextProducerV1 as Producer};
        use crate::mir::instruction::StringInvokeOperation as Op;
        context.require_ledger(self)?;
        context.require_original_source_loop_v2(product)?;
        if !product.owns_expression_v2(frame.source()) {
            return Err(freeze("source-loop/text-point-binding"));
        }
        let sources = product.expression_text_fault_producers_v2(frame.source())?;
        if reads.len() != sources.len() || sources.len() > 1 {
            // Distinct conditional Header generations need a source acquisition
            // order; the source map's key order never supplies that proof.
            return Err(freeze("source-loop/text-point-binding"));
        }
        let mut unwind = self.prior_home_unwind_for(frame.prior_homes())?;
        for ((binding, producer), read) in sources.iter().zip(reads) {
            super::expression_projection::ExpressionReferenceReadV1::validate_text_binding_with_finishing_v2(
                *binding, producer, read, context, function, finishing)?;
            let end = match producer {
                Producer::Static { .. } => continue,
                Producer::Header(_) => InvokeOperation::String(Op::EndIfOwned {
                    value: read.loan_text_join_v1(producer)?.value(),
                }),
                Producer::Residence(acquisition) => {
                    let operations = self.prior_home_unwind_for(&[LiveHomeV1::Text(acquisition.clone())])?;
                    let [end] = operations.as_slice() else { return Err(freeze("rebind-expression/old-text-end")); };
                    end.clone()
                }
                Producer::BranchNormal(_) => InvokeOperation::String(Op::EndIfOwned {
                    value: read.loan_text_branch_v1(producer)?.relation().values().0,
                }),
                _ => return Err(freeze("source-loop/text-point-binding")),
            };
            unwind = self.loop_text_unwind_with_end_v2(frame, product, *binding, producer, &end, &unwind)?;
        }
        Ok(unwind)
    }

    /// Each incoming source must place the joined End at the same cleanup
    /// position. Compute from original Home suffixes, never branch/map order.
    fn loop_text_unwind_with_end_v2(
        &self,
        frame: &crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1,
        product: &Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>,
        binding: crate::mir::resolved_semantics::BindingRefV1,
        producer: &crate::mir::resolved_semantics::home_new_prefix::SourceLoopTextProducerV1,
        end: &InvokeOperation,
        unwind: &[InvokeOperation],
    ) -> Result<Vec<InvokeOperation>, String> {
        use crate::mir::resolved_semantics::home_new_prefix::{LiveHomeV1,
            SourceLoopTextProducerV1 as Producer, SourceLoopValueProducerV1 as ValueProducer};
        if let Producer::BranchNormal(source) = producer {
            if source.binding() != binding || !(source.is_owned_residence_tree_v1()
                || product.source.headers.get(&binding).is_some_and(|basis|
                    source.is_loop_header_or_owned_residence_tree_v1(basis))) {
                return Err(freeze("source-loop/text-point-binding"));
            }
            let mut selected = None;
            for (_, _, incoming) in source.incoming() {
                let candidate = self.loop_text_unwind_with_end_v2(frame, product, binding, incoming, end, unwind)?;
                if selected.as_ref().is_some_and(|previous| previous != &candidate) {
                    return Err(freeze("rebind-expression/old-home-missing"));
                }
                selected = Some(candidate);
            }
            return selected.ok_or_else(|| freeze("source-loop/text-point-binding"));
        }
        let mut result = unwind.to_vec();
        let mut replace = |old: &InvokeOperation| -> Result<(), String> {
            let mut indices = unwind.iter().enumerate().filter_map(|(index, operation)| (operation == old).then_some(index));
            let index = indices.next().ok_or_else(|| freeze("rebind-expression/old-home-missing"))?;
            if indices.next().is_some() { return Err(freeze("rebind-expression/old-home-missing")); }
            result[index] = end.clone();
            Ok(())
        };
        let prior = match producer {
            Producer::Header(basis) if basis.binding() == binding => match basis.entry_producer() {
                ValueProducer::Text(Producer::Residence(acquisition)) => {
                    let old = self.prior_home_unwind_for(&[LiveHomeV1::Text(acquisition.clone())])?;
                    let [old] = old.as_slice() else { return Err(freeze("rebind-expression/old-text-end")); };
                    replace(old)?;
                    return Ok(result);
                }
                ValueProducer::Text(Producer::Static { .. }) => product.source.condition.prior_homes().to_vec(),
                _ => return Err(freeze("source-loop/text-point-binding")),
            },
            Producer::Residence(acquisition) if acquisition.binding() == binding => {
                let old = self.prior_home_unwind_for(&[LiveHomeV1::Text(acquisition.clone())])?;
                let [old] = old.as_slice() else { return Err(freeze("rebind-expression/old-text-end")); };
                if unwind.contains(old) {
                    replace(old)?;
                    return Ok(result);
                }
                let original = acquisition.rebind_origin_v1().ok_or_else(|| freeze("rebind-expression/old-text-end"))?;
                original.evaluation().prior_homes().iter().filter(|home| home.binding() != binding).cloned().collect()
            }
            _ => return Err(freeze("source-loop/text-point-binding")),
        };
        if !frame.prior_homes().starts_with(&prior) { return Err(freeze("rebind-expression/old-home-missing")); }
        let suffix = self.prior_home_unwind_for(&prior)?;
        if !unwind.ends_with(&suffix) { return Err(freeze("rebind-expression/old-home-missing")); }
        result.insert(unwind.len() - suffix.len(), end.clone());
        Ok(result)
    }

}
