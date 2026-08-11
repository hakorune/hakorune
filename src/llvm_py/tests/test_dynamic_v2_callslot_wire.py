import ctypes
import unittest
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from builders.dynamic_v2_callslot_wire import (
    DynamicV2CallOutV1,
    DynamicV2WireValueV1,
    DISPOSITION_END_AUTHORIZED,
    DISPOSITION_NONE,
    FAULT_NONE,
    FAULT_RUNTIME,
    FORWARDED_NONE,
    STATUS_FAULT,
    STATUS_NORMAL,
    STATUS_SUSPENDED,
    TAG_HOST_HANDLE,
    TAG_IMMEDIATE_I64,
    TAG_INVALID,
    validate_call_out,
    validate_wire_value,
)
from builders import dynamic_v2_callslot_wire as wire


def normal_end_authorized():
    return {
        "status": STATUS_NORMAL,
        "fault_code": FAULT_NONE,
        "result_tag": TAG_HOST_HANDLE,
        "disposition": DISPOSITION_END_AUTHORIZED,
        "forwarded_input": FORWARDED_NONE,
        "reserved": 0,
        "value_payload": 42,
        "lease_token": 7,
        "continuation_token": 0,
    }


class TestDynamicV2CallslotWire(unittest.TestCase):
    def test_layout_matches_c_schema(self):
        self.assertEqual(ctypes.sizeof(DynamicV2WireValueV1), 16)
        self.assertEqual(ctypes.alignment(DynamicV2WireValueV1), 8)
        self.assertEqual(ctypes.sizeof(DynamicV2CallOutV1), 48)
        self.assertEqual(ctypes.alignment(DynamicV2CallOutV1), 8)
        self.assertEqual(DynamicV2CallOutV1.status.offset, 0)
        self.assertEqual(DynamicV2CallOutV1.fault_code.offset, 4)
        self.assertEqual(DynamicV2CallOutV1.result_tag.offset, 8)
        self.assertEqual(DynamicV2CallOutV1.disposition.offset, 12)
        self.assertEqual(DynamicV2CallOutV1.forwarded_input.offset, 16)
        self.assertEqual(DynamicV2CallOutV1.reserved.offset, 20)
        self.assertEqual(DynamicV2CallOutV1.value_payload.offset, 24)
        self.assertEqual(DynamicV2CallOutV1.lease_token.offset, 32)
        self.assertEqual(DynamicV2CallOutV1.continuation_token.offset, 40)

    def test_constants_are_revision_two_wire_values(self):
        self.assertEqual(wire.WIRE_REVISION, 2)
        self.assertEqual(
            [wire.TAG_INVALID, wire.TAG_HOST_HANDLE, wire.TAG_IMMEDIATE_I64],
            [0, 1, 2],
        )
        self.assertEqual(
            [wire.STATUS_NORMAL, wire.STATUS_FAULT, wire.STATUS_SUSPENDED],
            [0, 1, 2],
        )
        self.assertEqual(
            [
                wire.DISPOSITION_NONE,
                wire.DISPOSITION_FORWARDED,
                wire.DISPOSITION_END_AUTHORIZED,
            ],
            [0, 1, 2],
        )
        self.assertEqual(
            [
                wire.FAULT_NONE,
                wire.FAULT_INVALID_RECEIVER,
                wire.FAULT_INVALID_HANDLE,
                wire.FAULT_ARITY,
                wire.FAULT_UNSUPPORTED_SLOT,
                wire.FAULT_TYPE_MISMATCH,
                wire.FAULT_RANGE,
                wire.FAULT_RUNTIME,
                wire.FAULT_INVALID_RESULT,
            ],
            list(range(9)),
        )

    def test_normal_zero_is_valid(self):
        out = normal_end_authorized()
        out["value_payload"] = 0
        validate_call_out(out, argc=0)

    def test_immediate_i64_normal_has_no_lifecycle_disposition(self):
        out = normal_end_authorized()
        out.update(
            result_tag=TAG_IMMEDIATE_I64,
            disposition=DISPOSITION_NONE,
            forwarded_input=FORWARDED_NONE,
            value_payload=0,
            lease_token=0,
        )
        validate_call_out(out, argc=0)

    def test_immediate_i64_normal_rejects_lease_or_forwarded_lane(self):
        out = normal_end_authorized()
        out.update(
            result_tag=TAG_IMMEDIATE_I64,
            disposition=DISPOSITION_NONE,
            forwarded_input=FORWARDED_NONE,
            lease_token=1,
        )
        with self.assertRaises(ValueError):
            validate_call_out(out)
        out.update(
            disposition=wire.DISPOSITION_END_AUTHORIZED,
            forwarded_input=FORWARDED_NONE,
            lease_token=1,
        )
        with self.assertRaises(ValueError):
            validate_call_out(out)
        out.update(lease_token=0)
        out.update(
            disposition=wire.DISPOSITION_FORWARDED,
            forwarded_input=0,
            lease_token=0,
        )
        with self.assertRaises(ValueError):
            validate_call_out(out)

    def test_end_authorized_sentinel_is_not_checked_as_a_lane(self):
        validate_call_out(normal_end_authorized(), argc=0)
        with self.assertRaises(ValueError):
            validate_call_out(normal_end_authorized(), argc=0xFFFFFFFF)

    def test_fault_cannot_publish_result_or_lease(self):
        out = normal_end_authorized()
        out.update(
            status=STATUS_FAULT,
            fault_code=FAULT_RUNTIME,
            result_tag=TAG_INVALID,
            disposition=DISPOSITION_NONE,
            value_payload=0,
            lease_token=0,
        )
        validate_call_out(out)
        out["lease_token"] = 1
        with self.assertRaises(ValueError):
            validate_call_out(out)

    def test_suspended_is_schema_only(self):
        out = normal_end_authorized()
        out.update(
            status=STATUS_SUSPENDED,
            result_tag=TAG_INVALID,
            disposition=DISPOSITION_NONE,
            forwarded_input=FORWARDED_NONE,
            value_payload=0,
            lease_token=0,
            continuation_token=9,
        )
        validate_call_out(out)
        with self.assertRaises(ValueError):
            validate_call_out(out, allow_suspended=False)

    def test_unknown_and_reserved_values_reject(self):
        with self.assertRaises(ValueError):
            validate_wire_value(99, 0, 0)
        out = normal_end_authorized()
        out["reserved"] = 1
        with self.assertRaises(ValueError):
            validate_call_out(out)

    def test_fixed_width_overflow_rejects_before_ctypes_truncation(self):
        with self.assertRaises(ValueError):
            validate_wire_value(0, 0, 1 << 64)
        out = normal_end_authorized()
        out["status"] = 1 << 32
        with self.assertRaises(ValueError):
            validate_call_out(out)
        with self.assertRaises(ValueError):
            validate_call_out(out, argc=1 << 32)


if __name__ == "__main__":
    unittest.main()
