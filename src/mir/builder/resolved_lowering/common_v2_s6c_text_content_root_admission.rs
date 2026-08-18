//! Source/entry co-seal for the first S6C Text content roots.
//!
//! The source corridor owns the Subject/Needle meaning and the entry bridge
//! owns physical lane/root indices.  This module only joins those existing
//! authorities once.  It owns no `ValueId`, runtime frame, pointer, lease, or
//! MIR effect; V9 remains a derived slice and never becomes a root.

use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1;
use crate::mir::loop_recipe_contract::S6CScalarScanSourceRefV1;
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1};

use super::common_v2_s6c_textref_entry_bridge::{
    CommonV2S6CTextRefEntryBridgePlanV1, CommonV2S6CTextRefEntryBridgeRowV1,
};

const SUBJECT_ROOT_INDEX: u32 = 0;
const NEEDLE_ROOT_INDEX: u32 = 1;
const SUBJECT_LOGICAL_ORDINAL: u32 = 0;
const NEEDLE_LOGICAL_ORDINAL: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum CommonV2S6CTextContentRootAdmissionRejectV1 {
    SourceOwnerMismatch,
    SourceBindingOwnerMismatch,
    RootCountMismatch,
    SubjectBindingMissingOrDuplicated,
    NeedleBindingMissingOrDuplicated,
    SubjectLogicalOrdinalMismatch,
    NeedleLogicalOrdinalMismatch,
    SubjectRootIndexMismatch,
    NeedleRootIndexMismatch,
    PublishedPairIndexMismatch,
    DuplicateRootIndex,
    CarrierMismatch,
    Callback(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum CommonV2S6CTextContentRootRoleV1 {
    Subject,
    Needle,
}

/// Physical-free projection of one existing ExactText entry occurrence.
/// Lane indices are indices into the already adopted entry parameters; they
/// are never slot or generation values themselves.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) struct CommonV2S6CTextContentRootRowV1 {
    role: CommonV2S6CTextContentRootRoleV1,
    binding: BindingRefV1,
    logical_ordinal: u32,
    root_index: u32,
    published_pair_index: u32,
    slot_lane_index: u32,
    generation_lane_index: u32,
    carrier: PhysicalCallableLaneCarrierV1,
}

impl CommonV2S6CTextContentRootRowV1 {
    pub(in crate::mir::builder) const fn role(self) -> CommonV2S6CTextContentRootRoleV1 {
        self.role
    }

    pub(in crate::mir::builder) const fn binding(self) -> BindingRefV1 {
        self.binding
    }

    pub(in crate::mir::builder) const fn logical_ordinal(self) -> u32 {
        self.logical_ordinal
    }

    pub(in crate::mir::builder) const fn root_index(self) -> u32 {
        self.root_index
    }

    pub(in crate::mir::builder) const fn published_pair_index(self) -> u32 {
        self.published_pair_index
    }

    pub(in crate::mir::builder) const fn slot_lane_index(self) -> u32 {
        self.slot_lane_index
    }

    pub(in crate::mir::builder) const fn generation_lane_index(self) -> u32 {
        self.generation_lane_index
    }

    pub(in crate::mir::builder) const fn carrier(self) -> PhysicalCallableLaneCarrierV1 {
        self.carrier
    }
}

/// One-shot source/physical root admission.  The source view is retained so
/// a later consumer cannot keep the physical rows while dropping the source
/// cohort; `consume` is the only way to lend both together.
#[must_use = "a text content root admission must be consumed exactly once"]
#[derive(Debug)]
pub(in crate::mir::builder) struct CommonV2S6CTextContentRootAdmissionV1<'source, 'rows, 'facts> {
    source: S6CScalarScanSourceRefV1<'source, 'rows, 'facts>,
    owner: FunctionOwnerIdV1,
    entry: crate::mir::BasicBlockId,
    plan_stamp: u64,
    roots: [CommonV2S6CTextContentRootRowV1; 2],
}

