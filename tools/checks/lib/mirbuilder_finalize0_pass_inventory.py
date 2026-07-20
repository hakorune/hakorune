#!/usr/bin/env python3
"""Validate the read-only FINALIZE0 pass inventory."""

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
FIXTURE = ROOT / "tools/checks/fixtures/mirbuilder_finalize0_pass_inventory_v1.json"
ALLOWED = {
    "VerifyCompletedDraft",
    "NormalizeRepresentation",
    "PublishDerivedArtifact",
    "RepairMissingLoweringFact",
    "LegacySemanticInference",
}


def fail(message: str) -> None:
    raise SystemExit(f"[mirbuilder-finalize0-pass-inventory] {message}")


def main() -> int:
    data = json.loads(FIXTURE.read_text())
    if data.get("schema") != "mirbuilder-finalize0-pass-inventory-v1":
        fail("schema mismatch")
    rows = data.get("rows")
    if not isinstance(rows, list) or len(rows) != 20:
        fail("expected exactly 20 inventory rows")
    ids = [row.get("id") for row in rows]
    if len(set(ids)) != len(ids) or any(not value for value in ids):
        fail("duplicate or empty row id")

    anchors = []
    for row in rows:
        if row.get("class") not in ALLOWED:
            fail(f"unknown class for {row.get('id')}")
        if row.get("status") not in {"retain", "parked"}:
            fail(f"unknown status for {row.get('id')}")
        source = ROOT / "src/mir/builder" / row["site"]
        if not source.exists():
            fail(f"missing source anchor file: {source}")
        text = source.read_text()
        if row["anchor"] not in text:
            fail(f"anchor missing for {row['id']}: {row['anchor']}")
        anchors.append((row["site"], row["anchor"]))

    if not any(row["class"] == "RepairMissingLoweringFact" for row in rows):
        fail("repair lane disappeared from census")
    if not any(row["class"] == "LegacySemanticInference" for row in rows):
        fail("semantic-inference lane disappeared from census")
    if not any(row["class"] == "VerifyCompletedDraft" for row in rows):
        fail("verification lane disappeared from census")
    if not any(row["class"] == "PublishDerivedArtifact" for row in rows):
        fail("derived-publication lane disappeared from census")
    print(
        "[mirbuilder-finalize0-pass-inventory] ok "
        f"rows={len(rows)} anchors={len(anchors)} "
        "behavior_delta=0 production_consumers=0"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
