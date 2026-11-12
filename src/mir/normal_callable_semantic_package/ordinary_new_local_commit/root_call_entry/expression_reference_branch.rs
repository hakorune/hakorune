//! Branch value observations over the existing source and physical join owners.
#[path = "expression_branch_geometry.rs"]
mod geometry;
use super::*;
use geometry::validate_branch_geometry_v1;

/// The original IfForm observation and its sole indexed Phi. This belongs to
/// the same value purpose; it is not another CFG writer or source registry.
#[derive(Debug)]
pub(in crate::mir) struct BranchScalarPhiObservationV1 {
    source:
        std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::BranchNormalScalarSourceV1>,
    pending: std::rc::Rc<crate::mir::builder::IfConditionMaterializationV1>,
    branch: Binding,
    exits: [BasicBlockId; 2],
    jumps: [Binding; 2],
    inputs: [BindingValueObservationV2; 2],
    producers: [ReferenceProducerV1; 2],
    relation: std::rc::Rc<crate::mir::builder::LocalSsaPhysicalRelationV1>,
}

/// Exact physical observation for the ordered Normal inputs of a source Text
/// branch join. This is separate from Loop Header/After Text point purpose.
#[derive(Debug)]
pub(in crate::mir) struct BranchTextPhiObservationV1 {
    source: std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::BranchNormalTextSourceV1>,
    pending: std::rc::Rc<crate::mir::builder::IfConditionMaterializationV1>,
    branch: Binding,
    exits: [BasicBlockId; 2],
    jumps: [Binding; 2],
    inputs: [BindingValueObservationV2; 2],
    relation: std::rc::Rc<crate::mir::builder::LocalSsaPhysicalRelationV1>,
}
/// A non-owning branch value retains every original incoming and the SAME
/// indexed SSA writer. Neither a Home nor a representative Call is issued.
#[derive(Debug)]
enum ReadPhiSourceV1 {
    Single {
        source: crate::mir::resolved_semantics::home_new_prefix::BranchCallResultSourceV1,
        loan: crate::mir::normal_callable_semantic_package::BorrowedNullableLocalSourceLoanV1,
    },
    Flow(std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::BorrowedReadFlowV1>),
}
#[derive(Debug)]
pub(in crate::mir) struct BranchReadPhiObservationV1 {
    source: ReadPhiSourceV1,
    pending: std::rc::Rc<crate::mir::builder::IfConditionMaterializationV1>,
    branch: Binding,
    exits: [BasicBlockId; 2],
    jumps: [Binding; 2],
    inputs: [BindingValueObservationV2; 2],
    relation: std::rc::Rc<crate::mir::builder::LocalSsaPhysicalRelationV1>,
}
impl BranchReadPhiObservationV1 {
    pub(in crate::mir) fn binding(&self) -> crate::mir::resolved_semantics::BindingRefV1 {
        match &self.source {
            ReadPhiSourceV1::Single { source, .. } => source.binding(),
            ReadPhiSourceV1::Flow(source) => source.binding(),
        }
    }
    pub(in crate::mir) fn matches_read_v1(
        &self,
        read: &crate::mir::resolved_semantics::home_new_prefix::BorrowedResultReadV1,
    ) -> bool {
        matches!(&self.source, ReadPhiSourceV1::Single { source, .. } if source.matches_read_v1(read))
    }
    pub(in crate::mir) fn relation(
        &self,
    ) -> &std::rc::Rc<crate::mir::builder::LocalSsaPhysicalRelationV1> {
        &self.relation
    }
    fn input_loans_v1(
        &self,
    ) -> Result<
        Vec<&crate::mir::normal_callable_semantic_package::BorrowedNullableLocalSourceLoanV1>,
        String,
    > {
        let mut loans = Vec::with_capacity(self.inputs.len());
        for (index, input) in self.inputs.iter().enumerate() {
            if input.binding_value().0 != self.binding() {
                return Err(freeze("expression/reference-source-node"));
            }
            let (loan, expression) = match &self.source {
                ReadPhiSourceV1::Single { source, loan } => {
                    (loan, source.incoming_expression_v1(index))
                }
                ReadPhiSourceV1::Flow(source) => {
                    let incoming = source
                        .incoming_loans(index)
                        .ok_or_else(|| freeze("expression/reference-source-node"))?;
                    if incoming.len() != 1 {
                        return Err(freeze("expression/reference-source-node"));
                    }
                    (incoming[0], source.incoming_expression(index))
                }
            };
            if !loan.is_declared_read_v1() {
                return Err(freeze("expression/reference-source-node"));
            }
            if let Some(frame) = input.source_declared_read_rebind_v1()? {
                if expression != frame.source().nodes().first().map(|node| node.site())
                    || frame
                        .declared_read_v1()
                        .is_none_or(|read| read.source() != loan)
                {
                    return Err(freeze("expression/reference-source-node"));
                }
            }
            loans.push(loan);
        }
        Ok(loans)
    }
    pub(in crate::mir) fn original_pair_v1(
        &self,
        loan: &crate::mir::normal_callable_semantic_package::BorrowedNullableLocalSourceLoanV1,
        context: CallValidationContextV1<'_>,
    ) -> Result<[Binding; 2], String> {
        let ReadPhiSourceV1::Single {
            source,
            loan: original_loan,
        } = &self.source
        else {
            return Err(freeze("expression/reference-source-node"));
        };
        if original_loan != loan
            || source.producer() != loan.call()
            || source.destination() != loan.destination()
            || source.incoming().is_none()
        {
            return Err(freeze("expression/reference-source-node"));
        }
        let mut original = None;
        for (input, expected) in self.inputs.iter().zip(self.input_loans_v1()?) {
            let (_, pair) = context.declared_result_binding_v1(expected, input)?;
            if original.as_ref().is_some_and(|previous| previous != &pair) {
                return Err(freeze("expression/reference-source-node"));
            }
            original = Some(pair);
        }
        original.ok_or_else(|| freeze("expression/reference-source-node"))
    }
    pub(in crate::mir) fn flow_definition_v1(
        &self,
        read: &crate::mir::resolved_semantics::home_new_prefix::BorrowedFlowReadV1,
        context: CallValidationContextV1<'_>,
    ) -> Result<Binding, String> {
        let ReadPhiSourceV1::Flow(source) = &self.source else {
            return Err(freeze("expression/reference-source-node"));
        };
        if source != read.source() {
            return Err(freeze("expression/reference-source-node"));
        }
        for (input, loan) in self.inputs.iter().zip(self.input_loans_v1()?) {
            context.declared_result_binding_v1(loan, input)?;
        }
        self.relation.identity_phi_binding_v1().cloned()
    }
    pub(in crate::mir) fn corroborates_phi_v1(
        &self,
        loan: &crate::mir::normal_callable_semantic_package::BorrowedNullableLocalSourceLoanV1,
        row: &Binding,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: &Finished,
    ) -> Result<bool, String> {
        if !self
            .input_loans_v1()?
            .into_iter()
            .any(|original| original == loan)
            || finishing
                .relation_binding(function, self.relation.identity_phi_binding_v1()?)?
                .as_ref()
                != Some(row)
        {
            return Ok(false);
        }
        self.validate_v1(context, function, Some(finishing))?;
        Ok(true)
    }
    pub(in crate::mir) fn validate_v1(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&Finished>,
    ) -> Result<(), String> {
        if let ReadPhiSourceV1::Single { loan, .. } = &self.source {
            self.original_pair_v1(loan, context)?;
        }
        validate_branch_geometry_v1(
            &self.pending,
            &self.branch,
            self.exits,
            &self.jumps,
            self.inputs.each_ref().map(|input| input.binding_value().1),
            &self.relation,
            &crate::mir::verification::geometry::FunctionVerificationGeometry::new(function),
            finishing,
            "expression/reference-source-node",
        )?;
        for (input, loan) in self.inputs.iter().zip(self.input_loans_v1()?) {
            context.validate_declared_read_transport_v1(loan, input, function, finishing)?;
        }
        Ok(())
    }
    pub(super) fn append_read_dependencies_v1(
        &self,
        context: CallValidationContextV1<'_>,
        rows: &mut Vec<Binding>,
    ) -> Result<(), String> {
        rows.extend_from_slice(self.pending.bindings());
        self.pending.relation().append_observed_bindings_v1(rows);
        rows.push(self.branch.clone());
        rows.extend_from_slice(&self.jumps);
        for (input, loan) in self.inputs.iter().zip(self.input_loans_v1()?) {
            context.append_declared_read_dependencies_v1(loan, input, rows)?;
        }
        self.relation.append_observed_bindings_v1(rows);
        Ok(())
    }
    pub(in crate::mir) fn prepare_v1(
        source: crate::mir::resolved_semantics::home_new_prefix::BranchCallResultSourceV1,
        loan: crate::mir::normal_callable_semantic_package::BorrowedNullableLocalSourceLoanV1,
        pending: std::rc::Rc<crate::mir::builder::IfConditionMaterializationV1>,
        branch: Binding,
        exits: [Option<BasicBlockId>; 2],
        inputs: [BindingValueObservationV2; 2],
        relation: std::rc::Rc<crate::mir::builder::LocalSsaPhysicalRelationV1>,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
    ) -> Result<std::rc::Rc<Self>, String> {
        Self::prepare_source_v1(
            ReadPhiSourceV1::Single { source, loan },
            pending,
            branch,
            exits,
            inputs,
            relation,
            context,
            function,
        )
    }
    pub(in crate::mir) fn prepare_flow_v1(
        source: std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::BorrowedReadFlowV1>,
        pending: std::rc::Rc<crate::mir::builder::IfConditionMaterializationV1>,
        branch: Binding,
        exits: [Option<BasicBlockId>; 2],
        inputs: [BindingValueObservationV2; 2],
        relation: std::rc::Rc<crate::mir::builder::LocalSsaPhysicalRelationV1>,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
    ) -> Result<std::rc::Rc<Self>, String> {
        Self::prepare_source_v1(
            ReadPhiSourceV1::Flow(source),
            pending,
            branch,
            exits,
            inputs,
            relation,
            context,
            function,
        )
    }
    fn prepare_source_v1(
        source: ReadPhiSourceV1,
        pending: std::rc::Rc<crate::mir::builder::IfConditionMaterializationV1>,
        branch: Binding,
        exits: [Option<BasicBlockId>; 2],
        inputs: [BindingValueObservationV2; 2],
        relation: std::rc::Rc<crate::mir::builder::LocalSsaPhysicalRelationV1>,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
    ) -> Result<std::rc::Rc<Self>, String> {
        let [Some(left), Some(right)] = exits else {
            return Err(freeze("expression/reference-source-node"));
        };
        let jump = |block| {
            function
                .blocks
                .get(&block)
                .and_then(|body| body.terminator.as_ref())
                .filter(|row| matches!(row, MirInstruction::Jump { .. }))
                .map(|row| (block, row.clone()))
                .ok_or_else(|| freeze("expression/reference-source-node"))
        };
        let observed = std::rc::Rc::new(Self {
            source,
            pending,
            branch,
            exits: [left, right],
            jumps: [jump(left)?, jump(right)?],
            inputs,
            relation,
        });
        observed.validate_v1(context, function, None)?;
        Ok(observed)
    }
}

