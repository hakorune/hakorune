#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="parser-callable-generated-anchor-r0"
GENERATED="$ROOT/src/parser/generated_callable_anchor.rs"
ANCHOR="$ROOT/src/parser/callable_source_anchor.rs"
AUTHORITY="$ROOT/src/parser/source_authority.rs"
PROJECTION="$ROOT/src/parser/callable_gate_projection.rs"
MODEL="$ROOT/src/parser/source_seal/model.rs"
GATE="$ROOT/src/parser/source_seal/gate_projection.rs"
FINALIZE="$ROOT/src/parser/source_seal/finalize.rs"
ENVELOPE="$ROOT/src/parser/postpass_envelope.rs"
README="$ROOT/src/parser/README.md"
TASK="$ROOT/docs/development/current/main/investigations/dynamic-carrier-ingress-lifecycle-d0-design-task-2026-08-10.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
INDEX="$ROOT/docs/tools/check-scripts-index.md"
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$GENERATED" "$ANCHOR" "$AUTHORITY" \
  "$PROJECTION" "$MODEL" "$GATE" "$FINALIZE" "$ENVELOPE" \
  "$README" "$TASK" "$STATE" "$INDEX"

python3 - "$GENERATED" "$ANCHOR" "$AUTHORITY" "$PROJECTION" "$MODEL" \
  "$GATE" "$FINALIZE" "$ENVELOPE" "$README" "$TASK" "$STATE" "$INDEX" <<'PY'
import re
import sys
from pathlib import Path

paths = list(map(Path, sys.argv[1:]))
(
    generated_path, anchor_path, authority_path, projection_path, model_path,
    gate_path, finalize_path, envelope_path, readme_path, task_path,
    state_path, index_path,
) = paths
texts = {path: path.read_text(encoding="utf-8") for path in paths}
generated = texts[generated_path]
anchor = texts[anchor_path]
authority = texts[authority_path]
projection = texts[projection_path]
model = texts[model_path]
gate = texts[gate_path]
finalize = texts[finalize_path]
envelope = texts[envelope_path]
readme = texts[readme_path]
task = texts[task_path]
state = texts[state_path]
index = texts[index_path]

for needle in (
    "PreparedGeneratedCallableSourceV1",
    "GeneratedPropertyCallableOriginV1",
    "GeneratedDelegateCallableOriginV1",
    "CallableDeclarationAnchorV1::issue()",
    "issue_property_callable_rows",
    "issue_delegate_callable_rows",
    "exact_selected_path_for_method_site",
):
    if needle not in generated + anchor + projection:
        raise SystemExit(f"missing generated callable anchor contract: {needle}")

for struct_name in (
    "CallableDeclarationAnchorV1",
    "PreparedGeneratedCallableSourceV1",
    "PreparedCallableSourceV1",
):
    pattern = rf"derive\([^)]*Clone[^)]*\)\s*pub\(super\) (?:struct|enum) {struct_name}"
    if re.search(pattern, anchor):
        raise SystemExit(f"{struct_name} must remain non-Clone")

for needle in (
    "source_site: SourceBoxMethodSiteV1",
    "placement: BoxMethodInventoryPlacementReceiptV1",
    "generated_property_callable_rows",
):
    if needle not in authority + model:
        raise SystemExit(f"property origin must retain exact source/placement: {needle}")

for needle in (
    "GeneratedDelegateSourceRelationV1",
    "host_delegate_member()",
    "generated_inventory_placement()",
    "member_gate_selection_receipts()",
):
    if needle not in generated:
        raise SystemExit(f"delegate origin must consume exact generator evidence: {needle}")

for needle in (
    "PreparedCallableSourceV1::Direct",
    "PreparedCallableSourceV1::Generated",
    "std::mem::take(&mut seal.generated_property_callable_rows)",
):
    if needle not in gate:
        raise SystemExit(f"atomic callable prune is missing generated carriage: {needle}")

for text, label in ((model, "model"), (finalize, "finalizer"), (envelope, "envelope")):
    if "PreparedCallableSourceV1" not in text:
        raise SystemExit(f"{label} must retain the common callable row set")

for forbidden in (
    'diagnostic_name() == "__',
    "eval_build_predicate",
    "MacroOrImport =>",
    "CompatibilityOnly =>",
    "fallback",
    "retry",
):
    if forbidden in generated:
        raise SystemExit(f"generated issuer must not recreate source authority: {forbidden}")

for needle in (
    "property_generator_issues_fresh_anchor_per_exact_placement",
    "property_generator_rejects_missing_duplicate_and_foreign_receipts",
    "delegate_generator_issues_fresh_anchor_from_exact_relation",
    "selected_member_gate_keeps_only_selected_generated_property_origins",
    "selected_member_gate_preserves_exact_delegate_origin_branch",
    "delegate_anchor_coverage_rejects_missing_and_duplicate_relations",
    "delegate_generator_rejects_foreign_source_relation",
):
    if needle not in generated:
        raise SystemExit(f"missing generated callable regression: {needle}")

for document, label in ((readme, "parser README"), (task, "active task")):
    if "PARSER-CALLABLE-GENERATED-ANCHOR-R0" not in document:
        raise SystemExit(f"{label} is missing the generated-anchor receipt")
section = task.split("#### `PARSER-CALLABLE-GENERATED-ANCHOR-R0`", 1)[1].split(
    "#### `PARSER-INITIAL-CALLABLE-SOURCE-COSEAL-I0`", 1
)[0]
if "Status: **closed**" not in section:
    raise SystemExit("generated-anchor row must be closed before pointer advancement")
if 'current_execution_row = "PARSER-CALLABLE-GENERATED-ANCHOR-R0"' in state:
    raise SystemExit("current pointer must advance beyond the closed generated-anchor row")
if "parser_callable_generated_anchor_r0_guard.sh" not in index:
    raise SystemExit("check index must list the generated callable anchor guard")

for path in (
    generated_path, anchor_path, authority_path, projection_path, model_path,
    gate_path, finalize_path, envelope_path,
):
    lines = len(texts[path].splitlines())
    if lines >= 760:
        raise SystemExit(f"parser source reached the 760-line split trigger: {path}: {lines}")

print("fresh_property_anchor=1")
print("fresh_delegate_anchor=1")
print("exact_selected_gate_origin=1")
print("macro_or_import_support=0")
print("compatibility_origin_support=0")
print("generated_issuer_verified_program_publication=0")
print("source_files_below_760=1")
print("summary=ok")
PY

echo "[$TAG] ok"
