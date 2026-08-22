//! One-shot, non-emitting observation of the selected Dynamic body.
//!
//! This bridge consumes existing resolver/W6 evidence into the existing
//! callable semantic state. It does not lower syntax, issue a semantic
//! receipt, or write MIR. The physical session remains the sole physical
//! owner; this module only checks the source-to-evidence relation.

use super::callout_corridor::DynamicV2CallOutCorridorV1;
use super::formal_header::DynamicV2OpenedFormalHeaderV1;
use super::profile_close::DynamicV2PhysicalProfileCloseV1;
use super::targets::{DynamicV2PhysicalTargetRoleV1, DynamicV2PhysicalTargetSetV1};
use super::value_ledger::DynamicV2PhysicalValueLedgerV1;
use super::DynamicV2PhysicalSessionBrandV1;
use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use crate::mir::builder::resolved_lowering::selected_dynamic_physical_capability::DynamicV2PhysicalRepresentationV1;
use crate::mir::checked_callout::CheckedCallOutLeaseSlotIdV1;
use crate::mir::compiler::a_prime_i64_physical_capability::VerifiedAPrimeI64PhysicalDemandV1;
use crate::mir::compiler::dynamic_full_body_recipe::DynamicAPrimeI64SourceRelationViewV1;
use crate::mir::loop_recipe_contract::{LoopItemKeyV1, LoopValueKeyV1};
use crate::mir::resolved_semantics::{BindingRefV1, SourceBindingSiteV1, SourceNodeSiteV1};
use crate::mir::{BasicBlockId, ValueId};

const I0: LoopItemKeyV1 = LoopItemKeyV1::new(0);
const I2: LoopItemKeyV1 = LoopItemKeyV1::new(2);
const I3: LoopItemKeyV1 = LoopItemKeyV1::new(3);
const I6: LoopItemKeyV1 = LoopItemKeyV1::new(6);
const I11: LoopItemKeyV1 = LoopItemKeyV1::new(11);
const I13: LoopItemKeyV1 = LoopItemKeyV1::new(13);
const I15: LoopItemKeyV1 = LoopItemKeyV1::new(15);
const V4: LoopValueKeyV1 = LoopValueKeyV1::new(4);
const V6: LoopValueKeyV1 = LoopValueKeyV1::new(6);
const V7: LoopValueKeyV1 = LoopValueKeyV1::new(7);
const V10: LoopValueKeyV1 = LoopValueKeyV1::new(10);
const V14: LoopValueKeyV1 = LoopValueKeyV1::new(14);
const V15: LoopValueKeyV1 = LoopValueKeyV1::new(15);
const V17: LoopValueKeyV1 = LoopValueKeyV1::new(17);

