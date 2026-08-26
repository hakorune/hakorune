#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mir-call-global-target-b0-machine-census"
CARD="$ROOT_DIR/docs/development/current/main/investigations/mir-call-d1b-root-lineage-exact-target-loan-d0-2026-08-26.toml"
REGISTRY="$ROOT_DIR/tools/checks/guard_rows.toml"
INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"
MANIFEST="$ROOT_DIR/tools/checks/manifests/mir_call_global_target_b0_machine_census.toml"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

fail() {
  echo "[$TAG] result_class=current-change failure status=fail: $*" >&2
  exit 1
}

[[ $# -eq 0 ]] || fail "usage: $0"
guard_require_command "$TAG" python3
guard_require_files "$TAG" "$CARD" "$REGISTRY" "$INDEX" "$MANIFEST"

python3 - "$ROOT_DIR" "$CARD" "$REGISTRY" "$INDEX" "$MANIFEST" <<'PY'
from __future__ import annotations

import hashlib
import re
import sys
import tomllib
from pathlib import Path

root, card_path, registry_path, index_path, manifest_path = map(Path, sys.argv[1:])


def fail(message: str) -> None:
    raise SystemExit(message)


def load(path: Path) -> dict[str, object]:
    try:
        with path.open("rb") as stream:
            value = tomllib.load(stream)
    except tomllib.TOMLDecodeError as exc:
        fail(f"TOML parse failed: {path.relative_to(root)}: {exc}")
    if not isinstance(value, dict):
        fail(f"TOML root is not a table: {path.relative_to(root)}")
    return value


card = load(card_path)
registry = load(registry_path)
manifest = load(manifest_path)
index = index_path.read_text(encoding="utf-8")

task_id = "MIR-CALL-GLOBAL-TARGET-B0-MACHINE-CENSUS-G0"
guard_id = "mir-call-global-target-b0-machine-census"
guard_script = "tools/checks/mir_call_global_target_b0_machine_census_guard.sh"
manifest_rel = "tools/checks/manifests/mir_call_global_target_b0_machine_census.toml"
card_rel = str(card_path.relative_to(root))

if manifest.get("schema_version") != 1:
    fail("machine census manifest schema_version must be 1")
if manifest.get("task_id") != task_id or manifest.get("guard_id") != guard_id:
    fail("machine census manifest task/guard identity drifted")
if manifest.get("card_path") != card_rel:
    fail("machine census manifest card_path drifted")
if "owning card" not in str(manifest.get("source_of_truth", "")):
    fail("machine census must name the owning card as its source of truth")
if "never a target issuer" not in str(manifest.get("source_of_truth", "")):
    fail("machine census must remain a projection, not a target issuer")
if "not an exhaustive caller proof" not in str(manifest.get("wpre_projection_scope", "")):
    fail("Wpre projection must be labeled as a known-current list, not completeness proof")
if manifest.get("allowed_dispositions") != ["adapt", "delete", "isolate"]:
    fail("machine census disposition vocabulary drifted")

card_row = card.get("b0_machine_census_guard_row")
if not isinstance(card_row, dict):
    fail("active card b0_machine_census_guard_row is missing")
if card_row.get("task_id") != task_id or card_row.get("status") not in {
    "selected_fast_guard_only",
    "landed_guard_only",
}:
    fail("active card B0 machine census row is not selected/landed guard-only")
if card.get("implementation_permission") is not False:
    fail("semantic implementation permission opened during B0 guard-only row")
expected_allowed_files = {
    manifest_rel,
    guard_script,
    "tools/checks/guard_rows.toml",
    "docs/tools/check-scripts-index.md",
    card_rel,
    "docs/development/current/main/CURRENT_STATE.toml",
    "docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md",
}
allowed_files = set(card_row.get("allowed_files") or [])
if allowed_files != expected_allowed_files:
    fail("B0 machine census allowed file boundary drifted")
if "machine-readable B0 census manifest" not in str(card_row.get("change", "")):
    fail("active card B0 row no longer names the machine-readable census")
if "unknown/stale/duplicate rows fail" not in str(card_row.get("contract", "")):
    fail("active card B0 row no longer requires fail-closed row validation")

registry_rows = registry.get("rows")
if not isinstance(registry_rows, list):
    fail("guard_rows.toml rows table is missing")
matches = [row for row in registry_rows if isinstance(row, dict) and row.get("id") == guard_id]
if len(matches) != 1:
    fail(f"expected one registry row for {guard_id}, found {len(matches)}")
registry_row = matches[0]
if registry_row.get("profiles") != ["pilot", "quick-static"]:
    fail("B0 machine census guard profiles drifted")
if registry_row.get("cmd") != ["bash", guard_script]:
    fail("B0 machine census guard command drifted")
if guard_script not in index or manifest_rel not in index:
    fail("check-script index does not list the B0 machine census guard and manifest")

proof_plan = card.get("proof_plan")
if not isinstance(proof_plan, dict):
    fail("active card proof_plan is missing")
if guard_id not in str(proof_plan.get("future_guard_rows", "")):
    fail("active card proof_plan does not name the B0 machine census guard")

global_b0 = card.get("global_target_b0")
if not isinstance(global_b0, dict):
    fail("active card global_target_b0 is missing")
wpre = card.get("wpre_contract")
if not isinstance(wpre, dict):
    fail("active card wpre_contract is missing")


def as_rows(table: object, label: str) -> list[str]:
    if not isinstance(table, list) or not table or not all(isinstance(row, str) and row for row in table):
        fail(f"active card {label} must be a non-empty string array")
    return table


family_rows = as_rows(global_b0.get("family_matrix"), "family_matrix")
consumer_rows = as_rows(global_b0.get("compiled_consumer_owner_inventory"), "compiled_consumer_owner_inventory")
wpre_rows = as_rows(wpre.get("entrance_owner_inventory"), "entrance_owner_inventory")


def projection_rows(table_name: str, expected: int, card_rows: list[str], *, kind: str) -> list[dict[str, object]]:
    entries = manifest.get(table_name)
    if not isinstance(entries, list) or len(entries) != expected or len(entries) != len(card_rows):
        count = len(entries) if isinstance(entries, list) else "missing"
        fail(f"{table_name} count drifted: manifest/card/expected={count}/{len(card_rows)}/{expected}")
    seen_ids: set[str] = set()
    seen_indices: set[int] = set()
    for entry in entries:
        if not isinstance(entry, dict):
            fail(f"{table_name} contains a non-table row")
        entry_id = entry.get("id")
        index = entry.get("card_index")
        key = entry.get("key")
        digest = entry.get("card_sha256")
        disposition = entry.get("disposition")
        terminal = entry.get("terminal")
        if not isinstance(entry_id, str) or not entry_id or entry_id in seen_ids:
            fail(f"{table_name} has a missing/duplicate id: {entry_id!r}")
        if not isinstance(index, int) or index < 0 or index >= len(card_rows) or index in seen_indices:
            fail(f"{table_name} has a missing/duplicate card_index: {index!r}")
        if not isinstance(key, str) or not key:
            fail(f"{table_name} {entry_id} key is empty")
        if not isinstance(digest, str) or not re.fullmatch(r"[0-9a-f]{64}", digest):
            fail(f"{table_name} {entry_id} card_sha256 is invalid")
        if disposition not in manifest["allowed_dispositions"]:
            fail(f"{table_name} {entry_id} disposition is not finite: {disposition!r}")
        if kind == "family":
            if terminal not in manifest["allowed_family_terminals"]:
                fail(f"family {entry_id} has unknown terminal: {terminal!r}")
        elif terminal not in {"compiled_consumer", "wpre_boundary"}:
            fail(f"{table_name} {entry_id} has unknown terminal: {terminal!r}")
        card_row = card_rows[index]
        if hashlib.sha256(card_row.encode()).hexdigest() != digest:
            fail(f"{table_name} {entry_id} card row fingerprint drifted")
        parts = card_row.split(" | ")
        expected_parts = 5 if kind == "family" else 3 if kind == "consumer" else 4
        if len(parts) != expected_parts:
            fail(f"{table_name} {entry_id} card row shape changed: expected {expected_parts}, got {len(parts)}")
        if parts[0] != key:
            fail(f"{table_name} {entry_id} key does not match card row")
        if kind == "family":
            expected_target = entry.get("expected_target")
            if not isinstance(expected_target, str) or not expected_target or expected_target not in parts[2]:
                fail(f"family {entry_id} target projection drifted")
        else:
            expected_path_count = entry.get("expected_path_count")
            if not isinstance(expected_path_count, int) or expected_path_count <= 0:
                fail(f"{table_name} {entry_id} path count is invalid")
            paths = [item.strip() for item in parts[1].split(";") if item.strip()]
            if len(paths) != expected_path_count:
                fail(f"{table_name} {entry_id} path count drifted: {len(paths)} != {expected_path_count}")
            for item in paths:
                if item == "co-located cfg-test modules":
                    continue
                path = Path(item)
                if path.is_absolute() or ".." in path.parts or "/" not in item:
                    fail(f"{table_name} {entry_id} has an invalid path token: {item!r}")
                if not (root / path).exists():
                    fail(f"{table_name} {entry_id} names missing path: {item}")
        seen_ids.add(entry_id)
        seen_indices.add(index)
    if seen_indices != set(range(len(card_rows))):
        fail(f"{table_name} does not cover every active-card row exactly once")
    return entries


family_entries = projection_rows(
    "family_rows",
    int(manifest.get("family_row_count", -1)),
    family_rows,
    kind="family",
)
consumer_entries = projection_rows(
    "consumer_rows",
    int(manifest.get("consumer_row_count", -1)),
    consumer_rows,
    kind="consumer",
)
wpre_entries = projection_rows(
    "wpre_rows",
    int(manifest.get("wpre_row_count", -1)),
    wpre_rows,
    kind="wpre",
)

wire_cohort = str(card.get("final_architecture", {}).get("wire_cohort", ""))
match = re.search(r"bounded to (.*?)\. Any other op is", wire_cohort)
if not match:
    fail("card wire_cohort no longer has a bounded-v2 operation list")
card_ops = []
for item in match.group(1).split(","):
    item = item.strip()
    if item.startswith("and "):
        item = item[4:]
    if item:
        card_ops.append(item)
if manifest.get("wire_schema_version") != "2.0":
    fail("machine census wire schema must remain exact v2.0")
if manifest.get("wire_ops") != card_ops:
    fail("bounded-v2 wire operation cohort drifted")
if manifest.get("effect_order") != card.get("final_architecture", {}).get("wire_effect_order"):
    fail("canonical wire effect order drifted")
if "schema_version=2.0" not in str(card.get("final_architecture", {}).get("wire", "")):
    fail("card no longer records exact schema_version=2.0")

print_row = family_rows[0]
if "print/1" not in print_row or "nyash.builtin.print" not in print_row or "Builtin(Print)" not in print_row:
    fail("exact print/1 attribution drifted in the active family matrix")
if manifest.get("print_source_contract") != "exact print/1":
    fail("machine census print source contract drifted")
if manifest.get("print_compatibility_alias") != "nyash.builtin.print":
    fail("machine census print compatibility alias drifted")
if manifest.get("print_target") != "Builtin(Print)":
    fail("machine census print target drifted")

for marker in manifest.get("forbidden_canonical_identity", []):
    if not isinstance(marker, str) or not marker:
        fail("forbidden canonical identity contains an empty marker")
    if not any(marker in str(item) for item in global_b0.get("forbidden", [])):
        fail(f"forbidden canonical identity marker is not present in the active card: {marker}")

census_text = str(global_b0.get("census", ""))
expected_census = {
    "compiled_rs_match_lines": "271 matching lines",
    "compiled_rs_files": "143 .rs files",
    "compiled_inc_match_lines": "five matches",
    "compiled_inc_files": "two compiled .inc files",
}
for field, phrase in expected_census.items():
    if phrase not in census_text:
        fail(f"active card census no longer records {field}: {phrase}")

print(
    f"[{guard_id}] status=pass families={len(family_entries)} "
    f"compiled_consumers={len(consumer_entries)} wpre_entrances={len(wpre_entries)} "
    f"wire_ops={len(card_ops)} effects={len(manifest['effect_order'])} "
    "print_attribution=exact source/compatibility/target"
)
PY
