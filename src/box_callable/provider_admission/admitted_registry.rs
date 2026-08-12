//! Immutable selected rows issued by the provider-admission seal.

use crate::abi::text_scan_aot_export_facts::TextScanAotEntryIdV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TextScanAdmittedRoleV1 {
    TextSliceRange,
    TextFindNeedle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AdmittedTextScanRowV1 {
    role: TextScanAdmittedRoleV1,
    slot: u16,
    entry: TextScanAotEntryIdV1,
}

impl AdmittedTextScanRowV1 {
    pub(crate) const fn role(self) -> TextScanAdmittedRoleV1 {
        self.role
    }

    pub(crate) const fn slot(self) -> u16 {
        self.slot
    }

    pub(crate) const fn entry(self) -> TextScanAotEntryIdV1 {
        self.entry
    }
}

/// Deterministic, non-mutable rows after one admission transaction.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct AdmittedTextScanRegistryV1 {
    rows: [AdmittedTextScanRowV1; 2],
    generation: u64,
}

impl AdmittedTextScanRegistryV1 {
    pub(super) fn new(
        substring_slot: u16,
        index_of_slot: u16,
        generation: u64,
    ) -> Result<Self, &'static str> {
        if substring_slot == index_of_slot || generation == 0 {
            return Err("duplicate TextScan slot or zero registry generation");
        }
        Ok(Self {
            rows: [
                AdmittedTextScanRowV1 {
                    role: TextScanAdmittedRoleV1::TextSliceRange,
                    slot: substring_slot,
                    entry: TextScanAotEntryIdV1::Substring,
                },
                AdmittedTextScanRowV1 {
                    role: TextScanAdmittedRoleV1::TextFindNeedle,
                    slot: index_of_slot,
                    entry: TextScanAotEntryIdV1::IndexOf,
                },
            ],
            generation,
        })
    }

    pub(crate) const fn branch_count(&self) -> usize {
        1
    }

    pub(crate) const fn generation(&self) -> u64 {
        self.generation
    }

    pub(crate) fn row(&self, role: TextScanAdmittedRoleV1) -> AdmittedTextScanRowV1 {
        self.rows
            .iter()
            .copied()
            .find(|row| row.role == role)
            .expect("admitted TextScan role is complete")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicate_slots_and_zero_generation_are_rejected() {
        assert!(AdmittedTextScanRegistryV1::new(301, 301, 1).is_err());
        assert!(AdmittedTextScanRegistryV1::new(301, 302, 0).is_err());
    }

    #[test]
    fn admitted_rows_are_deterministic_and_complete() {
        let registry = AdmittedTextScanRegistryV1::new(301, 302, 7).expect("admit rows");
        assert_eq!(registry.branch_count(), 1);
        assert_eq!(registry.generation(), 7);
        assert_eq!(
            registry.row(TextScanAdmittedRoleV1::TextSliceRange).entry(),
            TextScanAotEntryIdV1::Substring
        );
        assert_eq!(
            registry.row(TextScanAdmittedRoleV1::TextFindNeedle).entry(),
            TextScanAotEntryIdV1::IndexOf
        );
    }
}
