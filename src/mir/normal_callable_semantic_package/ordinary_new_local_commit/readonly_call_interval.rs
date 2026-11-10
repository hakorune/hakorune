//! Original-source readonly Call closure and independent borrowed-use intervals.
use super::*;
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
use super::read_argument::IntervalViewV1;
type Binding = (BasicBlockId, MirInstruction);

pub(super) fn readonly_instruction_v1(row: &MirInstruction) -> bool {
    matches!(row,
        MirInstruction::Const { .. } | MirInstruction::Copy { .. } | MirInstruction::Phi { .. }
        | MirInstruction::KeepAlive { .. }
        | MirInstruction::BinOp { .. } | MirInstruction::Compare { .. } | MirInstruction::NullCompare { .. } | MirInstruction::Branch { .. }
        | MirInstruction::Jump { .. } | MirInstruction::Return { .. } | MirInstruction::ReturnFault { .. }
        | MirInstruction::FaultFrameEnter { .. } | MirInstruction::InvokeNormalResult { .. } | MirInstruction::TypeOp { .. }
        | MirInstruction::ObjectFieldGet { .. } | MirInstruction::FieldGet { .. }
        | MirInstruction::Invoke { operation: InvokeOperation::FieldGet { .. } | InvokeOperation::ObjectFieldBorrow { .. }, .. })
}
pub(super) type ReadCallEdgeV1 = (String, (BasicBlockId, usize), hakorune_mir_defs::CanonicalSameModuleCallableKeyV1, FunctionOwnerIdV1);
pub(super) fn validate_readonly_edge_v1(handoff: &super::super::FinalizedRootSourceHandoffV1,
    module: &crate::mir::MirModule, caller: &MirFunction, at: (BasicBlockId, usize), edges: &[ReadCallEdgeV1],
    checked: &mut std::collections::BTreeSet<hakorune_mir_defs::CanonicalSameModuleCallableKeyV1>,
) -> Result<(), String> {
    let mut pending = vec![(caller, at)];
    while let Some((caller, at)) = pending.pop() {
        let mut original = edges.iter().filter(|(name, position, _, _)| name == &caller.signature.name && *position == at);
        let (_, _, target, owner) = original.next().ok_or_else(|| freeze("field-argument/source-missing"))?;
        if original.next().is_some() { return Err(freeze("field-argument/source-missing")); }
        let row = caller.blocks.get(&at.0).and_then(|block| block.all_instructions().nth(at.1)).ok_or_else(|| freeze("field-argument/source-missing"))?;
        if !matches!(row, MirInstruction::Invoke { operation: InvokeOperation::Call { call, .. }, .. } if match &call.callee {
            crate::mir::Callee::SameModuleInstance { key, .. } => key == target,
            crate::mir::Callee::Global(global) => target.canonical_global_target_v1().as_ref() == Ok(global), _ => false,
        }) { return Err(freeze("field-argument/finished-parent-link")); }
        if !checked.insert(target.clone()) { continue; }
        let callee = handoff.ledger.with_finished_projection(*owner, |symbol, _| {
            if module.canonical_callable_definition_symbol(target) != Some(symbol) { return Err(freeze("field-argument/source-missing")); }
            module.functions.get(symbol).ok_or_else(|| freeze("field-argument/source-missing"))
        })?;
        for (block, body) in &callee.blocks {
            for (index, row) in body.all_instructions().enumerate() {
                if matches!(row, MirInstruction::Invoke { operation: InvokeOperation::Call { .. }, .. }) { pending.push((callee, (*block, index))); }
                else if !readonly_instruction_v1(row) {
                    return Err(format!("{} callee={} block={block:?} instruction={row:?}",
                        freeze("field-argument/parent-or-child-invalidated"), callee.signature.name));
                }
            }
        }
    }
    Ok(())
}

/// The original support is a lifetime anchor, never the returned payload.
pub(super) fn borrowed_lifetime_starts_v1(context: CallValidationContextV1<'_>,
    loan: &crate::mir::normal_callable_semantic_package::BorrowedNullableLocalSourceLoanV1,
) -> Result<Vec<(ValueId, Binding)>, String> {
    let mut pending = vec![loan];
    let mut visited = std::collections::BTreeMap::new();
    let mut starts = Vec::new();
    while let Some(base) = pending.pop() {
        if let Some(original) = visited.insert(base.call(), base) {
            if original != base { return Err(freeze("read-argument/emitted-source-kind-drift")); }
            continue;
        }
        if let Some(parent) = base.reborrowed_read_v1() { pending.extend(parent.loans()); continue; }
        starts.push(context.with_taken_packet_v1(base.call(), |packet| {
            let original = context.ledger().borrowed_nullable_call_source_v1(base.call())
                .ok_or_else(|| freeze("read-argument/emitted-source-kind-drift"))?;
            if original.borrowed_nullable() != Some(base) { return Err(freeze("read-argument/emitted-source-kind-drift")); }
            let actual = packet.call_in_context(base.call().owner(), original.arguments(), context)?;
            let value = match base.result_support_v1() {
                crate::mir::instruction::BorrowedResultSupportV1::Formal(ordinal) => actual.args.get(ordinal as usize).copied(),
                crate::mir::instruction::BorrowedResultSupportV1::Receiver => actual.callee.receiver(),
                _ => None,
            }.ok_or_else(|| freeze("read-argument/emitted-source-kind-drift"))?;
            Ok((value, packet.outer_bindings()?.1.clone()))
        })?);
    }
    if starts.is_empty() { return Err(freeze("read-argument/emitted-source-kind-drift")); }
    Ok(starts)
}