impl BranchTextPhiObservationV1 {
    pub(in crate::mir) fn pending(
        &self,
    ) -> &std::rc::Rc<crate::mir::builder::IfConditionMaterializationV1> {
        &self.pending
    }
    pub(in crate::mir) fn source(
        &self,
    ) -> &std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::BranchNormalTextSourceV1>
    {
        &self.source
    }
    pub(in crate::mir) fn relation(
        &self,
    ) -> &std::rc::Rc<crate::mir::builder::LocalSsaPhysicalRelationV1> {
        &self.relation
    }
    pub(super) fn validate(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&Finished>,
    ) -> Result<(), String> {
        context
            .ledger()
            .require_branch_text_source_v1(&self.source)?;
        validate_branch_geometry_v1(
            &self.pending,
            &self.branch,
            self.exits,
            &self.jumps,
            [
                self.inputs[0].binding_value().1,
                self.inputs[1].binding_value().1,
            ],
            &self.relation,
            &crate::mir::verification::geometry::FunctionVerificationGeometry::new(function),
            finishing,
            "expression/text-producer-kind",
        )?;
        for (index, (_, _, source)) in self.source.incoming().iter().enumerate() {
            ExpressionReferenceReadV1::validate_text_binding_with_finishing_v2(
                self.source.binding(),
                source,
                &self.inputs[index],
                context,
                function,
                finishing,
            )?;
        }
        Ok(())
    }
}

