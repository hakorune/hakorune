//! Function-local, transport-only plans for the pinned Text leaf family.
//!
//! The numeric id is never semantic authority.  A plan table owns its stamp
//! and rows, and a transport census must prove that every row is emitted once
//! with the same kind and operands before JSON export.

use super::ValueId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct PinnedTextRootIdV1(u32);

impl PinnedTextRootIdV1 {
    pub(crate) const fn from_frame_row(row: u32) -> Self {
        Self(row)
    }

    pub(crate) const fn index(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct PinnedTextAccessPlanIdV1 {
    stamp: u64,
    index: u32,
}

impl PinnedTextAccessPlanIdV1 {
    pub(crate) const fn stamp(self) -> u64 {
        self.stamp
    }

    pub(crate) const fn index(self) -> u32 {
        self.index
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum PinnedTextAccessKindV1 {
    ByteLen {
        root: PinnedTextRootIdV1,
    },
    Utf8WidthAt {
        root: PinnedTextRootIdV1,
        byte_offset: ValueId,
    },
    Utf8ScalarSliceEqWholeText {
        lhs_root: PinnedTextRootIdV1,
        lhs_byte_offset: ValueId,
        lhs_width: ValueId,
        rhs_root: PinnedTextRootIdV1,
    },
}

impl PinnedTextAccessKindV1 {
    pub(crate) const fn tag(self) -> &'static str {
        match self {
            Self::ByteLen { .. } => "byte_len",
            Self::Utf8WidthAt { .. } => "utf8_width_at",
            Self::Utf8ScalarSliceEqWholeText { .. } => "utf8_scalar_slice_eq_whole_text",
        }
    }

    pub(crate) fn used_values(self) -> Vec<ValueId> {
        match self {
            Self::ByteLen { .. } => Vec::new(),
            Self::Utf8WidthAt { byte_offset, .. } => vec![byte_offset],
            Self::Utf8ScalarSliceEqWholeText {
                lhs_byte_offset,
                lhs_width,
                ..
            } => vec![lhs_byte_offset, lhs_width],
        }
    }

    pub(crate) fn remap_values(self, mut remap: impl FnMut(ValueId) -> ValueId) -> Self {
        match self {
            Self::ByteLen { root } => Self::ByteLen { root },
            Self::Utf8WidthAt { root, byte_offset } => Self::Utf8WidthAt {
                root,
                byte_offset: remap(byte_offset),
            },
            Self::Utf8ScalarSliceEqWholeText {
                lhs_root,
                lhs_byte_offset,
                lhs_width,
                rhs_root,
            } => Self::Utf8ScalarSliceEqWholeText {
                lhs_root,
                lhs_byte_offset: remap(lhs_byte_offset),
                lhs_width: remap(lhs_width),
                rhs_root,
            },
        }
    }

    pub(crate) fn rewrite_values(&mut self, mut rewrite: impl FnMut(&mut ValueId)) {
        match self {
            Self::ByteLen { .. } => {}
            Self::Utf8WidthAt { byte_offset, .. } => rewrite(byte_offset),
            Self::Utf8ScalarSliceEqWholeText {
                lhs_byte_offset,
                lhs_width,
                ..
            } => {
                rewrite(lhs_byte_offset);
                rewrite(lhs_width);
            }
        }
    }

    pub(crate) fn same_shape(self, other: Self) -> bool {
        match (self, other) {
            (Self::ByteLen { root: lhs }, Self::ByteLen { root: rhs }) => lhs == rhs,
            (
                Self::Utf8WidthAt {
                    root: lhs_root,
                    byte_offset: lhs_offset,
                },
                Self::Utf8WidthAt {
                    root: rhs_root,
                    byte_offset: rhs_offset,
                },
            ) => lhs_root == rhs_root && lhs_offset == rhs_offset,
            (
                Self::Utf8ScalarSliceEqWholeText {
                    lhs_root: lhs_lhs_root,
                    lhs_byte_offset: lhs_lhs_offset,
                    lhs_width: lhs_lhs_width,
                    rhs_root: lhs_rhs_root,
                },
                Self::Utf8ScalarSliceEqWholeText {
                    lhs_root: rhs_lhs_root,
                    lhs_byte_offset: rhs_lhs_offset,
                    lhs_width: rhs_lhs_width,
                    rhs_root: rhs_rhs_root,
                },
            ) => {
                lhs_lhs_root == rhs_lhs_root
                    && lhs_lhs_offset == rhs_lhs_offset
                    && lhs_lhs_width == rhs_lhs_width
                    && lhs_rhs_root == rhs_rhs_root
            }
            _ => false,
        }
    }

    pub(crate) fn json_payload(self) -> serde_json::Value {
        match self {
            Self::ByteLen { root } => serde_json::json!({
                "kind": self.tag(),
                "root": root.index(),
            }),
            Self::Utf8WidthAt { root, byte_offset } => serde_json::json!({
                "kind": self.tag(),
                "root": root.index(),
                "byte_offset": byte_offset.as_u32(),
            }),
            Self::Utf8ScalarSliceEqWholeText {
                lhs_root,
                lhs_byte_offset,
                lhs_width,
                rhs_root,
            } => serde_json::json!({
                "kind": self.tag(),
                "lhs_root": lhs_root.index(),
                "lhs_byte_offset": lhs_byte_offset.as_u32(),
                "lhs_width": lhs_width.as_u32(),
                "rhs_root": rhs_root.index(),
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PinnedTextAccessPlanTableV1 {
    stamp: u64,
    rows: Box<[PinnedTextAccessKindV1]>,
}

impl Default for PinnedTextAccessPlanTableV1 {
    fn default() -> Self {
        Self::new(0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PinnedTextPlanCensusErrorV1 {
    ForeignStamp,
    IndexOutOfRange,
    DuplicatePlan,
    MissingPlan,
    KindOperandMismatch,
}

impl PinnedTextAccessPlanTableV1 {
    pub(crate) fn new(stamp: u64) -> Self {
        Self {
            stamp,
            rows: Box::default(),
        }
    }

    pub(crate) fn issue(&mut self, kind: PinnedTextAccessKindV1) -> PinnedTextAccessPlanIdV1 {
        let index = self.rows.len() as u32;
        let mut rows = self.rows.to_vec();
        rows.push(kind);
        self.rows = rows.into_boxed_slice();
        PinnedTextAccessPlanIdV1 {
            stamp: self.stamp,
            index,
        }
    }

    pub(crate) fn row(
        &self,
        id: PinnedTextAccessPlanIdV1,
    ) -> Result<PinnedTextAccessKindV1, PinnedTextPlanCensusErrorV1> {
        if id.stamp != self.stamp {
            return Err(PinnedTextPlanCensusErrorV1::ForeignStamp);
        }
        self.rows
            .get(id.index as usize)
            .copied()
            .ok_or(PinnedTextPlanCensusErrorV1::IndexOutOfRange)
    }

    pub(crate) fn verify_census(
        &self,
        emitted: &[(PinnedTextAccessPlanIdV1, PinnedTextAccessKindV1)],
    ) -> Result<(), PinnedTextPlanCensusErrorV1> {
        if emitted.len() != self.rows.len() {
            return Err(PinnedTextPlanCensusErrorV1::MissingPlan);
        }
        let mut seen = vec![false; self.rows.len()];
        for (id, kind) in emitted {
            let expected = self.row(*id)?;
            let slot = id.index as usize;
            if seen[slot] {
                return Err(PinnedTextPlanCensusErrorV1::DuplicatePlan);
            }
            if !expected.same_shape(*kind) {
                return Err(PinnedTextPlanCensusErrorV1::KindOperandMismatch);
            }
            seen[slot] = true;
        }
        if seen.into_iter().any(|present| !present) {
            return Err(PinnedTextPlanCensusErrorV1::MissingPlan);
        }
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.rows.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plan_table_requires_exact_once_stamped_census() {
        let root = PinnedTextRootIdV1::from_frame_row(3);
        let mut table = PinnedTextAccessPlanTableV1::new(41);
        let width = table.issue(PinnedTextAccessKindV1::Utf8WidthAt {
            root,
            byte_offset: ValueId::new(7),
        });
        let length = table.issue(PinnedTextAccessKindV1::ByteLen { root });
        assert_eq!(table.len(), 2);
        assert!(table
            .verify_census(&[
                (
                    width,
                    PinnedTextAccessKindV1::Utf8WidthAt {
                        root,
                        byte_offset: ValueId::new(7),
                    }
                ),
                (length, PinnedTextAccessKindV1::ByteLen { root }),
            ])
            .is_ok());
        assert_eq!(
            table.verify_census(&[(
                width,
                PinnedTextAccessKindV1::Utf8WidthAt {
                    root,
                    byte_offset: ValueId::new(7),
                }
            )]),
            Err(PinnedTextPlanCensusErrorV1::MissingPlan)
        );
    }

    #[test]
    fn plan_table_rejects_foreign_or_mismatched_rows() {
        let root = PinnedTextRootIdV1::from_frame_row(0);
        let mut table = PinnedTextAccessPlanTableV1::new(1);
        let id = table.issue(PinnedTextAccessKindV1::ByteLen { root });
        let mut foreign = PinnedTextAccessPlanTableV1::new(2);
        let foreign_id = foreign.issue(PinnedTextAccessKindV1::ByteLen { root });
        assert_eq!(
            table.row(foreign_id),
            Err(PinnedTextPlanCensusErrorV1::ForeignStamp)
        );
        assert_eq!(
            table.verify_census(&[(
                id,
                PinnedTextAccessKindV1::Utf8WidthAt {
                    root,
                    byte_offset: ValueId::new(2),
                }
            )]),
            Err(PinnedTextPlanCensusErrorV1::KindOperandMismatch)
        );
    }
}
