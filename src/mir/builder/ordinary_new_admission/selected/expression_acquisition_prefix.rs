//! Physical acquisition correspondence for the same checked expression body.
//! This lends no source, terminal, argument, entry or ownership permission.
use super::*;
use crate::mir::instruction::StringInvokeOperand;
use crate::mir::resolved_semantics::home_new_prefix::{CheckedExpressionEvaluationV1, LocalCallObservationV1};

/// `None` checks the completed evaluation; a Call boundary checks exactly the
/// acquisitions preceding that SAME source observation in original postorder.
pub(super) fn validate(
    frame: &CheckedExpressionEvaluationV1, before: Option<&LocalCallObservationV1>,
    acquired: &[(usize, ValueId)], values: &[Option<StringInvokeOperand>], function: &MirFunction,
) -> Result<(), String> {
    if values.len() != frame.source().nodes().len() {
        return Err(freeze("expression-call/value-coverage"));
    }
    let order = frame.ordered_nodes();
    let end = if let Some(call) = before {
        if !frame.calls().iter().any(|original| std::ptr::eq(original, call)) {
            return Err(freeze("expression-call/original-observation"));
        }
        order.iter().position(|node| frame.source().nodes()[*node].site() == call.site().site())
            .ok_or_else(|| freeze("expression-call/source-node"))?
    } else { order.len() };
    let expected = order[..end].iter().copied().filter(|node| frame.owned_result_nodes().contains(node));
    if acquired.iter().map(|(node, _)| *node).ne(expected) {
        return Err(freeze("expression-call/acquisition-prefix"));
    }
    let mut seen_values = std::collections::BTreeSet::new();
    for (node, value) in acquired {
        if values.get(*node) != Some(&Some(StringInvokeOperand::OwnedString(*value))) {
            return Err(freeze("expression-call/acquisition-node-value"));
        }
        if !seen_values.insert(*value) || function.params.contains(value) {
            return Err(freeze("expression-call/duplicate-acquisition"));
        }
        let mut producers = function.blocks.iter().flat_map(|(id, block)|
            block.all_instructions().map(move |row| (*id, row)))
            .filter(|(_, row)| row.dst_value() == Some(*value));
        let Some((landing, MirInstruction::InvokeNormalResult { invoke_block, .. })) = producers.next() else {
            return Err(freeze("expression-call/acquisition-projection"));
        };
        if producers.next().is_some() || !matches!(function.blocks.get(invoke_block)
            .and_then(|block| block.terminator.as_ref()),
            Some(MirInstruction::Invoke { operation: InvokeOperation::String(operation), normal_landing, .. })
                if operation.owns_normal_result() && *normal_landing == landing) {
            return Err(freeze("expression-call/acquisition-producer"));
        }
    }
    Ok(())
}