impl BranchScalarPhiObservationV1 {
    pub(in crate::mir) fn pending(
        &self,
    ) -> &std::rc::Rc<crate::mir::builder::IfConditionMaterializationV1> {
        &self.pending
    }
    pub(in crate::mir) fn source(
        &self,
    ) -> &std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::BranchNormalScalarSourceV1>
    {
        &self.source
    }
    pub(in crate::mir) fn relation(
        &self,
    ) -> &std::rc::Rc<crate::mir::builder::LocalSsaPhysicalRelationV1> {
        &self.relation
    }
    pub(in crate::mir) fn validate(
        self: &std::rc::Rc<Self>,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&Finished>,
    ) -> Result<(), String> {
        context.ledger().with_physical_geometry_v1(
            function,
            finishing,
            matches!(context, CallValidationContextV1::Finalized(_)),
            |geometry, completed| {
                // A query borrows exactly one immutable MIR/context/projection. Every
                // read edge is checked; only completed shared observation bodies reuse
                // that query's successful geometry/children check.
                let mut pending = vec![(std::rc::Rc::clone(self), false)];
                let mut active = std::collections::BTreeSet::new();

                while let Some((point, exiting)) = pending.pop() {
                    let key = std::rc::Rc::as_ptr(&point);
                    if exiting {
                        active.remove(&key);
                        completed.borrow_mut().insert(key, point);
                        continue;
                    }
                    if completed.borrow().contains_key(&key) {
                        continue;
                    }
                    if !active.insert(key) {
                        return Err(freeze("expression/scalar-producer-kind"));
                    }
                    context
                        .ledger()
                        .require_branch_scalar_source_v1(&point.source)?;
                    validate_branch_geometry_v1(
                        &point.pending,
                        &point.branch,
                        point.exits,
                        &point.jumps,
                        [
                            point.inputs[0].binding_value().1,
                            point.inputs[1].binding_value().1,
                        ],
                        &point.relation,
                        geometry,
                        finishing,
                        "expression/scalar-producer-kind",
                    )?;
                    let mut children = Vec::new();
                    for (index, (_, _, source)) in point.source.incoming().iter().enumerate() {
                        validate_scalar_binding_observation_with_branches_v1(
                            point.source.binding(),
                            source,
                            &point.inputs[index],
                            &point.producers[index],
                            context,
                            function,
                            finishing,
                            &mut |child| {
                                children.push(std::rc::Rc::clone(child));
                                Ok(())
                            },
                        )?;
                    }
                    pending.push((point, true));
                    pending.extend(children.into_iter().rev().map(|child| (child, false)));
                }
                Ok(())
            },
        )
    }
}
pub(super) fn validate_scalar_binding_observation_v1(
    binding: crate::mir::resolved_semantics::BindingRefV1,
    source: &Producer,
    read: &BindingValueObservationV2,
    producer: &ReferenceProducerV1,
    context: CallValidationContextV1<'_>,
    function: &MirFunction,
    finishing: Option<&Finished>,
) -> Result<(), String> {
    validate_scalar_binding_observation_with_branches_v1(
        binding,
        source,
        read,
        producer,
        context,
        function,
        finishing,
        &mut |point| point.validate(context, function, finishing),
    )
}

