use crate::mir::{BasicBlockId, ValueId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeContractSiteKind {
    BoxFieldWrite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeContractProofKind {
    ExactNumericConstantInRange,
}

/// Verifier-backed proof for one activated type-contract boundary.
///
/// The first slice rebuilds these records from canonical MIR on every semantic
/// refresh. The full structural site key therefore owns freshness; this is not
/// a persisted proof cache.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeContractProof {
    pub site_kind: TypeContractSiteKind,
    pub function: String,
    pub block: BasicBlockId,
    pub instruction_index: usize,
    pub field: String,
    pub value: ValueId,
    pub expected_type: String,
    pub proof_kind: TypeContractProofKind,
}

impl TypeContractProof {
    pub(crate) fn matches_box_field_site(
        &self,
        function: &str,
        block: BasicBlockId,
        instruction_index: usize,
        field: &str,
        value: ValueId,
        expected_type: &str,
    ) -> bool {
        self.site_kind == TypeContractSiteKind::BoxFieldWrite
            && self.function == function
            && self.block == block
            && self.instruction_index == instruction_index
            && self.field == field
            && self.value == value
            && self.expected_type == expected_type
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proof_freshness_uses_the_complete_structural_site_key() {
        let proof = TypeContractProof {
            site_kind: TypeContractSiteKind::BoxFieldWrite,
            function: "main".to_string(),
            block: BasicBlockId::new(0),
            instruction_index: 2,
            field: "capacity".to_string(),
            value: ValueId::new(1),
            expected_type: "usize".to_string(),
            proof_kind: TypeContractProofKind::ExactNumericConstantInRange,
        };
        assert!(proof.matches_box_field_site(
            "main",
            BasicBlockId::new(0),
            2,
            "capacity",
            ValueId::new(1),
            "usize",
        ));
        assert!(!proof.matches_box_field_site(
            "main",
            BasicBlockId::new(0),
            3,
            "capacity",
            ValueId::new(1),
            "usize",
        ));
    }
}
