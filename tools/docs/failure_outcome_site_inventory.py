#!/usr/bin/env python3
"""Build a deterministic evidence queue for Failure/Outcome sites.

This first-stage generator records evidence only. It deliberately does not
infer semantic ownership or target carriers; S4 will reject pending rows.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
OUTPUT = ROOT / "tools/checks/manifests/failure_outcome_site_inventory_v0.json"
SCAN_ROOTS = (ROOT / "src", ROOT / "docs/reference")
SKIP_PARTS = {"target", ".git"}

EVIDENCE_QUERIES = (
    ("VMValue::Void", "runtime_value_carrier"),
    ("ConstValue::Null", "mir_constant_carrier"),
    ("ConstValue::Void", "mir_constant_carrier"),
    ("weak_to_strong", "weak_upgrade_surface"),
    ("MissingBox", "null_like_box"),
    ("postfix_catch", "catch_profile_surface"),
    ("env.get", "provider_optional_or_missing"),
    ("env.file.read", "provider_failure_surface"),
    ("env.now_ms", "provider_failure_surface"),
    ("Option::None", "option_constructor"),
    ("Result::Err", "result_constructor"),
)

REFERENCE_DEFAULTS = {
    ("reference", "Option::None"): {
        "semantic_class": "optional_absence",
        "target_carrier": "Option::None",
        "owner": "OptionValueOwner",
        "migration_action": "relation_only",
        "backend_policy": "reference_only",
    },
    ("reference", "Result::Err"): {
        "semantic_class": "recoverable_failure",
        "target_carrier": "Result::Err",
        "owner": "ResultValueOwner",
        "migration_action": "relation_only",
        "backend_policy": "reference_only",
    },
}


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--write", action="store_true")
    mode.add_argument("--check", action="store_true")
    parser.add_argument("--strict", action="store_true")
    parser.add_argument("--output", type=Path, default=OUTPUT)
    return parser.parse_args()


def tracked_text_files() -> list[Path]:
    files: list[Path] = []
    for root in SCAN_ROOTS:
        if not root.is_dir():
            continue
        for path in root.rglob("*"):
            if not path.is_file() or any(part in SKIP_PARTS for part in path.parts):
                continue
            if path.suffix in {".rs", ".py", ".md", ".toml"}:
                files.append(path)
    return sorted(files)


def layer_for(path: Path) -> str:
    relative = path.relative_to(ROOT).as_posix()
    if relative.startswith("docs/reference/"):
        return "reference"
    if "/tests/" in relative or relative.endswith("_tests.rs"):
        return "test"
    if relative.startswith("src/backend/"):
        return "runtime_backend"
    if relative.startswith("src/mir/"):
        return "mir"
    if relative.startswith("src/parser") or relative.startswith("src/tokenizer"):
        return "parser"
    return "implementation"


def current_carrier(token: str) -> str:
    if token == "VMValue::Void":
        return "VMValue::Void"
    if token.startswith("ConstValue::"):
        return token
    if token == "MissingBox":
        return "BoxRef(MissingBox)"
    if token == "Option::None":
        return "Option::None"
    if token == "Result::Err":
        return "Result::Err"
    return token


def evidence_rows() -> list[dict[str, object]]:
    rows: list[dict[str, object]] = []
    for path in tracked_text_files():
        text = path.read_text(encoding="utf-8", errors="ignore")
        relative = path.relative_to(ROOT).as_posix()
        for line_number, line in enumerate(text.splitlines(), start=1):
            for token, evidence_kind in EVIDENCE_QUERIES:
                if token not in line:
                    continue
                site_id = f"{relative}:{line_number}:{token}"
                defaults = REFERENCE_DEFAULTS.get((layer_for(path), token), {})
                rows.append(
                    {
                        "site_id": site_id,
                        "layer": layer_for(path),
                        "surface_or_symbol": token,
                        "source_path": relative,
                        "line": line_number,
                        "current_carrier": current_carrier(token),
                        "semantic_class": defaults.get("semantic_class", ""),
                        "target_carrier": defaults.get("target_carrier", ""),
                        "owner": defaults.get("owner", ""),
                        "profile": "canonical" if layer_for(path) == "reference" else "implementation",
                        "migration_action": defaults.get("migration_action", ""),
                        "backend_policy": defaults.get("backend_policy", ""),
                        "evidence_kind": evidence_kind,
                        "evidence": line.strip(),
                        "review_status": "classified" if defaults else "pending",
                        "classification_source": "reference_relation_default"
                        if defaults
                        else "pending_review",
                    }
                )
    return sorted(rows, key=lambda row: row["site_id"])


def build_manifest() -> dict[str, object]:
    rows = evidence_rows()
    return {
        "schema_version": 0,
        "status": "seed_evidence_queue",
        "scope": ["src", "docs/reference"],
        "semantic_activation": 0,
        "classification_stage": "pending_review",
        "queries": [
            {"token": token, "evidence_kind": kind}
            for token, kind in EVIDENCE_QUERIES
        ],
        "rows": rows,
    }


def validate(manifest: dict[str, object], strict: bool) -> list[str]:
    errors: list[str] = []
    if manifest.get("schema_version") != 0:
        errors.append("schema_version must be 0")
    if manifest.get("semantic_activation") != 0:
        errors.append("semantic activation must remain 0")
    rows = manifest.get("rows", [])
    site_ids = [row.get("site_id") for row in rows]
    if len(site_ids) != len(set(site_ids)):
        errors.append("duplicate site_id")
    if strict and not rows:
        errors.append("evidence queue is empty")
    for row in rows:
        if not row.get("source_path") or not row.get("line"):
            errors.append(f"incomplete evidence row: {row.get('site_id')}")
        if row.get("review_status") not in {"pending", "classified"}:
            errors.append(f"invalid review status: {row.get('site_id')}")
    return errors


def main() -> int:
    args = parse_args()
    expected = json.dumps(build_manifest(), ensure_ascii=True, indent=2, sort_keys=True) + "\n"
    if args.write:
        args.output.write_text(expected, encoding="utf-8")
        print(f"[failure-outcome-inventory] wrote {args.output}")
        return 0
    actual = args.output.read_text(encoding="utf-8") if args.output.is_file() else ""
    if actual != expected:
        print("[failure-outcome-inventory] drift detected")
        return 1
    errors = validate(json.loads(actual), args.strict)
    if errors:
        for error in errors:
            print(f"[failure-outcome-inventory] {error}")
        return 1
    print(f"[failure-outcome-inventory] current rows={len(json.loads(actual)['rows'])}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
