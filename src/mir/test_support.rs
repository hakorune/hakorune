//! Shared test-only fixtures for MIR lifecycle boundary tests.

use super::a_prime_i64_physical_receipt::{
    APrimeI64BackendFamilyV1, APrimeI64CallArgumentReceiptV1, APrimeI64CallEdgeReceiptV1,
    APrimeI64LaneV1, APrimeI64ParameterReceiptV1, APrimeI64PhysicalReceiptV1,
    APrimeI64ReturnReceiptV1, A_PRIME_I64_FORMAL_PARAMETER_COUNT,
};
use super::checked_callout::CheckedCallOutSiteIdV1;
use super::{BasicBlockId, ValueId};

pub(crate) fn a_prime_receipt() -> APrimeI64PhysicalReceiptV1 {
    APrimeI64PhysicalReceiptV1::seal_for_test(
        APrimeI64BackendFamilyV1::Llvm,
        A_PRIME_I64_FORMAL_PARAMETER_COUNT,
        vec![
            APrimeI64ParameterReceiptV1 {
                role: "pos".into(),
                formal_parameter_index: 1,
                value_id: ValueId::new(11),
                lane: APrimeI64LaneV1::ImmediateI64,
            },
            APrimeI64ParameterReceiptV1 {
                role: "end".into(),
                formal_parameter_index: 2,
                value_id: ValueId::new(12),
                lane: APrimeI64LaneV1::ImmediateI64,
            },
        ],
        vec![
            APrimeI64CallEdgeReceiptV1 {
                site_id: CheckedCallOutSiteIdV1::from_test(0),
                role: "substring".into(),
                target_fingerprint: "substring/2".into(),
                receiver_role: "src".into(),
                receiver_value_id: ValueId::new(10),
                receiver_lane: APrimeI64LaneV1::OpaqueHandle,
                arguments: vec![
                    APrimeI64CallArgumentReceiptV1 {
                        ordinal: 0,
                        role: "start".into(),
                        value_id: ValueId::new(12),
                        lane: APrimeI64LaneV1::ImmediateI64,
                    },
                    APrimeI64CallArgumentReceiptV1 {
                        ordinal: 1,
                        role: "end".into(),
                        value_id: ValueId::new(13),
                        lane: APrimeI64LaneV1::ImmediateI64,
                    },
                ],
                result_value_id: ValueId::new(20),
                result_lane: APrimeI64LaneV1::OpaqueHandle,
            },
            APrimeI64CallEdgeReceiptV1 {
                site_id: CheckedCallOutSiteIdV1::from_test(1),
                role: "index_of".into(),
                target_fingerprint: "indexOf/1".into(),
                receiver_role: "pred_chars".into(),
                receiver_value_id: ValueId::new(14),
                receiver_lane: APrimeI64LaneV1::OpaqueHandle,
                arguments: vec![APrimeI64CallArgumentReceiptV1 {
                    ordinal: 0,
                    role: "ch".into(),
                    value_id: ValueId::new(20),
                    lane: APrimeI64LaneV1::OpaqueHandle,
                }],
                result_value_id: ValueId::new(21),
                result_lane: APrimeI64LaneV1::ImmediateI64,
            },
        ],
        vec![
            APrimeI64ReturnReceiptV1 {
                site: "inner".into(),
                block: BasicBlockId::new(2),
                value_id: ValueId::new(30),
                lane: APrimeI64LaneV1::ImmediateI64,
            },
            APrimeI64ReturnReceiptV1 {
                site: "outer".into(),
                block: BasicBlockId::new(3),
                value_id: ValueId::new(31),
                lane: APrimeI64LaneV1::ImmediateI64,
            },
        ],
    )
    .expect("valid A-prime test fixture")
}
