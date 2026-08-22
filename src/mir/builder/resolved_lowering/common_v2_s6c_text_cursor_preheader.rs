//! Effect-free S6C cursor/preheader plan.
//!
//! This is the first consumer of the Subject/Needle base-root admission.  It
//! records the existing source relation and the two root-load obligations,
//! but it does not issue a `ValueId`, MIR instruction, access-plan row, block,
//! pointer, or runtime frame.  The canonical SSA session remains the only
//! physical issuer for the later leaf row.

use super::{
    CommonV2S6CTextContentRootAdmissionRejectV1, CommonV2S6CTextContentRootAdmissionV1,
    CommonV2S6CTextContentRootRoleV1, CommonV2S6CTextContentRootRowV1,
};
use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1;
use crate::mir::core_method_result_kind::CoreMethodSemanticLawV2;
use crate::mir::loop_recipe_contract::{LoopItemKeyV1, LoopValueKeyV1, S6CScalarScanSourceRefV1};
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum CommonV2S6CTextCursorPreheaderRejectV1 {
    RootAdmission(CommonV2S6CTextContentRootAdmissionRejectV1),
    SourceOwnerMismatch,
    SourceInitialIndexMismatch,
    LengthLawMismatch,
    SubstringLawMismatch,
    SubjectRootMismatch,
    NeedleRootMismatch,
    RootIndexMismatch,
    CarrierMismatch,
    CursorInvariant(String),
}

/// The only root operation admitted by this row: one pair load in the
/// preheader, with the role and entry-side lane provenance retained.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) struct CommonV2S6CTextCursorRootLoadV1 {
    root: CommonV2S6CTextContentRootRowV1,
}

impl CommonV2S6CTextCursorRootLoadV1 {
    pub(in crate::mir::builder) const fn role(self) -> CommonV2S6CTextContentRootRoleV1 {
        self.root.role()
    }

    pub(in crate::mir::builder) const fn root(self) -> CommonV2S6CTextContentRootRowV1 {
        self.root
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) struct CommonV2S6CTextCursorInitialStateV1 {
    cp_index: i64,
    byte_offset: u64,
}

impl CommonV2S6CTextCursorInitialStateV1 {
    pub(in crate::mir::builder) const fn cp_index(self) -> i64 {
        self.cp_index
    }

    pub(in crate::mir::builder) const fn byte_offset(self) -> u64 {
        self.byte_offset
    }
}

/// Existing source identities needed by the next scalar-equality row.  These
/// are source keys, not Recipe keys or physical SSA ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) struct CommonV2S6CTextCursorSourceRelationV1 {
    index_binding: BindingRefV1,
    index_input: LoopValueKeyV1,
    length_result: LoopValueKeyV1,
    substring_result: LoopValueKeyV1,
    slice_end: LoopValueKeyV1,
    text_equal_item: LoopItemKeyV1,
    text_equal_result: LoopValueKeyV1,
    text_equal_if: LoopItemKeyV1,
    step_add: LoopValueKeyV1,
}

impl CommonV2S6CTextCursorSourceRelationV1 {
    pub(in crate::mir::builder) const fn index_binding(self) -> BindingRefV1 {
        self.index_binding
    }

    pub(in crate::mir::builder) const fn index_input(self) -> LoopValueKeyV1 {
        self.index_input
    }

    pub(in crate::mir::builder) const fn length_result(self) -> LoopValueKeyV1 {
        self.length_result
    }

    pub(in crate::mir::builder) const fn substring_result(self) -> LoopValueKeyV1 {
        self.substring_result
    }

    pub(in crate::mir::builder) const fn slice_end(self) -> LoopValueKeyV1 {
        self.slice_end
    }

    pub(in crate::mir::builder) const fn text_equal_item(self) -> LoopItemKeyV1 {
        self.text_equal_item
    }

    pub(in crate::mir::builder) const fn text_equal_result(self) -> LoopValueKeyV1 {
        self.text_equal_result
    }

    pub(in crate::mir::builder) const fn text_equal_if(self) -> LoopItemKeyV1 {
        self.text_equal_if
    }

    pub(in crate::mir::builder) const fn step_add(self) -> LoopValueKeyV1 {
        self.step_add
    }
}

/// One-shot, physical-free cursor/preheader admission.  It keeps the source
/// view and the root rows together so the next consumer cannot reconstruct a
/// different V9/Subject/Needle pairing.
#[must_use = "a cursor/preheader plan must be consumed exactly once"]
#[derive(Debug)]
pub(in crate::mir::builder) struct CommonV2S6CTextCursorPreheaderPlanV1<'source, 'rows, 'facts> {
    source: S6CScalarScanSourceRefV1<'source, 'rows, 'facts>,
    owner: FunctionOwnerIdV1,
    entry: crate::mir::BasicBlockId,
    root_plan_stamp: u64,
    roots: [CommonV2S6CTextCursorRootLoadV1; 2],
    initial: CommonV2S6CTextCursorInitialStateV1,
    relation: CommonV2S6CTextCursorSourceRelationV1,
}

