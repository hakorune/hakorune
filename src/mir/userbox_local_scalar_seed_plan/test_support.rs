use super::*;

pub(crate) fn point_local_i64() -> UserBoxLocalScalarSeedRoute {
    UserBoxLocalScalarSeedRoute {
        kind: UserBoxLocalScalarSeedKind::PointLocalI64,
        box_name: "Point".to_string(),
        block: BasicBlockId::new(0),
        newbox_instruction_index: 2,
        box_value: ValueId::new(3),
        copy_value: None,
        result_value: ValueId::new(6),
        proof: UserBoxLocalScalarSeedProof::PointFieldLocalScalarSeed,
        payload: UserBoxLocalScalarSeedPayload::PointI64Pair {
            x_field: "x".to_string(),
            y_field: "y".to_string(),
            set_x_instruction_index: 3,
            set_y_instruction_index: 4,
            get_x_instruction_index: 5,
            get_y_instruction_index: 6,
            x_value: ValueId::new(1),
            y_value: ValueId::new(2),
            get_x_value: ValueId::new(4),
            get_y_value: ValueId::new(5),
            x_i64: 41,
            y_i64: 2,
        },
    }
}

pub(crate) fn point_copy_local_i64() -> UserBoxLocalScalarSeedRoute {
    UserBoxLocalScalarSeedRoute {
        kind: UserBoxLocalScalarSeedKind::PointCopyLocalI64,
        box_name: "Point".to_string(),
        block: BasicBlockId::new(0),
        newbox_instruction_index: 2,
        box_value: ValueId::new(3),
        copy_value: Some(ValueId::new(6)),
        result_value: ValueId::new(9),
        proof: UserBoxLocalScalarSeedProof::PointFieldLocalScalarSeed,
        payload: UserBoxLocalScalarSeedPayload::PointI64Pair {
            x_field: "x".to_string(),
            y_field: "y".to_string(),
            set_x_instruction_index: 3,
            set_y_instruction_index: 4,
            get_x_instruction_index: 6,
            get_y_instruction_index: 7,
            x_value: ValueId::new(1),
            y_value: ValueId::new(2),
            get_x_value: ValueId::new(7),
            get_y_value: ValueId::new(8),
            x_i64: 41,
            y_i64: 2,
        },
    }
}

pub(crate) fn flag_copy_local_bool() -> UserBoxLocalScalarSeedRoute {
    UserBoxLocalScalarSeedRoute {
        kind: UserBoxLocalScalarSeedKind::FlagCopyLocalBool,
        box_name: "Flag".to_string(),
        block: BasicBlockId::new(0),
        newbox_instruction_index: 1,
        box_value: ValueId::new(2),
        copy_value: Some(ValueId::new(3)),
        result_value: ValueId::new(4),
        proof: UserBoxLocalScalarSeedProof::FlagFieldLocalScalarSeed,
        payload: UserBoxLocalScalarSeedPayload::SingleField {
            field: "enabled".to_string(),
            set_instruction_index: 2,
            get_instruction_index: 4,
            field_value: ValueId::new(1),
            get_field_value: ValueId::new(4),
            payload: UserBoxLocalScalarSeedSinglePayload::I64(1),
        },
    }
}

pub(crate) fn pointf_copy_local_f64() -> UserBoxLocalScalarSeedRoute {
    UserBoxLocalScalarSeedRoute {
        kind: UserBoxLocalScalarSeedKind::PointFCopyLocalF64,
        box_name: "PointF".to_string(),
        block: BasicBlockId::new(0),
        newbox_instruction_index: 1,
        box_value: ValueId::new(2),
        copy_value: Some(ValueId::new(3)),
        result_value: ValueId::new(4),
        proof: UserBoxLocalScalarSeedProof::PointFFieldLocalScalarSeed,
        payload: UserBoxLocalScalarSeedPayload::SingleField {
            field: "x".to_string(),
            set_instruction_index: 2,
            get_instruction_index: 4,
            field_value: ValueId::new(1),
            get_field_value: ValueId::new(4),
            payload: UserBoxLocalScalarSeedSinglePayload::F64Bits(1.5f64.to_bits()),
        },
    }
}