impl FinalizedRootSourceHandoffV1 {
    /// Verification follows retained source loans. MIR use scanning cannot issue
    /// a loan or widen its source permissions; it detects extra/changed uses.
    pub(in crate::mir) fn validate_declared_read_uses_v1(&self, module: &crate::mir::MirModule) -> Result<(), String> {
        let mut edges = Vec::new();
        let mut reads = Vec::new();
        let mut call_points = std::collections::BTreeMap::<_, Vec<_>>::new();
        self.visit_finalized_call_structure_v1(module, |owner, _, packet, _, caller, at, _| {
            let points = call_points.entry(owner).or_default();
            for point in packet.read_argument_branch_points_v1()? {
                if !points.iter().any(|old| std::rc::Rc::ptr_eq(old, &point)) { points.push(point); }
            }
            let original = packet.original_source();
            edges.push((caller.signature.name.clone(), at, original.target().clone(), original.callee_owner()));
            let loan = match original {
                CallPacketSourceLoanV1::Instance(row) => row.source_target().borrowed_nullable_local_source(),
                CallPacketSourceLoanV1::Static { observation, .. }
                    | CallPacketSourceLoanV1::SelectedStaticLoop { observation, .. } => observation.borrowed_nullable(),
            };
            if let Some(loan) = loan.filter(|loan| loan.is_declared_read_v1()) {
                let normal = packet.outer_bindings()?.1.clone();
                reads.push((owner, caller.signature.name.clone(), loan.clone(), normal));
            }
            Ok(())
        })?;
        let context = CallValidationContextV1::Finalized(self);
        let mut checked = std::collections::BTreeSet::new();
        let mut points = std::collections::BTreeMap::new();
        for (owner, symbol, loan, normal) in reads {
            if !points.contains_key(&owner) {
                let mut retained = context.declared_read_branch_points_v1(owner)?;
                retained.extend(call_points.remove(&owner).unwrap_or_default());
                points.insert(owner, retained);
            }
            let function = module.functions.get(&symbol).ok_or_else(|| freeze("field-argument/source-missing"))?;
            self.ledger.with_finished_projection(owner, |original_symbol, projection| {
                if original_symbol != &symbol { return Err(freeze("field-argument/source-missing")); }
                // The finishing owner permits removal only when definition AND
                // all uses are absent; a live argument still requires Some.
                let Some(normal) = projection.relation_binding(function, &normal)? else { return Ok(()); };
                let MirInstruction::InvokeNormalResult { dst: payload, .. } = normal.1 else { return Err(freeze("field-argument/finished-read-removed")); };
                let starts = borrowed_lifetime_starts_v1(context, &loan)?;
                let mut aliases = std::collections::BTreeSet::from([payload]);
                loop {
                    let count = aliases.len();
                    for row in function.blocks.values().flat_map(|body| body.all_instructions()) {
                        match row {
                            MirInstruction::Copy { dst, src } if aliases.contains(src) => { aliases.insert(*dst); },
                            MirInstruction::Phi { dst, inputs, .. } if inputs.iter().any(|(_, value)| aliases.contains(value)) => { aliases.insert(*dst); },
                            _ => {},
                        }
                    }
                    if count == aliases.len() { break; }
                }
                let intervals = IntervalViewV1::new(function);
                for (block, body) in &function.blocks {
                    for (index, row) in body.all_instructions().enumerate() {
                        // KeepAlive is SSA/liveness metadata, not a payload read.
                        // Its operands still undergo the independent SSA checks.
                        if matches!(row, MirInstruction::KeepAlive { .. })
                            || !row.used_values().iter().any(|value| aliases.contains(value)) { continue; }
                        if matches!(row, MirInstruction::Phi { inputs, .. } if inputs.iter().any(|(_, value)| !aliases.contains(value))) {
                            let mut original = false;
                            for point in &points[&owner] {
                                if point.corroborates_phi_v1(&loan, &(*block, row.clone()), context, function, projection)? { original = true; break; }
                            }
                            if !original { return Err(freeze("field-argument/child-origin")); }
                        }
                        let at = (*block, index);
                        let check = |at, row: &MirInstruction, checked: &mut std::collections::BTreeSet<_>| {
                            if matches!(row, MirInstruction::Invoke { operation: InvokeOperation::Call { .. }, .. }) {
                                validate_readonly_edge_v1(self, module, function, at, &edges, checked)?;
                                Ok(false)
                            } else if readonly_instruction_v1(row) { Ok(false) }
                            else { Err(format!("{} function={symbol} position={at:?} instruction={row:?}",
                                freeze("field-argument/parent-or-child-invalidated"))) }
                        };
                        if check(at, row, &mut checked)? { return Err(freeze("field-argument/parent-or-child-invalidated")); }
                        let finish = (*block, row.clone());
                        intervals.require_before(&normal, &finish)?;
                        for (anchor, start) in &starts {
                            let start = projection.relation_binding(function, start)?.ok_or_else(|| freeze("field-argument/finished-parent-removed"))?;
                            intervals.borrowed_result_v1(*anchor, payload)?.check_with_effects(&start, &finish,
                                |at, row| check(at, row, &mut checked))?;
                        }
                    }
                }
                Ok(())
            })?;
        }
        Ok(())
    }
}
