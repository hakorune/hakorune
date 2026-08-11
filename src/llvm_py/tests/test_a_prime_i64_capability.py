#!/usr/bin/env python3
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from builders.a_prime_i64_capability import (
    APrimeI64CapabilityError,
    load_selected_a_prime_capability,
)


def _valid_function_data():
    return {
        "name": "ParserScanLoopBox.skip_while/4",
        "params": [10, 11, 12, 13],
        "metadata": {
            "a_prime_i64_physical_receipt": {
                "schema_version": 2,
                "backend_family": "llvm",
                "formal_parameter_count": 4,
                "fallback": False,
                "retry": False,
                "parameters": [
                    {"role": "pos", "formal_parameter_index": 1, "value_id": 11, "lane": "immediate_i64"},
                    {"role": "end", "formal_parameter_index": 2, "value_id": 12, "lane": "immediate_i64"},
                ],
                "call_edges": [
                    {
                        "role": "substring",
                        "block": 1,
                        "instruction_index": 3,
                        "target_fingerprint": "substring/2",
                        "receiver_role": "src",
                        "receiver_value_id": 10,
                        "receiver_lane": "opaque_handle",
                        "arguments": [
                            {"ordinal": 0, "role": "start", "value_id": 12, "lane": "immediate_i64"},
                            {"ordinal": 1, "role": "end", "value_id": 13, "lane": "immediate_i64"},
                        ],
                        "result_value_id": 20,
                        "result_lane": "opaque_handle",
                    },
                    {
                        "role": "index_of",
                        "block": 1,
                        "instruction_index": 4,
                        "target_fingerprint": "indexOf/1",
                        "receiver_role": "pred_chars",
                        "receiver_value_id": 14,
                        "receiver_lane": "opaque_handle",
                        "arguments": [
                            {"ordinal": 0, "role": "ch", "value_id": 20, "lane": "opaque_handle"}
                        ],
                        "result_value_id": 21,
                        "result_lane": "immediate_i64",
                    },
                ],
                "returns": [
                    {"site": "inner", "block": 2, "value_id": 30, "lane": "immediate_i64"},
                    {"site": "outer", "block": 3, "value_id": 31, "lane": "immediate_i64"},
                ],
            }
        },
    }


class TestAPrimeI64Capability(unittest.TestCase):
    def test_absent_marker_preserves_legacy_route(self):
        self.assertIsNone(load_selected_a_prime_capability({"params": [1]}))

    def test_valid_receipt_is_strictly_indexable(self):
        view = load_selected_a_prime_capability(_valid_function_data())
        pos = view.require_parameter("pos")
        end = view.require_parameter("end")
        self.assertEqual((pos.formal_parameter_index, pos.value_id), (1, 11))
        self.assertEqual((end.formal_parameter_index, end.value_id), (2, 12))
        self.assertEqual(view.require_call_edge(1, 4).role, "index_of")
        self.assertEqual(view.require_return("outer").value_id, 31)

    def test_missing_params_are_rejected_before_effect(self):
        data = _valid_function_data()
        del data["params"]
        with self.assertRaises(APrimeI64CapabilityError):
            load_selected_a_prime_capability(data)

    def test_duplicate_call_site_and_wrong_lane_are_rejected(self):
        data = _valid_function_data()
        calls = data["metadata"]["a_prime_i64_physical_receipt"]["call_edges"]
        calls[1]["block"] = calls[0]["block"]
        calls[1]["instruction_index"] = calls[0]["instruction_index"]
        with self.assertRaises(APrimeI64CapabilityError):
            load_selected_a_prime_capability(data)

        data = _valid_function_data()
        data["metadata"]["a_prime_i64_physical_receipt"]["returns"][0]["lane"] = "opaque_handle"
        with self.assertRaises(APrimeI64CapabilityError):
            load_selected_a_prime_capability(data)

        data = _valid_function_data()
        data["metadata"]["a_prime_i64_physical_receipt"]["call_edges"][1][
            "result_lane"
        ] = "opaque_handle"
        with self.assertRaises(APrimeI64CapabilityError):
            load_selected_a_prime_capability(data)

    def test_parameter_role_index_mismatch_is_rejected(self):
        data = _valid_function_data()
        data["metadata"]["a_prime_i64_physical_receipt"]["parameters"][0][
            "formal_parameter_index"
        ] = 0
        with self.assertRaises(APrimeI64CapabilityError):
            load_selected_a_prime_capability(data)

    def test_formal_parameter_value_ids_are_strict(self):
        data = _valid_function_data()
        data["params"][0] = "src"
        with self.assertRaises(APrimeI64CapabilityError):
            load_selected_a_prime_capability(data)

        data = _valid_function_data()
        data["params"][3] = data["params"][2]
        with self.assertRaises(APrimeI64CapabilityError):
            load_selected_a_prime_capability(data)

    def test_transport_shape_mismatch_is_rejected(self):
        data = _valid_function_data()
        data["metadata"]["a_prime_i64_physical_receipt"]["formal_parameter_count"] = 3
        with self.assertRaises(APrimeI64CapabilityError):
            load_selected_a_prime_capability(data)

        data = _valid_function_data()
        data["metadata"]["a_prime_i64_physical_receipt"]["returns"][1]["site"] = "cleanup"
        with self.assertRaises(APrimeI64CapabilityError):
            load_selected_a_prime_capability(data)

        data = _valid_function_data()
        data["metadata"]["a_prime_i64_physical_receipt"]["call_edges"][0][
            "target_fingerprint"
        ] = "indexOf/1"
        with self.assertRaises(APrimeI64CapabilityError):
            load_selected_a_prime_capability(data)

        data = _valid_function_data()
        data["metadata"]["a_prime_i64_physical_receipt"]["call_edges"][0]["arguments"][1][
            "ordinal"
        ] = 0
        with self.assertRaises(APrimeI64CapabilityError):
            load_selected_a_prime_capability(data)

        data = _valid_function_data()
        data["metadata"]["a_prime_i64_physical_receipt"]["parameters"][1][
            "formal_parameter_index"
        ] = 1
        with self.assertRaises(APrimeI64CapabilityError):
            load_selected_a_prime_capability(data)

    def test_schema2_requires_explicit_false_transport_flags(self):
        for field in ("fallback", "retry"):
            data = _valid_function_data()
            del data["metadata"]["a_prime_i64_physical_receipt"][field]
            with self.assertRaises(APrimeI64CapabilityError):
                load_selected_a_prime_capability(data)

    def test_schema2_rejects_unknown_nested_fields(self):
        cases = [
            ("receipt", "unexpected"),
            ("parameters", "unexpected"),
            ("call_edges", "unexpected"),
            ("arguments", "unexpected"),
            ("returns", "unexpected"),
        ]
        for section, field in cases:
            data = _valid_function_data()
            receipt = data["metadata"]["a_prime_i64_physical_receipt"]
            if section == "receipt":
                receipt[field] = True
            elif section == "parameters":
                receipt[section][0][field] = True
            elif section == "call_edges":
                receipt[section][0][field] = True
            elif section == "arguments":
                receipt["call_edges"][0]["arguments"][0][field] = True
            else:
                receipt[section][0][field] = True
            with self.assertRaises(APrimeI64CapabilityError):
                load_selected_a_prime_capability(data)


if __name__ == "__main__":
    unittest.main()
