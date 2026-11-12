//! Normal/Fault cleanup for one recorded New argument phase.
use super::*;

/// Recorded New phase boundaries, replayed against source-derived cleanup.
#[derive(Debug)]
pub(in crate::mir) struct NewArgumentCleanupV1 {
    allocation: Binding,
    birth: Option<Binding>,
    continuation: BasicBlockId,
    inherited: Box<[InvokeOperation]>,
}
impl NewArgumentCleanupV1 {
    pub(in crate::mir) fn new(
        allocation: Binding,
        birth: Option<Binding>,
        continuation: BasicBlockId,
        inherited: Box<[InvokeOperation]>,
    ) -> Self {
        Self {
            allocation,
            birth,
            continuation,
            inherited,
        }
    }

    pub(in super::super) fn inherited(&self) -> &[InvokeOperation] {
        &self.inherited
    }
    /// Lend the same Normal acquisition prefix to the enclosing expression.
    /// The independent validator below still checks all Fault obligations.
    pub(in super::super) fn ordered_prefix_v1(
        &self,
        arguments: &[EmittedNewArgumentV1],
        context: CallValidationContextV1<'_>,
    ) -> Result<(Vec<Binding>, BasicBlockId), String> {
        let mut ordered = Vec::new();
        for argument in arguments {
            if let Some(packet) = argument.expression_v1() {
                ordered.extend(packet.entry.iter().cloned());
                ordered.extend(packet.projection().ordered_bindings_in_context(context)?);
                ordered.extend(packet.terminal.iter().cloned());
                ordered.extend(packet.continuation.iter().cloned());
            }
        }
        ordered.push(self.allocation.clone());
        if let Some(birth) = &self.birth {
            ordered.push(birth.clone());
        }
        Ok((ordered, self.continuation))
    }

