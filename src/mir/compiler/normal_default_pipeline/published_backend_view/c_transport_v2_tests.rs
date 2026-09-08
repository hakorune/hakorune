//! Wire projection tests; these do not claim C consumer or source admission.
use super::*;

fn row(action: ProjectionAction) -> ValueProjectionRow {
    action.row(std::ptr::null(), 7, false)
}

#[test]
fn map_literal_v2_exact_bits_do_not_use_lossy_original_registers() {
    for (action, kind, bits) in [
        (ProjectionAction::ExactI64(-1), ValueKind::I64, u64::MAX),
        (ProjectionAction::ExactBool(true), ValueKind::Bool, 1),
        (ProjectionAction::ExactBool(false), ValueKind::Bool, 0),
        (
            ProjectionAction::ExactF64(0x7ff8_0000_0000_0042),
            ValueKind::F64,
            0x7ff8_0000_0000_0042,
        ),
        (
            ProjectionAction::ExactF64((-0.0_f64).to_bits()),
            ValueKind::F64,
            1 << 63,
        ),
        (ProjectionAction::ExactVoid, ValueKind::Void, 0),
    ] {
        let row = row(action);
        assert_eq!(row.action, ActionKind::ExactBits as u32);
        assert_eq!(row.value_kind, kind as u32);
        assert_eq!(row.payload, bits);
        assert_eq!((row.encoding, row.flags, row.source_ordinal), (0, 0, 0));
        assert_eq!(row.operation, 0);
    }
}

#[test]
fn map_literal_v2_original_actions_cannot_omit_their_producer() {
    for (action, kind, encoding) in [
        (
            ProjectionAction::OriginalI64,
            ValueKind::I64,
            OriginalEncoding::I64Bits,
        ),
        (
            ProjectionAction::OriginalBoolI64,
            ValueKind::Bool,
            OriginalEncoding::I64Bits,
        ),
        (
            ProjectionAction::OriginalBoolI1,
            ValueKind::Bool,
            OriginalEncoding::BoolI1ZeroExtend,
        ),
        (
            ProjectionAction::OriginalHandle,
            ValueKind::Handle,
            OriginalEncoding::I64Bits,
        ),
    ] {
        let row = row(action);
        assert_eq!(row.action, ActionKind::OriginalValue as u32);
        assert_eq!(row.value_kind, kind as u32);
        assert_eq!(row.encoding, encoding as u32);
        assert_eq!(row.flags, ORIGINAL_REQUIRED);
        assert_eq!((row.payload, row.source_ordinal), (0, 0));
        assert_eq!(row.operation, 0);
    }
}

#[test]
fn map_literal_v2_graph_actions_carry_no_second_operand_graph() {
    for (action, kind, ordinal) in [
        (ProjectionAction::Copy, ActionKind::Copy, 0),
        (ProjectionAction::Phi, ActionKind::Phi, 0),
        (ProjectionAction::Select, ActionKind::Select, 0),
        (ProjectionAction::Formal(3), ActionKind::Formal, 3),
    ] {
        let row = action.row(std::ptr::null(), 7, true);
        assert_eq!(row.action, kind as u32);
        assert_eq!(row.source_ordinal, ordinal);
        assert_eq!(row.flags, ORIGINAL_REQUIRED);
        assert_eq!((row.value_kind, row.encoding, row.payload), (0, 0, 0));
        assert_eq!(row.operation, 0);
    }
}

#[test]
fn map_literal_v2_same_result_kind_does_not_erase_selected_operation() {
    for (operation, wire, kind, encoding) in [
        (
            PhysicalOperation::I64Binary,
            1,
            ValueKind::I64,
            OriginalEncoding::I64Bits,
        ),
        (
            PhysicalOperation::I64Compare,
            2,
            ValueKind::Bool,
            OriginalEncoding::BoolI1ZeroExtend,
        ),
        (
            PhysicalOperation::BoolCompare,
            3,
            ValueKind::Bool,
            OriginalEncoding::BoolI1ZeroExtend,
        ),
        (
            PhysicalOperation::StringCompare,
            4,
            ValueKind::Bool,
            OriginalEncoding::BoolI1ZeroExtend,
        ),
        (
            PhysicalOperation::StringConcat,
            5,
            ValueKind::Handle,
            OriginalEncoding::I64Bits,
        ),
        (
            PhysicalOperation::I64Not,
            6,
            ValueKind::Bool,
            OriginalEncoding::BoolI1ZeroExtend,
        ),
        (
            PhysicalOperation::BoolNot,
            7,
            ValueKind::Bool,
            OriginalEncoding::BoolI1ZeroExtend,
        ),
    ] {
        let action = ProjectionAction::Operation(operation);
        let row = row(action);
        assert!(action.requires_original_producer());
        assert_eq!(row.action, ActionKind::Operation as u32);
        assert_eq!(row.operation, wire);
        assert_eq!(
            (row.value_kind, row.encoding),
            (kind as u32, encoding as u32)
        );
        assert_eq!(row.flags, ORIGINAL_REQUIRED);
        assert_eq!((row.payload, row.source_ordinal), (0, 0));
    }
}