pub(super) fn validate_scalar_binding_observation_with_branches_v1(
    binding: crate::mir::resolved_semantics::BindingRefV1,
    source: &Producer,
    read: &BindingValueObservationV2,
    producer: &ReferenceProducerV1,
    context: CallValidationContextV1<'_>,
    function: &MirFunction,
    finishing: Option<&Finished>,
    branches: &mut impl FnMut(&std::rc::Rc<BranchScalarPhiObservationV1>) -> Result<(), String>,
) -> Result<(), String> {
    validate_entry_layout(binding, source, producer, context, function)?;
    validate_scalar_origin(binding, source, read, producer, context)?;
    let finite_initializer_entry = matches!((source, producer),
        (Producer::Initializer(frame), ReferenceProducerV1::Entry(_))
            if matches!(frame.evaluation().references().get(&0), Some(Reference::FiniteEntry { .. })));
    if let (Producer::Initializer(frame), ReferenceProducerV1::Entry(entry)) = (source, producer) {
        if finite_initializer_entry {
            validate_initializer_finite_entry_v1(frame, read, *entry, context, function)?;
        }
    }
    if matches!(source, Producer::BranchNormal(_)) {
        let point = read.loan_scalar_join_v1(source)?;
        point
            .branch()
            .ok_or_else(|| freeze("expression/scalar-producer-kind"))?;
    }
    if let Some(projection) = finishing {
        // The retained producer owns this obligation, including a Branch
        // forwarded through a BindingRebind rather than a direct Branch source.
        if let ReferenceProducerV1::Branch(point) = producer {
            branches(point)?;
        }
        if let ReferenceProducerV1::Phi(point) = producer {
            point.require_completed_v1()?;
            context
                .ledger()
                .verify_relation_geometry_v1(point, function, Some(projection))?;
        }
        if let ReferenceProducerV1::Definition(row) = producer {
            projection.relation_binding(function, row)?;
        }
        for row in read.original_copy_dependencies_v1()? {
            projection.relation_binding(function, row)?;
        }
    } else if !finite_initializer_entry {
        validate_dependencies_with_branches_v1(read, producer, context, function, branches)?;
    }
    Ok(())
}
impl ExpressionReferenceReadV1 {
    pub(in crate::mir) fn prepare_branch_text_phi_v1(
        source: std::rc::Rc<
            crate::mir::resolved_semantics::home_new_prefix::BranchNormalTextSourceV1,
        >,
        pending: std::rc::Rc<crate::mir::builder::IfConditionMaterializationV1>,
        branch: Binding,
        exits: [Option<BasicBlockId>; 2],
        inputs: [BindingValueObservationV2; 2],
        relation: std::rc::Rc<crate::mir::builder::LocalSsaPhysicalRelationV1>,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
    ) -> Result<std::rc::Rc<BranchTextPhiObservationV1>, String> {
        context.ledger().require_branch_text_source_v1(&source)?;
        let [Some(then_exit), Some(else_exit)] = exits else {
            return Err(freeze("expression/text-producer-kind"));
        };
        let get_jump = |block| {
            function
                .blocks
                .get(&block)
                .and_then(|body| body.terminator.clone())
                .map(|row| (block, row))
                .ok_or_else(|| freeze("expression/reference-dependency-missing"))
        };
        let observed = std::rc::Rc::new(BranchTextPhiObservationV1 {
            source,
            pending,
            branch,
            exits: [then_exit, else_exit],
            jumps: [get_jump(then_exit)?, get_jump(else_exit)?],
            inputs,
            relation,
        });
        observed.validate(context, function, None)?;
        Ok(observed)
    }

