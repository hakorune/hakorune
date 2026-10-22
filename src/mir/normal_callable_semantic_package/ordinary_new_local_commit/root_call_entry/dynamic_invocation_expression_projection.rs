//! Source-bound Dynamic invocation emitted inside one checked expression.
use super::expression_projection::{EvaluatedExpressionValueV1, ExpressionPhysicalProjectionRefV1};
use crate::mir::instruction::{
    DynamicInvocationOperandV1 as Operand, DynamicInvocationOperationV1 as Operation,
    InvokeOperation, StringInvokeOperand,
};
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
use crate::mir::resolved_semantics::home_new_prefix::{
    CheckedExpressionEvaluationV1, LocalCallArgumentV1, LocalCallObservationV1,
    LocalCallResultSourceV1,
};
use crate::mir::resolved_semantics::ResolvedLiteralSourceV1 as Literal;
use crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1 as Node;
use crate::mir::{BasicBlockId, MirInstruction, ValueId};

type Binding = (BasicBlockId, MirInstruction);

/// One exact source call and its only physical Invoke/Normal result pair.
#[derive(Debug)]
pub(crate) struct EmittedDynamicInvocationCallProjectionV1 {
    source: LocalCallObservationV1,
    receiver_read: std::rc::Rc<crate::mir::builder::ExactLexicalReadV1>,
    operation: Operation,
    invoke: Binding,
    projection: Binding,
}

impl EmittedDynamicInvocationCallProjectionV1 {
    pub(in crate::mir) fn operation_from_source_v1(
        frame: &CheckedExpressionEvaluationV1,
        values: &[Option<EvaluatedExpressionValueV1>],
        source: &LocalCallObservationV1,
        receiver: Operand,
    ) -> Result<Operation, String> {
        let Some(LocalCallResultSourceV1::DynamicInvocation(loan)) = source.result_source() else {
            return Err(freeze("dynamic-expression/source-loan"));
        };
        let target = loan.target();
        if values.len() != frame.source().nodes().len()
            || !frame.calls().iter().any(|row| std::ptr::eq(row, source))
            || source.owner() != frame.source().owner()
            || source.result() != crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Dynamic
            || source.dynamic_invocation_result_lifecycle_v1().is_none()
            || loan.call_site() != source.site()
            || target.owner() != source.owner()
            || target.call_site() != source.site().site()
            || target.result_site() != source.site().site()
            || target.dispatch().arity() as usize != loan.argument_sites().len()
            || loan.argument_sites().len() != source.arguments().len()
        {
            return Err(freeze("dynamic-expression/source-drift"));
        }

        let receiver_node = unique_node(frame, target.receiver_site())?;
        let receiver_source = matches!(frame.source().nodes()[receiver_node].source(),
            Node::Reference(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding))
                if *binding == target.receiver_binding());
        if !receiver_source || !matches!(receiver, Operand::TaggedDynamic(_) | Operand::Carrier(_)) {
            return Err(freeze("dynamic-expression/receiver-source"));
        }

