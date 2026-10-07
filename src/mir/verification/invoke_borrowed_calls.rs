//! Lend exact original ordinary Call slots to final module verification.
//! This is not a new semantic issuer. Physical publication still validates
//! producer provenance, full use closure and the immutable module snapshot.
use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1 as Carrier;
use crate::mir::finalized_root_handoff::FinalizedRootHandoffV1;
use crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1;
use crate::mir::{BasicBlockId, MirModule};
use std::collections::{BTreeMap, BTreeSet};

type CallSlots = BTreeMap<(String, BasicBlockId, usize), BTreeSet<usize>>;

pub(super) fn original_call_slots(
    module: &MirModule,
    root: Option<&FinalizedRootHandoffV1>,
) -> Result<CallSlots, String> {
    // Source-less verification retains its existing sealed scalar policy.
    let Some(root) = root else {
        return Ok(BTreeMap::new());
    };
    let selected: BTreeSet<_> = module
        .functions
        .values()
        .filter(|function| {
            function
                .metadata
                .physical_param_carriers
                .as_deref()
                .is_some_and(|rows| rows.contains(&Carrier::BorrowedTaggedValue))
        })
        .map(|function| function.signature.name.clone())
        .collect();
    let Some(source) = root.root_source() else {
        return if selected.is_empty() {
            Ok(BTreeMap::new())
        } else {
            Err("borrowed-source-missing".into())
        };
    };
    let mut loans = BTreeMap::new();
    let mut callees = BTreeSet::new();
    let mut incoming = BTreeSet::new();
    let mut original = BTreeSet::new();
    source.visit_finalized_lexical_call_nodes_v1(
        module,
        |_, _, packet, arguments, caller, coordinate, _| {
            let row = packet.original_row();
            let symbol = module
                .canonical_callable_definition_symbol(row.target())
                .ok_or_else(|| "borrowed-source-target-missing".to_owned())?;
            let selected_source = arguments
                .iter()
                .any(|argument| matches!(argument, LocalCallArgumentV1::BorrowedActual { .. }));
            if !selected_source && !selected.contains(symbol) {
                return Ok(());
            }
            if row.target().namespace()
                != hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod
            {
                return Err("borrowed-source-nonordinary-target".into());
            }
            let callee = module
                .functions
                .get(symbol)
                .ok_or_else(|| "borrowed-source-callee-missing".to_owned())?;
            let entry = source
                .borrowed_ordinary_entry_source_for_function_v1(row.callee_owner(), callee)?;
            let values = source.borrowed_ordinary_entry_values_v1(row.callee_owner())?;
            let carriers = callee
                .metadata
                .physical_param_carriers
                .as_deref()
                .ok_or_else(|| "borrowed-source-carrier-missing".to_owned())?;
            let offset = 1; // Original selected ordinary instance owner has `me`.
            if carriers.len() != callee.params.len()
                || callee.signature.params.len() != callee.params.len()
            {
                return Err("borrowed-source-entry-shape".into());
            }
            let slots: BTreeSet<_> = carriers
                .iter()
                .enumerate()
                .filter_map(|(slot, carrier)| {
                    (*carrier == Carrier::BorrowedTaggedValue).then_some(slot)
                })
                .collect();
            let mut proven = BTreeSet::new();
            for (ordinal, _, value) in values.iter() {
                let slot = *ordinal as usize + offset;
                if callee.params.get(slot) != Some(value)
                    || carriers.get(slot) != Some(&Carrier::BorrowedTaggedValue)
                {
                    return Err("borrowed-source-entry-correspondence".into());
                }
                proven.insert(slot);
            }
            if slots != proven || slots.is_empty() || slots.contains(&0) {
                return Err("borrowed-source-entry-coverage".into());
            }
            for target in entry.incoming_targets() {
                let target = target?;
                if target.target() != row.target() || target.callee_owner() != row.callee_owner() {
                    return Err("borrowed-source-target-drift".into());
                }
                incoming.insert(target.call_site().clone());
            }
            let actuals = source
                .borrowed_call_actuals_v1(row)?
                .ok_or_else(|| "borrowed-source-actuals-missing".to_owned())?;
            let actual_slots: BTreeSet<_> = actuals
                .iter()
                .map(|actual| actual.ordinal as usize)
                .collect();
            let argument_slots: BTreeSet<_> = slots.iter().map(|slot| slot - offset).collect();
            if actual_slots != argument_slots || actual_slots.len() != actuals.len() {
                return Err("borrowed-source-actual-coverage".into());
            }
            if loans
                .insert(
                    (caller.signature.name.clone(), coordinate.0, coordinate.1),
                    actual_slots,
                )
                .is_some()
            {
                return Err("borrowed-source-coordinate-duplicate".into());
            }
            callees.insert(symbol.to_owned());
            original.insert(row.call_site().clone());
            Ok(())
        },
    )?;
    if callees != selected || incoming != original {
        return Err("borrowed-source-incoming-coverage".into());
    }
    Ok(loans)
}