    pub(in crate::mir) fn prepare_branch_scalar_phi_v1(
        source: std::rc::Rc<
            crate::mir::resolved_semantics::home_new_prefix::BranchNormalScalarSourceV1,
        >,
        pending: std::rc::Rc<crate::mir::builder::IfConditionMaterializationV1>,
        branch: Binding,
        exits: [Option<BasicBlockId>; 2],
        inputs: [BindingValueObservationV2; 2],
        relation: std::rc::Rc<crate::mir::builder::LocalSsaPhysicalRelationV1>,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
    ) -> Result<std::rc::Rc<BranchScalarPhiObservationV1>, String> {
        let _physical_scope = context.ledger().physical_validation_scope_v1(
            function,
            None,
            matches!(context, CallValidationContextV1::Finalized(_)),
        );
        context.ledger().require_branch_scalar_source_v1(&source)?;
        let [Some(then_exit), Some(else_exit)] = exits else {
            return Err(freeze("expression/scalar-producer-kind"));
        };
        let get_jump = |block| {
            function
                .blocks
                .get(&block)
                .and_then(|body| body.terminator.clone())
                .map(|row| (block, row))
                .ok_or_else(|| freeze("expression/reference-dependency-missing"))
        };
        let producers = [
            prepare_scalar_binding_producer(
                source.binding(),
                &source.incoming()[0].2,
                &inputs[0],
                context,
                function,
            )?,
            prepare_scalar_binding_producer(
                source.binding(),
                &source.incoming()[1].2,
                &inputs[1],
                context,
                function,
            )?,
        ];
        let observed = std::rc::Rc::new(BranchScalarPhiObservationV1 {
            source,
            pending,
            branch,
            exits: [then_exit, else_exit],
            jumps: [get_jump(then_exit)?, get_jump(else_exit)?],
            inputs,
            producers,
            relation,
        });
        observed.validate(context, function, None)?;
        Ok(observed)
    }
}
