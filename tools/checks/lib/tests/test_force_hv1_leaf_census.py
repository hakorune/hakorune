#!/usr/bin/env python3
"""Focused tests for the body-derived force-hv1 observation contract."""

from __future__ import annotations

import json
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from force_hv1_leaf_census import derive_inventory, derive_leaf


ROOT = Path(__file__).resolve().parents[4]
MANIFEST = ROOT / "docs/development/current/main/investigations/force-hv1-caller-disposition-manifest-v1.json"


class ForceHv1LeafCensusTest(unittest.TestCase):
    def test_checked_in_inventory_matches_reviewed_matrix(self) -> None:
        manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
        observations = derive_inventory(ROOT, manifest["leaf_paths"])
        self.assertEqual(len(observations), 86)
        self.assertEqual(sum(len(item["sites"]) for item in observations), 90)
        self.assertEqual(
            {
                key: sum(item["derived"]["route_class"] == key for item in observations)
                for key in manifest["observed_counts"]["route_class"]
            },
            manifest["observed_counts"]["route_class"],
        )

    def test_retired_inventory_is_disjoint_and_absent(self) -> None:
        manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
        active_paths = set(manifest["leaf_paths"])
        retired = manifest["retired_inventory"]
        retired_paths = {record["path"] for record in retired["records"]}
        self.assertEqual(retired["status"], "r0a_landed")
        self.assertEqual(len(retired_paths), 30)
        self.assertTrue(active_paths.isdisjoint(retired_paths))
        self.assertTrue(all(not (ROOT / path).exists() for path in retired_paths))
        direct_paths = {
            item["path"]
            for item in derive_inventory(ROOT, manifest["leaf_paths"])
            if item["derived"]["route_class"] == "DirectForceSealed"
        }
        self.assertEqual(
            direct_paths,
            {
                "tools/smokes/v2/profiles/integration/core/phase2050/flow_phi2_select_by_pred_rc99_primary_canary_vm.sh",
                "tools/smokes/v2/profiles/integration/core/phase2051/selfhost_v1_primary_rc42_canary_vm.sh",
                "tools/smokes/v2/profiles/integration/core/phase2051/selfhost_v1_provider_primary_rc42_canary_vm.sh",
            },
        )

    def test_comments_are_not_entry_sites(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            path = root / "leaf.sh"
            path.write_text(
                "#!/bin/bash\n"
                "# verify_mir_rc should not count\n"
                "verify_mir_rc \"$1\"\n",
                encoding="utf-8",
            )
            observation = derive_leaf(root, "leaf.sh")
            self.assertEqual(observation["derived"]["lexical_entry_sites"], 1)

    def test_mixed_entry_forms_fail_closed(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            path = root / "leaf.sh"
            path.write_text(
                "verify_v1_inline_file \"$1\"\n"
                "verify_mir_rc \"$1\"\n",
                encoding="utf-8",
            )
            with self.assertRaisesRegex(ValueError, "mixes direct and helper"):
                derive_leaf(root, "leaf.sh")

    def test_unknown_wrapper_fails_closed(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            path = root / "leaf.sh"
            path.write_text(
                "run_verify_mir_canary_and_expect_rc unknown_runner \"$1\"\n",
                encoding="utf-8",
            )
            with self.assertRaisesRegex(ValueError, "outside the finite allowlist"):
                derive_leaf(root, "leaf.sh")


if __name__ == "__main__":
    unittest.main()