impl<'source, 'rows, 'facts> CommonV2S6CTextCursorPreheaderPlanV1<'source, 'rows, 'facts> {
    pub(in crate::mir::builder) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder) const fn entry(&self) -> crate::mir::BasicBlockId {
        self.entry
    }

    pub(in crate::mir::builder) const fn root_plan_stamp(&self) -> u64 {
        self.root_plan_stamp
    }

    pub(in crate::mir::builder) const fn initial(&self) -> CommonV2S6CTextCursorInitialStateV1 {
        self.initial
    }

    pub(in crate::mir::builder) const fn roots(&self) -> &[CommonV2S6CTextCursorRootLoadV1; 2] {
        &self.roots
    }

    pub(in crate::mir::builder) const fn relation(&self) -> CommonV2S6CTextCursorSourceRelationV1 {
        self.relation
    }

    /// Lend the existing source cohort and root-load obligations together.
    /// No part of the plan remains available after this consuming call.
    pub(in crate::mir::builder) fn consume<R>(
        self,
        callback: impl FnOnce(
            S6CScalarScanSourceRefV1<'source, 'rows, 'facts>,
            &[CommonV2S6CTextCursorRootLoadV1; 2],
            CommonV2S6CTextCursorInitialStateV1,
            CommonV2S6CTextCursorSourceRelationV1,
        ) -> Result<R, String>,
    ) -> Result<R, CommonV2S6CTextCursorPreheaderRejectV1> {
        callback(self.source, &self.roots, self.initial, self.relation)
            .map_err(CommonV2S6CTextCursorPreheaderRejectV1::CursorInvariant)
    }
}

/// Consume the base-root admission exactly once and issue only the
/// effect-free preheader plan.  The first MIR/SSA consumer is deliberately a
/// later row.
pub(in crate::mir::builder) fn issue_common_v2_s6c_text_cursor_preheader_v1<
    'source,
    'rows,
    'facts,
>(
    admission: CommonV2S6CTextContentRootAdmissionV1<'source, 'rows, 'facts>,
) -> Result<
    CommonV2S6CTextCursorPreheaderPlanV1<'source, 'rows, 'facts>,
    CommonV2S6CTextCursorPreheaderRejectV1,
> {
    let owner = admission.owner();
    let entry = admission.entry();
    let root_plan_stamp = admission.plan_stamp();
    admission
        .consume(|source, roots| {
            if source.owner() != owner {
                return Err("source owner differs from the root admission".to_string());
            }
            if source.initial_index() != 0 {
                return Err("source index initializer is not zero".to_string());
            }
            if source.length_law() != CoreMethodSemanticLawV2::CodePointCount {
                return Err("Length law is not CodePointCount".to_string());
            }
            if source.substring_law() != CoreMethodSemanticLawV2::CodePointHalfOpenClamped {
                return Err("Substring law is not CodePointHalfOpenClamped".to_string());
            }
            if roots[0].role() != CommonV2S6CTextContentRootRoleV1::Subject
                || roots[0].root_index() != 0
                || roots[0].binding() != source.subject_binding()
            {
                return Err("subject root is not the admitted root zero".to_string());
            }
            if roots[1].role() != CommonV2S6CTextContentRootRoleV1::Needle
                || roots[1].root_index() != 1
                || roots[1].binding() != source.needle_binding()
            {
                return Err("needle root is not the admitted root one".to_string());
            }
            if roots
                .iter()
                .any(|root| root.carrier() != PhysicalCallableLaneCarrierV1::U64BitsOnI64)
            {
                return Err("root carrier is not U64BitsOnI64".to_string());
            }

            Ok(CommonV2S6CTextCursorPreheaderPlanV1 {
                source,
                owner,
                entry,
                root_plan_stamp,
                roots: [
                    CommonV2S6CTextCursorRootLoadV1 { root: roots[0] },
                    CommonV2S6CTextCursorRootLoadV1 { root: roots[1] },
                ],
                initial: CommonV2S6CTextCursorInitialStateV1 {
                    cp_index: source.initial_index(),
                    byte_offset: 0,
                },
                relation: CommonV2S6CTextCursorSourceRelationV1 {
                    index_binding: source.index_binding(),
                    index_input: source.index_input(),
                    length_result: source.length_result(),
                    substring_result: source.substring_result(),
                    slice_end: source.slice_end(),
                    text_equal_item: source.text_equal_item(),
                    text_equal_result: source.text_equal_result(),
                    text_equal_if: source.text_equal_if(),
                    step_add: source.step_add(),
                },
            })
        })
        .map_err(|error| match error {
            CommonV2S6CTextContentRootAdmissionRejectV1::Callback(detail) => {
                match detail.as_str() {
                    "source owner differs from the root admission" => {
                        CommonV2S6CTextCursorPreheaderRejectV1::SourceOwnerMismatch
                    }
                    "source index initializer is not zero" => {
                        CommonV2S6CTextCursorPreheaderRejectV1::SourceInitialIndexMismatch
                    }
                    "Length law is not CodePointCount" => {
                        CommonV2S6CTextCursorPreheaderRejectV1::LengthLawMismatch
                    }
                    "Substring law is not CodePointHalfOpenClamped" => {
                        CommonV2S6CTextCursorPreheaderRejectV1::SubstringLawMismatch
                    }
                    "subject root is not the admitted root zero" => {
                        CommonV2S6CTextCursorPreheaderRejectV1::SubjectRootMismatch
                    }
                    "needle root is not the admitted root one" => {
                        CommonV2S6CTextCursorPreheaderRejectV1::NeedleRootMismatch
                    }
                    "root carrier is not U64BitsOnI64" => {
                        CommonV2S6CTextCursorPreheaderRejectV1::CarrierMismatch
                    }
                    _ => CommonV2S6CTextCursorPreheaderRejectV1::CursorInvariant(detail),
                }
            }
            other => CommonV2S6CTextCursorPreheaderRejectV1::RootAdmission(other),
        })
}
