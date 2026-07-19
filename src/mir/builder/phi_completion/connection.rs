//! Builder-facing connection for the pure PHI completion vocabulary.
//!
//! This module is intentionally thin: it borrows the existing transient type
//! facts for preparation and delegates the final write to the existing
//! `phi_type_publication` owner. It owns no CFG, materialization, origin, or
//! instruction mutation policy.

use super::{CompletedPhiV1, PhiCompletionPreparationErrorV1, PhiDraftV1, PreparedPhiCompletionV1};
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, MirType, ValueId};

pub(in crate::mir::builder) fn prepare_for_builder(
    builder: &MirBuilder,
    block: BasicBlockId,
    dst: ValueId,
    logical_inputs: &[(BasicBlockId, ValueId)],
    type_hint: Option<MirType>,
) -> Result<PreparedPhiCompletionV1, String> {
    PhiDraftV1::new(block, dst, type_hint)
        .prepare_input_completion(
            logical_inputs,
            &builder.function_state.type_ctx.value_types,
            builder.function_state.type_ctx.value_types.get(&dst),
        )
        .map_err(render_preparation_error)
}

pub(in crate::mir::builder) fn commit_for_builder(
    builder: &mut MirBuilder,
    completed: CompletedPhiV1,
) {
    let (dst, prepared_type) = completed.into_type_publication();
    crate::mir::builder::phi_type_publication::commit_for_builder(builder, dst, prepared_type);
}

fn render_preparation_error(error: PhiCompletionPreparationErrorV1) -> String {
    match error {
        PhiCompletionPreparationErrorV1::ConcreteTypeConflict(conflict) => conflict.to_string(),
        PhiCompletionPreparationErrorV1::DuplicateIncomingPredecessor { predecessor } => format!(
            "[freeze:contract][phi_completion/duplicate_incoming_predecessor] predecessor={predecessor}"
        ),
        PhiCompletionPreparationErrorV1::DuplicateExpectedPredecessor { predecessor } => format!(
            "[freeze:contract][phi_completion/duplicate_expected_predecessor] predecessor={predecessor}"
        ),
        PhiCompletionPreparationErrorV1::PhantomIncomingPredecessor { predecessor } => format!(
            "[freeze:contract][phi_completion/phantom_incoming_predecessor] predecessor={predecessor}"
        ),
        PhiCompletionPreparationErrorV1::MissingIncomingPredecessor { predecessor } => format!(
            "[freeze:contract][phi_completion/missing_incoming_predecessor] predecessor={predecessor}"
        ),
    }
}
