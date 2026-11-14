//! Apply the existing whole-argument cleanup after construction completes.
use super::*;

/// Every actual is a synchronous loan through Birth. Computed Text stays owned
/// by its argument packet; only the field's independent materialization moves.
pub(in crate::mir::builder::ordinary_new_admission::selected) fn temporary_ends_v1(
    arguments: &[EmittedNewArgumentV1],
) -> Result<Vec<crate::mir::instruction::InvokeOperation>, String> {
    let mut operations = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for argument in arguments.iter().rev() {
        for value in argument.deferred_text_values_v1()?.into_iter().rev() {
            if !seen.insert(value) {
                return Err(freeze("argument-expression/prefix-duplicate-acquisition"));
            }
            operations.push(crate::mir::instruction::InvokeOperation::String(
                crate::mir::instruction::StringInvokeOperation::End { value },
            ));
        }
    }
    Ok(operations)
}

pub(in crate::mir::builder::ordinary_new_admission::selected) fn finish_new_arguments_v1(
    builder: &mut MirBuilder,
    ledger: &OrdinaryNewClaimLedgerV1,
    site: &crate::mir::resolved_semantics::OwnedExprSiteV1,
    result: crate::mir::ValueId,
    frame: crate::mir::ValueId,
    arguments: &[EmittedNewArgumentV1],
    inherited: &[crate::mir::instruction::InvokeOperation],
    bindings: &mut Vec<(crate::mir::BasicBlockId, MirInstruction)>,
) -> Result<(), String> {
    use super::super::{RawRootGraphWriterV1, RootGraphWriterPortV1};
    let normal = temporary_ends_v1(arguments)?;
    if normal.is_empty() {
        return Ok(());
    }
    let (_, object_cleanup) = ledger.new_argument_object_cleanup_v1(site, result)?;
    let plan = super::super::FaultCleanupPlanV1::new_arguments(
        object_cleanup,
        normal,
        inherited
            .iter()
            .cloned()
            .chain(ledger.new_argument_home_unwind_v1(site, true)?)
            .collect(),
    )?;
    let mut writer = RawRootGraphWriterV1;
    let clean = writer.reserve_block(builder)?;
    builder.ensure_block_exists(clean)?;
    let graph = super::super::root_graph::emit_cleanup_suffixes_with_writer(
        &mut writer,
        builder,
        frame,
        clean,
        &plan,
        bindings,
    )?;
    bindings.push(writer.emit_current(
        builder,
        MirInstruction::Jump {
            target: graph.clean,
            edge_args: None,
        },
    )?);
    builder.start_new_block(clean)
}
