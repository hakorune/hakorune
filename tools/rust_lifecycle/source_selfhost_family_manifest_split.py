#!/usr/bin/env python3
"""Build and verify the active/history Source Selfhost manifest split."""

from __future__ import annotations

import argparse
import json
from pathlib import Path

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
LEGACY = FIXTURES / "source-selfhost-family-guard-manifest-v0.json"
ACTIVE = FIXTURES / "source-selfhost-family-guard-active-v1.json"
HISTORY = FIXTURES / "source-selfhost-family-guard-history-v1.jsonl"
PROVENANCE = FIXTURES / "source-selfhost-family-manifest-split-v1.json"
CARD = ROOT / "docs/development/current/main/phases/phase-296x/3444-SOURCE-SELFHOST-FAMILY-MANIFEST-ACTIVE-HISTORY-SPLIT-001.md"
ACTIVE_ROLES = {"current_semantic", "current_maintenance"}
HISTORY_ROLES = {"current_queue", "historical_traceability"}


def read_json(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def split_rows(rows: list[dict]) -> tuple[list[dict], list[dict]]:
    active: list[dict] = []
    history: list[dict] = []
    for row in rows:
        role = row.get("role")
        if role in ACTIVE_ROLES:
            active.append(row)
        elif role in HISTORY_ROLES:
            history.append(row)
        else:
            raise SystemExit(f"unsupported manifest role: {role!r}")
    return active, history


def active_object(legacy: dict, rows: list[dict]) -> dict:
    return {
        "schema_version": 1,
        "kind": "SourceSelfhostFamilyGuardActiveManifestV1",
        "family_scope": legacy["family_scope"],
        "current_blocker_token": legacy["current_blocker_token"],
        "guard_policy": legacy["guard_policy"],
        "active_roles": sorted(ACTIVE_ROLES),
        "rows": rows,
        "forbidden_claims": legacy["forbidden_claims"],
    }


def history_text(rows: list[dict]) -> str:
    return "".join(json.dumps(row, ensure_ascii=True, separators=(",", ":")) + "\n" for row in rows)


def provenance(legacy: dict, active_rows: list[dict], history_rows: list[dict]) -> dict:
    current_source_hash = sha256_file(LEGACY)
    frozen_source_hash = current_source_hash
    if PROVENANCE.exists():
        previous = read_json(PROVENANCE)
        frozen_source_hash = previous.get("frozen_source_manifest_sha256", current_source_hash)
        if frozen_source_hash != current_source_hash:
            raise SystemExit("frozen v0 manifest changed after active/history split")
    return {
        "schema_version": 1,
        "kind": "SourceSelfhostFamilyManifestSplitV1",
        "source_manifest": str(LEGACY.relative_to(ROOT)),
        "source_manifest_sha256": current_source_hash,
        "frozen_source_manifest_sha256": frozen_source_hash,
        "active_manifest": str(ACTIVE.relative_to(ROOT)),
        "active_manifest_sha256": sha256_file(ACTIVE),
        "history_ledger": str(HISTORY.relative_to(ROOT)),
        "history_ledger_sha256": sha256_file(HISTORY),
        "migration_card": str(CARD.relative_to(ROOT)),
        "counts": {
            "source_rows": len(legacy["rows"]),
            "active_rows": len(active_rows),
            "history_rows": len(history_rows),
        },
        "active_roles": sorted(ACTIVE_ROLES),
        "history_roles": sorted(HISTORY_ROLES),
        "claims": {
            "exact_partition": 1,
            "active_manifest_is_current_index": 1,
            "history_is_append_only_ledger": 1,
            "legacy_manifest_frozen_compatibility_snapshot": 1,
            "source_selfhost_claim": 0,
        },
    }


def build() -> tuple[str, str]:
    legacy = read_json(LEGACY)
    active_rows, history_rows = split_rows(legacy.get("rows") or [])
    active = stable_json(active_object(legacy, active_rows))
    history = history_text(history_rows)
    return active, history


def validate_split() -> tuple[dict, list[dict], list[dict]]:
    legacy = read_json(LEGACY)
    active = read_json(ACTIVE)
    history_rows = [json.loads(line) for line in HISTORY.read_text(encoding="utf-8").splitlines() if line.strip()]
    legacy_rows = legacy.get("rows") or []
    baseline_active, baseline_history = split_rows(legacy_rows)
    active_by_token = {row.get("token"): row for row in active.get("rows") or []}
    baseline_active_by_token = {row.get("token"): row for row in baseline_active}
    history_by_token = {row.get("token"): row for row in history_rows}
    if set(history_by_token) != {row.get("token") for row in baseline_history}:
        raise SystemExit("history ledger no longer matches frozen v0 history rows")
    for token, row in baseline_active_by_token.items():
        if active_by_token.get(token) != row:
            raise SystemExit(f"baseline active row drift: {token}")
    for token, row in active_by_token.items():
        if token not in baseline_active_by_token and row.get("role") not in ACTIVE_ROLES:
            raise SystemExit(f"post-split active row has invalid role: {token}")
    if set(active_by_token) & set(history_by_token):
        raise SystemExit("active/history token overlap")
    if active.get("kind") != "SourceSelfhostFamilyGuardActiveManifestV1":
        raise SystemExit("active manifest kind mismatch")
    if active.get("family_scope") != legacy.get("family_scope"):
        raise SystemExit("active manifest family scope mismatch")
    if set(active.get("active_roles") or []) != ACTIVE_ROLES:
        raise SystemExit("active manifest role set mismatch")
    return legacy, active.get("rows") or [], history_rows


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--bootstrap", action="store_true", help="Create the initial split from v0.")
    parser.add_argument("--refresh-provenance", action="store_true", help="Refresh hashes after active edits.")
    parser.add_argument("--check", action="store_true", help="Verify the split outputs.")
    args = parser.parse_args()
    if sum(bool(value) for value in [args.bootstrap, args.refresh_provenance, args.check]) > 1:
        raise SystemExit("choose only one mode")
    if args.bootstrap:
        active_text, history_text_value = build()
        write_if_changed(ACTIVE, active_text)
        write_if_changed(HISTORY, history_text_value)
        legacy = read_json(LEGACY)
        active_rows, history_rows = split_rows(legacy["rows"])
        write_if_changed(PROVENANCE, stable_json(provenance(legacy, active_rows, history_rows)))
        print(f"updated={ACTIVE.relative_to(ROOT)}")
        print(f"updated={HISTORY.relative_to(ROOT)}")
        print(f"updated={PROVENANCE.relative_to(ROOT)}")
        return 0
    legacy, active_rows, history_rows = validate_split()
    if args.refresh_provenance:
        write_if_changed(PROVENANCE, stable_json(provenance(legacy, active_rows, history_rows)))
        print(f"updated={PROVENANCE.relative_to(ROOT)}")
        return 0
    expected_provenance = stable_json(provenance(legacy, active_rows, history_rows))
    if not PROVENANCE.exists() or PROVENANCE.read_text(encoding="utf-8") != expected_provenance:
        raise SystemExit(f"{PROVENANCE.relative_to(ROOT)} is stale")
    print("source-selfhost family manifest split unchanged")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
