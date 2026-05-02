use super::*;

pub(crate) fn json_len_only_route() -> ArrayStringLenWindowRoute {
    ArrayStringLenWindowRoute {
        block: BasicBlockId::new(7),
        instruction_index: 3,
        array_value: ValueId::new(10),
        index_value: ValueId::new(11),
        source_value: ValueId::new(12),
        len_instruction_index: 5,
        len_value: ValueId::new(13),
        skip_instruction_indices: vec![4, 5],
        mode: ArrayStringLenWindowMode::LenOnly,
        proof: ArrayStringLenWindowProof::ArrayGetLenNoLaterSourceUse,
    }
}

pub(crate) fn json_keep_get_live_route() -> ArrayStringLenWindowRoute {
    ArrayStringLenWindowRoute {
        block: BasicBlockId::new(8),
        instruction_index: 4,
        array_value: ValueId::new(20),
        index_value: ValueId::new(21),
        source_value: ValueId::new(22),
        len_instruction_index: 6,
        len_value: ValueId::new(23),
        skip_instruction_indices: vec![6],
        mode: ArrayStringLenWindowMode::KeepGetLive,
        proof: ArrayStringLenWindowProof::ArrayGetLenKeepSourceLive,
    }
}
