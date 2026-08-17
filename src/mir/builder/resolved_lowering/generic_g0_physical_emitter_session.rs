//! Caller-zero Generic G0 whole-admission session preflight.
//!
//! This is the first consumer of the combined prephysical admission.  It
//! materializes a shell, adopts the source entry lanes, reads the canonical
//! preheader rows, and allocates layout-keyed segment blocks.  The callback may
//! consume the existing common leaf dispatcher, while the unpublished outer
//! transaction owns every rollback.

use crate::mir::builder::resolved_lowering::canonical_ssa::CanonicalSsaFunctionSessionV2;
use crate::mir::builder::resolved_lowering::loop_recipe_physicalizer::ReadyLoopEntryV1;
use crate::mir::builder::resolved_lowering::loop_recipe_physicalizer::{
    allocate_for_layout, emit_loop_segment_operation_dispatch_v1,
    preflight_loop_segment_operation_dispatch_v1, ready_loop_entry_from_canonical_rows,
    LoopOperationDispatchServicesV1, LoopPhysicalSegmentBlockReceiptV1, LoopPhysicalServicesV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::compiler::generic_g0_physical_function_entry_input::GenericG0PhysicalLaneRoleV1;
use crate::mir::compiler::generic_g0_physical_operation_cohort::{
    issue_generic_g0_physical_emitter_admission_v1, PreparedGenericG0PhysicalEmitterAdmissionV1,
};
use crate::mir::function::MirParamDecl;
use crate::mir::loop_recipe_contract::PreparedLoopPhysicalLayoutV1;
use crate::mir::loop_route_policy::CanonicalLoopFamilySelectionV1;
use std::marker::PhantomData;

/// Mechanical dispatch inputs branded to one callback scope.  The input owns
/// the already-issued receipts, so a later consumer cannot re-pair a segment
/// receipt with another admission.  It contains no source meaning or leaf
/// instruction state.
struct GenericG0SegmentDispatchInputV1<'scope> {
    layout: PreparedLoopPhysicalLayoutV1,
    entry: ReadyLoopEntryV1,
    segment_receipt: LoopPhysicalSegmentBlockReceiptV1,
    _scope: PhantomData<&'scope mut ()>,
}

impl GenericG0SegmentDispatchInputV1<'_> {
    fn preflight(self) -> Result<(), String> {
        preflight_loop_segment_operation_dispatch_v1(self.layout, self.entry, self.segment_receipt)
    }

    fn emit(self, services: &mut LoopOperationDispatchServicesV1<'_, '_>) -> Result<usize, String> {
        emit_loop_segment_operation_dispatch_v1(
            self.layout,
            self.entry,
            self.segment_receipt,
            services,
        )
    }
}

fn with_dispatch_input<R>(
    layout: PreparedLoopPhysicalLayoutV1,
    entry: ReadyLoopEntryV1,
    segment_receipt: LoopPhysicalSegmentBlockReceiptV1,
    callback: impl for<'scope> FnOnce(GenericG0SegmentDispatchInputV1<'scope>) -> Result<R, String>,
) -> Result<R, String> {
    callback(GenericG0SegmentDispatchInputV1 {
        layout,
        entry,
        segment_receipt,
        _scope: PhantomData,
    })
}

/// Consume one whole Generic admission, run the unpublished
/// shell/entry/segment preflight, and lend one branded dispatcher input.  The
/// callback remains caller-zero: it cannot publish a module, and every late
/// failure returns through the outer discard owner.
pub(in crate::mir::builder) fn with_generic_g0_physical_emitter_session_preflight<R>(
    builder: &mut MirBuilder,
    admission: PreparedGenericG0PhysicalEmitterAdmissionV1<'_>,
    callback: impl for<'scope> FnOnce(
        &mut CanonicalSsaFunctionSessionV2<'_>,
        &mut MirBuilder,
        GenericG0SegmentDispatchInputV1<'scope>,
    ) -> Result<R, String>,
) -> Result<R, String> {
    if builder.function_state.current_function.is_some()
        || builder.function_state.current_block.is_some()
    {
        return Err("generic emitter preflight requires an empty Builder".to_owned());
    }

    let mut preflight = admission.into_session_preflight();
    let input = preflight.input();
    let function_name = preflight.shell_plan().symbol().as_mir_name().to_owned();
    let expectation = preflight.take_expectation()?;
    let outer_if = preflight.take_outer_if()?;
    let mut outer = builder.open_resolved_function_draft_seal_session_v1(&function_name);
    let result = (|| {
        let draft = outer.builder_view_mut_for_lowering();
        draft
            .function_state
            .resolved_binding_state
            .install(input.function())?;

        let param_decls = preflight
            .shell_plan()
            .descriptors()
            .iter()
            .map(|descriptor| MirParamDecl {
                name: descriptor.diagnostic_name().to_owned(),
                declared_type_name: Some("i64".to_owned()),
                implicit_receiver: descriptor.role()
                    == GenericG0PhysicalLaneRoleV1::InstanceReceiver,
            })
            .collect::<Vec<_>>();
        draft.create_resolved_function_skeleton(
            function_name.clone(),
            &param_decls,
            Some(preflight.shell_plan().result_abi().source_type_name()),
            preflight.shell_plan().effects().effect_mask(),
        )?;

        let mut session = CanonicalSsaFunctionSessionV2::new_generic(
            input,
            outer_if,
            &expectation,
            preflight.completion(),
        )?;
        session.adopt_generic_g0_entry_lanes(draft, preflight.shell_plan().descriptors())?;

        let preheader = session.entry_block(draft)?;
        let mut canonical_rows = Vec::with_capacity(preflight.entries().len());
        for row in preflight.entries() {
            let receipt = session.identity.read_entry_receipt(
                draft,
                &mut session.phis,
                preheader,
                row.binding(),
            )?;
            if receipt.owner() != input.owner()
                || receipt.binding() != row.binding()
                || receipt.physical_block() != preheader
            {
                return Err("generic canonical entry receipt drift".to_owned());
            }
            canonical_rows.push((row.recipe_value(), row.binding(), receipt.physical_value()));
        }
        let ready_entry =
            ready_loop_entry_from_canonical_rows(input.owner(), preheader, canonical_rows);
        let mut services = LoopPhysicalServicesV1::new(draft, &mut session.cfg);
        let segment_receipt = allocate_for_layout(preflight.layout(), &ready_entry, &mut services)
            .map_err(|error| format!("generic segment preflight: {error:?}"))?;
        let layout = preflight.take_layout();
        with_dispatch_input(layout, ready_entry, segment_receipt, |dispatch_input| {
            callback(&mut session, draft, dispatch_input)
        })
    })();
    outer.discard_unpublished();
    result
}

