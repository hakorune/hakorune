use super::{
    commit_prepared_phi_type, MirType, PhiTransientTypeDecisionV1, PreparedPhiTypePublicationV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, ValueId};

/// Prepare one Builder-owned transient type publication from logical inputs.
pub(in crate::mir::builder) fn prepare_for_builder(
    builder: &MirBuilder,
    dst: ValueId,
    logical_inputs: &[(BasicBlockId, ValueId)],
    type_hint: Option<&MirType>,
) -> Result<PreparedPhiTypePublicationV1, String> {
    PhiTransientTypeDecisionV1::prepare(
        dst,
        logical_inputs,
        &builder.function_state.type_ctx.value_types,
        builder.function_state.type_ctx.value_types.get(&dst),
        type_hint,
    )
    .map_err(|error| error.to_string())
}

/// Commit only after the owning PHI lifecycle mutation has succeeded.
pub(in crate::mir::builder) fn commit_for_builder(
    builder: &mut MirBuilder,
    dst: ValueId,
    prepared: PreparedPhiTypePublicationV1,
) {
    commit_prepared_phi_type(
        &mut builder.function_state.type_ctx.value_types,
        dst,
        prepared,
    );
}