pub(super) fn observe(
    state: &mut CallableSemanticLoweringState,
    demand: &VerifiedAPrimeI64PhysicalDemandV1<'_>,
    formals: &DynamicV2OpenedFormalHeaderV1,
    targets: &DynamicV2PhysicalTargetSetV1,
    corridor: &DynamicV2CallOutCorridorV1,
    values: &DynamicV2PhysicalValueLedgerV1,
    profile: &DynamicV2PhysicalProfileCloseV1,
    lease_slot: CheckedCallOutLeaseSlotIdV1,
    brand: &DynamicV2PhysicalSessionBrandV1,
) -> Result<(), String> {
    if demand.identity().owner() != state.owner()
        || demand.identity().owner() != brand.owner()
        || !values.matches_brand(brand)
    {
        return Err("[freeze:contract][selected-dynamic/body-bridge/owner]".to_owned());
    }
    let relation = demand.source_relation();
    let formal_rows = relation.formal_rows();
    if formal_rows.len() != 4 {
        return Err("[freeze:contract][selected-dynamic/body-bridge/formals]".to_owned());
    }
    let entry = formals.entry_values();
    state.install_entry_values(&entry)?;
    state.require_empty_side_observations()?;

    let pos = formal_value(formals, relation.pos_binding(), relation)?;
    let src = formal_value(formals, relation.src_binding(), relation)?;
    let end = formal_value(formals, relation.end_binding(), relation)?;
    let pred_chars = formal_value(formals, relation.pred_chars_binding(), relation)?;
    let header = target_block(targets, DynamicV2PhysicalTargetRoleV1::Header);
    let body = target_block(targets, DynamicV2PhysicalTargetRoleV1::BodyPrelude);
    let continuation = target_block(targets, DynamicV2PhysicalTargetRoleV1::Continuation);
    let then_terminal = target_block(targets, DynamicV2PhysicalTargetRoleV1::ThenTerminal);
    let after = target_block(targets, DynamicV2PhysicalTargetRoleV1::After);
    let header_current = formals.header_current();
    check_binding_receipt(
        header_current,
        relation.owner(),
        relation.induction_binding(),
        header,
        "header-current",
    )?;
    if formals.value_for_recipe(relation.entry_value()) != Some(pos) {
        return Err("[freeze:contract][selected-dynamic/body-bridge/entry-value]".to_owned());
    }

    let v4 = ledger_value(
        values,
        V4,
        I0,
        header,
        DynamicV2PhysicalRepresentationV1::ImmediateI64,
        "condition-i",
    )?;
    let v6 = ledger_value(
        values,
        V6,
        I2,
        body,
        DynamicV2PhysicalRepresentationV1::ImmediateI64,
        "substring-start",
    )?;
    let v7 = ledger_value(
        values,
        V7,
        I3,
        body,
        DynamicV2PhysicalRepresentationV1::ImmediateI64,
        "substring-end",
    )?;
    let v10_block = corridor.with_i6_normal(|target| target.block());
    let v10 = ledger_value(
        values,
        V10,
        I6,
        v10_block,
        DynamicV2PhysicalRepresentationV1::EndAuthorizedHandle { lease_slot },
        "iteration-local",
    )?;
    let v14_block = then_terminal;
    let _v14 = ledger_value(
        values,
        V14,
        I11,
        v14_block,
        DynamicV2PhysicalRepresentationV1::ImmediateI64,
        "inner-return",
    )?;
    let v15 = ledger_value(
        values,
        V15,
        I13,
        continuation,
        DynamicV2PhysicalRepresentationV1::ImmediateI64,
        "step-read",
    )?;
    let v17 = ledger_value(
        values,
        V17,
        I15,
        continuation,
        DynamicV2PhysicalRepresentationV1::ImmediateI64,
        "backedge",
    )?;

    observe_preloop_alias(state, relation, pos)?;
    observe_existing_iteration_local(state, relation, v10)?;
    observe_reads(
        state,
        [
            (
                relation.initializer().node().clone(),
                relation.pos_binding(),
                pos,
            ),
            (
                relation.condition_i().node().clone(),
                relation.induction_binding(),
                pos,
            ),
            (
                relation.condition_end().node().clone(),
                relation.end_binding(),
                end,
            ),
            (
                relation.substring_receiver().node().clone(),
                relation.src_binding(),
                src,
            ),
            (
                relation.substring_start().node().clone(),
                relation.induction_binding(),
                pos,
            ),
            (
                relation.substring_end_i().node().clone(),
                relation.induction_binding(),
                pos,
            ),
            (
                relation.index_of_receiver().node().clone(),
                relation.pred_chars_binding(),
                pred_chars,
            ),
            (
                relation.iteration_local().read().node().clone(),
                relation.iteration_local().binding(),
                v10,
            ),
            (
                relation.inner_return_i().node().clone(),
                relation.induction_binding(),
                pos,
            ),
            (
                relation.step_read_i().node().clone(),
                relation.induction_binding(),
                pos,
            ),
        ],
    )?;

    if state.has_dynamic_origin(relation.induction_binding()) {
        let prepared = state.prepare_source_backed_dynamic_rebind(
            relation.step_target_i().node(),
            relation.induction_binding(),
            pos,
            v17,
            relation.induction_binding(),
        )?;
        state.commit_source_backed_dynamic_rebind(prepared);
    } else {
        let prepared = state.prepare_source_backed_static_rebind(
            relation.step_target_i().node(),
            relation.induction_binding(),
            pos,
            v17,
        )?;
        state.commit_source_backed_static_rebind(prepared);
    }

    let outer_return = profile.outer_return();
    check_binding_receipt(
        outer_return,
        relation.owner(),
        relation.induction_binding(),
        after,
        "outer-return",
    )?;
    state.observe_tail_site(
        relation.outer_return_i().node(),
        relation.induction_binding(),
        v17,
    )?;

    if v4 != header_current.physical_value()
        || v6 != header_current.physical_value()
        || v7 != header_current.physical_value()
        || v15 != header_current.physical_value()
    {
        return Err("[freeze:contract][selected-dynamic/body-bridge/carrier-value]".to_owned());
    }
    Ok(())
}

