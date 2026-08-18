//! Effect-free S6C scalar-equality leaf capability.
//!
//! This row consumes the one-shot cursor/preheader product and records the
//! two existing backend-neutral leaf shapes.  It deliberately does not issue
//! an SSA value, MIR instruction, access-plan row, block, or Bool.  The next
//! cursor/CFG row is the only place that may materialize those effects.

use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1;
use crate::mir::resolved_semantics::FunctionOwnerIdV1;

use super::super::common_v2_s6c_text_content_root_admission::CommonV2S6CTextContentRootRoleV1;
use super::super::common_v2_s6c_text_cursor_preheader::{
    CommonV2S6CTextCursorInitialStateV1, CommonV2S6CTextCursorPreheaderPlanV1,
    CommonV2S6CTextCursorPreheaderRejectV1, CommonV2S6CTextCursorRootLoadV1,
    CommonV2S6CTextCursorSourceRelationV1,
};
use super::CommonV2CanonicalSessionRefV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum CommonV2S6CTextScalarEqualityLeafShapeV1 {
    Utf8WidthAt {
        root_index: u32,
    },
    Utf8ScalarSliceEqWholeText {
        lhs_root_index: u32,
        rhs_root_index: u32,
    },
}

/// One source/entry-cohort capability for the scalar-equality leaf.  The
/// Subject root plus the cursor state is the only V9 representation; there
/// is no derived root or runtime-produced Text value in this product.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct CommonV2S6CTextScalarEqualityLeafCapabilityV1 {
    owner: FunctionOwnerIdV1,
    entry: crate::mir::BasicBlockId,
    root_plan_stamp: u64,
    subject: CommonV2S6CTextCursorRootLoadV1,
    needle: CommonV2S6CTextCursorRootLoadV1,
    initial: CommonV2S6CTextCursorInitialStateV1,
    relation: CommonV2S6CTextCursorSourceRelationV1,
    shapes: [CommonV2S6CTextScalarEqualityLeafShapeV1; 2],
}

impl CommonV2S6CTextScalarEqualityLeafCapabilityV1 {
    pub(in crate::mir::builder) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder) const fn entry(&self) -> crate::mir::BasicBlockId {
        self.entry
    }

    pub(in crate::mir::builder) const fn root_plan_stamp(&self) -> u64 {
        self.root_plan_stamp
    }

    pub(in crate::mir::builder) const fn subject_root_index(&self) -> u32 {
        self.subject.root().root_index()
    }

    pub(in crate::mir::builder) const fn needle_root_index(&self) -> u32 {
        self.needle.root().root_index()
    }

    pub(in crate::mir::builder) const fn initial(&self) -> CommonV2S6CTextCursorInitialStateV1 {
        self.initial
    }

    pub(in crate::mir::builder) const fn relation(&self) -> CommonV2S6CTextCursorSourceRelationV1 {
        self.relation
    }

    pub(in crate::mir::builder) const fn shapes(
        &self,
    ) -> &[CommonV2S6CTextScalarEqualityLeafShapeV1; 2] {
        &self.shapes
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum CommonV2S6CTextScalarEqualityLeafRejectV1 {
    AlreadyIssued,
    MissingPhysicalEntryStamp,
    OwnerMismatch,
    EntryMismatch,
    ZeroRootPlanStamp,
    SubjectRootMismatch,
    NeedleRootMismatch,
    Sidecar(String),
    Cursor(CommonV2S6CTextCursorPreheaderRejectV1),
}

/// The receipt keeps the capability and the canonical session borrow paired.
/// A later materializer must consume this callback-scoped receipt rather than
/// extracting a detached leaf shape.
pub(in crate::mir::builder) struct CommonV2S6CTextScalarEqualityLeafReceiptV1<
    'session,
    'source,
    'envelope,
> {
    _session: &'session mut CommonV2CanonicalSessionRefV1<'source, 'envelope>,
    capability: CommonV2S6CTextScalarEqualityLeafCapabilityV1,
}

impl<'session, 'source, 'envelope>
    CommonV2S6CTextScalarEqualityLeafReceiptV1<'session, 'source, 'envelope>
{
    pub(in crate::mir::builder) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.capability.owner()
    }

    pub(in crate::mir::builder) const fn entry(&self) -> crate::mir::BasicBlockId {
        self.capability.entry()
    }

    pub(in crate::mir::builder) const fn root_plan_stamp(&self) -> u64 {
        self.capability.root_plan_stamp()
    }

    pub(in crate::mir::builder) const fn subject_root_index(&self) -> u32 {
        self.capability.subject_root_index()
    }

    pub(in crate::mir::builder) const fn needle_root_index(&self) -> u32 {
        self.capability.needle_root_index()
    }

    pub(in crate::mir::builder) const fn initial(&self) -> CommonV2S6CTextCursorInitialStateV1 {
        self.capability.initial()
    }

    pub(in crate::mir::builder) const fn relation(&self) -> CommonV2S6CTextCursorSourceRelationV1 {
        self.capability.relation()
    }

    pub(in crate::mir::builder) const fn shapes(
        &self,
    ) -> &[CommonV2S6CTextScalarEqualityLeafShapeV1; 2] {
        self.capability.shapes()
    }

    /// Consume the callback-scoped leaf together with its canonical session.
    /// The capability is borrowed only for the materializer callback; no
    /// detached shape or session pointer can escape this handoff.
    pub(in crate::mir::builder) fn with_session<R, E>(
        self,
        callback: impl FnOnce(
            &'session mut CommonV2CanonicalSessionRefV1<'source, 'envelope>,
            &CommonV2S6CTextScalarEqualityLeafCapabilityV1,
        ) -> Result<R, E>,
    ) -> Result<R, E> {
        let Self {
            _session,
            capability,
        } = self;
        callback(_session, &capability)
    }
}

