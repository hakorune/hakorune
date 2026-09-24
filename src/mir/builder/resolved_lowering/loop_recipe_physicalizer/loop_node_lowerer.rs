//! Canonical physicalize edge for one loop-node winner admission
//! (M10b-I0-P2-E).
//!
//! This is the thin Builder bridge behind the same sole physical owner the
//! Callable/Generic drafts already use: it constructs
//! `ResolvedSsaIdentityStateV2` + `CanonicalCfgSessionV1` + `PhiTxn` once per
//! node, publishes entry inputs at the walk's current block, and hands the
//! prepared layout to the segment allocator -> dispatcher -> recursive-after
//! pipeline. It owns no Recipe, demand, layout, selector, or retry state.

use super::operation_dispatcher::LoopOperationDispatchServicesV1;
use super::operation_ledger::LoopOperationValueLedgerV1;
use super::recursive_after::prepare_recursive_after_v1;
use super::segment_allocator::allocate_for_layout;
use super::segment_dispatcher::prepare_loop_segment_operation_dispatch_v1;
use super::topology::{ready_loop_entry_from_canonical_rows, LoopPhysicalServicesV1};
use crate::mir::builder::emission::phi_lifecycle::PhiTxn;
use crate::mir::builder::resolved_lowering::canonical_cfg::CanonicalCfgSessionV1;
use crate::mir::builder::resolved_lowering::canonical_ssa::ResolvedSsaIdentityStateV2;
use crate::mir::builder::MirBuilder;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::loop_node_physical_admission::VerifiedLoopNodePhysicalAdmissionV1;
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1};
use crate::mir::{BasicBlockId, ValueId};

/// One completed loop-node physicalization: the sealed root After block and
/// the per-binding values the caller writes back into the walk's
/// `variable_map`. The outer publication owner stays the only publisher.
#[derive(Debug)]
pub(in crate::mir::builder) struct LoopNodeWinnerPhysicalContinuationV1 {
    root_after: BasicBlockId,
    writebacks: Box<[(BindingRefV1, ValueId)]>,
}

impl LoopNodeWinnerPhysicalContinuationV1 {
    pub(in crate::mir::builder) const fn root_after(&self) -> BasicBlockId {
        self.root_after
    }

    /// `(source binding, post-loop physical value)` rows for every declared
    /// input binding, read at the sealed root After through the canonical
    /// identity — never a second reaching-value map.
    pub(in crate::mir::builder) fn writebacks(&self) -> &[(BindingRefV1, ValueId)] {
        &self.writebacks
    }
}

/// Physicalize one issued loop-node admission against the live walk state.
///
/// `variable_map` is the walk's name -> `ValueId` table. Entry input values
/// are adopted from it by resolver binding name; a missing lane or an
/// unknown resolver name is a typed terminal error, never a fallback.
pub(in crate::mir::builder) fn lower_loop_node_physical_admission_v1<'source>(
    builder: &mut MirBuilder,
    input: ResolvedFunctionLoweringInputV1<'source>,
    admission: VerifiedLoopNodePhysicalAdmissionV1,
) -> Result<LoopNodeWinnerPhysicalContinuationV1, String> {
    lower_loop_node_physical_admission_inner(builder, input, admission, None)
}

/// Callable ingress supplies the already-materialized values from the exact
/// BindingRef ledger. Names remain diagnostic and are not used to establish
/// the VAR entry relation.
pub(in crate::mir::builder) fn lower_loop_node_physical_admission_with_callable_entry_values_v1(
    builder: &mut MirBuilder,
    input: ResolvedFunctionLoweringInputV1<'_>,
    admission: VerifiedLoopNodePhysicalAdmissionV1,
    entry_values: &[(BindingRefV1, ValueId)],
) -> Result<LoopNodeWinnerPhysicalContinuationV1, String> {
    lower_loop_node_physical_admission_inner(builder, input, admission, Some(entry_values))
}