fn observe_preloop_alias(
    state: &mut CallableSemanticLoweringState,
    relation: &DynamicAPrimeI64SourceRelationViewV1<'_>,
    value: ValueId,
) -> Result<(), String> {
    let SourceBindingSiteV1::Local { statement, ordinal } = relation.induction_declaration() else {
        return Err(
            "[freeze:contract][selected-dynamic/body-bridge/induction-declaration]".to_owned(),
        );
    };
    state.observe_preloop_alias(
        statement.node(),
        relation.induction_binding(),
        relation.pos_binding(),
        value,
        *ordinal,
    )
}

fn observe_existing_iteration_local(
    state: &mut CallableSemanticLoweringState,
    relation: &DynamicAPrimeI64SourceRelationViewV1<'_>,
    value: ValueId,
) -> Result<(), String> {
    let local = relation.iteration_local();
    state.observe_existing_local(local.declaration_statement().node(), local.binding(), value)
}

fn observe_reads(
    state: &mut CallableSemanticLoweringState,
    reads: [(SourceNodeSiteV1, BindingRefV1, ValueId); 10],
) -> Result<(), String> {
    for (site, binding, value) in reads {
        state.observe_variable_site(&site, binding, value)?;
    }
    Ok(())
}

fn formal_value(
    formals: &DynamicV2OpenedFormalHeaderV1,
    binding: BindingRefV1,
    relation: &DynamicAPrimeI64SourceRelationViewV1<'_>,
) -> Result<ValueId, String> {
    let row = relation
        .formal_rows()
        .into_iter()
        .find(|row| row.binding() == binding)
        .ok_or_else(|| "[freeze:contract][selected-dynamic/body-bridge/formal]".to_owned())?;
    formals
        .value_for_recipe(row.recipe_value())
        .ok_or_else(|| "[freeze:contract][selected-dynamic/body-bridge/formal-value]".to_owned())
}

fn target_block(
    targets: &DynamicV2PhysicalTargetSetV1,
    role: DynamicV2PhysicalTargetRoleV1,
) -> BasicBlockId {
    targets.with_role(role, |target| target.block())
}

fn ledger_value(
    values: &DynamicV2PhysicalValueLedgerV1,
    result: LoopValueKeyV1,
    producer: LoopItemKeyV1,
    block: BasicBlockId,
    representation: DynamicV2PhysicalRepresentationV1,
    role: &str,
) -> Result<ValueId, String> {
    values
        .with_value(result, representation, |view| {
            if view.producer() == producer && view.result() == result && view.block() == block {
                Ok(view.value())
            } else {
                Err(format!(
                    "[freeze:contract][selected-dynamic/body-bridge/{role}-row]"
                ))
            }
        })
        .map_err(|error| {
            format!("[freeze:contract][selected-dynamic/body-bridge/{role}-ledger/{error:?}]")
        })?
}

fn check_binding_receipt(
    receipt: crate::mir::builder::resolved_lowering::canonical_ssa::CanonicalBindingReadReceiptV1,
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    binding: BindingRefV1,
    block: BasicBlockId,
    role: &str,
) -> Result<(), String> {
    if receipt.owner() != owner || receipt.binding() != binding || receipt.physical_block() != block
    {
        return Err(format!(
            "[freeze:contract][selected-dynamic/body-bridge/{role}-receipt]"
        ));
    }
    Ok(())
}
