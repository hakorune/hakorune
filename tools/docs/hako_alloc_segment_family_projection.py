#!/usr/bin/env python3
"""Generate a deterministic content-evidence queue for the segment family."""

from __future__ import annotations

import argparse
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
INPUT = ROOT / "tools/checks/manifests/repository_artifact_lifecycle_v0.json"
OUTPUT = ROOT / "tools/checks/manifests/hako_alloc_segment_family_projection_v0.json"
FAMILY_PREFIX = "hako-alloc-segment-"

SUBFAMILY_MARKERS = {
    "segment-lifecycle-and-membership": (
        "segment lifecycle",
        "lifecycle scalar",
        "page membership",
        "segment state",
        "state vocabulary",
        "transition",
    ),
    "segment-allocation-and-local-reuse": (
        "segment allocation",
        "allocation readiness",
        "local free",
        "local-free",
        "modeled consume",
        "reuse",
    ),
    "segment-arena-backing-and-residence": (
        "arena backing",
        "arena-backing",
        "pointer residence",
        "address residence",
        "bitmap",
        "backing",
    ),
    "segment-map-and-release": (
        "segment map",
        "segment-map",
        "lookup",
        "mutation",
        "release",
        "recycle",
    ),
}

ROLE_HINT_MARKERS = {
    "status-ledger-candidate": (
        "ledger",
        "inventory",
        "readiness",
        "matrix",
        "current blocker",
    ),
    "sidecar-candidate": ("diagnostics", "closeout"),
    "supporting-candidate": (
        "proof-only",
        "model-only",
        "pilot",
        "bridge",
        "composition",
        "non-responsibility",
    ),
}


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--write", action="store_true")
    mode.add_argument("--check", action="store_true")
    parser.add_argument("--output", type=Path, default=OUTPUT)
    return parser.parse_args()


def read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8", errors="ignore")[:16000].lower()


def matched_markers(
    text: str, marker_map: dict[str, tuple[str, ...]]
) -> dict[str, list[str]]:
    return {
        family: [marker for marker in markers if marker in text]
        for family, markers in marker_map.items()
        if any(marker in text for marker in markers)
    }


def primary_subfamily(path: str, evidence: dict[str, list[str]]) -> str:
    name = path.lower()
    priority = (
        ("segment-arena-backing-and-residence", ("arena", "residence", "bitmap", "backing")),
        ("segment-map-and-release", ("map", "lookup", "mutation", "release", "recycle")),
        ("segment-lifecycle-and-membership", ("lifecycle", "membership", "state")),
        ("segment-allocation-and-local-reuse", ("allocation", "local-free", "reuse")),
    )
    for family, tokens in priority:
        if any(token in name for token in tokens) and family in evidence:
            return family
    return next(iter(sorted(evidence)), "needs-review")


def project() -> dict[str, object]:
    manifest = json.loads(INPUT.read_text(encoding="utf-8"))
    files = sorted(
        path
        for path in manifest["design_registry"]["unregistered"]
        if path.startswith(FAMILY_PREFIX)
    )
    rows = []
    for path in files:
        text = read_text(ROOT / "docs/development/current/main/design" / path)
        subfamily_evidence = matched_markers(text, SUBFAMILY_MARKERS)
        role_hints = matched_markers(text + path.lower(), ROLE_HINT_MARKERS)
        rows.append(
            {
                "path": path,
                "subfamily_candidate": primary_subfamily(path, subfamily_evidence),
                "subfamily_evidence": subfamily_evidence,
                "role_hints": role_hints,
                "owner": "",
                "precedence_parent": "",
                "sidecar_owner": "",
                "review_status": "pending",
                "classification_basis": "body evidence queue only; reviewer decision required",
            }
        )
    return {
        "schema_version": 0,
        "family": "hako-alloc-segment",
        "role_assignment": "none",
        "physical_move": 0,
        "superseded_assignment": 0,
        "input_unregistered_count": len(files),
        "rows": rows,
    }


def main() -> int:
    args = parse_args()
    expected = json.dumps(project(), ensure_ascii=True, indent=2, sort_keys=True) + "\n"
    if args.write:
        args.output.write_text(expected, encoding="utf-8")
        print(f"[hako-alloc-segment-projection] wrote {args.output}")
        return 0
    actual = args.output.read_text(encoding="utf-8") if args.output.is_file() else ""
    if actual != expected:
        print("[hako-alloc-segment-projection] drift detected")
        return 1
    print("[hako-alloc-segment-projection] current")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
