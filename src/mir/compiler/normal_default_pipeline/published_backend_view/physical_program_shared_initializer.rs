//! Physical membership of Shared New/Clone, lent by the source owner.
//! Instruction shape alone never admits a Shared initializer.
use super::*;

pub(super) fn bindings(
    module: Option<&MirModule>,
    function: &MirFunction,
    view: Option<&PublishedMirBackendView<'_>>,
) -> Result<Vec<(BasicBlockId, MirInstruction)>, String> {
    let lent = match (module, view.and_then(|view| view.retained_root_source())) {
        (Some(module), Some(source)) => source.finished_shared_initializers_v1(module, function)?,
        _ => Vec::new(),
    };
    let actual: Vec<_> = function.blocks.iter().flat_map(|(block, body)| body.all_instructions()
        .filter_map(move |row| matches!(row, MirInstruction::Invoke {
            operation: InvokeOperation::Shared(crate::mir::instruction::SharedInvokeOperation::New { .. } | crate::mir::instruction::SharedInvokeOperation::Clone { .. }), ..
        }).then(|| (*block, row.clone())))).collect();
    if actual.len() != lent.len() || actual.iter().any(|row| !lent.contains(row))
        || lent.iter().any(|row| !actual.contains(row)) {
        return Err(fault("shared-initializer/source-coverage"));
    }
    Ok(lent)
}
