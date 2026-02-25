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

from stability_app_log import (  # noqa: E402
    build_apps_baseline_fields,
    collect_app_aggregate,
    parse_app_log,
)


class StabilityAppLogTests(unittest.TestCase):
    def test_parse_app_log_prefers_json(self) -> None:
        with tempfile.TemporaryDirectory() as td:
            path = pathlib.Path(td) / "apps_vm.round1.log"
            payload = {"total_ms": 321, "cases": {"a": 100, "b": 221}}
            path.write_text(json.dumps(payload) + "\nnoise\n")
            total, cases = parse_app_log(path)
            self.assertEqual(total, 321)
            self.assertEqual(cases, {"a": 100, "b": 221})

    def test_parse_app_log_fallback_text(self) -> None:
        with tempfile.TemporaryDirectory() as td:
            path = pathlib.Path(td) / "apps_vm.round1.log"
            path.write_text(
                "\n".join(
                    [
                        "[bench-app] name=foo backend=vm ms=33",
                        "[bench-app] name=bar backend=vm ms=44",
                    ]
                )
                + "\n"
            )
            total, cases = parse_app_log(path)
            self.assertEqual(total, 77)
            self.assertEqual(cases, {"foo": 33, "bar": 44})

    def test_collect_and_build_apps_baseline(self) -> None:
        with tempfile.TemporaryDirectory() as td:
            out_dir = pathlib.Path(td)
            (out_dir / "apps_vm.round1.log").write_text(
                json.dumps({"total_ms": 300, "cases": {"x": 100, "y": 200}}) + "\n"
            )
            (out_dir / "apps_vm.round2.log").write_text(
                json.dumps({"total_ms": 320, "cases": {"x": 120, "y": 200}}) + "\n"
            )
            agg = collect_app_aggregate(out_dir)
            self.assertEqual(sorted(agg["app_totals"]), [300, 320])
            self.assertEqual(sorted(agg["app_by_name"]["x"]), [100, 120])

            fields = build_apps_baseline_fields(agg)
            self.assertEqual(fields["apps_vm_total_ms"], 310)
            self.assertEqual(fields["apps_vm_per_app_ms"]["x"], 110)
            self.assertEqual(fields["apps_vm_per_app_ms"]["y"], 200)


if __name__ == "__main__":
    unittest.main()
