use crate::mir::{BasicBlockId, ValueId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ArrayTextEditKind {
    InsertMidConst,
}

impl ArrayTextEditKind {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::InsertMidConst => "insert_mid_const",
        }
    }
}

impl std::fmt::Display for ArrayTextEditKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ArrayTextEditSplitPolicy {
    SourceLenDivConst { divisor: i64 },
}

impl std::fmt::Display for ArrayTextEditSplitPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SourceLenDivConst { divisor } => {
                write!(f, "source_len_div_const({divisor})")
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ArrayTextEditProof {
    ArrayGetLenHalfInsertMidSameSlot,
    ArrayGetLenHalfInsertMidDestSlotLenOnly,
}

impl ArrayTextEditProof {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::ArrayGetLenHalfInsertMidSameSlot => "array_get_lenhalf_insert_mid_same_slot",
            Self::ArrayGetLenHalfInsertMidDestSlotLenOnly => {
                "array_get_lenhalf_insert_mid_dest_slot_len_only"
            }
        }
    }
}

impl std::fmt::Display for ArrayTextEditProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayTextEditRoute {
    pub(super) block: BasicBlockId,
    pub(super) get_instruction_index: usize,
    pub(super) set_instruction_index: usize,
    pub(super) array_value: ValueId,
    pub(super) destination_array_value: ValueId,
    pub(super) index_value: ValueId,
    pub(super) source_value: ValueId,
    pub(super) length_value: ValueId,
    pub(super) split_value: ValueId,
    pub(super) result_value: ValueId,
    pub(super) result_len_value: Option<ValueId>,
    pub(super) middle_value: ValueId,
    pub(super) middle_text: String,
    pub(super) middle_byte_len: usize,
    pub(super) skip_instruction_indices: Vec<usize>,
    pub(super) edit_kind: ArrayTextEditKind,
    pub(super) split_policy: ArrayTextEditSplitPolicy,
    pub(super) proof: ArrayTextEditProof,
}

impl ArrayTextEditRoute {
    pub fn block(&self) -> BasicBlockId {
        self.block
    }

    pub fn get_instruction_index(&self) -> usize {
        self.get_instruction_index
    }

    pub fn set_instruction_index(&self) -> usize {
        self.set_instruction_index
    }

    pub fn array_value(&self) -> ValueId {
        self.array_value
    }

    pub fn destination_array_value(&self) -> ValueId {
        self.destination_array_value
    }

    pub fn index_value(&self) -> ValueId {
        self.index_value
    }

    pub fn source_value(&self) -> ValueId {
        self.source_value
    }

    pub fn length_value(&self) -> ValueId {
        self.length_value
    }

    pub fn split_value(&self) -> ValueId {
        self.split_value
    }

    pub fn result_value(&self) -> ValueId {
        self.result_value
    }

    pub fn result_len_value(&self) -> Option<ValueId> {
        self.result_len_value
    }

    pub fn middle_value(&self) -> ValueId {
        self.middle_value
    }

    pub fn middle_text(&self) -> &str {
        &self.middle_text
    }

    pub fn middle_byte_len(&self) -> usize {
        self.middle_byte_len
    }

    pub fn skip_instruction_indices(&self) -> &[usize] {
        &self.skip_instruction_indices
    }

    pub fn edit_kind(&self) -> &'static str {
        self.edit_kind.as_str()
    }

    pub fn split_policy(&self) -> String {
        self.split_policy.to_string()
    }

    pub fn proof(&self) -> &'static str {
        self.proof.as_str()
    }

    pub fn is_lenhalf_insert_mid_same_slot(&self) -> bool {
        self.edit_kind == ArrayTextEditKind::InsertMidConst
            && self.split_policy == (ArrayTextEditSplitPolicy::SourceLenDivConst { divisor: 2 })
            && self.proof == ArrayTextEditProof::ArrayGetLenHalfInsertMidSameSlot
    }

    pub fn is_lenhalf_insert_mid_dest_slot_len_only(&self) -> bool {
        self.edit_kind == ArrayTextEditKind::InsertMidConst
            && self.split_policy == (ArrayTextEditSplitPolicy::SourceLenDivConst { divisor: 2 })
            && self.proof == ArrayTextEditProof::ArrayGetLenHalfInsertMidDestSlotLenOnly
    }
}
