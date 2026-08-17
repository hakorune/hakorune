//! Caller-zero Generic G0 physical-entry/session consumer.

use crate::mir::builder::resolved_lowering::canonical_ssa::CanonicalSsaFunctionSessionV2;
use crate::mir::builder::MirBuilder;
use crate::mir::compiler::generic_g0_physical_entry_admission::GenericG0DetachedEntryCanaryV1;

/// Consume one Generic admission, install one unpublished shell, and adopt
/// only the receiver/ordinary declaration lanes.  The outer draft session is
/// the sole rollback owner; no module publication is reachable here.
pub(in crate::mir::builder) fn with_generic_g0_physical_entry_session<R>(
    builder: &mut MirBuilder,
    admission: GenericG0DetachedEntryCanaryV1<'_, '_>,
    callback: impl FnOnce(&mut CanonicalSsaFunctionSessionV2<'_>, &mut MirBuilder) -> Result<R, String>,
) -> Result<R, String> {
    if builder.function_state.current_function.is_some()
        || builder.function_state.current_block.is_some()
    {
        return Err("generic physical entry session requires an empty Builder".to_owned());
    }

    let (skeleton, expectation, outer_if, stamp) = admission.into_parts();
    let function_name = skeleton.function().signature.name.clone();
    let (parent, _effects, descriptors, detached) = skeleton.into_parts();
    let input = parent.source_input();
    stamp.validate(parent.owner(), &function_name, descriptors.len())?;
    let completion = parent.completion();
    let mut outer = builder.open_resolved_function_draft_seal_session_v1(&function_name);
    let result = (|| {
        let draft = outer.builder_view_mut_for_lowering();
        draft
            .function_state
            .resolved_binding_state
            .install(input.function())?;
        draft.install_prepared_physical_function_skeleton(detached)?;
        let mut session =
            CanonicalSsaFunctionSessionV2::new_generic(input, outer_if, &expectation, completion)?;
        session.adopt_generic_g0_entry_lanes(draft, &descriptors)?;
        callback(&mut session, draft)
    })();
    outer.discard_unpublished();
    result
}

#[cfg(test)]
mod tests {
    use super::with_generic_g0_physical_entry_session;
    use crate::mir::builder::MirBuilder;
    use crate::mir::compiler::generic_g0_physical_entry_admission::issue_generic_g0_detached_entry_canary_v1;
    use crate::mir::compiler::generic_g0_physical_function_entry_input::issue_generic_g0_physical_function_entry_input_v1;
    use crate::mir::compiler::generic_g0_physical_function_skeleton::reserve_generic_g0_physical_function_skeleton;
    use crate::mir::compiler::generic_g0_source_parent::with_generic_g0_source_parent_v1;
    use crate::mir::loop_route_policy::generic_source_unit_and_selection_for_test;
    use crate::mir::MirType;

    #[test]
    fn installs_and_adopts_one_unpublished_generic_entry() {
        let (unit, selection) = generic_source_unit_and_selection_for_test();
        let input = unit.root_function_input().expect("root input");
        let mut builder = MirBuilder::new();
        let result = with_generic_g0_source_parent_v1(input, selection, |parent| {
            let prepared = issue_generic_g0_physical_function_entry_input_v1(parent)
                .map_err(|_| "entry input".to_owned())?;
            let skeleton = reserve_generic_g0_physical_function_skeleton(prepared)
                .map_err(|_| "skeleton".to_owned())?;
            let admission = issue_generic_g0_detached_entry_canary_v1(skeleton)
                .map_err(|error| format!("admission: {error:?}"))?;
            with_generic_g0_physical_entry_session(&mut builder, admission, |session, draft| {
                let function = draft
                    .function_state
                    .current_function
                    .as_ref()
                    .ok_or_else(|| "missing installed function".to_owned())?;
                assert_eq!(function.signature.name, "generic_g0/2");
                assert_eq!(function.params.len(), 3);
                assert_eq!(function.signature.params, vec![MirType::Integer; 3]);
                assert_eq!(
                    draft.function_state.current_block,
                    Some(function.entry_block)
                );
                assert!(session
                    .adopt_generic_g0_entry_lanes(draft, &[])
                    .expect_err("duplicate adoption must reject")
                    .contains("already adopted"));
                Ok(())
            })
        });
        result
            .expect("Generic source parent")
            .expect("Generic entry session");
        assert!(builder.current_function_name().is_none());
        assert!(builder.current_block_id().is_none());
    }

    #[test]
    fn rejects_nonempty_builder_before_consuming_admission() {
        let (unit, selection) = generic_source_unit_and_selection_for_test();
        let input = unit.root_function_input().expect("root input");
        let mut builder = MirBuilder::new();
        builder
            .create_function_skeleton("occupied/0".to_owned(), &[], &[])
            .expect("test occupied skeleton");
        let result = with_generic_g0_source_parent_v1(input, selection, |parent| {
            let prepared = issue_generic_g0_physical_function_entry_input_v1(parent)
                .map_err(|_| "entry input".to_owned())?;
            let skeleton = reserve_generic_g0_physical_function_skeleton(prepared)
                .map_err(|_| "skeleton".to_owned())?;
            let admission = issue_generic_g0_detached_entry_canary_v1(skeleton)
                .map_err(|error| format!("admission: {error:?}"))?;
            with_generic_g0_physical_entry_session(&mut builder, admission, |_session, _draft| {
                Ok(())
            })
        });
        let error = result
            .expect("Generic source parent")
            .expect_err("occupied builder must reject");
        assert!(error.contains("empty Builder"));
        assert_eq!(builder.current_function_name(), Some("occupied/0"));
    }
}
