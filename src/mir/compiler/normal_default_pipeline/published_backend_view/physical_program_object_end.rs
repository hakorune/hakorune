//! Physical membership of complete conditional End, lent by the source owner.
//! Instruction shape alone never admits an End or reconstructs its body.
use super::*;

pub(super) fn bindings(
    module: Option<&MirModule>,
    function: &MirFunction,
    view: Option<&PublishedMirBackendView<'_>>,
) -> Result<Vec<(BasicBlockId, MirInstruction)>, String> {
    let actual: Vec<_> = function.blocks.iter().flat_map(|(block, body)| {
        body.all_instructions().filter_map(move |row| matches!(row,
            MirInstruction::Invoke { operation: InvokeOperation::ObjectEndIfOwned { .. }, .. }
        ).then(|| (*block, row.clone())))
    }).collect();
    let Some(view) = view else {
        return if actual.is_empty() { Ok(Vec::new()) }
            else { Err(fault("conditional-end/source-view-missing")) };
    };
    let module = module.ok_or_else(|| fault("conditional-end/module-missing"))?;
    if !std::ptr::eq(module, view.module())
        || !module.functions.get(&function.signature.name)
            .is_some_and(|stored| std::ptr::eq(stored, function))
    { return Err(fault("conditional-end/view-membership")); }
    let Some(source) = view.retained_root_source() else {
        return if actual.is_empty() { Ok(Vec::new()) }
            else { Err(fault("conditional-end/source-handoff-missing")) };
    };
    let expected = source.object_end_bindings_v1(module, function)?;
    if actual.len() != expected.len()
        || actual.iter().any(|binding| !expected.contains(binding))
        || expected.iter().any(|binding| !actual.contains(binding))
    {
        return Err(format!("{} function={} actual={} source={} unexpected={:?} missing={:?}",
            fault("conditional-end/source-coverage"), function.signature.name,
            actual.len(), expected.len(),
            actual.iter().find(|binding| !expected.contains(binding)),
            expected.iter().find(|binding| !actual.contains(binding))));
    }
    Ok(expected)
}