        let mut arguments = Vec::with_capacity(loan.argument_sites().len());
        for (index, (site, original)) in loan
            .argument_sites()
            .iter()
            .zip(target.arguments())
            .enumerate()
        {
            if original.ordinal() as usize != index
                || original.site() != site
                || !matches!(source.arguments().get(index), Some(LocalCallArgumentV1::Expression {
                    ordinal, site: actual,
                }) if *ordinal as usize == index && actual == site)
            {
                return Err(freeze("dynamic-expression/argument-source"));
            }
            let node = unique_node(frame, site)?;
            arguments.push(operand_at(frame, values, node)?);
        }
        Operation::from_source(loan, receiver, arguments)
    }

    pub(in crate::mir) fn from_source_v1(
        frame: &CheckedExpressionEvaluationV1,
        values: &[Option<EvaluatedExpressionValueV1>],
        source: &LocalCallObservationV1,
        receiver_read: crate::mir::builder::ExactLexicalReadV1,
        operation: Operation,
        invoke: Binding,
        projection: Binding,
    ) -> Result<Self, String> {
        let Some(LocalCallResultSourceV1::DynamicInvocation(loan)) = source.result_source() else {
            return Err(freeze("dynamic-expression/source-loan"));
        };
        let (receiver_site, receiver_binding, receiver_value) = receiver_read.source_value();
        if receiver_site != crate::mir::resolved_semantics::OwnedExprSiteV1::new(
            source.owner(), loan.target().receiver_site().clone())
            || receiver_binding != loan.target().receiver_binding()
            || receiver_binding.owner() != source.owner()
        {
            return Err(freeze("dynamic-expression/receiver-read"));
        }
        let expected = Self::operation_from_source_v1(
            frame, values, source, Operand::TaggedDynamic(receiver_value),
        )?;
        let MirInstruction::Invoke {
            operation: InvokeOperation::DynamicInvocation(actual),
            normal_landing,
            ..
        } = &invoke.1 else {
            return Err(freeze("dynamic-expression/invoke-kind"));
        };
        let MirInstruction::InvokeNormalResult { invoke_block, .. } = projection.1 else {
            return Err(freeze("dynamic-expression/projection-kind"));
        };
        if actual != &operation
            || operation != expected
            || invoke.0 == projection.0
            || *normal_landing != projection.0
            || invoke_block != invoke.0
        {
            return Err(freeze("dynamic-expression/pair-drift"));
        }
        Ok(Self { source: source.clone(), receiver_read: std::rc::Rc::new(receiver_read), operation, invoke, projection })
    }

    pub(crate) fn source(&self) -> &LocalCallObservationV1 { &self.source }
    pub(crate) fn operation(&self) -> &Operation { &self.operation }
    pub(crate) fn value(&self) -> ValueId {
        match self.projection.1 {
            MirInstruction::InvokeNormalResult { dst, .. } => dst,
            _ => unreachable!("validated Dynamic projection"),
        }
    }
    pub(crate) fn invoke_binding(&self) -> &Binding { &self.invoke }
    pub(crate) fn projection_binding(&self) -> &Binding { &self.projection }
    pub(crate) fn ordered_bindings(&self) -> [Binding; 2] {
        [self.invoke.clone(), self.projection.clone()]
    }

    pub(crate) fn validate_source_v1(
        &self,
        frame: &CheckedExpressionEvaluationV1,
        values: &[Option<EvaluatedExpressionValueV1>],
        observed: &LocalCallObservationV1,
        context: CallValidationContextV1<'_>,
        original: &[Binding],
    ) -> Result<(), String> {
        let (receiver_site, receiver_binding, receiver_value) = self.receiver_read.source_value();
        let Some(LocalCallResultSourceV1::DynamicInvocation(loan)) = observed.result_source() else {
            return Err(freeze("dynamic-expression/source-loan"));
        };
        if receiver_site != crate::mir::resolved_semantics::OwnedExprSiteV1::new(
            observed.owner(), loan.target().receiver_site().clone())
            || receiver_binding != loan.target().receiver_binding()
            || receiver_binding.owner() != observed.owner()
        {
            return Err(freeze("dynamic-expression/receiver-read"));
        }
        let expected = Self::operation_from_source_v1(
            frame, values, observed, Operand::TaggedDynamic(receiver_value),
        )?;
        let index = unique_node(frame, observed.site().site())?;
        let result = values[index]
            .as_ref()
            .ok_or_else(|| freeze("dynamic-expression/result-value"))?
            .as_operand()
            .map_err(|error| error.clone())?;
        let expected_value = StringInvokeOperand::DynamicCarrier(self.value());
        let ledger_source = context.ledger().local_call_for_owner(
            observed.owner(), observed.site().site(),
        );
        if self.source != *observed
            || ledger_source != Some(observed)
            || self.operation != expected
            || result != &expected_value
            || !original.contains(&self.invoke)
            || !original.contains(&self.projection)
            || self.ordered_bindings().iter().any(|binding| {
                original.iter().filter(|row| *row == binding).count() != 1
            })
        {
            return Err(freeze("dynamic-expression/physical-source"));
        }
        Ok(())
    }

    pub(crate) fn validate_recorded_projected_v1(
        &self,
        bindings: &[Binding],
        finishing: Option<&super::physical_boundary::FinishedBindings>,
    ) -> Result<(), String> {
        for original in self.ordered_bindings() {
            let mapped = match finishing {
                Some(projection) => projection.binding(original.0, &original.1)?,
                None => Some(original.clone()),
            };
            if mapped.is_none_or(|mapped| {
                bindings.iter().filter(|binding| **binding == mapped).count() != 1
            }) {
                return Err(freeze("dynamic-expression/recorded-pair"));
            }
        }
        Ok(())
    }
}

fn unique_node(
    frame: &CheckedExpressionEvaluationV1,
    site: &crate::mir::resolved_semantics::SourceExprSiteV1,
) -> Result<usize, String> {
    let mut nodes = frame.source().nodes().iter().enumerate().filter(|(_, node)| node.site() == site);
    let (index, _) = nodes.next().ok_or_else(|| freeze("dynamic-expression/site-missing"))?;
    if nodes.next().is_some() { return Err(freeze("dynamic-expression/site-duplicate")); }
    Ok(index)
}

fn operand_at(
    frame: &CheckedExpressionEvaluationV1,
    values: &[Option<EvaluatedExpressionValueV1>],
    node: usize,
) -> Result<Operand, String> {
    let source = &frame.source().nodes()[node];
    if matches!(source.source(), Node::Literal(Literal::Null)) {
        return Ok(Operand::Null);
    }
    match values.get(node).and_then(Option::as_ref).ok_or_else(|| freeze("dynamic-expression/value-missing"))? {
        EvaluatedExpressionValueV1::Bool(value) => Ok(Operand::Bool(*value)),
        EvaluatedExpressionValueV1::Operand(value) => match value {
            StringInvokeOperand::NormalInteger(value) => Ok(Operand::Integer(*value)),
            StringInvokeOperand::Dynamic(value) => Ok(Operand::TaggedDynamic(*value)),
            StringInvokeOperand::DynamicCarrier(value) => Ok(Operand::Carrier(*value)),
            _ => Err(freeze("dynamic-expression/value-domain")),
        },
    }
}

fn freeze(reason: &str) -> String {
    format!("[freeze:contract][ordinary-new/dynamic-expression/{reason}]")
}