pub(in crate::mir::builder) fn issue_common_v2_s6c_text_scalar_equality_leaf_v1<
    'source,
    'rows,
    'facts,
>(
    cursor: CommonV2S6CTextCursorPreheaderPlanV1<'source, 'rows, 'facts>,
) -> Result<CommonV2S6CTextScalarEqualityLeafCapabilityV1, CommonV2S6CTextScalarEqualityLeafRejectV1>
{
    let owner = cursor.owner();
    let entry = cursor.entry();
    let root_plan_stamp = cursor.root_plan_stamp();
    if root_plan_stamp == 0 {
        return Err(CommonV2S6CTextScalarEqualityLeafRejectV1::ZeroRootPlanStamp);
    }
    cursor
        .consume(|source, roots, initial, relation| {
            if source.owner() != owner {
                return Err("source owner differs from cursor owner".to_owned());
            }
            if initial.cp_index() != 0 || initial.byte_offset() != 0 {
                return Err("cursor does not start at code point and byte offset zero".to_owned());
            }
            let [subject, needle] = *roots;
            if subject.role() != CommonV2S6CTextContentRootRoleV1::Subject
                || subject.root().root_index() != 0
                || subject.root().binding() != source.subject_binding()
                || subject.root().carrier() != PhysicalCallableLaneCarrierV1::U64BitsOnI64
            {
                return Err("subject root is not the verified source root zero".to_owned());
            }
            if needle.role() != CommonV2S6CTextContentRootRoleV1::Needle
                || needle.root().root_index() != 1
                || needle.root().binding() != source.needle_binding()
                || needle.root().carrier() != PhysicalCallableLaneCarrierV1::U64BitsOnI64
            {
                return Err("needle root is not the verified source root one".to_owned());
            }
            Ok(CommonV2S6CTextScalarEqualityLeafCapabilityV1 {
                owner,
                entry,
                root_plan_stamp,
                subject,
                needle,
                initial,
                relation,
                shapes: [
                    CommonV2S6CTextScalarEqualityLeafShapeV1::Utf8WidthAt { root_index: 0 },
                    CommonV2S6CTextScalarEqualityLeafShapeV1::Utf8ScalarSliceEqWholeText {
                        lhs_root_index: 0,
                        rhs_root_index: 1,
                    },
                ],
            })
        })
        .map_err(|reject| match reject {
            CommonV2S6CTextCursorPreheaderRejectV1::CursorInvariant(detail) => {
                if detail == "subject root is not the verified source root zero" {
                    CommonV2S6CTextScalarEqualityLeafRejectV1::SubjectRootMismatch
                } else if detail == "needle root is not the verified source root one" {
                    CommonV2S6CTextScalarEqualityLeafRejectV1::NeedleRootMismatch
                } else {
                    CommonV2S6CTextScalarEqualityLeafRejectV1::Cursor(
                        CommonV2S6CTextCursorPreheaderRejectV1::CursorInvariant(detail),
                    )
                }
            }
            other => CommonV2S6CTextScalarEqualityLeafRejectV1::Cursor(other),
        })
}

