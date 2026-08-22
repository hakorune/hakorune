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
    data["metadata"]["dynamic_v2_aot_call_admission_v2"] = {
        "schema_version": 2,
        "contract_id": "hako.text.scan@1",
        "profile": 1,
        "abi_revision": 1,
        "wire_revision": 2,
        "registry_generation": 7,
        "plan_stamp": {"compiler_domain": 1, "invocation_ordinal": 9},
        "return_type": "i64",
        "return_lane": "immediate_i64",
        "function_effects": 16,
        "formal_parameters": [
            {"role": "src", "value_id": 0, "lane": "opaque_handle"},
            {"role": "pos", "value_id": 1, "lane": "immediate_i64"},
            {"role": "end", "value_id": 2, "lane": "immediate_i64"},
            {"role": "pred_chars", "value_id": 3, "lane": "opaque_handle"},
        ],
        "calls": [
            {
                "role": "substring",
                "site_id": 0,
                "entry_id": 1,
                "symbol": "hako.text.scan.substring.v1",
                "abi_revision": 1,
                "wire_revision": 2,
                "receiver_lane": "opaque_handle",
                "argument_lanes": ["immediate_i64", "immediate_i64"],
                "result_lane": "opaque_handle",
                "lease": "end_authorized",
                "normal_shape": "end_authorized_handle",
                "outcome_slot": 0,
                "normal_result_dst": 20,
                "effects": 16,
                "source_block": 0,
                "receiver": 10,
                "arguments": [12, 13],
                "normal_landing": 1,
                "fault_landing": 2,
                "fault_terminal_block": 2,
                "normal_result_block": 1,
                "normal_result_index": 0,
            },
            {
                "role": "index_of",
                "site_id": 1,
                "entry_id": 2,
                "symbol": "hako.text.scan.index_of.v1",
                "abi_revision": 1,
                "wire_revision": 2,
                "receiver_lane": "opaque_handle",
                "argument_lanes": ["opaque_handle"],
                "result_lane": "immediate_i64",
                "lease": "none",
                "normal_shape": "immediate_i64",
                "outcome_slot": 1,
                "normal_result_dst": 21,
                "effects": 16,
                "source_block": 3,
                "receiver": 14,
                "arguments": [20],
                "normal_landing": 4,
                "fault_landing": 5,
                "fault_terminal_block": 5,
                "normal_result_block": 4,
                "normal_result_index": 0,
            },
        ],
        "end_facts": [
            {"site_id": 0, "lease_slot": 0, "block": 2, "instruction_index": 0},
            {"site_id": 0, "lease_slot": 0, "block": 4, "instruction_index": 0},
            {"site_id": 0, "lease_slot": 0, "block": 5, "instruction_index": 0},
        ],
    }
    return data


