//! One physical read observation; source proof stays at its original frame node.
use super::*;
use crate::mir::builder::{BindingValueObservationV2, ExactLexicalReadV1};
use crate::mir::instruction::StringInvokeOperand as Operand;
use crate::mir::resolved_semantics::home_new_prefix::{
    CheckedExpressionEvaluationV1, CheckedExpressionReferenceV1 as Reference,
};
use crate::mir::resolved_semantics::home_new_prefix::{
    CurrentScalarProducerV1 as Producer, CurrentScalarReadV1,
};
use crate::mir::{ConstValue, ValueId};

type Finished = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit::physical_boundary::FinishedBindings;
#[path = "expression_reference_nullable.rs"]
mod nullable;
#[path = "expression_reference_source.rs"]
mod source;
use source::*;
#[path = "expression_reference_dynamic.rs"]
mod dynamic;
use dynamic::*;
#[path = "expression_reference_branch.rs"]
mod branch;
use branch::validate_scalar_binding_observation_v1;
pub(in crate::mir) use branch::{
    BranchReadPhiObservationV1, BranchScalarPhiObservationV1, BranchTextPhiObservationV1,
};

#[derive(Debug, Clone)]
pub(in crate::mir) enum ReferenceProducerV1 {
    Definition(Binding),
    Entry(ValueId),
    Phi(std::rc::Rc<crate::mir::builder::LocalSsaPhysicalRelationV1>),
    Branch(std::rc::Rc<BranchScalarPhiObservationV1>),
}
impl ReferenceProducerV1 {
    pub(super) fn join_relation(
        &self,
    ) -> Option<&std::rc::Rc<crate::mir::builder::LocalSsaPhysicalRelationV1>> {
        match self {
            Self::Phi(point) => Some(point),
            Self::Branch(point) => Some(point.relation()),
            _ => None,
        }
    }
}
impl PartialEq for ReferenceProducerV1 {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Definition(a), Self::Definition(b)) => a == b,
            (Self::Entry(a), Self::Entry(b)) => a == b,
            (Self::Phi(a), Self::Phi(b)) => std::rc::Rc::ptr_eq(a, b),
            (Self::Branch(a), Self::Branch(b)) => std::rc::Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}
