//! One original physical SSA dependency walk for Call arguments and controls.
//! No result class, source permission or replacement value is learned here.
use super::*;

pub(in super::super) fn original_value_dependencies_v1(
    originals: &Bindings, params: &[ValueId], values: &[ValueId],
) -> Result<Vec<(BasicBlockId, MirInstruction)>, String> {
    let mut definitions = BTreeMap::<_, Vec<_>>::new();
    for original in originals {
        if let Some(dst) = original.1.dst_value() { definitions.entry(dst).or_default().push(original); }
    }
    let mut pending = values.to_vec();
    let mut seen = BTreeSet::new();
    let mut dependencies = Vec::new();
    while let Some(value) = pending.pop() {
        if !seen.insert(value) { continue; }
        let rows = definitions.get(&value).map(Vec::as_slice).unwrap_or_default();
        if params.contains(&value) {
            if !rows.is_empty() { return Err(fault("dependency-parameter-collision")); }
            continue;
        }
        let [original] = rows else { return Err(fault("dependency-producer-unique")) };
        pending.extend(original.1.used_values());
        dependencies.push((*original).clone());
        if let MirInstruction::InvokeNormalResult { invoke_block, .. } = &original.1 {
            let mut invokes = originals.iter().filter(|(block, row)| block == invoke_block
                && matches!(row, MirInstruction::Invoke { normal_landing, .. } if *normal_landing == original.0));
            let invoke = invokes.next().ok_or_else(|| fault("dependency-normal-origin"))?;
            if invokes.next().is_some() { return Err(fault("dependency-normal-duplicate")); }
            pending.extend(invoke.1.used_values());
            if !dependencies.contains(invoke) { dependencies.push(invoke.clone()); }
        }
    }
    Ok(dependencies)
}

impl PhysicalBoundary {
    /// Lend exactly the producers whose identities the control sequence consumes
    /// to the existing finishing preservation owner, before optimization.
    pub(in super::super) fn control_producer_bindings_v1(&self, function: &MirFunction)
        -> Result<Vec<(BasicBlockId, MirInstruction)>, String> {
        let projection = self.project(function)?;
        let used: BTreeSet<_> = projection.captured.iter().flat_map(|(_, row)| row.used_values()).collect();
        let mut values = Vec::new();
        for (rows, terminal) in projection.sequences.values() {
            for row in rows.iter().chain(std::iter::once(terminal)) {
                match row {
                    MirInstruction::Branch { condition, .. } => values.push(*condition),
                    MirInstruction::Phi { dst, inputs, .. } if used.contains(dst) => {
                        values.push(*dst);
                        values.extend(inputs.iter().map(|(_, value)| *value));
                    }
                    MirInstruction::Invoke { operation: operation @ crate::mir::instruction::InvokeOperation::Birth(_), .. } =>
                        values.extend(operation.used_values()),
                    MirInstruction::Call(call) => values.extend(call.args.iter().copied()),
                    MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::Call { call, .. }, .. } =>
                        values.extend(call.args.iter().copied()),
                    _ => (),
                }
            }
        }
        original_value_dependencies_v1(&projection.captured, &function.params, &values)
    }
}