impl<'source, 'envelope> CommonV2CanonicalSessionRefV1<'source, 'envelope> {
    pub(in crate::mir::builder) fn consume_s6c_scalar_equality_leaf<
        'session,
        'cursor,
        'rows,
        'facts,
    >(
        &'session mut self,
        cursor: CommonV2S6CTextCursorPreheaderPlanV1<'cursor, 'rows, 'facts>,
    ) -> Result<
        CommonV2S6CTextScalarEqualityLeafReceiptV1<'session, 'source, 'envelope>,
        CommonV2S6CTextScalarEqualityLeafRejectV1,
    > {
        if self.s6c_scalar_equality_leaf_issued {
            return Err(CommonV2S6CTextScalarEqualityLeafRejectV1::AlreadyIssued);
        }
        let owner = self.session.owner();
        if self.envelope.owner() != owner || cursor.owner() != owner {
            return Err(CommonV2S6CTextScalarEqualityLeafRejectV1::OwnerMismatch);
        }
        let stamp = self
            .session
            .physical_entry_stamp()
            .map_err(|_| CommonV2S6CTextScalarEqualityLeafRejectV1::MissingPhysicalEntryStamp)?;
        if stamp.owner() != owner {
            return Err(CommonV2S6CTextScalarEqualityLeafRejectV1::OwnerMismatch);
        }
        let entry = self
            .session
            .physical_entry_sidecar_entry()
            .map_err(|_| CommonV2S6CTextScalarEqualityLeafRejectV1::MissingPhysicalEntryStamp)?;
        if cursor.entry() != entry {
            return Err(CommonV2S6CTextScalarEqualityLeafRejectV1::EntryMismatch);
        }
        if cursor.root_plan_stamp() == 0 {
            return Err(CommonV2S6CTextScalarEqualityLeafRejectV1::ZeroRootPlanStamp);
        }

        let roots = *cursor.roots();
        for root in roots {
            let expected = root.root();
            let row_result = self
                .session
                .with_exact_text_sidecar_row(
                    expected.binding(),
                    expected.logical_ordinal(),
                    |row| {
                        if row.binding() != expected.binding()
                            || row.logical_ordinal() != expected.logical_ordinal()
                            || row.slot().as_u32() != expected.slot_lane_index()
                            || row.generation().as_u32() != expected.generation_lane_index()
                            || row.carrier() != expected.carrier()
                        {
                            Err("cursor root differs from the canonical entry sidecar".to_owned())
                        } else {
                            Ok(())
                        }
                    },
                )
                .map_err(CommonV2S6CTextScalarEqualityLeafRejectV1::Sidecar)?;
            row_result.map_err(CommonV2S6CTextScalarEqualityLeafRejectV1::Sidecar)?;
        }

        // Poison the one-shot seam before consuming the cursor.  Any late
        // failure is handled by the outer unpublished session transaction.
        self.s6c_scalar_equality_leaf_issued = true;
        let capability = issue_common_v2_s6c_text_scalar_equality_leaf_v1(cursor)?;
        Ok(CommonV2S6CTextScalarEqualityLeafReceiptV1 {
            _session: self,
            capability,
        })
    }
}
