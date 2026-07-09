#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-read-policy-row-identity-transport"
source "$ROOT/tools/checks/lib/guard_common.sh"

CARD="$ROOT/docs/development/current/main/phases/phase-296x/3426-MIRBUILDER-SCALAR-KNOWN-FASTPATH-READ-POLICY-ROW-IDENTITY-TRANSPORT-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
MAP_GENERATOR="$ROOT/tools/rust_lifecycle/generate_mapload_scalar_i64_hako_policy.py"
STRING_GENERATOR="$ROOT/tools/rust_lifecycle/generate_string_search_scalar_i64_hako_policy.py"
COLLECTION_GENERATOR="$ROOT/tools/rust_lifecycle/generate_collection_len_scalar_i64_hako_policy.py"
MAP_ARTIFACT="$ROOT/src/mir/generic_method_route_plan/generated/mapload_scalar_i64_hako_policy.rs"
STRING_ARTIFACT="$ROOT/src/mir/generic_method_route_plan/generated/string_search_scalar_i64_hako_policy.rs"
COLLECTION_ARTIFACT="$ROOT/src/mir/generic_method_route_plan/generated/collection_len_scalar_i64_hako_policy.rs"
MAP_SOURCE="$ROOT/lang/src/compiler/lib/map_load_scalar_i64_policy_classifier.hako"
STRING_SOURCE="$ROOT/lang/src/compiler/lib/string_search_scalar_i64_policy_classifier.hako"
COLLECTION_SOURCE="$ROOT/lang/src/compiler/lib/collection_len_scalar_i64_policy_classifier.hako"

guard_require_command "$TAG" python3
guard_require_command "$TAG" diff
guard_require_files "$TAG" "$CARD" "$TASK_ORDER" "$MANIFEST" "$MAP_GENERATOR" "$STRING_GENERATOR" \
  "$COLLECTION_GENERATOR" "$MAP_ARTIFACT" "$STRING_ARTIFACT" "$COLLECTION_ARTIFACT" \
  "$MAP_SOURCE" "$STRING_SOURCE" "$COLLECTION_SOURCE"

for pair in \
  "$MAP_GENERATOR:$MAP_ARTIFACT" \
  "$STRING_GENERATOR:$STRING_ARTIFACT" \
  "$COLLECTION_GENERATOR:$COLLECTION_ARTIFACT"; do
  generator="${pair%%:*}"
  artifact="${pair#*:}"
  tmp="$(mktemp)"
  trap 'rm -f "$tmp"' EXIT
  python3 "$generator" > "$tmp"
  diff -u "$artifact" "$tmp"
  rm -f "$tmp"
  trap - EXIT
done

python3 - "$CARD" "$TASK_ORDER" "$MANIFEST" "$MAP_SOURCE" "$STRING_SOURCE" "$COLLECTION_SOURCE" "$MAP_ARTIFACT" "$STRING_ARTIFACT" "$COLLECTION_ARTIFACT" <<'PY'
import json
import sys
from pathlib import Path

card = Path(sys.argv[1]).read_text(encoding="utf-8")
task_order = Path(sys.argv[2]).read_text(encoding="utf-8")
manifest = json.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))
sources = [Path(path).read_text(encoding="utf-8") for path in sys.argv[4:7]]
artifacts = [Path(path).read_text(encoding="utf-8") for path in sys.argv[7:10]]


def need(condition, message):
    if not condition:
        raise SystemExit(message)


token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-READ-POLICY-ROW-IDENTITY-TRANSPORT-001"
next_card = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-CALLER-ORIENTATION-LIVE-ASSERT-CONSUMER-001"
need(token in card, "card token missing")
need(token in {row.get("token") for row in manifest.get("rows", [])}, "manifest token missing")
need(next_card in card and next_card in task_order, "next task pointer missing")

expected = [
    ["map_load_scalar_i64_routes"],
    [
        "string_indexof_scalar_i64_routes",
        "string_lastindexof_scalar_i64_routes",
        "string_contains_scalar_i64_routes",
    ],
    [
        "collection_map_entry_count_scalar_i64_routes",
        "collection_array_slot_len_scalar_i64_routes",
        "collection_string_len_scalar_i64_routes",
        "collection_any_length_scalar_i64_routes",
    ],
]

for index, (row_ids, source, artifact) in enumerate(zip(expected, sources, artifacts)):
    source_ids = []
    if index == 0:
        for line in source.splitlines():
            stripped = line.strip()
            if stripped.startswith('return "map_load_scalar_i64_routes|'):
                source_ids.append(stripped.split("|", 1)[0].split('"', 1)[1])
    for line in source.splitlines():
        stripped = line.strip()
        if index == 0:
            continue
        if stripped.startswith('"'):
            source_ids.append(stripped.split("|", 1)[0].strip('"'))
    need(source_ids == row_ids, f"source row identity drift at surface {index}")
    need(artifact.count('policy_row_id: "') == len(row_ids), f"artifact row value count drift at surface {index}")
    actual = []
    for line in artifact.splitlines():
        stripped = line.strip()
        if stripped.startswith("policy_row_id:"):
            actual.append(stripped.split('"')[1])
    need(actual == row_ids, f"artifact row identity drift at surface {index}")
    for row_id in row_ids:
        need(artifact.count(f'policy_row_id: "{row_id}"') == 1, f"duplicate row ID: {row_id}")

for artifact in artifacts:
    need("route_selection_authority_switch" not in artifact, "route authority claim leaked")
    need("caller_orientation_runtime_path" not in artifact, "runtime claim leaked")

print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-read-policy-row-identity-transport")
print("read_policy_row_identity_transport=1")
print("eight_row_identity_exact=1")
print("mapload_row_count=1")
print("string_row_count=3")
print("collection_row_count=4")
print("route_selection_authority_switch=0")
print("caller_orientation_runtime_path=0")
print("source_selfhost_claim=0")
print("summary=ok")
PY
