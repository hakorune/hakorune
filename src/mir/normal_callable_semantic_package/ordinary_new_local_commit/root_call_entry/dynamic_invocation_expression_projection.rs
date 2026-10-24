//! Source-bound Dynamic invocation emitted inside one checked expression.
use super::expression_projection::EvaluatedExpressionValueV1;
use crate::mir::instruction::{
    DynamicInvocationOperandV1 as Operand, DynamicInvocationOperationV1 as Operation,
    InvokeOperation, StringInvokeOperand,
};
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
use crate::mir::normal_callable_semantic_package::LexicalCallArgumentProjectionV1;
use crate::mir::resolved_semantics::home_new_prefix::{
    CheckedExpressionEvaluationV1, LocalCallArgumentV1, LocalCallObservationV1,
    LocalCallResultSourceV1,
};
use crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1 as Node;
use crate::mir::{BasicBlockId, MirInstruction, ValueId};

type Binding = (BasicBlockId, MirInstruction);

/// One exact source call and its only physical Invoke/Normal result pair.
#[derive(Debug)]
pub(crate) struct EmittedDynamicInvocationCallProjectionV1 {
    source: LocalCallObservationV1,
    receiver_read: std::rc::Rc<crate::mir::builder::ExactLexicalReadV1>,
    arguments: Box<[LexicalCallArgumentProjectionV1]>,
    operation: Operation,
    invoke: Binding,
    projection: Binding,
}

impl EmittedDynamicInvocationCallProjectionV1 {
    pub(in crate::mir) fn operation_from_source_v1(
        frame: &CheckedExpressionEvaluationV1,
        source: &LocalCallObservationV1,
        arguments: &[LexicalCallArgumentProjectionV1],
        context: CallValidationContextV1<'_>,
        receiver: Operand,
    ) -> Result<Operation, String> {
        let Some(LocalCallResultSourceV1::DynamicInvocation(loan)) = source.result_source() else {
            return Err(freeze("dynamic-expression/source-loan"));
        };
        let target = loan.target();
        if !frame.calls().iter().any(|row| std::ptr::eq(row, source))
            || source.owner() != frame.source().owner()
            || source.result()
                != crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Dynamic
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
        if !receiver_source || !matches!(receiver, Operand::TaggedDynamic(_) | Operand::Carrier(_))
        {
            return Err(freeze("dynamic-expression/receiver-source"));
        }

        if arguments.len() != loan.argument_sites().len()
            || source.arguments().len() != loan.argument_sites().len()
        {
            return Err(freeze("dynamic-expression/argument-census"));
        }
        let mut operands = Vec::with_capacity(loan.argument_sites().len());
        for (index, ((site, original), argument)) in loan
            .argument_sites()
            .iter()
            .zip(target.arguments())
            .zip(arguments)
            .enumerate()
        {
            let actual = source.arguments().get(index).ok_or_else(|| freeze("dynamic-expression/argument-source"))?;
            if original.ordinal() as usize != index
                || original.site() != site
                || !matches!(actual, LocalCallArgumentV1::Expression { ordinal, site: actual_site }
                    if *ordinal as usize == index && actual_site == site)
            {
                return Err(freeze("dynamic-expression/argument-source"));
            }
            let value = argument.value_for_source_v1(actual, source.owner(), site, None, Some(context))?;
            operands.push(Operand::Integer(value));
        }
        Operation::from_source(loan, receiver, operands)
    }

    pub(in crate::mir) fn from_source_v1(
        frame: &CheckedExpressionEvaluationV1,
        source: &LocalCallObservationV1,
        receiver_read: crate::mir::builder::ExactLexicalReadV1,
        operation: Operation,
        arguments: Vec<LexicalCallArgumentProjectionV1>,
        context: CallValidationContextV1<'_>,
        invoke: Binding,
        projection: Binding,
    ) -> Result<Self, String> {
        let Some(LocalCallResultSourceV1::DynamicInvocation(loan)) = source.result_source() else {
            return Err(freeze("dynamic-expression/source-loan"));
        };
        let (receiver_site, receiver_binding, receiver_value) = receiver_read.source_value();
        if receiver_site
            != crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                source.owner(),
                loan.target().receiver_site().clone(),
            )
            || receiver_binding != loan.target().receiver_binding()
            || receiver_binding.owner() != source.owner()
        {
            return Err(freeze("dynamic-expression/receiver-read"));
        }
        let expected = Self::operation_from_source_v1(
            frame,
            source,
            &arguments,
            context,
            Operand::TaggedDynamic(receiver_value),
        )?;
        let MirInstruction::Invoke {
            operation: InvokeOperation::DynamicInvocation(actual),
            normal_landing,
            ..
        } = &invoke.1
        else {
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
        Ok(Self {
            source: source.clone(),
            receiver_read: std::rc::Rc::new(receiver_read),
            arguments: arguments.into_boxed_slice(),
            operation,
            invoke,
            projection,
        })
    }