impl<'source, 'rows, 'facts> CommonV2S6CTextContentRootAdmissionV1<'source, 'rows, 'facts> {
    pub(in crate::mir::builder) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder) const fn entry(&self) -> crate::mir::BasicBlockId {
        self.entry
    }

    pub(in crate::mir::builder) const fn plan_stamp(&self) -> u64 {
        self.plan_stamp
    }

    /// Lend the source cohort and exactly two role-labelled roots together.
    /// No root row or source view is available after this consuming call.
    pub(in crate::mir::builder) fn consume<R>(
        self,
        callback: impl FnOnce(
            S6CScalarScanSourceRefV1<'source, 'rows, 'facts>,
            &[CommonV2S6CTextContentRootRowV1; 2],
        ) -> Result<R, String>,
    ) -> Result<R, CommonV2S6CTextContentRootAdmissionRejectV1> {
        callback(self.source, &self.roots)
            .map_err(CommonV2S6CTextContentRootAdmissionRejectV1::Callback)
    }
}

/// Co-seal the source Subject/Needle roles with one already-issued entry
/// bridge plan.  The bridge plan is consumed by value, so no second mapping
/// owner can survive this admission.
pub(in crate::mir::builder) fn issue_common_v2_s6c_text_content_root_admission_v1<
    'source,
    'rows,
    'facts,
>(
    source: S6CScalarScanSourceRefV1<'source, 'rows, 'facts>,
    plan: CommonV2S6CTextRefEntryBridgePlanV1,
) -> Result<
    CommonV2S6CTextContentRootAdmissionV1<'source, 'rows, 'facts>,
    CommonV2S6CTextContentRootAdmissionRejectV1,
> {
    let owner = plan.owner();
    if source.owner() != owner {
        return Err(CommonV2S6CTextContentRootAdmissionRejectV1::SourceOwnerMismatch);
    }
    if source.subject_binding().owner() != owner
        || source.needle_binding().owner() != owner
        || source.subject_binding() == source.needle_binding()
    {
        return Err(CommonV2S6CTextContentRootAdmissionRejectV1::SourceBindingOwnerMismatch);
    }

    let entry = plan.entry();
    let plan_stamp = plan.plan_stamp();
    let rows = plan.rows();
    if rows.len() != 2 {
        return Err(CommonV2S6CTextContentRootAdmissionRejectV1::RootCountMismatch);
    }

    let subject = unique_binding_row(rows, source.subject_binding())
        .ok_or(CommonV2S6CTextContentRootAdmissionRejectV1::SubjectBindingMissingOrDuplicated)?;
    let needle = unique_binding_row(rows, source.needle_binding())
        .ok_or(CommonV2S6CTextContentRootAdmissionRejectV1::NeedleBindingMissingOrDuplicated)?;

    if subject.logical_ordinal() != SUBJECT_LOGICAL_ORDINAL {
        return Err(CommonV2S6CTextContentRootAdmissionRejectV1::SubjectLogicalOrdinalMismatch);
    }
    if needle.logical_ordinal() != NEEDLE_LOGICAL_ORDINAL {
        return Err(CommonV2S6CTextContentRootAdmissionRejectV1::NeedleLogicalOrdinalMismatch);
    }
    if subject.root_index() != SUBJECT_ROOT_INDEX {
        return Err(CommonV2S6CTextContentRootAdmissionRejectV1::SubjectRootIndexMismatch);
    }
    if needle.root_index() != NEEDLE_ROOT_INDEX {
        return Err(CommonV2S6CTextContentRootAdmissionRejectV1::NeedleRootIndexMismatch);
    }
    if subject.published_pair_index() != SUBJECT_ROOT_INDEX
        || needle.published_pair_index() != NEEDLE_ROOT_INDEX
    {
        return Err(CommonV2S6CTextContentRootAdmissionRejectV1::PublishedPairIndexMismatch);
    }
    if subject.carrier() != PhysicalCallableLaneCarrierV1::U64BitsOnI64
        || needle.carrier() != PhysicalCallableLaneCarrierV1::U64BitsOnI64
    {
        return Err(CommonV2S6CTextContentRootAdmissionRejectV1::CarrierMismatch);
    }
    if rows.iter().any(|row| {
        rows.iter()
            .filter(|other| other.root_index() == row.root_index())
            .count()
            != 1
    }) {
        return Err(CommonV2S6CTextContentRootAdmissionRejectV1::DuplicateRootIndex);
    }

    let subject = root_row(CommonV2S6CTextContentRootRoleV1::Subject, subject);
    let needle = root_row(CommonV2S6CTextContentRootRoleV1::Needle, needle);
    Ok(CommonV2S6CTextContentRootAdmissionV1 {
        source,
        owner,
        entry,
        plan_stamp,
        roots: [subject, needle],
    })
}

