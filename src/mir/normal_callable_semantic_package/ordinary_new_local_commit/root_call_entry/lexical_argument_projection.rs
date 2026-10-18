//! Tie an observed argument value to the original lexical argument meaning.
use super::*;
use crate::mir::resolved_semantics::OwnedExprSiteV1;
use super::super::expression_projection::ExpressionCallTransientPrefixV1;

impl LexicalCallArgumentProjectionV1 {
    pub(in crate::mir) fn value_for_source_v1(
        &self,
        source: &LocalCallArgumentV1,
        owner: FunctionOwnerIdV1,
        site: &crate::mir::resolved_semantics::SourceExprSiteV1,
        row: Option<CallPacketSourceLoanV1<'_>>,
        context: Option<CallValidationContextV1<'_>>,
    ) -> Result<ValueId, String> {
        Ok(match (self, source) {
            (
                LexicalCallArgumentProjectionV1::Integer((
                    _,
                    MirInstruction::Const {
                        dst,
                        value: crate::mir::ConstValue::Integer(actual),
                    },
                )),
                LocalCallArgumentV1::Integer(expected),
            ) if actual == expected => *dst,
            (
                LexicalCallArgumentProjectionV1::Scalar(read),
                LocalCallArgumentV1::Scalar(binding),
            ) => read
                .value_for(owner, site.node(), *binding)
                .map_err(|error| format!("[freeze:contract][lexical-i64/argument/{error:?}]"))?,
            (
                LexicalCallArgumentProjectionV1::CallResult(emitted),
                LocalCallArgumentV1::CallResult(inner),
            ) if inner.site().owner() == owner
                && inner.site().site() == site
                && emitted.call_site() == inner.site() =>
            {
                emitted.value_using(owner, inner.arguments(), context)?
            }
            (
                projection,
                LocalCallArgumentV1::BorrowedActual {
                    ordinal,
                    site: original,
                },
            ) if original == site => borrowed_projection::value(
                projection,
                owner,
                row.ok_or_else(|| freeze("lexical-i64/borrowed-source-row-missing"))?,
                *ordinal,
                original,
                context.ok_or_else(|| freeze("lexical-i64/borrowed-lender-missing"))?,
            )?,
            (
                Self::Expression(packet),
                LocalCallArgumentV1::Expression {
                    ordinal,
                    site: original,
                },
            ) if original == site => {
                if packet.ordinal()? != *ordinal
                    || packet.frame.source().owner() != owner
                    || packet
                        .frame
                        .source()
                        .nodes()
                        .first()
                        .is_none_or(|root| root.site() != site)
                {
                    return Err(freeze("argument-expression/ordinal-owner"));
                }
                packet.validate_source(
                    context.ok_or_else(|| freeze("argument-expression/context-missing"))?,
                )?
            }
            _ => return Err(freeze("lexical-i64/argument-projection-drift")),
        })
    }
}