    pub(in super::super) fn validate_v1(
        &self,
        context: CallValidationContextV1<'_>,
        site: &OwnedExprSiteV1,
        result: ValueId,
        arguments: &[EmittedNewArgumentV1],
        function: &MirFunction,
        finishing: Option<&physical_boundary::FinishedBindings>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        use super::root_cleanup_graph::ordered_paths as paths;
        let ledger = context.ledger();
        let map = |row: &Binding| -> Result<Binding, String> {
            match finishing {
                Some(boundary) => boundary
                    .binding(row.0, &row.1)?
                    .ok_or_else(|| freeze("emission-binding-drift")),
                None => Ok(row.clone()),
            }
        };
        let (ordered, _) = self.ordered_prefix_v1(arguments, context)?;
        let mut mapped = Vec::new();
        for row in ordered {
            let projected = finishing
                .map(|boundary| {
                    if matches!(row.1, MirInstruction::Copy { .. }) {
                        boundary.relation_binding(function, &row)
                    } else {
                        boundary.binding(row.0, &row.1)
                    }
                })
                .transpose()?;
            match projected {
                Some(Some(actual)) => mapped.push(actual),
                Some(None)
                    if matches!(
                        row.1,
                        MirInstruction::Jump {
                            edge_args: None,
                            ..
                        } | MirInstruction::Copy { .. }
                    ) => {}
                Some(None) => return Err(freeze("emission-binding-drift")),
                None => mapped.push(row),
            }
        }
        super::root_home::ExpressionPhysicalProjectionRefV1::validate_normal_order_v1(
            function, &mapped,
        )?;
        let prior = ledger.new_argument_home_unwind_v1(site, false)?;
        let mut ends = Vec::new();
        let mut seen = std::collections::BTreeSet::new();
        for argument in arguments.iter().rev() {
            for value in argument.deferred_text_values_v1()?.into_iter().rev() {
                if !seen.insert(value) {
                    return Err(freeze("argument-expression/prefix-duplicate-acquisition"));
                }
                ends.push(InvokeOperation::String(
                    crate::mir::instruction::StringInvokeOperation::End { value },
                ));
            }
        }
        let pending: Vec<_> = ends
            .iter()
            .cloned()
            .chain(self.inherited.iter().cloned())
            .chain(prior)
            .collect();
        let (object, _) = ledger.new_argument_object_cleanup_v1(site, result)?;
        let expected_birth = {
            let rows = ledger.local_commits.borrow();
            match rows.get(site) {
                Some(LocalCommitV1::Ordinary(row)) => row.birth_target.clone(),
                Some(LocalCommitV1::Result(row)) => row.birth_target.clone(),
                _ => return Err(freeze("argument-source-drift")),
            }
        };
        let (allocation_block, allocation) = map(&self.allocation)?;
        let MirInstruction::Invoke {
            operation:
                InvokeOperation::NewBox {
                    object: actual_object,
                },
            fault_frame,
            fault_landing,
            normal_landing,
        } = allocation
        else {
            return Err(freeze("emission-binding-drift"));
        };
        if actual_object != object
            || !function.blocks.get(&normal_landing).is_some_and(|block| {
                block.instructions.iter().any(|row| {
                    matches!(row, MirInstruction::InvokeNormalResult { invoke_block, dst }
                if *invoke_block == allocation_block && *dst == result)
                })
            })
        {
            return Err(freeze("emission-binding-drift"));
        }
        let mut covered =
            paths::pending_operations(function, fault_landing, &pending, Some(fault_frame))?;
        let normal = if let Some(birth) = &self.birth {
            let (_, birth) = map(birth)?;
            let MirInstruction::Invoke {
                operation: InvokeOperation::Birth(call),
                fault_frame: same,
                fault_landing,
                normal_landing: after,
            } = birth
            else {
                return Err(freeze("argument-call-drift"));
            };
            if same != fault_frame
                || expected_birth.as_ref() != Some(&call.key)
                || call.receiver != result
                || !call
                    .arguments
                    .iter()
                    .eq(arguments.iter().map(|argument| argument.payload()))
            {
                return Err(freeze("argument-call-drift"));
            }
            let object = ledger.new_argument_object_cleanup_v1(site, result)?.0;
            let reclaim = InvokeOperation::ReclaimUnpublished {
                object,
                value: result,
            };
            let head = paths::skip_until(function, fault_landing, None)?;
            let (normal, fault) = paths::release(function, head, &reclaim, Some(fault_frame))?;
            covered.extend(paths::pending_operations(
                function,
                normal,
                &pending,
                Some(fault_frame),
            )?);
            covered.extend(paths::pending_operations(
                function,
                fault,
                &pending,
                Some(fault_frame),
            )?);
            after
        } else {
            if expected_birth.is_some() {
                return Err(freeze("argument-call-drift"));
            }
            normal_landing
        };
        let stop = match finishing {
            Some(boundary) => boundary
                .destination(self.continuation)
                .ok_or_else(|| freeze("emission-binding-drift"))?,
            None => self.continuation,
        };
        let mut normal = normal;
        let (_, candidate) = ledger.new_argument_object_cleanup_v1(site, result)?;
        let homes = ledger.new_argument_home_unwind_v1(site, true)?;
        for (step, operation) in ends.iter().enumerate() {
            normal = paths::skip_until(function, normal, Some(stop))?;
            let (next, fault) = paths::release(function, normal, operation, Some(fault_frame))?;
            let pending: Vec<_> = candidate
                .iter()
                .cloned()
                .chain(ends[step + 1..].iter().cloned())
                .chain(self.inherited.iter().cloned())
                .chain(homes.iter().cloned())
                .collect();
            covered.extend(paths::pending_operations(
                function,
                fault,
                &pending,
                Some(fault_frame),
            )?);
            normal = next;
        }
        if paths::skip_until(function, normal, Some(stop))? != stop {
            return Err(freeze("emission-binding-drift"));
        }
        Ok(covered)
    }
}
