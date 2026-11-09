//! Common Normal/Fault temporary cleanup correspondence for expression purposes.
use super::*;
use crate::mir::normal_callable_semantic_package::EvaluatedExpressionValueV1;
use crate::mir::normal_callable_semantic_package::LexicalCallArgumentProjectionV1;
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use crate::mir::resolved_semantics::home_new_prefix::{
    ArgumentCallObservationV1, CheckedArgumentExpressionFrameV1, LocalCallArgumentV1,
    LocalCallObservationV1,
};
pub(in crate::mir) use crate::mir::builder::ExpressionOwnedTemporaryV1;
/// Borrow original ordered arguments and live Homes across Call purposes;
/// an expression result never gains a synthetic local destination.
#[derive(Clone, Copy)]
pub(in crate::mir) struct ExpressionCallArgumentSourceV1<'a> {
    site: &'a OwnedExprSiteV1,
    arguments: &'a [LocalCallArgumentV1],
    prior_homes: &'a [LiveHomeV1],
}
impl<'a> ExpressionCallArgumentSourceV1<'a> {
    pub(in crate::mir) fn from_local(call: &'a LocalCallObservationV1) -> Self {
        Self {
            site: call.site(),
            arguments: call.arguments(),
            prior_homes: call.prior_homes(),
        }
    }
    pub(in crate::mir) fn from_nested(call: &'a ArgumentCallObservationV1) -> Self {
        Self {
            site: call.site(),
            arguments: call.arguments(),
            prior_homes: call.prior_homes(),
        }
    }
    pub(in crate::mir) fn for_owned_array_candidate_v1(
        ledger: &'a OrdinaryNewClaimLedgerV1,
        source: &'a crate::mir::normal_callable_semantic_package::OwnedArrayAppendSourceLoanV1,
        row: &'a crate::mir::normal_callable_semantic_package::LexicalInstanceCallDispositionRowV1,
    ) -> Result<Self, String> {
        source.check_candidate_v1(row)?;
        if source.candidate() != *row.call_site() {
            return Err(freeze("argument-expression/prefix-original-call"));
        }
        Ok(Self {
            site: row.call_site(),
            arguments: ledger.object_packet_arguments_v1(row)?,
            prior_homes: source.prior_homes()?,
        })
    }
    pub(in crate::mir) fn site(self) -> &'a OwnedExprSiteV1 {
        self.site
    }
    pub(in crate::mir) fn arguments(self) -> &'a [LocalCallArgumentV1] {
        self.arguments
    }
    pub(in crate::mir) fn prior_homes(self) -> &'a [LiveHomeV1] {
        self.prior_homes
    }
}
/// A synchronous loan of the parent's already checked live stack at one Call.
/// It carries no source permission and never survives into a child packet.
pub(in crate::mir) struct ExpressionCallTransientPrefixV1<'a> {
    origin: TransientPrefixOriginV1<'a>,
    parent: Option<&'a ExpressionCallTransientPrefixV1<'a>>,
}
enum TransientPrefixOriginV1<'a> {
    NewArguments {
        current: &'a crate::mir::normal_callable_semantic_package::OrdinaryNewTrivialArgumentV1,
        earlier: &'a [crate::mir::normal_callable_semantic_package::EmittedNewArgumentV1],
    },
    Emitted {
        call: &'a EmittedLexicalCallProjectionV1,
        live: &'a [ExpressionOwnedTemporaryV1],
        prior_homes: &'a [LiveHomeV1],
    },
    Arguments {
        call: ExpressionCallArgumentSourceV1<'a>,
        ordinal: usize,
        earlier: &'a [LexicalCallArgumentProjectionV1],
        context: CallValidationContextV1<'a>,
    },
    Evaluation {
        frame: &'a CheckedExpressionEvaluationV1,
        call: &'a crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1,
        acquired: &'a [(usize, ValueId)],
    },
}
impl<'a> ExpressionCallTransientPrefixV1<'a> {
    pub(in crate::mir) fn original_observation_v1(&self) -> Option<&LocalCallObservationV1> {
        match &self.origin { TransientPrefixOriginV1::Evaluation { call, .. } => Some(call), _ => None }
    }
    pub(in crate::mir) fn for_prepared_new_argument_v1(
        current: &'a crate::mir::normal_callable_semantic_package::OrdinaryNewTrivialArgumentV1,
        earlier: &'a [crate::mir::normal_callable_semantic_package::EmittedNewArgumentV1],
        rows: &[crate::mir::normal_callable_semantic_package::OrdinaryNewTrivialArgumentV1],
    ) -> Result<Self, String> {
        let crate::mir::normal_callable_semantic_package::OrdinaryNewTrivialArgumentKindV1::Expression { evaluation } = current.kind()
            else { return Err(freeze("argument-source-drift")) };
        let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::NewArgument { construction, ordinal, .. } = evaluation.source().root()
            else { return Err(freeze("argument-expression/transient-purpose")) };
        if construction.site() != current.new_site().site()
            || *ordinal != current.ordinal()
            || construction.arguments().len() != rows.len()
            || rows
                .iter()
                .zip(construction.arguments())
                .any(|(row, original)| row.site() != original)
            || earlier.len() != current.ordinal() as usize
            || rows.get(earlier.len()) != Some(current)
            || rows.iter().enumerate().any(|(ordinal, row)| {
                row.new_site() != current.new_site()
                    || row.ordinal() as usize != ordinal
                    || row.owner() != current.owner()
            })
        {
            return Err(freeze("argument-expression/prefix-ordinal-frame"));
        }
        for (packet, source) in earlier.iter().zip(rows) {
            if packet.source() != source {
                return Err(freeze("argument-source-drift"));
            }
            packet.deferred_text_values_v1()?;
        }
        let prefix = Self {
            origin: TransientPrefixOriginV1::NewArguments { current, earlier },
            parent: None,
        };
        prefix.operations()?;
        Ok(prefix)
    }
    pub(in crate::mir) fn from_evaluation(
        frame: &'a CheckedExpressionEvaluationV1,
        call: &'a crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1,
        acquired: &'a [(usize, ValueId)],
        values: &[Option<EvaluatedExpressionValueV1>],
        function: &MirFunction,
        parent: Option<&'a Self>,
    ) -> Result<Self, String> {
        validate_expression_acquisition_prefix_v1(frame, Some(call), acquired, values, function)?;
        if let Some(parent) = parent {
            parent.for_argument(frame)?;
        }
        Ok(Self {
            origin: TransientPrefixOriginV1::Evaluation {
                frame,
                call,
                acquired,
            },
            parent,
        })
    }
    pub(in crate::mir) fn for_prepared_argument(
        frame: &'a std::rc::Rc<CheckedArgumentExpressionFrameV1>,
        earlier: &'a [LexicalCallArgumentProjectionV1],
        context: CallValidationContextV1<'a>,
        parent: Option<&'a Self>,
    ) -> Result<Self, String> {
        let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { call, ordinal, .. } = frame.source().root()
            else { return Err(freeze("argument-expression/transient-purpose")) };
        let site = OwnedExprSiteV1::new(frame.source().owner(), call.site().clone());
        let original = context.ledger().qualified_argument_expression_frame_v1(
            &site,
            *ordinal,
            frame
                .source()
                .nodes()
                .first()
                .ok_or_else(|| freeze("argument-expression/root-missing"))?
                .site(),
        )?;
        if earlier.len() != *ordinal as usize || !std::rc::Rc::ptr_eq(&original, frame) {
            return Err(freeze("argument-expression/prefix-ordinal-frame"));
        }
        let call = context
            .ledger()
            .local_call_for_owner(site.owner(), site.site())
            .ok_or_else(|| freeze("argument-expression/prefix-original-call"))?;
        Self::for_prepared_call_argument(
            ExpressionCallArgumentSourceV1::from_local(call),
            *ordinal as usize,
            earlier,
            context,
            parent,
        )
    }
    /// The original Call/position also covers a CallResult argument, which has
    /// no expression frame. Callers lend the same source observation used by
    /// their packet; this synchronous view never becomes stored authority.
    pub(in crate::mir) fn for_prepared_call_argument(
        call: ExpressionCallArgumentSourceV1<'a>,
        ordinal: usize,
        earlier: &'a [LexicalCallArgumentProjectionV1],
        context: CallValidationContextV1<'a>,
        parent: Option<&'a Self>,
    ) -> Result<Self, String> {
        if ordinal >= call.arguments().len() || earlier.len() != ordinal {
            return Err(freeze("argument-expression/prefix-ordinal-frame"));
        }
        for (position, packet) in earlier.iter().enumerate() {
            packet.require_argument_frame_position_v1(
                call.site(),
                position,
                call.prior_homes(),
                context,
            )?;
            packet.deferred_temporaries_v1(context)?;
        }
        if let Some(parent) = parent {
            parent.unwind_for_call(call.site(), call.prior_homes())?;
        }
        let prefix = Self {
            origin: TransientPrefixOriginV1::Arguments {
                call,
                ordinal,
                earlier,
                context,
            },
            parent,
        };
        prefix.operations()?;
        Ok(prefix)
    }
    fn contains(&self, site: &OwnedExprSiteV1) -> Result<bool, String> {
        fn in_arguments(arguments: &[LocalCallArgumentV1], site: &OwnedExprSiteV1) -> bool {
            arguments.iter().any(|argument| {
                matches!(argument, LocalCallArgumentV1::CallResult(inner)
                if inner.site() == site || in_arguments(inner.arguments(), site))
            })
        }
        Ok(match &self.origin {
            TransientPrefixOriginV1::NewArguments { current, .. } => {
                let crate::mir::normal_callable_semantic_package::OrdinaryNewTrivialArgumentKindV1::Expression { evaluation } = current.kind()
                    else { return Err(freeze("argument-source-drift")) };
                current.new_site() == site || evaluation.source().contains_call_site_v1(site)
            }
            TransientPrefixOriginV1::Emitted { call, .. } => {
                call.retained_node_at_v1(site)?.is_some()
            }
            TransientPrefixOriginV1::Evaluation { call, .. } => {
                call.site() == site || in_arguments(call.arguments(), site)
            }
            TransientPrefixOriginV1::Arguments { call, ordinal, .. } => {
                call.site() == site
                    || matches!(call.arguments().get(*ordinal),
                    Some(LocalCallArgumentV1::CallResult(inner))
                        if inner.site() == site || in_arguments(inner.arguments(), site))
            }
        })
    }
    fn prior_homes(&self) -> &[LiveHomeV1] {
        match &self.origin {
            TransientPrefixOriginV1::NewArguments { current, .. } => {
                let crate::mir::normal_callable_semantic_package::OrdinaryNewTrivialArgumentKindV1::Expression { evaluation } = current.kind()
                    else { unreachable!("checked original New expression prefix") };
                evaluation.prior_homes()
            }
            TransientPrefixOriginV1::Emitted { prior_homes, .. } => prior_homes,
            TransientPrefixOriginV1::Evaluation { frame, .. } => frame.prior_homes(),
            TransientPrefixOriginV1::Arguments { call, .. } => call.prior_homes(),
        }
    }
    pub(in super::super) fn require_call(
        &self,
        call: &EmittedLexicalCallProjectionV1,
    ) -> Result<(), String> {
        if !self.contains(call.call_site())?
            || matches!(&self.origin,
            TransientPrefixOriginV1::Emitted { call: original, .. }
                if !original.retained_node_at_v1(call.call_site())?.is_some_and(|packet| std::ptr::eq(packet, call)))
        {
            return Err(freeze("argument-expression/transient-call"));
        }
        Ok(())
    }
    fn operations(&self) -> Result<Vec<InvokeOperation>, String> {
        let end = |value| {
            InvokeOperation::String(crate::mir::instruction::StringInvokeOperation::End { value })
        };
        let mut operations: Vec<_> = match &self.origin {
            TransientPrefixOriginV1::NewArguments { earlier, .. } => {
                let mut operations = Vec::new();
                for packet in earlier.iter().rev() {
                    operations.extend(packet.deferred_text_values_v1()?.into_iter().rev().map(end));
                }
                operations
            }
            TransientPrefixOriginV1::Emitted { live, .. } => {
                live.iter().rev().map(|temporary| temporary.end_operation()).collect()
            }
            TransientPrefixOriginV1::Evaluation { acquired, .. } => acquired
                .iter()
                .rev()
                .map(|(_, value)| end(*value))
                .collect(),
            TransientPrefixOriginV1::Arguments {
                earlier, context, ..
            } => {
                let mut operations = Vec::new();
                for argument in earlier.iter().rev() {
                    operations.extend(
                        argument
                            .deferred_temporaries_v1(*context)?
                            .into_iter()
                            .rev()
                            .map(ExpressionOwnedTemporaryV1::end_operation),
                    );
                }
                operations
            }
        };
        if let Some(parent) = self.parent {
            operations.extend(parent.operations()?);
        }
        // Every acquisition is affine across the complete borrowed prefix,
        // including different arguments and ancestors. Never deduplicate an
        // overlapping stack: that would hide a double owner and alter faults.
        let mut seen = std::collections::BTreeSet::new();
        for operation in &operations {
            if let InvokeOperation::String(crate::mir::instruction::StringInvokeOperation::End { value })
                | InvokeOperation::DynamicCarrierEnd { carrier: value } = operation
            {
                if !seen.insert(*value) {
                    return Err(freeze("argument-expression/prefix-duplicate-acquisition"));
                }
            }
        }
        Ok(operations)
    }
    pub(in crate::mir) fn unwind_for_call(
        &self,
        site: &OwnedExprSiteV1,
        homes: &[LiveHomeV1],
    ) -> Result<Vec<InvokeOperation>, String> {
        if self.prior_homes() != homes || !self.contains(site)? {
            return Err(freeze("expression-call/context-drift"));
        }
        self.operations()
    }
    pub(in crate::mir) fn for_argument(
        &self,
        frame: &CheckedExpressionEvaluationV1,
    ) -> Result<Vec<InvokeOperation>, String> {
        if let TransientPrefixOriginV1::NewArguments { current, .. } = &self.origin {
            let crate::mir::normal_callable_semantic_package::OrdinaryNewTrivialArgumentKindV1::Expression { evaluation } = current.kind()
                else { return Err(freeze("argument-source-drift")) };
            let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::NewArgument { construction, ordinal, .. } = frame.source().root()
                else {
                    let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { call, .. } = frame.source().root()
                        else { return Err(freeze("argument-expression/transient-purpose")) };
                    return self.unwind_for_call(&OwnedExprSiteV1::new(frame.source().owner(), call.site().clone()), frame.prior_homes());
                };
            if !std::ptr::eq(evaluation.as_ref(), frame)
                || frame.source().owner() != current.owner()
                || construction.site() != current.new_site().site()
                || *ordinal != current.ordinal()
                || construction.arguments().get(*ordinal as usize) != Some(current.site())
            {
                return Err(freeze("argument-expression/prefix-current-frame"));
            }
            return self.unwind_for_call(current.new_site(), frame.prior_homes());
        }
        if let TransientPrefixOriginV1::Arguments {
            call,
            ordinal,
            context,
            ..
        } = &self.origin
        {
            let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { call: original, ordinal: position, .. } = frame.source().root()
                else { return Err(freeze("argument-expression/transient-purpose")) };
            let site = frame
                .source()
                .nodes()
                .first()
                .ok_or_else(|| freeze("argument-expression/root-missing"))?
                .site();
            let canonical = context.ledger().qualified_argument_expression_frame_v1(
                call.site(),
                *position,
                site,
            )?;
            if call.site().owner() != frame.source().owner()
                || call.site().site() != original.site()
                || *ordinal != *position as usize
                || !std::ptr::eq(canonical.evaluation(), frame)
            {
                return Err(freeze("argument-expression/prefix-current-frame"));
            }
        }
        let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { call, .. } = frame.source().root()
            else { return Err(freeze("argument-expression/transient-purpose")) };
        self.unwind_for_call(
            &OwnedExprSiteV1::new(frame.source().owner(), call.site().clone()),
            frame.prior_homes(),
        )
    }
}


#[path = "expression_terminal_cleanup.rs"]
mod expression_terminal_cleanup;

#[path = "expression_acquisition_paths.rs"]
mod expression_acquisition_paths;

#[path = "expression_continuation_paths.rs"]
mod expression_continuation_paths;

#[path = "expression_reference_projection.rs"]
mod expression_reference_projection;