/// Observations remain within the enclosing Call packet, never a second pool.
#[derive(Debug)]
pub(in crate::mir) struct EmittedArgumentExpressionProjectionV1 {
    frame: std::rc::Rc<
        crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1,
    >,
    values: Vec<Option<crate::mir::instruction::StringInvokeOperand>>,
    calls: Vec<std::rc::Rc<EmittedLexicalCallProjectionV1>>,
    path_reads: Vec<crate::mir::builder::ExactLexicalReadV1>,
    reference_reads: std::collections::BTreeMap<
        usize,
        super::super::expression_projection::ExpressionReferenceReadV1,
    >,
    entry: Binding,
    terminal: Binding,
    continuation: Binding,
    original: Vec<Binding>,
}
impl EmittedArgumentExpressionProjectionV1 {
    fn ordinal(&self) -> Result<u32, String> {
        let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { ordinal, .. } = self.frame.source().root()
            else { return Err(freeze("argument-expression/source-purpose")) };
        Ok(*ordinal)
    }
    fn expression(
        &self,
    ) -> super::super::expression_projection::ExpressionPhysicalProjectionRefV1<'_> {
        super::super::expression_projection::ExpressionPhysicalProjectionRefV1::new(
            self.frame.evaluation(),
            &self.values,
            &self.calls,
            &self.path_reads,
            &self.reference_reads,
            &self.original,
        )
    }
    fn validate_source(&self, context: CallValidationContextV1<'_>) -> Result<ValueId, String> {
        let source = self.frame.source();
        let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { call, ordinal, .. } = source.root()
            else { return Err(freeze("argument-expression/source-purpose")) };
        let site = source
            .nodes()
            .first()
            .ok_or_else(|| freeze("argument-expression/root-missing"))?
            .site();
        let original = context.ledger().argument_expression_frame_v1(
            &crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                source.owner(),
                call.site().clone(),
            ),
            *ordinal,
            site,
        )?;
        if !std::rc::Rc::ptr_eq(&original, &self.frame)
            || !self.frame.is_i64()
            || !matches!(
                self.terminal.1,
                MirInstruction::Jump {
                    edge_args: None,
                    ..
                }
            )
            || !matches!(
                self.continuation.1,
                MirInstruction::Jump {
                    edge_args: None,
                    ..
                }
            )
        {
            return Err(freeze("argument-expression/source-or-result"));
        }
        let Some(Some(crate::mir::instruction::StringInvokeOperand::NormalInteger(value))) =
            self.values.first()
        else {
            return Err(freeze("argument-expression/result"));
        };
        self.expression().validate_source(context)?;
        Ok(*value)
    }
    fn validate(
        &self,
        context: CallValidationContextV1<'_>,
        function: &crate::mir::MirFunction,
        finishing: Option<&super::super::super::physical_boundary::FinishedBindings>,
        inherited: Option<&ExpressionCallTransientPrefixV1<'_>>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        self.validate_source(context)?;
        self.expression().validate_scalar_continuation_v1(
            context,
            function,
            finishing,
            &self.entry,
            &self.terminal,
            &self.continuation,
            inherited,
        )
    }
}
impl LexicalCallArgumentProjectionV1 {
    pub(in crate::mir) fn from_expression_v1(
        frame: std::rc::Rc<
            crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1,
        >,
        values: Vec<Option<crate::mir::instruction::StringInvokeOperand>>,
        calls: Vec<std::rc::Rc<EmittedLexicalCallProjectionV1>>,
        path_reads: Vec<crate::mir::builder::ExactLexicalReadV1>,
        reference_reads: std::collections::BTreeMap<
            usize,
            super::super::expression_projection::ExpressionReferenceReadV1,
        >,
        entry: Binding,
        terminal: Binding,
        continuation: Binding,
        original: Vec<Binding>,
        ledger: &crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1,
        function: &crate::mir::MirFunction,
        inherited: Option<&ExpressionCallTransientPrefixV1<'_>>,
    ) -> Result<Self, String> {
        let packet = EmittedArgumentExpressionProjectionV1 {
            frame,
            values,
            calls,
            path_reads,
            reference_reads,
            entry,
            terminal,
            continuation,
            original,
        };
        let context = CallValidationContextV1::Lowering(ledger);
        let covered = packet.validate(context, function, None, inherited)?;
        let mut ordered = packet.expression().ordered_bindings_in_context(context)?;
        ordered.push(packet.terminal.clone());
        packet.expression().validate_original_scope(
            ledger,
            function,
            &covered,
            &packet.entry,
            &packet.terminal,
            &packet.continuation,
            &ordered,
        )?;
        Ok(Self::Expression(Box::new(packet)))
    }
    pub(in crate::mir) fn expression_bindings_v1(&self) -> &[Binding] {
        match self {
            Self::Expression(packet) => &packet.original,
            _ => &[],
        }
    }
    pub(in crate::mir) fn validate_expression_physical_v1(
        &self,
        context: CallValidationContextV1<'_>,
        function: &crate::mir::MirFunction,
        finishing: Option<&super::super::super::physical_boundary::FinishedBindings>,
        bindings: &[Binding],
        coordinates: Option<&super::super::super::physical_boundary::FinishedBindings>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        self.validate_expression_physical_with_prefix_v1(context, function, finishing, bindings, coordinates, None)
    }
    fn validate_expression_physical_with_prefix_v1(
        &self, context: CallValidationContextV1<'_>, function: &crate::mir::MirFunction,
        finishing: Option<&FinishedBindings>, bindings: &[Binding], coordinates: Option<&FinishedBindings>,
        inherited: Option<&ExpressionCallTransientPrefixV1<'_>>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        if let Self::Expression(packet) = self {
            let covered = packet.validate(context, function, finishing, inherited)?;
            packet.expression().validate_recorded(bindings, coordinates)?;
            return Ok(covered);
        }
        Ok(Default::default())
    }

    pub(super) fn expression_copy_dependencies_v1(&self, context: CallValidationContextV1<'_>) -> Result<Vec<Binding>, String> {
        let mut copies = Vec::new();
        if let Self::Expression(packet) = self {
            packet.validate_source(context)?;
            for read in &packet.path_reads { copies.extend_from_slice(read.original_copy_dependencies_v1()?); }
            for read in packet.reference_reads.values() { copies.extend_from_slice(read.original_copy_dependencies_v1()?); }
            for node in packet.frame.source().nodes() {
                copies.extend(context.ledger().checked_field_copy_dependencies_v1(
                    &OwnedExprSiteV1::new(packet.frame.source().owner(), node.site().clone()), context)?);
            }
        }
        Ok(copies)
    }

    /// Child packets remain borrowed from their original evaluation, never
    /// reconstructed from the flattened MIR or promoted to another pool.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn expression_calls_v1(&self) -> impl Iterator<Item = &EmittedLexicalCallProjectionV1> {
        match self { Self::Expression(packet) => Some(packet.as_ref()), _ => None }
            .into_iter().flat_map(|packet| packet.calls.iter().map(std::rc::Rc::as_ref))
    }

    pub(super) fn visit_expression_calls_v1(
        &self, owner: FunctionOwnerIdV1, context: CallValidationContextV1<'_>,
        visit: &mut impl FnMut(&EmittedLexicalCallProjectionV1, &[LocalCallArgumentV1]) -> Result<(), String>,
    ) -> Result<(), String> {
        if let Self::Expression(packet) = self {
            packet.validate_source(context)?;
            for (source, call) in packet.frame.evaluation().calls().iter().zip(&packet.calls) {
                call.visit_original_nodes_in_context_v1(owner, source.arguments(), context, visit)?;
            }
        }
        Ok(())
    }
}

