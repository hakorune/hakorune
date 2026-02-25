#!/usr/bin/env python3
from __future__ import annotations

import json
import pathlib
import sys
import tempfile
import unittest

THIS_DIR = pathlib.Path(__file__).resolve().parent
LIB_DIR = THIS_DIR.parent
if str(LIB_DIR) not in sys.path:
    sys.path.insert(0, str(LIB_DIR))

from stability_entry_mode import (  # noqa: E402
    build_entry_mode_baseline_fields,
    collect_entry_mode_aggregate,
    parse_entry_mode_log,
)


def _summary_payload(
    source_total: int,
    prebuilt_total: int,
    delta_abs: int,
    threshold: int,
    source_cases: dict[str, int],
    prebuilt_cases: dict[str, int],
    case_delta: dict[str, int],
) -> dict:
    return {
        "backend": "vm",
        "source_total_ms": source_total,
        "mir_shape_prebuilt_total_ms": prebuilt_total,
        "delta_ms_abs": delta_abs,
        "significance_ms_threshold": threshold,
        "source_cases_ms": source_cases,
        "mir_shape_prebuilt_cases_ms": prebuilt_cases,
        "case_delta_ms": case_delta,
        "hotspot_case_delta": {"case": "a", "delta_ms_abs": abs(case_delta.get("a", 0))},
    }


class StabilityEntryModeTests(unittest.TestCase):
    def test_parse_entry_mode_log_uses_summary_line(self) -> None:
        with tempfile.TemporaryDirectory() as td:
            path = pathlib.Path(td) / "apps_entry_mode.round1.json"
            sample = {"kind": "sample", "source_total_ms": 1, "mir_shape_prebuilt_total_ms": 1}
            summary = _summary_payload(
                source_total=120,
                prebuilt_total=45,
                delta_abs=75,
                threshold=10,
                source_cases={"a": 80, "b": 40},
                prebuilt_cases={"a": 25, "b": 20},
                case_delta={"a": -55, "b": -20},
            )
            summary["kind"] = "summary"
            path.write_text(json.dumps(sample) + "\n" + json.dumps(summary) + "\n")

            parsed = parse_entry_mode_log(path)
            self.assertIsNotNone(parsed)
            self.assertEqual(parsed["source_total_ms"], 120)
            self.assertEqual(parsed["mir_shape_prebuilt_total_ms"], 45)

    def test_collect_and_build_baseline_fields(self) -> None:
        with tempfile.TemporaryDirectory() as td:
            out_dir = pathlib.Path(td)
            payload1 = _summary_payload(
                source_total=100,
                prebuilt_total=40,
                delta_abs=60,
                threshold=10,
                source_cases={"a": 70, "b": 30},
                prebuilt_cases={"a": 20, "b": 20},
                case_delta={"a": -50, "b": -10},
            )
            payload2 = _summary_payload(
                source_total=120,
                prebuilt_total=50,
                delta_abs=70,
                threshold=10,
                source_cases={"a": 80, "b": 40},
                prebuilt_cases={"a": 25, "b": 25},
                case_delta={"a": -55, "b": -15},
            )
            (out_dir / "apps_entry_mode.round1.json").write_text(json.dumps(payload1) + "\n")
            (out_dir / "apps_entry_mode.round2.json").write_text(json.dumps(payload2) + "\n")

            agg = collect_entry_mode_aggregate(out_dir)
            self.assertEqual(sorted(agg["source_totals"]), [100, 120])
            self.assertEqual(sorted(agg["prebuilt_totals"]), [40, 50])
            self.assertEqual(sorted(agg["delta_abs_vals"]), [60, 70])
            self.assertEqual(sorted(agg["source_by_name"]["a"]), [70, 80])
            self.assertEqual(sorted(agg["delta_by_name"]["b"]), [-15, -10])

            fields = build_entry_mode_baseline_fields(agg)
            self.assertEqual(fields["apps_entry_mode_source_total_ms"], 110)
            self.assertEqual(fields["apps_entry_mode_prebuilt_total_ms"], 45)
            self.assertEqual(fields["apps_entry_mode_delta_abs_ms"], 65)
            self.assertEqual(fields["apps_entry_mode_source_per_app_ms"]["a"], 75)
            self.assertEqual(fields["apps_entry_mode_prebuilt_per_app_ms"]["a"], 22)
            self.assertEqual(fields["apps_entry_mode_case_delta_ms"]["a"], -52)
            self.assertEqual(fields["apps_entry_mode_hotspot_case"], "a")

    def test_parse_entry_mode_log_rejects_invalid_payload(self) -> None:
        with tempfile.TemporaryDirectory() as td:
            path = pathlib.Path(td) / "apps_entry_mode.round1.json"
            path.write_text(json.dumps({"source_total_ms": 0}) + "\n")
            self.assertIsNone(parse_entry_mode_log(path))


if __name__ == "__main__":
    unittest.main()
