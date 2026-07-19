use super::{commit_prepared_phi_type, PreparedPhiTypePublicationV1};
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;

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