#[cfg(test)]
mod tests {
    use super::with_generic_g0_physical_emitter_session_preflight;
    use crate::mir::builder::resolved_lowering::loop_recipe_physicalizer::LoopOperationDispatchServicesV1;
    use crate::mir::builder::MirBuilder;
    use crate::mir::compiler::generic_g0_physical_operation_cohort::issue_generic_g0_physical_emitter_admission_v1;
    use crate::mir::loop_route_policy::generic_source_unit_and_selection_for_test;

    #[test]
    fn whole_admission_preflight_reads_entries_and_allocates_segments() {
        let (unit, selection) = generic_source_unit_and_selection_for_test();
        let input = unit.root_function_input().expect("root input");
        let admission = issue_generic_g0_physical_emitter_admission_v1(input, selection)
            .expect("Generic emitter admission");
        let mut builder = MirBuilder::new();
        let result = with_generic_g0_physical_emitter_session_preflight(
            &mut builder,
            admission,
            |session, draft, dispatch_input| {
                assert_eq!(session.owner(), input.owner());
                assert!(draft.current_function_name().is_some());
                let mut services = LoopOperationDispatchServicesV1::new(
                    draft,
                    &mut session.identity,
                    &mut session.phis,
                );
                let emitted = dispatch_input
                    .emit(&mut services)
                    .expect("dispatch emission");
                assert!(emitted > 0);
                drop(services);
                assert!(draft
                    .function_state
                    .current_function
                    .as_ref()
                    .expect("unpublished shell")
                    .blocks
                    .values()
                    .any(|block| !block.instructions.is_empty()));
                Ok(())
            },
        );
        result.expect("session preflight");
        assert!(builder.current_function_name().is_none());
        assert!(builder.current_block_id().is_none());
    }

    #[test]
    fn preflight_rejects_an_occupied_builder_without_consuming_effects() {
        let (unit, selection) = generic_source_unit_and_selection_for_test();
        let input = unit.root_function_input().expect("root input");
        let admission = issue_generic_g0_physical_emitter_admission_v1(input, selection)
            .expect("Generic emitter admission");
        let mut builder = MirBuilder::new();
        builder
            .create_function_skeleton("occupied/0".to_owned(), &[], &[])
            .expect("occupied skeleton");
        let error = with_generic_g0_physical_emitter_session_preflight(
            &mut builder,
            admission,
            |_session, _draft, _dispatch_input| Ok(()),
        )
        .expect_err("occupied builder must reject");
        assert!(error.contains("empty Builder"));
        assert_eq!(builder.current_function_name(), Some("occupied/0"));
    }

    #[test]
    fn late_callback_failure_discards_the_whole_unpublished_candidate() {
        let (unit, selection) = generic_source_unit_and_selection_for_test();
        let input = unit.root_function_input().expect("root input");
        let admission = issue_generic_g0_physical_emitter_admission_v1(input, selection)
            .expect("Generic emitter admission");
        let mut builder = MirBuilder::new();
        let error = with_generic_g0_physical_emitter_session_preflight(
            &mut builder,
            admission,
            |session, draft, dispatch_input| {
                let mut services = LoopOperationDispatchServicesV1::new(
                    draft,
                    &mut session.identity,
                    &mut session.phis,
                );
                dispatch_input.emit(&mut services)?;
                Err::<(), _>("late emission failure".to_owned())
            },
        )
        .expect_err("late callback failure");
        assert_eq!(error, "late emission failure");
        assert!(builder.current_function_name().is_none());
        assert!(builder.current_block_id().is_none());
    }
}
