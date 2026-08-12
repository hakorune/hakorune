#!/usr/bin/env python3
import copy
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))
TESTS = Path(__file__).resolve().parent
if str(TESTS) not in sys.path:
    sys.path.insert(0, str(TESTS))

from builders.dynamic_v2_aot_admission import (
    DynamicV2AotAdmissionError,
    load_selected_dynamic_v2_aot_admission,
)
from instructions.mir_call.selected_dynamic_v2 import (
    inspect_selected_dynamic_v2_call,
    require_selected_dynamic_v2_call,
)

from test_a_prime_i64_capability import _valid_function_data


def _valid_admission_data():
    data = _valid_function_data()
    data["metadata"]["dynamic_v2_aot_call_admission_v1"] = {
        "schema_version": 1,
        "contract_id": "hako.text.scan@1",
        "profile": 1,
        "abi_revision": 1,
        "wire_revision": 2,
        "registry_generation": 7,
        "plan_stamp": {"compiler_domain": 1, "invocation_ordinal": 9},
        "calls": [
            {
                "role": "substring",
                "block": 1,
                "instruction_index": 3,
                "entry_id": 1,
                "symbol": "hako.text.scan.substring.v1",
                "abi_revision": 1,
                "wire_revision": 2,
                "receiver_lane": "opaque_handle",
                "argument_lanes": ["immediate_i64", "immediate_i64"],
                "result_lane": "opaque_handle",
                "lease": "end_authorized",
            },
            {
                "role": "index_of",
                "block": 1,
                "instruction_index": 4,
                "entry_id": 2,
                "symbol": "hako.text.scan.index_of.v1",
                "abi_revision": 1,
                "wire_revision": 2,
                "receiver_lane": "opaque_handle",
                "argument_lanes": ["opaque_handle"],
                "result_lane": "immediate_i64",
                "lease": "none",
            },
        ],
    }
    return data


class TestDynamicV2AotAdmission(unittest.TestCase):
    def test_absent_metadata_is_not_selected(self):
        self.assertIsNone(load_selected_dynamic_v2_aot_admission(_valid_function_data()))
        self.assertIsNone(inspect_selected_dynamic_v2_call(_valid_function_data(), 1, 3))
        with self.assertRaises(DynamicV2AotAdmissionError):
            require_selected_dynamic_v2_call(_valid_function_data(), 1, 3)

    def test_valid_admission_is_site_indexable(self):
        view = load_selected_dynamic_v2_aot_admission(_valid_admission_data())
        self.assertEqual(view.registry_generation, 7)
        self.assertEqual(view.invocation_ordinal, 9)
        self.assertEqual(view.require_call_site(1, 3).entry_id, 1)
        self.assertEqual(inspect_selected_dynamic_v2_call(_valid_admission_data(), 1, 4).role, "index_of")
        self.assertEqual(require_selected_dynamic_v2_call(_valid_admission_data(), 1, 3).role, "substring")

    def test_unknown_or_duplicate_metadata_is_rejected(self):
        data = _valid_admission_data()
        data["metadata"]["dynamic_v2_aot_call_admission_v1"]["unexpected"] = True
        with self.assertRaises(DynamicV2AotAdmissionError):
            load_selected_dynamic_v2_aot_admission(data)

        data = _valid_admission_data()
        calls = data["metadata"]["dynamic_v2_aot_call_admission_v1"]["calls"]
        calls[1]["block"] = calls[0]["block"]
        calls[1]["instruction_index"] = calls[0]["instruction_index"]
        with self.assertRaises(DynamicV2AotAdmissionError):
            load_selected_dynamic_v2_aot_admission(data)

    def test_stamp_entry_and_lane_drift_is_rejected(self):
        for mutate in (
            lambda d: d["metadata"]["dynamic_v2_aot_call_admission_v1"]["plan_stamp"].update(
                {"invocation_ordinal": 0}
            ),
            lambda d: d["metadata"]["dynamic_v2_aot_call_admission_v1"]["calls"][0].update(
                {"entry_id": 2}
            ),
            lambda d: d["metadata"]["dynamic_v2_aot_call_admission_v1"]["calls"][1].update(
                {"result_lane": "opaque_handle"}
            ),
            lambda d: d["metadata"]["dynamic_v2_aot_call_admission_v1"]["calls"][0].update(
                {"instruction_index": 99}
            ),
        ):
            data = _valid_admission_data()
            mutate(data)
            with self.assertRaises(DynamicV2AotAdmissionError):
                load_selected_dynamic_v2_aot_admission(data)

    def test_site_lookup_must_not_repair_or_fallback(self):
        data = _valid_admission_data()
        with self.assertRaises(DynamicV2AotAdmissionError):
            inspect_selected_dynamic_v2_call(data, 9, 9)

        data = copy.deepcopy(_valid_admission_data())
        data["metadata"]["a_prime_i64_physical_receipt"]["call_edges"][0][
            "target_fingerprint"
        ] = "indexOf/1"
        with self.assertRaises(DynamicV2AotAdmissionError):
            load_selected_dynamic_v2_aot_admission(data)


if __name__ == "__main__":
    unittest.main()
