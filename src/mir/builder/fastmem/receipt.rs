//! Prepared receipt for one successfully emitted FastMem MemOp.
//!
//! This is intentionally disconnected during FASTMEM-RECEIPT0-S0. It proves
//! the existing function/region preflight without changing the legacy
//! `note_fastmem_memop` timing until the later one-consumer cutover.

use crate::mir::builder::MirBuilder;
use crate::mir::instruction::FastMemRegionId;

#[derive(Debug, PartialEq, Eq)]
pub(super) enum FastMemMemOpReceiptPreparationErrorV1 {
    NoCurrentFunction,
    UnknownRegion { region: FastMemRegionId },
}

impl std::fmt::Display for FastMemMemOpReceiptPreparationErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoCurrentFunction => {
                write!(formatter, "[freeze:contract][fastmem/outside_function]")
            }
            Self::UnknownRegion { region } => write!(
                formatter,
                "[freeze:contract][fastmem/unknown_region] region={}",
                region.0
            ),
        }
    }
}

/// A validated region receipt. It is deliberately non-Clone and owns no
/// Builder reference, metadata reference, value fact, or result policy.
#[derive(Debug)]
pub(super) struct PreparedFastMemMemOpReceiptV1 {
    region: FastMemRegionId,
}

impl PreparedFastMemMemOpReceiptV1 {
    pub(super) fn prepare(
        builder: &MirBuilder,
        region: FastMemRegionId,
    ) -> Result<Self, FastMemMemOpReceiptPreparationErrorV1> {
        let function = builder
            .function_state
            .current_function
            .as_ref()
            .ok_or(FastMemMemOpReceiptPreparationErrorV1::NoCurrentFunction)?;
        if !function
            .metadata
            .fastmem_regions
            .iter()
            .any(|entry| entry.id == region)
        {
            return Err(FastMemMemOpReceiptPreparationErrorV1::UnknownRegion { region });
        }
        Ok(Self { region })
    }

    /// Commit only after the matching physical MemOp has been emitted.
    ///
    /// Preparation sealed both lookups, so a commit failure would mean the
    /// current function or its region metadata changed within one emission.
    pub(super) fn commit(self, builder: &mut MirBuilder) {
        let function = builder
            .function_state
            .current_function
            .as_mut()
            .expect("[freeze:contract][fastmem/receipt_commit_outside_function]");
        let metadata = function
            .metadata
            .fastmem_regions
            .iter_mut()
            .find(|entry| entry.id == self.region)
            .expect("[freeze:contract][fastmem/receipt_commit_missing_region]");
        metadata.emitted_memop_count += 1;
    }

    #[cfg(test)]
    fn region(&self) -> FastMemRegionId {
        self.region
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;

    #[test]
    fn prepare_validates_region_without_mutating_metadata() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("fastmem_receipt_prepare/0".to_string());
        let region = builder
            .register_fastmem_region("ReceiptV1".to_string(), Span::unknown(), 0)
            .unwrap();

        let receipt = PreparedFastMemMemOpReceiptV1::prepare(&builder, region).unwrap();
        assert_eq!(receipt.region(), region);
        assert_eq!(
            builder
                .function_state
                .current_function
                .as_ref()
                .unwrap()
                .metadata
                .fastmem_regions[0]
                .emitted_memop_count,
            0
        );

        receipt.commit(&mut builder);
        assert_eq!(
            builder
                .function_state
                .current_function
                .as_ref()
                .unwrap()
                .metadata
                .fastmem_regions[0]
                .emitted_memop_count,
            1
        );
    }

    #[test]
    fn prepare_rejects_missing_function() {
        let builder = MirBuilder::new();
        assert_eq!(
            PreparedFastMemMemOpReceiptV1::prepare(&builder, FastMemRegionId::new(0)).unwrap_err(),
            FastMemMemOpReceiptPreparationErrorV1::NoCurrentFunction
        );
    }

    #[test]
    fn prepare_rejects_unknown_region_without_mutating_metadata() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("fastmem_receipt_unknown/0".to_string());
        let region = builder
            .register_fastmem_region("ReceiptV1".to_string(), Span::unknown(), 0)
            .unwrap();
        let unknown = FastMemRegionId::new(region.0 + 1);

        assert_eq!(
            PreparedFastMemMemOpReceiptV1::prepare(&builder, unknown).unwrap_err(),
            FastMemMemOpReceiptPreparationErrorV1::UnknownRegion { region: unknown }
        );
        assert_eq!(
            builder
                .function_state
                .current_function
                .as_ref()
                .unwrap()
                .metadata
                .fastmem_regions[0]
                .emitted_memop_count,
            0
        );
    }
}