fn lower_loop_node_physical_admission_inner<'source>(
    builder: &mut MirBuilder,
    input: ResolvedFunctionLoweringInputV1<'source>,
    admission: VerifiedLoopNodePhysicalAdmissionV1,
    callable_entry_values: Option<&[(BindingRefV1, ValueId)]>,
) -> Result<LoopNodeWinnerPhysicalContinuationV1, String> {
    let (layout, inputs) = admission.into_parts();
    let owner: FunctionOwnerIdV1 = layout.program().demand().context().owner();
    let preheader = builder
        .function_state
        .current_block
        .ok_or_else(|| "[freeze:contract][loop_node_physicalize/no_current_block]".to_owned())?;

    let mut identity = ResolvedSsaIdentityStateV2::new(input.function());
    let mut cfg = CanonicalCfgSessionV1::new_for_owner(owner);
    let mut phis = PhiTxn::begin("canonical_binding_ssa");

    // Entry input publication: one `publish_declaration_exact` per resolver
    // input row at the preheader, with the walk's materialized value.
    if let Some(entries) = callable_entry_values {
        let expected = inputs
            .rows()
            .iter()
            .map(|row| row.source_binding())
            .collect::<std::collections::BTreeSet<_>>();
        let actual = entries
            .iter()
            .map(|(binding, _)| *binding)
            .collect::<std::collections::BTreeSet<_>>();
        if expected != actual || actual.len() != entries.len() {
            return Err(
                "[freeze:contract][loop_node_physicalize/callable-entry-binding-set]".to_owned(),
            );
        }
    }
    let mut rows = Vec::with_capacity(inputs.rows().len());
    for input_row in inputs.rows() {
        let binding = input_row.source_binding();
        let value = match callable_entry_values {
            Some(entries) => entries
                .iter()
                .find_map(|(candidate, value)| (*candidate == binding).then_some(*value))
                .ok_or_else(|| {
                    format!(
                        "[freeze:contract][loop_node_physicalize/callable-entry-value-missing] binding={binding:?}"
                    )
                })?,
            None => {
                let record = input.function().binding(binding).ok_or_else(|| {
                    format!(
                        "[freeze:contract][loop_node_physicalize/input_binding_unknown] binding={binding:?}"
                    )
                })?;
                *builder
                    .function_state
                    .variable_ctx
                    .variable_map
                    .get(record.diagnostic_name())
                    .ok_or_else(|| {
                        format!(
                            "[freeze:contract][loop_node_physicalize/input_value_missing] name={}",
                            record.diagnostic_name()
                        )
                    })?
            }
        };
        let published = identity
            .publish_declaration_exact(input_row.declaration(), binding, preheader, value)
            .map_err(|error| {
                format!("[freeze:contract][loop_node_physicalize/input_publish] {error}")
            })?;
        if published != binding {
            return Err(
                "[freeze:contract][loop_node_physicalize/input_binding_mismatch]".to_owned(),
            );
        }
        rows.push((input_row.recipe_value(), binding, value));
    }
    let ready_entry = ready_loop_entry_from_canonical_rows(owner, preheader, rows);

    let segment_receipt = {
        let mut services = LoopPhysicalServicesV1::new(builder, &mut cfg);
        allocate_for_layout(&layout, &ready_entry, &mut services).map_err(|error| {
            format!("[freeze:contract][loop_node_physicalize/segments] {error:?}")
        })?
    };
    let dispatch = prepare_loop_segment_operation_dispatch_v1(layout, ready_entry, segment_receipt)
        .map_err(|error| {
            format!("[freeze:contract][loop_node_physicalize/dispatch_preflight] {error:?}")
        })?;
    let completed = {
        let mut services = LoopOperationDispatchServicesV1::new(builder, &mut identity, &mut phis);
        dispatch
            .emit_all(LoopOperationValueLedgerV1::default(), &mut services)
            .map_err(|error| {
                format!("[freeze:contract][loop_node_physicalize/dispatch] {error:?}")
            })?
    };
    let prepared_after = prepare_recursive_after_v1(completed, builder).map_err(|error| {
        format!("[freeze:contract][loop_node_physicalize/after_preflight] {error:?}")
    })?;
    let ready_after = prepared_after
        .emit_and_seal(builder, &mut cfg, &mut identity, &mut phis)
        .map_err(|error| {
            format!("[freeze:contract][loop_node_physicalize/after_seal] {error:?}")
        })?;
    let root_after = ready_after.root_after();

    // Writeback bridge: post-loop values for declared input bindings, read
    // through the same identity the emission used.
    let mut writebacks = Vec::with_capacity(inputs.rows().len());
    for input_row in inputs.rows() {
        let binding = input_row.source_binding();
        let value = identity
            .read_entry(builder, &mut phis, root_after, binding)
            .map_err(|error| {
                format!("[freeze:contract][loop_node_physicalize/writeback_read] {error}")
            })?;
        writebacks.push((binding, value));
    }
    Ok(LoopNodeWinnerPhysicalContinuationV1 {
        root_after,
        writebacks: writebacks.into_boxed_slice(),
    })
}
