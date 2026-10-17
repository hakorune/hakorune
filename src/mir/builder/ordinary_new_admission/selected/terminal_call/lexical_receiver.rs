//! One source-qualified receiver preparation for local and terminal calls.
use super::*;
use crate::mir::builder::normal_callable_semantic_lowering_state::ExactLexicalReadV1;
type Binding = (BasicBlockId, MirInstruction);

pub(super) fn prepare(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    owner: FunctionOwnerIdV1,
    row: &LexicalInstanceCallDispositionRowV1,
    bindings: &mut Vec<Binding>,
) -> Result<(ExactLexicalReadV1, Option<Binding>), String> {
    let stored = row.source_target().stored_receiver();
    let (site, binding) = match stored {
        Some((binding, site, _, _)) => (site, binding),
        None => (row.receiver_site(), row.receiver_binding()?),
    };
    let receiver = state.take_exact_lexical_read(owner, site.node(), binding)
        .map_err(|error| error.to_string())?;
    let field = if let Some((_, site, field, _)) = stored {
        let base = receiver.value_for(owner, site.node(), binding)
            .map_err(|error| error.to_string())?;
        let (_, read) = builder.append_exact_object_field_read_v1(
            base, field, MirType::Box(row.target().owner().into()))?;
        bindings.push(read.clone());
        Some(read)
    } else { None };
    Ok((receiver, field))
}