impl EmittedLexicalCallProjectionV1 {
    /// The enclosing Call is the common consumer of its source-bound argument
    /// evaluations. Completed scalar children stay outside its live-temp stack.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_expression_arguments_v1(
        &self, context: CallValidationContextV1<'_>, function: &crate::mir::MirFunction,
        finishing: Option<&FinishedBindings>, bindings: &[Binding], coordinates: Option<&FinishedBindings>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        self.validate_expression_arguments_with_prefix_v1(context, function, finishing, bindings, coordinates, None)
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_expression_arguments_with_prefix_v1(
        &self, context: CallValidationContextV1<'_>, function: &crate::mir::MirFunction,
        finishing: Option<&FinishedBindings>, bindings: &[Binding], coordinates: Option<&FinishedBindings>,
        inherited: Option<&ExpressionCallTransientPrefixV1<'_>>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        if let Some(prefix) = inherited { prefix.require_call(self)?; }
        let mut covered = std::collections::BTreeSet::new();
        for (ordinal, argument) in self.prepared.arguments.iter().enumerate() {
            if let LexicalCallArgumentProjectionV1::Expression(packet) = argument {
                let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { call, ordinal: original, .. } = packet.frame.source().root()
                    else { return Err(freeze("argument-expression/source-purpose")) };
                if packet.frame.source().owner() != self.call_site().owner()
                    || call.site() != self.call_site().site() || *original as usize != ordinal
                    || self.row.loan().argument_sites().get(ordinal) != packet.frame.source().nodes().first().map(|node| node.site()) {
                    return Err(freeze("argument-expression/enclosing-call"));
                }
            }
            covered.extend(argument.validate_expression_physical_with_prefix_v1(context, function, finishing, bindings, coordinates, inherited)?);
            if let LexicalCallArgumentProjectionV1::CallResult(inner) = argument {
                covered.extend(inner.validate_expression_arguments_with_prefix_v1(context, function, finishing, bindings, coordinates, inherited)?);
            }
        }
        Ok(covered)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn same_owner_and_ordinal_cannot_substitute_another_original_argument_frame() {
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog_selection(
            "static box Values { find(p: i64): i64 { return p } run(n: i64): i64 { local a = Values.find(n + 1) local b = Values.find(n + 2) return 0 } } static box Main { main(args) { local out = Values.run(1) return 0 } }",
            crate::mir::normal_callable_semantic_package::PackagePhysicalSelectionV1::ClosedAppExecutable,
        ).unwrap();
        let ledger = &package.ordinary_new_claim_ledger;
        let mut frames = Vec::new();
        for declaration in package.batch().declarations() {
            if let Some(flow) = ledger.completion_for_owner(declaration.owner()).and_then(|completion| completion.cleanup().root_flow()) {
                frames.extend(flow.expression_plans().iter().filter_map(|plan| plan.argument_frame()).cloned());
            }
        }
        assert_eq!(frames.len(), 2);
        assert_eq!(frames[0].source().owner(), frames[1].source().owner());
        let source = LocalCallArgumentV1::Expression { ordinal: 0, site: frames[1].source().nodes()[0].site().clone() };
        let projection = LexicalCallArgumentProjectionV1::Expression(Box::new(EmittedArgumentExpressionProjectionV1 {
            frame: std::rc::Rc::clone(&frames[0]), values: Vec::new(), calls: Vec::new(), path_reads: Vec::new(),
            reference_reads: Default::default(), entry: (BasicBlockId(1), MirInstruction::Jump { target: BasicBlockId(2), edge_args: None }),
            terminal: (BasicBlockId(2), MirInstruction::Jump { target: BasicBlockId(3), edge_args: None }),
            continuation: (BasicBlockId(3), MirInstruction::Jump { target: BasicBlockId(4), edge_args: None }), original: Vec::new(),
        }));
        assert!(projection.value_for_source_v1(&source, frames[1].source().owner(), frames[1].source().nodes()[0].site(),
            None, Some(CallValidationContextV1::Lowering(ledger))).unwrap_err().contains("argument-expression/ordinal-owner"),
            "foreign occurrence must refuse before examining any physical payload");
    }
}
