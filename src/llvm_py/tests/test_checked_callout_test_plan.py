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

from builders.checked_callout_test_plan import (
    CheckedCallOutTestPlanError,
    validate_checked_callout_test_fixture,
)
from test_dynamic_v2_aot_admission import _valid_admission_data


def _valid_operations():
    return [
        {
            "op": "checked_callout",
            "site_id": 0,
            "receiver": 4,
            "args": [6, 9],
            "normal": 11,
            "fault": 12,
            "effects": 0x10,
        },
        {"op": "checked_callout_normal_result", "site_id": 0, "dst": 10},
        {"op": "checked_callout_end", "site_id": 0, "lease_slot": 0},
        {"op": "checked_callout_fault", "site_id": 0},
        {
            "op": "checked_callout",
            "site_id": 1,
            "receiver": 10,
            "args": [20],
            "normal": 13,
            "fault": 14,
            "effects": 0x10,
        },
        {"op": "checked_callout_normal_result", "site_id": 1, "dst": 11},
        {"op": "checked_callout_fault", "site_id": 1},
    ]


class TestCheckedCallOutTestPlan(unittest.TestCase):
    def test_complete_two_site_fixture_is_observable(self):
        report = validate_checked_callout_test_fixture(_valid_operations(), _valid_admission_data())
        self.assertEqual(report.site_ids, (0, 1))
        self.assertEqual(report.normal_landing_pairs, ((11, 12), (13, 14)))
        self.assertEqual(report.normal_projection_sites, (0, 1))
        self.assertEqual(report.end_sites, (0,))
        self.assertEqual(report.plan_stamp, (1, 9))

    def test_missing_projection_or_fault_is_rejected(self):
        for operation in (
            {"op": "checked_callout_normal_result", "site_id": 1, "dst": 11},
            {"op": "checked_callout_fault", "site_id": 1},
        ):
            operations = _valid_operations()
            operations.remove(operation)
            with self.assertRaises(CheckedCallOutTestPlanError):
                validate_checked_callout_test_fixture(operations, _valid_admission_data())

    def test_duplicate_or_shared_lifecycle_is_rejected(self):
        operations = _valid_operations()
        operations.append(copy.deepcopy(operations[0]))
        with self.assertRaises(CheckedCallOutTestPlanError):
            validate_checked_callout_test_fixture(operations, _valid_admission_data())

        operations = _valid_operations()
        operations.append({"op": "checked_callout_end", "site_id": 1, "lease_slot": 0})
        with self.assertRaises(CheckedCallOutTestPlanError):
            validate_checked_callout_test_fixture(operations, _valid_admission_data())


if __name__ == "__main__":
    unittest.main()