fn unique_binding_row(
    rows: &[CommonV2S6CTextRefEntryBridgeRowV1],
    binding: BindingRefV1,
) -> Option<CommonV2S6CTextRefEntryBridgeRowV1> {
    let mut matches = rows.iter().filter(|row| row.binding() == binding);
    let row = matches.next().copied()?;
    matches.next().is_none().then_some(row)
}

fn root_row(
    role: CommonV2S6CTextContentRootRoleV1,
    row: CommonV2S6CTextRefEntryBridgeRowV1,
) -> CommonV2S6CTextContentRootRowV1 {
    CommonV2S6CTextContentRootRowV1 {
        role,
        binding: row.binding(),
        logical_ordinal: row.logical_ordinal(),
        root_index: row.root_index(),
        published_pair_index: row.published_pair_index(),
        slot_lane_index: row.slot_lane_index(),
        generation_lane_index: row.generation_lane_index(),
        carrier: row.carrier(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::resolved_lowering::physical_entry_lane_adoption::{
        PhysicalTextEntryLaneSidecarRowV1, PhysicalTextEntryLaneSidecarV1,
    };
    use crate::mir::builder::resolved_lowering::{
        issue_common_v2_s6c_text_cursor_preheader_v1, CommonV2S6CTextCursorPreheaderRejectV1,
    };
    use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1;
    use crate::mir::loop_recipe_contract::{
        issue_s6c_prephysical_ingress_v2, issue_s6c_scan_with_init_logical_output_v1,
        produce_s6c_scan_with_init_recipe_v2, S6CScalarScanSourceRejectV1,
        VerifiedS6CPrephysicalIngressV2,
    };
    use crate::mir::resolved_semantics::{
        CoreMethodInstanceTargetIssuerV1, FunctionOwnerIssuerV1, FunctionSemanticResolverSessionV1,
    };
    use crate::mir::BindingId;
    use crate::parser::{NyashParser, ParserBuildConfig};

    const FIXTURE: &str = include_str!("../../../../apps/tests/scan_with_init_typed_ok_min.hako");

    fn issue_facts(
        source: &str,
        ordinal: u32,
    ) -> crate::mir::loop_structural_facts::VerifiedS6CScanWithInitFactsV1 {
        let parsed = NyashParser::parse_normal_callable_program_with_build_config(
            source,
            ParserBuildConfig::default(),
        )
        .expect("normal callable source");
        let source = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
            let transformed = crate::r#macro::transform_normal_callable_program_v1(parsed)
                .expect("exact callable transform");
            let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
                transformed
            else {
                panic!("fixture must remain source-backed")
            };
            source
        });
        let mut resolver = FunctionSemanticResolverSessionV1::new(ordinal).unwrap();
        let batch = crate::mir::callable_semantic_batch::issue_resolved_callable_semantic_batch_v1(
            &mut resolver,
            source,
        )
        .unwrap();
        let completion = batch
            .with_lowering_input(
                0,
                crate::mir::resolved_control_flow::verify_function_completion_v1,
            )
            .unwrap()
            .unwrap();
        let coseal = batch
            .with_declaration_semantics(|view| {
                let row = &view.declarations()[0];
                let loop_site = row.function().only_loop_site().unwrap();
                let typed = crate::mir::callable_semantic_batch::issue_s6c_typed_input_relation_v1(
                    row, &loop_site,
                )
                .unwrap();
                let mut targets = CoreMethodInstanceTargetIssuerV1::string_box_text(
                    crate::mir::core_method_result_kind::CORE_METHOD_MANIFEST_BRAND_V2,
                )
                .unwrap();
                let length = targets
                    .issue(
                        crate::mir::core_method_result_kind::issue_core_method_manifest_row_ref_v2(
                            crate::mir::core_method_op::CoreMethodOp::StringLen,
                            0,
                        )
                        .unwrap(),
                    )
                    .unwrap();
                let substring = targets
                    .issue(
                        crate::mir::core_method_result_kind::issue_core_method_manifest_row_ref_v2(
                            crate::mir::core_method_op::CoreMethodOp::StringSubstring,
                            2,
                        )
                        .unwrap(),
                    )
                    .unwrap();
                row.with_source_ledger(|ledger| {
                    let calls =
                        crate::mir::source_call_target::issue_source_bound_s6c_call_relation_v1(
                            &ledger, typed, length, substring,
                        )
                        .unwrap();
                    crate::mir::loop_structural_facts::issue_s6c_exit_tail_source_coseal_v1(
                        &ledger, calls, completion,
                    )
                })
                .unwrap()
            })
            .unwrap();
        crate::mir::loop_structural_facts::issue_s6c_scan_with_init_facts_v1(
            coseal.expect("Exit/Tail source co-seal"),
        )
        .expect("closed S6C Facts")
    }

    fn ingress(ordinal: u32) -> VerifiedS6CPrephysicalIngressV2 {
        issue_s6c_prephysical_ingress_v2(
            issue_s6c_scan_with_init_logical_output_v1(
                produce_s6c_scan_with_init_recipe_v2(issue_facts(FIXTURE, ordinal))
                    .expect("exact S6C Recipe product"),
            )
            .expect("logical output rows"),
        )
        .expect("prephysical ingress")
    }

    fn owner() -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().expect("owner issuer");
        issuer.issue().expect("owner")
    }

    fn bridge_plan(
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        rows: &[(BindingRefV1, u32, u32, u32)],
    ) -> CommonV2S6CTextRefEntryBridgePlanV1 {
        let rows = rows
            .iter()
            .map(|&(binding, ordinal, slot, generation)| {
                PhysicalTextEntryLaneSidecarRowV1::new(
                    binding,
                    ordinal,
                    crate::mir::ValueId::new(slot),
                    crate::mir::ValueId::new(generation),
                    PhysicalCallableLaneCarrierV1::U64BitsOnI64,
                )
            })
            .collect();
        let sidecar =
            PhysicalTextEntryLaneSidecarV1::new(owner, crate::mir::BasicBlockId::new(3), rows);
        super::super::common_v2_s6c_textref_entry_bridge::
            issue_common_v2_s6c_textref_entry_bridge_plan_v1(&sidecar, 77)
            .expect("bridge plan")
    }

    fn with_source<R>(
        ingress: &VerifiedS6CPrephysicalIngressV2,
        callback: impl FnOnce(S6CScalarScanSourceRefV1<'_, '_, '_>) -> R,
    ) -> R {
        ingress
            .with_scalar_scan_source(|source| {
                Ok::<_, S6CScalarScanSourceRejectV1>(callback(source))
            })
            .expect("source corridor")
    }

    #[test]
    fn admission_labels_subject_root_zero_and_needle_root_one() {
        let ingress = ingress(1201);
        with_source(&ingress, |source| {
            let owner = source.owner();
            let plan = bridge_plan(
                owner,
                &[
                    (source.subject_binding(), 0, 1, 2),
                    (source.needle_binding(), 1, 3, 4),
                ],
            );
            let admission = issue_common_v2_s6c_text_content_root_admission_v1(source, plan)
                .expect("base-root admission");
            assert_eq!(admission.owner(), owner);
            assert_eq!(admission.entry(), crate::mir::BasicBlockId::new(3));
            assert_eq!(admission.plan_stamp(), 77);
            admission
                .consume(|source, roots| {
                    assert_eq!(source.subject_binding(), roots[0].binding());
                    assert_eq!(source.needle_binding(), roots[1].binding());
                    assert_eq!(roots[0].role(), CommonV2S6CTextContentRootRoleV1::Subject);
                    assert_eq!(roots[1].role(), CommonV2S6CTextContentRootRoleV1::Needle);
                    assert_eq!(roots[0].root_index(), 0);
                    assert_eq!(roots[1].root_index(), 1);
                    assert_eq!(roots[0].slot_lane_index(), 1);
                    assert_eq!(roots[1].slot_lane_index(), 3);
                    Ok(())
                })
                .expect("one-shot root consumer");
        });
    }

    #[test]
    fn admission_rejects_swapped_source_roles_before_effect() {
        let ingress = ingress(1202);
        with_source(&ingress, |source| {
            let plan = bridge_plan(
                source.owner(),
                &[
                    (source.needle_binding(), 0, 1, 2),
                    (source.subject_binding(), 1, 3, 4),
                ],
            );
            assert!(matches!(
                issue_common_v2_s6c_text_content_root_admission_v1(source, plan),
                Err(CommonV2S6CTextContentRootAdmissionRejectV1::SubjectLogicalOrdinalMismatch)
                    | Err(
                        CommonV2S6CTextContentRootAdmissionRejectV1::NeedleLogicalOrdinalMismatch
                    )
            ));
        });
    }

    #[test]
    fn admission_rejects_missing_root_and_foreign_owner() {
        let ingress = ingress(1203);
        with_source(&ingress, |source| {
            let missing = bridge_plan(source.owner(), &[(source.subject_binding(), 0, 1, 2)]);
            assert!(matches!(
                issue_common_v2_s6c_text_content_root_admission_v1(source, missing),
                Err(CommonV2S6CTextContentRootAdmissionRejectV1::RootCountMismatch)
            ));

            let foreign_owner = owner();
            let foreign = bridge_plan(
                foreign_owner,
                &[
                    (BindingRefV1::new(foreign_owner, BindingId::new(0)), 0, 1, 2),
                    (BindingRefV1::new(foreign_owner, BindingId::new(1)), 1, 3, 4),
                ],
            );
            assert!(matches!(
                issue_common_v2_s6c_text_content_root_admission_v1(source, foreign),
                Err(CommonV2S6CTextContentRootAdmissionRejectV1::SourceOwnerMismatch)
            ));
        });
    }

    #[test]
    fn cursor_preheader_consumes_admission_and_keeps_two_roots_together() {
        let ingress = ingress(1204);
        with_source(&ingress, |source| {
            let plan = bridge_plan(
                source.owner(),
                &[
                    (source.subject_binding(), 0, 1, 2),
                    (source.needle_binding(), 1, 3, 4),
                ],
            );
            let admission = issue_common_v2_s6c_text_content_root_admission_v1(source, plan)
                .expect("base-root admission");
            let cursor = issue_common_v2_s6c_text_cursor_preheader_v1(admission)
                .expect("cursor/preheader plan");
            assert_eq!(cursor.owner(), source.owner());
            assert_eq!(cursor.entry(), crate::mir::BasicBlockId::new(3));
            assert_eq!(cursor.root_plan_stamp(), 77);
            assert_eq!(cursor.initial().cp_index(), 0);
            assert_eq!(cursor.initial().byte_offset(), 0);
            cursor
                .consume(|source, roots, initial, relation| {
                    assert_eq!(roots[0].role(), CommonV2S6CTextContentRootRoleV1::Subject);
                    assert_eq!(roots[1].role(), CommonV2S6CTextContentRootRoleV1::Needle);
                    assert_eq!(roots[0].root().root_index(), 0);
                    assert_eq!(roots[1].root().root_index(), 1);
                    assert_eq!(initial.cp_index(), source.initial_index());
                    assert_eq!(initial.byte_offset(), 0);
                    assert_eq!(relation.index_binding(), source.index_binding());
                    assert_eq!(relation.index_input(), source.index_input());
                    assert_eq!(relation.length_result(), source.length_result());
                    assert_eq!(relation.substring_result(), source.substring_result());
                    assert_eq!(relation.slice_end(), source.slice_end());
                    assert_eq!(relation.text_equal_item(), source.text_equal_item());
                    assert_eq!(relation.text_equal_result(), source.text_equal_result());
                    assert_eq!(relation.text_equal_if(), source.text_equal_if());
                    assert_eq!(relation.step_add(), source.step_add());
                    Ok(())
                })
                .expect("one-shot cursor consumer");
        });
    }

    #[test]
    fn cursor_preheader_callback_failure_is_typed_and_has_no_physical_effect() {
        let ingress = ingress(1205);
        with_source(&ingress, |source| {
            let plan = bridge_plan(
                source.owner(),
                &[
                    (source.subject_binding(), 0, 1, 2),
                    (source.needle_binding(), 1, 3, 4),
                ],
            );
            let admission = issue_common_v2_s6c_text_content_root_admission_v1(source, plan)
                .expect("base-root admission");
            let cursor = issue_common_v2_s6c_text_cursor_preheader_v1(admission)
                .expect("cursor/preheader plan");
            assert!(matches!(
                cursor.consume(|_, _, _, _| Err::<(), _>("late plan consumer".to_string())),
                Err(CommonV2S6CTextCursorPreheaderRejectV1::CursorInvariant(detail))
                    if detail == "late plan consumer"
            ));
        });
    }
}
