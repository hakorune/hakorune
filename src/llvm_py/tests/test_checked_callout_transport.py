import copy
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from builders.checked_callout_transport import (
    CheckedCallOutEndView,
    CheckedCallOutFaultView,
    CheckedCallOutNormalResultView,
    CheckedCallOutTerminatorView,
    CheckedCallOutTransportError,
    parse_checked_callout_transport,
)


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
    ]


class TestCheckedCallOutTransport(unittest.TestCase):
    def test_all_operations_round_trip_without_aliases(self):
        expected = (
            CheckedCallOutTerminatorView(0, 4, (6, 9), 11, 12, 0x10),
            CheckedCallOutNormalResultView(0, 10),
            CheckedCallOutEndView(0, 0),
            CheckedCallOutFaultView(0),
        )
        for raw, expected_view in zip(_valid_operations(), expected):
            view = parse_checked_callout_transport(raw)
            self.assertEqual(view, expected_view)
            self.assertEqual(parse_checked_callout_transport(view.to_json()), expected_view)

    def test_missing_or_unknown_fields_reject(self):
        for raw in _valid_operations():
            malformed = copy.deepcopy(raw)
            malformed.pop("site_id")
            with self.assertRaises(CheckedCallOutTransportError):
                parse_checked_callout_transport(malformed)

        malformed = _valid_operations()[0]
        malformed["unexpected"] = 1
        with self.assertRaises(CheckedCallOutTransportError):
            parse_checked_callout_transport(malformed)

        with self.assertRaises(CheckedCallOutTransportError):
            parse_checked_callout_transport({"op": "mir_call", "site_id": 0})

    def test_numeric_boundaries_and_effect_bits_reject(self):
        valid = _valid_operations()[0]
        for field in ("site_id", "receiver", "normal", "fault"):
            malformed = copy.deepcopy(valid)
            malformed[field] = 1 << 32
            with self.assertRaises(CheckedCallOutTransportError):
                parse_checked_callout_transport(malformed)

        malformed = copy.deepcopy(valid)
        malformed["effects"] = 1 << 15
        with self.assertRaises(CheckedCallOutTransportError):
            parse_checked_callout_transport(malformed)

        malformed = copy.deepcopy(valid)
        malformed["normal"] = malformed["fault"]
        with self.assertRaises(CheckedCallOutTransportError):
            parse_checked_callout_transport(malformed)

        malformed = copy.deepcopy(valid)
        malformed["args"] = [True]
        with self.assertRaises(CheckedCallOutTransportError):
            parse_checked_callout_transport(malformed)


if __name__ == "__main__":
    unittest.main()