    pub(crate) fn source(&self) -> &LocalCallObservationV1 {
        &self.source
    }
    pub(crate) fn operation(&self) -> &Operation {
        &self.operation
    }
    pub(crate) fn value(&self) -> ValueId {
        match self.projection.1 {
            MirInstruction::InvokeNormalResult { dst, .. } => dst,
            _ => unreachable!("validated Dynamic projection"),
        }
    }
    pub(crate) fn invoke_binding(&self) -> &Binding {
        &self.invoke
    }
    pub(crate) fn projection_binding(&self) -> &Binding {
        &self.projection
    }
    pub(crate) fn ordered_bindings(&self) -> [Binding; 2] {
        [self.invoke.clone(), self.projection.clone()]
    }

    pub(crate) fn scope_bindings_in_context_v1(
        &self,
        context: CallValidationContextV1<'_>,
    ) -> Result<(Vec<Binding>, Vec<Binding>), String> {
        let Some(LocalCallResultSourceV1::DynamicInvocation(loan)) = self.source.result_source()
        else {
            return Err(freeze("dynamic-expression/source-loan"));
        };
        if self.arguments.len() != loan.argument_sites().len()
            || self.source.arguments().len() != loan.argument_sites().len()
        {
            return Err(freeze("dynamic-expression/argument-census"));
        }
        let call_site = self.source.site();
        let mut ordered = Vec::new();
        let mut copies = self
            .receiver_read
            .original_copy_dependencies_v1()?
            .to_vec();
        for (ordinal, ((site, actual), argument)) in loan
            .argument_sites()
            .iter()
            .zip(self.source.arguments())
            .zip(self.arguments.iter())
            .enumerate()
        {
            if !matches!(actual, LocalCallArgumentV1::Expression { ordinal: source_ordinal, site: source_site }
                if *source_ordinal as usize == ordinal && source_site == site)
            {
                return Err(freeze("dynamic-expression/argument-source"));
            }
            let (argument_ordered, argument_copies) = argument
                .dynamic_call_argument_scope_bindings_v1(
                    actual,
                    call_site,
                    ordinal,
                    site,
                    context,
                )?;
            ordered.extend(argument_ordered);
            copies.extend(argument_copies);
        }
        ordered.extend(self.ordered_bindings());
        Ok((ordered, copies))
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
        let Some(LocalCallResultSourceV1::DynamicInvocation(loan)) = observed.result_source()
        else {
            return Err(freeze("dynamic-expression/source-loan"));
        };
        if receiver_site
            != crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                observed.owner(),
                loan.target().receiver_site().clone(),
            )
            || receiver_binding != loan.target().receiver_binding()
            || receiver_binding.owner() != observed.owner()
        {
            return Err(freeze("dynamic-expression/receiver-read"));
        }
        let expected = Self::operation_from_source_v1(
            frame,
            observed,
            &self.arguments,
            context,
            Operand::TaggedDynamic(receiver_value),
        )?;
        let index = unique_node(frame, observed.site().site())?;
        let result = values[index]
            .as_ref()
            .ok_or_else(|| freeze("dynamic-expression/result-value"))?
            .as_operand()
            .map_err(|error| error.clone())?;
        let expected_value = StringInvokeOperand::DynamicCarrier(self.value());
        let frame_source = frame
            .calls()
            .iter()
            .any(|source| std::ptr::eq(source, observed));
        if self.source != *observed
            || !frame_source
            || self.operation != expected
            || result != &expected_value
            || !original.contains(&self.invoke)
            || !original.contains(&self.projection)
            || self
                .ordered_bindings()
                .iter()
                .any(|binding| original.iter().filter(|row| *row == binding).count() != 1)
        {
            return Err(freeze("dynamic-expression/physical-source"));
        }
        for argument in self.arguments.iter() {
            argument.validate_expression_recorded_v1(original, None)?;
        }
        Ok(())
    }

    pub(crate) fn validate_recorded_projected_v1(
        &self,
        bindings: &[Binding],
        finishing: Option<&super::physical_boundary::FinishedBindings>,
    ) -> Result<(), String> {
        for argument in self.arguments.iter() {
            argument.validate_expression_recorded_v1(bindings, finishing)?;
        }
        for original in self.ordered_bindings() {
            let mapped = match finishing {
                Some(projection) => projection.binding(original.0, &original.1)?,
                None => Some(original.clone()),
            };
            if mapped.is_none_or(|mapped| {
                bindings
                    .iter()
                    .filter(|binding| **binding == mapped)
                    .count()
                    != 1
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
    let mut nodes = frame
        .source()
        .nodes()
        .iter()
        .enumerate()
        .filter(|(_, node)| node.site() == site);
    let (index, _) = nodes
        .next()
        .ok_or_else(|| freeze("dynamic-expression/site-missing"))?;
    if nodes.next().is_some() {
        return Err(freeze("dynamic-expression/site-duplicate"));
    }
    Ok(index)
}

fn freeze(reason: &str) -> String {
    format!("[freeze:contract][ordinary-new/dynamic-expression/{reason}]")
}
