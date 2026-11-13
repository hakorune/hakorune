//! One `m.set("<key>", <i64>)` statement on a named-local Map, emitted from its
//! sealed store entry with the same checked key/install/outcome operations the
//! constructing literal uses. The Map value is the constructing emission's own.
use super::map::{invoke, record_literal};
use super::*;
use crate::mir::instruction::{MapInvokeOperation as Map, MapValueKind};
use crate::mir::resolved_semantics::home_new_prefix::{
    MapLocalStoreV1, MapValueSource, SourceScalarKind,
};
use crate::mir::resolved_semantics::OwnedExprSiteV1;

pub(in crate::mir::builder) fn emit(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    store: &MapLocalStoreV1,
    map_site: &OwnedExprSiteV1,
) -> Result<(), String> {
    if state.owner() != store.call().owner() {
        return Err(freeze("map-store-owner"));
    }
    let entry = store.entry();
    // The original receiver read is consumed here, so the Map value is the
    // local's own value and must be the one the construction emitted.
    let (receiver, binding) = store.receiver();
    let (_, _, map) = state
        .take_exact_lexical_read(store.call().owner(), receiver.node(), binding)
        .map_err(|error| error.to_string())?
        .source_value();
    if map != ledger.map_local_store_map_value_v1(map_site)? {
        return Err(freeze("map-store-receiver-drift"));
    }
    // Newest-first unwind of every Home live at the statement, the Map included.
    let unwind = ledger.prior_home_unwind_for(store.homes())?;
    let frame = state.borrow_fault_frame(builder)?;
    let mut bindings = vec![fault_frame_binding(builder, state, frame)?];
    let outward = builder.next_block_id();
    append_block(
        builder,
        outward,
        MirInstruction::ReturnFault { fault_frame: frame },
        &mut bindings,
    )?;
    let fault = cleanup_chain(
        builder,
        frame,
        &FaultCleanupPlanV1::unwind(unwind.clone()),
        outward,
        &mut bindings,
    )?;
    let Some(source) = entry.value_source() else {
        return Err(freeze("map-value-consumer-missing"));
    };
    if source.scalar_kind() != Some(SourceScalarKind::Integer) {
        return Err(freeze("map-value-consumer-missing"));
    }
    let value = match source {
        MapValueSource::Integer(n) => {
            let dst = crate::mir::builder::emission::constant::emit_integer(builder, *n)?;
            record_literal(
                builder,
                dst,
                crate::mir::ConstValue::Integer(*n),
                &mut bindings,
            )?;
            dst
        }
        MapValueSource::Local { binding, .. } => {
            let value = state.read_variable(entry.site().node())?;
            if state
                .value_for_exact_binding(store.call().owner(), *binding)
                .map_err(|_| freeze("map-scalar-binding"))?
                != value
            {
                return Err(freeze("map-scalar-binding-drift"));
            }
            value
        }
        _ => return Err(freeze("map-value-consumer-missing")),
    };
    let key = invoke(
        builder,
        frame,
        Map::PrepareKey {
            utf8: entry.key().into(),
        },
        fault,
        &mut bindings,
    )?
    .0
    .expect("key prepare produces an opaque result");
    let outcome = invoke(
        builder,
        frame,
        Map::InstallValue {
            map,
            key,
            value,
            kind: MapValueKind::I64,
        },
        fault,
        &mut bindings,
    )?
    .0
    .expect("install produces its detached outcome");
    // After the install commit, a displaced old value's End Faults with the
    // same live Homes; the committed entry is not undone.
    let committed = cleanup_chain(
        builder,
        frame,
        &FaultCleanupPlanV1::unwind(unwind),
        outward,
        &mut bindings,
    )?;
    invoke(
        builder,
        frame,
        Map::EndOutcome { outcome },
        committed,
        &mut bindings,
    )?;
    ledger.record_map_local_store_v1(map_site, bindings)
}