#[derive(Debug)]
pub(in crate::mir) struct ExpressionReferenceReadV1 {
    read: std::rc::Rc<ExactLexicalReadV1>,
    producer: ReferenceProducerV1,
}
pub(in crate::mir) struct PreparedExpressionReferenceReadV1 {
    producer: ReferenceProducerV1,
}
impl PreparedExpressionReferenceReadV1 {
    pub(in crate::mir) fn finish(self, read: ExactLexicalReadV1) -> ExpressionReferenceReadV1 {
        self.finish_shared(std::rc::Rc::new(read))
    }
    pub(in crate::mir) fn finish_shared(
        self,
        read: std::rc::Rc<ExactLexicalReadV1>,
    ) -> ExpressionReferenceReadV1 {
        ExpressionReferenceReadV1 {
            read,
            producer: self.producer,
        }
    }
}
impl ExpressionReferenceReadV1 {
    pub(in crate::mir) fn exact_read_v1(&self) -> &ExactLexicalReadV1 {
        &self.read
    }
    /// External initializer/Copy producers are dependencies, not newly emitted
    /// expression instructions. Only the sealed finisher approves omissions.
    pub(in crate::mir) fn validate_dependencies_v1(
        &self,
        frame: &CheckedExpressionEvaluationV1,
        node: usize,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit::physical_boundary::FinishedBindings>,
    ) -> Result<(), String> {
        self.validate_layout(frame, node, context, function)?;
        if let source @ (Reference::NullableResult(_)
        | Reference::Alias(_)
        | Reference::DynamicInvocationResult(_)) = original_reference(frame, node)?
        {
            self.validate_source_reference_v1(
                frame.source(),
                node,
                source,
                context,
                function,
                finishing,
            )?;
        }
        if let Reference::Scalar(read) = original_reference(frame, node)? {
            if let Some(source) = read.original_read() {
                if matches!(source.producer(), Producer::BranchNormal(_)) {
                    self.read
                        .loan_scalar_join_v1(source.producer())?
                        .branch()
                        .ok_or_else(|| freeze("expression/scalar-producer-kind"))?
                        .validate(context, function, finishing)?;
                }
            }
        }
        if let Reference::LoopText(source) = original_reference(frame, node)? {
            if matches!(source.producer(), crate::mir::resolved_semantics::home_new_prefix::SourceLoopTextProducerV1::BranchNormal(_)) {
                self.read.binding_observation_v2().loan_text_branch_v1(source.producer())?
                    .validate(context, function, finishing)?;
            }
        }
        if let Some(point) = self.producer.join_relation() {
            if let Some(projection) = finishing {
                if matches!(context, CallValidationContextV1::Finalized(_)) {
                    point.require_completed_v1()?;
                    point.verify_projected(function, &mut |row| {
                        projection.relation_binding(function, row)
                    })?;
                }
            } else {
                point.verify(function)?;
            }
        }
        if let (Some(projection), CallValidationContextV1::Finalized(_)) = (finishing, context) {
            return self.validate_finished_dependencies_v1(function, projection);
        }
        for original in self.dependencies()? {
            match (finishing, context) {
                (Some(projection), CallValidationContextV1::Finalized(_)) => {
                    projection.relation_binding(function, &original)?;
                }
                (Some(projection), CallValidationContextV1::Lowering(_)) => {
                    let dst = original
                        .1
                        .dst_value()
                        .ok_or_else(|| freeze("expression/reference-dependency-dst"))?;
                    if projection.captured_definition(dst)? != &original {
                        return Err(freeze("expression/reference-captured-dependency"));
                    }
                }
                (None, _) => {
                    if !super::super::super::physical_boundary::check_binding(
                        function,
                        None,
                        original.0,
                        &original.1,
                    )? {
                        return Err(freeze("expression/reference-dependency-missing"));
                    }
                }
            }
        }
        Ok(())
    }
    pub(in crate::mir) fn validate_finished_dependencies_v1(
        &self,
        function: &MirFunction,
        projection: &crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit::physical_boundary::FinishedBindings,
    ) -> Result<(), String> {
        if let Some(point) = self.producer.join_relation() {
            point.require_completed_v1()?;
            point.verify_projected(function, &mut |row| {
                projection.relation_binding(function, row)
            })?;
        }
        for original in self.dependencies()? {
            projection.relation_binding(function, &original)?;
        }
        Ok(())
    }
    /// Prepare before the existing exact-read batch commits. Source proof stays
    /// in the shared evaluation, not in another reference/operand registry.
    pub(in crate::mir) fn prepare(
        frame: &CheckedExpressionEvaluationV1,
        node: usize,
        read: &ExactLexicalReadV1,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
    ) -> Result<PreparedExpressionReferenceReadV1, String> {
        let source = original_reference(frame, node)?;
        let producer = match source {
            Reference::Scalar(_)
            | Reference::FiniteEntry { .. }
            | Reference::DynamicEntry(_)
            | Reference::Alias(_)
            | Reference::NullableResult(_)
            | Reference::DynamicInvocationResult(_) => {
                return Self::prepare_source_reference_v1(
                    frame.source(),
                    node,
                    source,
                    read,
                    context,
                    function,
                )
            }
            Reference::StaticText(original) => {
                let value = read
                    .binding_observation_v2()
                    .loan_root_initializer_v1(original.definition_binding())?;
                context
                    .ledger()
                    .with_function_geometry_v1(function, |geometry| {
                        let (block, _, row) = geometry
                            .definition(value)
                            .ok_or_else(|| freeze("expression/static-initializer-missing"))?;
                        if geometry.instruction_definition_count(value) != 1 {
                            return Err(freeze("expression/static-initializer-producer"));
                        }
                        Ok(ReferenceProducerV1::Definition((block, row.clone())))
                    })?
            }
            Reference::Residence(original) => {
                let binding = context.with_text_residence_v1(
                    original.acquisition(),
                    read,
                    |_, binding| Ok(binding.clone()),
                )?;
                ReferenceProducerV1::Definition(binding)
            }
            Reference::LoopText(original) => ReferenceProducerV1::Phi(
                loop_text_reference_relation_v1(frame, node, original, read, context)?,
            ),
            Reference::LoopDynamic(original) => {
                ReferenceProducerV1::Phi(loop_dynamic_reference_relation_v1(
                    frame.source(),
                    node,
                    original,
                    read,
                    context,
                    function,
                )?)
            }
            Reference::ArgumentEntry(requirement) => ReferenceProducerV1::Entry(prepare_entry(
                requirement,
                None,
                read,
                context,
                function,
            )?),
        };
        reference_operand(frame, node, read, &producer, context)?;
        if !matches!(
            source,
            Reference::ArgumentEntry(_) | Reference::FiniteEntry { .. }
        ) {
            validate_dependencies(read.binding_observation_v2(), &producer, context, function)?;
        }
        Ok(PreparedExpressionReferenceReadV1 { producer })
    }
    pub(in crate::mir) fn operand(
        &self,
        frame: &CheckedExpressionEvaluationV1,
        node: usize,
        context: CallValidationContextV1<'_>,
    ) -> Result<Operand, String> {
        reference_operand(frame, node, &self.read, &self.producer, context)
    }
    pub(super) fn validate_layout(
        &self,
        frame: &CheckedExpressionEvaluationV1,
        node: usize,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
    ) -> Result<(), String> {
        if let Reference::Scalar(source) = original_reference(frame, node)? {
            if matches!(self.producer, ReferenceProducerV1::Phi(_)) {
                validate_source(
                    frame.source(),
                    source
                        .original_read()
                        .ok_or_else(|| freeze("expression/reference-scalar-obligation"))?,
                    &self.read,
                    &self.producer,
                    context,
                )?;
            }
            let original = source
                .original_read()
                .ok_or_else(|| freeze("expression/reference-scalar-obligation"))?;
            validate_entry_layout(
                original.binding(),
                original.producer(),
                &self.producer,
                context,
                function,
            )?;
        }
        if let Reference::LoopText(original) = original_reference(frame, node)? {
            let relation =
                loop_text_reference_relation_v1(frame, node, original, &self.read, context)?;
            if !matches!(&self.producer, ReferenceProducerV1::Phi(point) if std::rc::Rc::ptr_eq(point, &relation))
            {
                return Err(freeze("source-loop/text-point-binding"));
            }
        }
        if let Reference::LoopDynamic(original) = original_reference(frame, node)? {
            let relation = loop_dynamic_reference_relation_v1(
                frame.source(),
                node,
                original,
                &self.read,
                context,
                function,
            )?;
            if !matches!(&self.producer, ReferenceProducerV1::Phi(point) if std::rc::Rc::ptr_eq(point, &relation))
            {
                return Err(freeze("source-loop/dynamic-point-binding"));
            }
        }
        let entry = match original_reference(frame, node)? {
            Reference::ArgumentEntry(required) => Some(prepare_entry(
                required, None, &self.read, context, function,
            )?),
            Reference::FiniteEntry { read, .. } => Some(prepare_entry(
                read.requirement(),
                Some(read),
                &self.read,
                context,
                function,
            )?),
            Reference::DynamicEntry(read) => {
                Some(prepare_dynamic_entry(read, &self.read, context, function)?)
            }
            _ => None,
        };
        if entry.is_some_and(|entry| !matches!(self.producer, ReferenceProducerV1::Entry(original) if original == entry)) {
            return Err(freeze("expression/reference-text-entry-producer"));
        }
        Ok(())
    }
    pub(in crate::mir) fn definition(&self) -> Option<&Binding> {
        match &self.producer {
            ReferenceProducerV1::Definition(binding) => Some(binding),
            _ => None,
        }
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn original_copy_dependencies_v1(
        &self,
    ) -> Result<&[Binding], String> {
        self.read.original_copy_dependencies_v1()
    }
    pub(in crate::mir) fn dependencies(&self) -> Result<std::vec::IntoIter<Binding>, String> {
        let mut rows: Vec<_> = self.definition().cloned().into_iter().collect();
        if let Some(point) = self.producer.join_relation() {
            point.append_observed_bindings_v1(&mut rows);
        }
        rows.extend_from_slice(self.read.original_copy_dependencies_v1()?);
        Ok(rows.into_iter())
    }
    /// A parent Loop owns its indexed Phi once. Reads keep the full dependency,
    /// but the child cannot claim that Phi as its own emitted instruction.
    pub(in crate::mir) fn without_borrowed_phis_v2<'a>(
        bindings: Vec<Binding>,
        reads: impl Iterator<Item = &'a Self>,
    ) -> Result<Vec<Binding>, String> {
        let reads: Vec<_> = reads.collect();
        let mut own = Vec::new();
        for binding in bindings {
            let mut borrowed = false;
            if matches!(binding.1, MirInstruction::Phi { .. }) {
                for read in &reads {
                    if let Some(point) = read.producer.join_relation() {
                        if point
                            .resolve_observed_binding_v1(&binding, false)?
                            .is_some()
                        {
                            borrowed = true;
                        }
                    }
                }
            }
            if !borrowed {
                own.push(binding);
            }
        }
        Ok(own)
    }
    /// A source Loop owns its indexed Header Phi once. This second pass accepts
    /// only relations already lent by exact LoopHeader reads in the same
    /// checked source evaluation; ordinary branch and unrelated Phi rows stay
    /// in the child inventory.
    pub(in crate::mir) fn without_borrowed_source_loop_phis_v2(
        bindings: Vec<Binding>,
        relations: &[std::rc::Rc<crate::mir::builder::LocalSsaPhysicalRelationV1>],
    ) -> Result<Vec<Binding>, String> {
        bindings
            .into_iter()
            .try_fold(Vec::new(), |mut own, binding| {
                let borrowed = if matches!(binding.1, MirInstruction::Phi { .. }) {
                    let mut borrowed = false;
                    for relation in relations {
                        if relation
                            .resolve_observed_binding_v1(&binding, false)?
                            .is_some()
                        {
                            borrowed = true;
                            break;
                        }
                    }
                    borrowed
                } else {
                    false
                };
                if !borrowed {
                    own.push(binding);
                }
                Ok(own)
            })
    }
    /// Reuse the RHS's already checked producer. An alias Copy is itself the
    /// result definition; self-assignment of an entry has no new instruction.
    pub(in crate::mir) fn result_producer_v1(
        &self,
        value: ValueId,
    ) -> Result<ReferenceProducerV1, String> {
        if self.read.source_value().2 != value {
            return Err(freeze("rebind-expression/rhs-read-value"));
        }
        if let Some(copy) = self.read.original_copy_dependencies_v1()?.last() {
            if copy.1.dst_value() == Some(value) {
                return Ok(ReferenceProducerV1::Definition(copy.clone()));
            }
        }
        match &self.producer {
            ReferenceProducerV1::Definition(binding) if binding.1.dst_value() == Some(value) => {
                Ok(self.producer.clone())
            }
            ReferenceProducerV1::Entry(entry) if *entry == value => Ok(self.producer.clone()),
            _ if self
                .producer
                .join_relation()
                .is_some_and(|point| point.values().0 == value) =>
            {
                Ok(self.producer.clone())
            }
            _ => Err(freeze("rebind-expression/rhs-producer")),
        }
    }
    pub(in crate::mir) fn prepare_scalar_read_v2(
        plan: &std::rc::Rc<
            crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1,
        >,
        source: &CurrentScalarReadV1,
        read: &ExactLexicalReadV1,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
    ) -> Result<PreparedExpressionReferenceReadV1, String> {
        let producer = prepare_scalar_binding_producer(
            source.binding(),
            source.producer(),
            read.binding_observation_v2(),
            context,
            function,
        )?;
        validate_source(plan, source, read, &producer, context)?;
        Ok(PreparedExpressionReferenceReadV1 { producer })
    }
    /// A terminal source read uses the same producer/Copy/Phi verifier without
    /// manufacturing an expression frame or consuming another lexical read.
    pub(in crate::mir) fn scalar_read_value_v2(
        &self,
        plan: &std::rc::Rc<
            crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1,
        >,
        source: &CurrentScalarReadV1,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
    ) -> Result<ValueId, String> {
        self.validate_scalar_read_v2(plan, source, context, function, None)
    }
    pub(in crate::mir::normal_callable_semantic_package) fn validate_scalar_read_v2(
        &self,
        plan: &std::rc::Rc<
            crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1,
        >,
        source: &CurrentScalarReadV1,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&Finished>,
    ) -> Result<ValueId, String> {
        let value = validate_source(plan, source, &self.read, &self.producer, context)?;
        validate_scalar_binding_observation_v1(
            source.binding(),
            source.producer(),
            self.read.binding_observation_v2(),
            &self.producer,
            context,
            function,
            finishing,
        )?;
        Ok(value)
    }
    /// The same value reference serves eager RHS and Bool control purposes.
    /// Bool remains a scalar value, never a StringInvokeOperand or I64 cast.
    pub(in crate::mir) fn prepare_source_reference_v1(
        plan: &std::rc::Rc<
            crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1,
        >,
        node: usize,
        source: &Reference,
        read: &ExactLexicalReadV1,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
    ) -> Result<PreparedExpressionReferenceReadV1, String> {
        if !source.matches_node(plan, node) {
            return Err(freeze("expression/reference-source-node"));
        }
        match source {
            Reference::Alias(_) => {
                read.value_for(
                    plan.owner(),
                    plan.nodes()[node].site().node(),
                    source.binding(),
                )
                .map_err(|error| error.to_string())?;
                let (initialized, _) = read.loan_local_initializer_v1(source.binding())?;
                let producer = definition_for_initializer(initialized, function, context)?;
                validate_dependencies(read.binding_observation_v2(), &producer, context, function)?;
                Ok(PreparedExpressionReferenceReadV1 { producer })
            }
            Reference::NullableResult(frame) => {
                let (_, normal) = context.nullable_result_read_v1(frame, read, function, None)?;
                let producer = ReferenceProducerV1::Definition(normal);
                validate_dependencies(read.binding_observation_v2(), &producer, context, function)?;
                Ok(PreparedExpressionReferenceReadV1 { producer })
            }
            Reference::DynamicInvocationResult(initializer) => {
                let (_, binding) = dynamic_initializer_result_producer_v1(
                    initializer,
                    plan.owner(),
                    plan.nodes()[node].site().node(),
                    read,
                    context,
                )?;
                let producer = ReferenceProducerV1::Definition(binding);
                validate_dependencies(read.binding_observation_v2(), &producer, context, function)?;
                Ok(PreparedExpressionReferenceReadV1 { producer })
            }
            Reference::Scalar(source) => Self::prepare_scalar_read_v2(
                plan,
                source
                    .original_read()
                    .ok_or_else(|| freeze("expression/reference-scalar-obligation"))?,
                read,
                context,
                function,
            ),
            Reference::FiniteEntry { read: original, .. } => {
                Ok(PreparedExpressionReferenceReadV1 {
                    producer: ReferenceProducerV1::Entry(prepare_entry(
                        original.requirement(),
                        Some(original),
                        read,
                        context,
                        function,
                    )?),
                })
            }
            Reference::DynamicEntry(original) => Ok(PreparedExpressionReferenceReadV1 {
                producer: ReferenceProducerV1::Entry(prepare_dynamic_entry(
                    original, read, context, function,
                )?),
            }),
            _ => Err(freeze("expression/reference-source-node")),
        }
    }
    pub(in crate::mir) fn source_reference_value_v1(
        &self,
        plan: &std::rc::Rc<
            crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1,
        >,
        node: usize,
        source: &Reference,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
    ) -> Result<ValueId, String> {
        self.validate_source_reference_v1(plan, node, source, context, function, None)
    }
    pub(in crate::mir::normal_callable_semantic_package) fn validate_source_reference_v1(
        &self,
        plan: &std::rc::Rc<
            crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1,
        >,
        node: usize,
        source: &Reference,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&Finished>,
    ) -> Result<ValueId, String> {
        if !source.matches_node(plan, node) {
            return Err(freeze("expression/reference-source-node"));
        }
        match source {
            Reference::Alias(_) => {
                let value = self
                    .read
                    .value_for(
                        plan.owner(),
                        plan.nodes()[node].site().node(),
                        source.binding(),
                    )
                    .map_err(|error| error.to_string())?;
                let (initialized, _) = self.read.loan_local_initializer_v1(source.binding())?;
                let expected = match finishing {
                    Some(finishing) => ReferenceProducerV1::Definition(
                        finishing.captured_definition(initialized)?.clone(),
                    ),
                    None => definition_for_initializer(initialized, function, context)?,
                };
                if self.producer != expected {
                    return Err(freeze("expression/reference-source-node"));
                }
                if let Some(finishing) = finishing {
                    self.validate_finished_dependencies_v1(function, finishing)?;
                }
                Ok(value)
            }
            Reference::NullableResult(frame) => {
                let (value, normal) =
                    context.nullable_result_read_v1(frame, &self.read, function, finishing)?;
                if !matches!(&self.producer, ReferenceProducerV1::Definition(original) if original == &normal)
                {
                    return Err(freeze("expression/reference-source-node"));
                }
                self.read
                    .value_for(
                        plan.owner(),
                        plan.nodes()[node].site().node(),
                        frame.binding(),
                    )
                    .map_err(|error| error.to_string())?;
                if let Some(finishing) = finishing {
                    self.validate_finished_dependencies_v1(function, finishing)?;
                }
                Ok(value)
            }
            Reference::DynamicInvocationResult(initializer) => {
                let (value, binding) = dynamic_initializer_result_producer_v1(
                    initializer,
                    plan.owner(),
                    plan.nodes()[node].site().node(),
                    &self.read,
                    context,
                )?;
                if !matches!(&self.producer, ReferenceProducerV1::Definition(original) if original == &binding)
                {
                    return Err(freeze("expression/dynamic-initializer-producer"));
                }
                if let Some(finishing) = finishing {
                    self.validate_finished_dependencies_v1(function, finishing)?;
                }
                Ok(value)
            }
            Reference::Scalar(source) => self.validate_scalar_read_v2(
                plan,
                source
                    .original_read()
                    .ok_or_else(|| freeze("expression/reference-scalar-obligation"))?,
                context,
                function,
                finishing,
            ),
            Reference::FiniteEntry { read, .. } => {
                let entry = prepare_entry(
                    read.requirement(),
                    Some(read),
                    &self.read,
                    context,
                    function,
                )?;
                if !matches!(self.producer, ReferenceProducerV1::Entry(original) if original == entry)
                {
                    return Err(freeze("expression/reference-text-entry-producer"));
                }
                if let Some(finishing) = finishing {
                    self.validate_finished_dependencies_v1(function, finishing)?;
                }
                entry_value(read.requirement(), &self.read, &self.producer, context)
            }
            Reference::DynamicEntry(read) => {
                let entry = prepare_dynamic_entry(read, &self.read, context, function)?;
                if !matches!(self.producer, ReferenceProducerV1::Entry(original) if original == entry)
                {
                    return Err(freeze("expression/reference-dynamic-entry-producer"));
                }
                if let Some(finishing) = finishing {
                    self.validate_finished_dependencies_v1(function, finishing)?;
                }
                dynamic_entry_value(read.requirement(), &self.read, &self.producer)
            }
            Reference::LoopDynamic(original) => {
                let relation = loop_dynamic_reference_relation_v1(
                    plan, node, original, &self.read, context, function,
                )?;
                if !matches!(&self.producer, ReferenceProducerV1::Phi(point) if std::rc::Rc::ptr_eq(point, &relation))
                {
                    return Err(freeze("source-loop/dynamic-point-binding"));
                }
                let value = self
                    .read
                    .value_for(
                        plan.owner(),
                        plan.nodes()[node].site().node(),
                        original.binding(),
                    )
                    .map_err(|error| error.to_string())?;
                if relation.values().0 != value {
                    return Err(freeze("source-loop/dynamic-point-value"));
                }
                if let Some(finishing) = finishing {
                    self.validate_finished_dependencies_v1(function, finishing)?;
                }
                Ok(value)
            }
            _ => Err(freeze("expression/reference-source-node")),
        }
    }
}

#[path = "expression_binding_provenance.rs"]
mod binding_provenance;
pub(in crate::mir) use binding_provenance::ScalarBindingReadProofV1;
