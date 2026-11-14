//! Fulfil the existing exact-read obligations before value/CFG emission.
//! This bundle transports read receipts; it owns no expression graph or cleanup.
use super::*;

pub(super) struct ExpressionReadInputsV1 {
    pub(super) reads: Vec<Option<crate::mir::builder::ExactLexicalReadV1>>,
    pub(super) path_reads:
        std::collections::BTreeMap<OwnedExprSiteV1, Rc<crate::mir::builder::ExactLexicalReadV1>>,
    pub(super) reference_reads: std::collections::BTreeMap<usize, ExpressionReferenceReadV1>,
    pub(super) direct_fields: std::collections::BTreeMap<
        usize,
        (
            ValueId,
            Rc<crate::mir::normal_callable_semantic_package::LocalFieldReadSourceV1>,
            usize,
        ),
    >,
    pub(super) inline_start: Option<(BasicBlockId, usize, bool)>,
    pub(super) needs_fault_frame: bool,
}
pub(super) fn fulfill_v1(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    frame: &CheckedExpressionEvaluationV1,
    order: &[usize],
) -> Result<ExpressionReadInputsV1, String> {
    let owner = frame.source().owner();
    let source = frame.source();
    let mut pending = Vec::new();
    let mut path_slots = std::collections::BTreeMap::new();
    let mut reference_slots = std::collections::BTreeMap::new();
    let mut direct_fields = std::collections::BTreeMap::new();
    // An array alias or a named-local Map receiver is read once, by its Core
    // call, through one shared path read.
    for core in frame
        .cores()
        .values()
        .filter(|core| order.contains(&core.node()))
    {
        let Some((site, binding)) = core.path_read_receiver_v1() else {
            continue;
        };
        if path_slots.contains_key(&site) {
            continue;
        }
        let read = state
            .prepare_exact_lexical_read(owner, site.site().node(), binding)
            .map_err(|error| error.to_string())?;
        path_slots.insert(site, pending.len());
        pending.push(read);
    }
    // Fulfill every field-root read before any FaultFrame, CFG or value effect.
    // Reservation remains in original source order; binding nested receivers
    // still waits for the actual ancestor field writer below.
    for &index in order {
        if frame.transfers_node(index)
            && !frame
                .direct_array_receiver_v1(index)
                .is_some_and(|core| order.contains(&core.node()))
        {
            continue;
        }
        let node = &source.nodes()[index];
        if frame.node_is_identity(index)
            && !matches!(
                node.source(),
                Node::Shape(
                    crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { .. }
                )
            )
        {
            continue;
        }
        let site = OwnedExprSiteV1::new(owner, node.site().clone());
        if let Some(original) = frame.references().get(&index) {
            let read = state
                .prepare_exact_lexical_read(owner, node.site().node(), original.binding())
                .map_err(|error| error.to_string())?;
            let producer = ExpressionReferenceReadV1::prepare(
                frame,
                index,
                read.observation_v1(),
                CallValidationContextV1::Lowering(ledger),
                builder
                    .function_state
                    .current_function
                    .as_ref()
                    .ok_or_else(|| freeze("no-function"))?,
            )?;
            reference_slots.insert(index, (pending.len(), producer));
            pending.push(read);
        }
        if let Some(path) = ledger.statement_field_path_v1(
            &site,
            crate::mir::normal_callable_semantic_package::CallValidationContextV1::Lowering(ledger),
        )? {
            if !ledger.reserve_result_path_field_v1(&site, |binding| {
                let root_site = OwnedExprSiteV1::new(owner, path.root_site().clone());
                let slot = match path_slots.get(&root_site) {
                    Some(slot) => *slot,
                    None => {
                        let read = state
                            .prepare_exact_lexical_read(owner, path.root_site().node(), binding)
                            .map_err(|error| error.to_string())?;
                        let slot = pending.len();
                        pending.push(read);
                        path_slots.insert(root_site, slot);
                        slot
                    }
                };
                pending[slot]
                    .value_for(owner, path.root_site().node(), binding)
                    .map_err(|error| error.to_string())
            })? {
                return Err(freeze("print-expression/path-source-missing"));
            }
        } else if let Node::Shape(
            crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { object, .. },
        ) = node.source()
        {
            let mut slot = None;
            let (base, descriptor) = ledger
                .take_local_field_read(&site, |binding| {
                    let read = state
                        .prepare_exact_lexical_read(owner, object.node(), binding)
                        .map_err(|error| error.to_string())?;
                    let value = read
                        .value_for(owner, object.node(), binding)
                        .map_err(|error| error.to_string())?;
                    slot = Some(pending.len());
                    if frame.direct_array_receiver_v1(index).is_some() {
                        let root_site = OwnedExprSiteV1::new(owner, object.clone());
                        if path_slots.insert(root_site, pending.len()).is_some() {
                            return Err(freeze("expression-value/read-slot"));
                        }
                    }
                    pending.push(read);
                    Ok(value)
                })?
                .ok_or_else(|| freeze("print-expression/field-source-missing"))?;
            direct_fields.insert(
                index,
                (
                    base,
                    descriptor,
                    slot.ok_or_else(|| freeze("print-expression/field-exact-read-missing"))?,
                ),
            );
        }
    }
    let needs_fault_frame = ledger.expression_requires_fault_frame_v1(frame);
    let inline_start = (!needs_fault_frame)
        .then(|| cleanup::begin_inline_scope_v1(builder, state))
        .transpose()?;
    let mut reads: Vec<_> = state
        .commit_exact_lexical_reads(pending)
        .map_err(|error| error.to_string())?
        .into_iter()
        .map(Some)
        .collect();
    let mut path_reads = std::collections::BTreeMap::new();
    for (site, slot) in path_slots {
        path_reads.insert(
            site,
            Rc::new(
                reads[slot]
                    .take()
                    .ok_or_else(|| freeze("expression-value/read-slot"))?,
            ),
        );
    }
    let reference_reads = reference_slots
        .into_iter()
        .map(|(node, (slot, producer))| {
            Ok((
                node,
                producer.finish(
                    reads[slot]
                        .take()
                        .ok_or_else(|| freeze("expression-value/reference-slot"))?,
                ),
            ))
        })
        .collect::<Result<std::collections::BTreeMap<_, _>, String>>()?;
    Ok(ExpressionReadInputsV1 {
        reads,
        path_reads,
        reference_reads,
        direct_fields,
        inline_start,
        needs_fault_frame,
    })
}