class TestDynamicV2AotAdmission(unittest.TestCase):
    def test_absent_metadata_is_not_selected(self):
        self.assertIsNone(load_selected_dynamic_v2_aot_admission(_valid_function_data()))
        self.assertIsNone(inspect_selected_dynamic_v2_call(_valid_function_data(), 0))
        with self.assertRaises(DynamicV2AotAdmissionError):
            require_selected_dynamic_v2_call(_valid_function_data(), 0)

    def test_valid_admission_is_site_indexable(self):
        view = load_selected_dynamic_v2_aot_admission(_valid_admission_data())
        self.assertEqual(view.registry_generation, 7)
        self.assertEqual(view.invocation_ordinal, 9)
        self.assertEqual(view.return_type, "i64")
        self.assertEqual(view.return_lane, "immediate_i64")
        self.assertEqual(view.function_effects, 16)
        self.assertEqual(view.formal_parameters[1].role, "pos")
        self.assertEqual(view.require_call_site(0).entry_id, 1)
        self.assertEqual(view.require_call_site(0).normal_result_dst, 20)
        self.assertEqual(view.require_call_site(0).arguments, (12, 13))
        self.assertEqual(view.require_call_site(1).source_block, 3)
        self.assertEqual(len(view.end_facts), 3)
        self.assertEqual(inspect_selected_dynamic_v2_call(_valid_admission_data(), 1).role, "index_of")
        self.assertEqual(require_selected_dynamic_v2_call(_valid_admission_data(), 0).role, "substring")

    def test_unknown_or_duplicate_metadata_is_rejected(self):
        data = _valid_admission_data()
        data["metadata"]["dynamic_v2_aot_call_admission_v2"]["unexpected"] = True
        with self.assertRaises(DynamicV2AotAdmissionError):
            load_selected_dynamic_v2_aot_admission(data)

        data = _valid_admission_data()
        data["metadata"]["dynamic_v2_aot_call_admission_v2"]["calls"][0]["block"] = 1
        with self.assertRaises(DynamicV2AotAdmissionError):
            load_selected_dynamic_v2_aot_admission(data)

        data = _valid_admission_data()
        calls = data["metadata"]["dynamic_v2_aot_call_admission_v2"]["calls"]
        calls[1]["site_id"] = calls[0]["site_id"]
        with self.assertRaises(DynamicV2AotAdmissionError):
            load_selected_dynamic_v2_aot_admission(data)

    def test_stamp_entry_and_lane_drift_is_rejected(self):
        for mutate in (
            lambda d: d["metadata"]["dynamic_v2_aot_call_admission_v2"]["plan_stamp"].update(
                {"invocation_ordinal": 0}
            ),
            lambda d: d["metadata"]["dynamic_v2_aot_call_admission_v2"]["calls"][0].update(
                {"entry_id": 2}
            ),
            lambda d: d["metadata"]["dynamic_v2_aot_call_admission_v2"]["calls"][1].update(
                {"result_lane": "opaque_handle"}
            ),
            lambda d: d["metadata"]["dynamic_v2_aot_call_admission_v2"]["calls"][0].update(
                {"normal_shape": "immediate_i64"}
            ),
            lambda d: d["metadata"]["dynamic_v2_aot_call_admission_v2"]["formal_parameters"][1].update(
                {"lane": "opaque_handle"}
            ),
            lambda d: d["metadata"]["dynamic_v2_aot_call_admission_v2"].update(
                {"return_lane": "opaque_handle"}
            ),
            lambda d: d["metadata"]["dynamic_v2_aot_call_admission_v2"]["calls"][0].update(
                {"site_id": 99}
            ),
        ):
            data = _valid_admission_data()
            mutate(data)
            with self.assertRaises(DynamicV2AotAdmissionError):
                load_selected_dynamic_v2_aot_admission(data)

    def test_physical_site_and_end_drift_is_rejected(self):
        mutations = (
            lambda d: d["metadata"]["dynamic_v2_aot_call_admission_v2"]["calls"][1].update(
                {"arguments": [99]}
            ),
            lambda d: d["metadata"]["dynamic_v2_aot_call_admission_v2"]["calls"][0].update(
                {"normal_landing": 2}
            ),
            lambda d: d["metadata"]["dynamic_v2_aot_call_admission_v2"]["end_facts"].pop(),
        )
        for mutate in mutations:
            data = _valid_admission_data()
            mutate(data)
            with self.assertRaises(DynamicV2AotAdmissionError):
                load_selected_dynamic_v2_aot_admission(data)

    def test_u64_metadata_boundaries_are_checked(self):
        max_u64 = (1 << 64) - 1
        data = _valid_admission_data()
        admission = data["metadata"]["dynamic_v2_aot_call_admission_v2"]
        admission["registry_generation"] = max_u64
        admission["plan_stamp"] = {
            "compiler_domain": max_u64,
            "invocation_ordinal": max_u64,
        }
        view = load_selected_dynamic_v2_aot_admission(data)
        self.assertEqual(view.registry_generation, max_u64)
        self.assertEqual(view.compiler_domain, max_u64)
        self.assertEqual(view.invocation_ordinal, max_u64)

        for field, value in (
            ("registry_generation", max_u64 + 1),
            ("compiler_domain", max_u64 + 1),
            ("invocation_ordinal", max_u64 + 1),
        ):
            data = _valid_admission_data()
            admission = data["metadata"]["dynamic_v2_aot_call_admission_v2"]
            if field in admission["plan_stamp"]:
                admission["plan_stamp"][field] = value
            else:
                admission[field] = value
            with self.assertRaises(DynamicV2AotAdmissionError):
                load_selected_dynamic_v2_aot_admission(data)

    def test_site_lookup_must_not_repair_or_fallback(self):
        data = _valid_admission_data()
        with self.assertRaises(DynamicV2AotAdmissionError):
            inspect_selected_dynamic_v2_call(data, 9)

        data = copy.deepcopy(_valid_admission_data())
        data["metadata"]["a_prime_i64_physical_receipt"]["call_edges"][0][
            "target_fingerprint"
        ] = "indexOf/1"
        with self.assertRaises(DynamicV2AotAdmissionError):
            load_selected_dynamic_v2_aot_admission(data)

        data = _valid_admission_data()
        data["metadata"]["dynamic_v2_aot_call_admission_v2"]["calls"][0]["role"] = "index_of"
        with self.assertRaises(DynamicV2AotAdmissionError):
            load_selected_dynamic_v2_aot_admission(data)


if __name__ == "__main__":
    unittest.main()
