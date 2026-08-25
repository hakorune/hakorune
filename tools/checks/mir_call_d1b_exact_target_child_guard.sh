#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mir-call-d1b-exact-target-child-guard"
BRAND="$ROOT_DIR/src/mir/builder/calls/function_call_brand_source_demand.rs"
PREFLIGHT="$ROOT_DIR/src/mir/builder/calls/function_call_preflight_route.rs"
BUILD="$ROOT_DIR/src/mir/builder/calls/build.rs"
TESTS="$ROOT_DIR/src/mir/builder/calls/function_call_preflight_route_tests.rs"

fail() {
  echo "[$TAG] $*" >&2
  exit 1
}

for file in "$BRAND" "$PREFLIGHT" "$BUILD" "$TESTS"; do
  [[ -f "$file" ]] || fail "missing owner ${file#$ROOT_DIR/}"
done

python3 - "$BRAND" "$PREFLIGHT" "$BUILD" "$TESTS" <<'PY'
from pathlib import Path
import sys

brand, preflight, build, tests = map(Path, sys.argv[1:])
brand_text = brand.read_text()
preflight_text = preflight.read_text()
build_text = build.read_text()
tests_text = tests.read_text()

if brand_text.count("InstalledNonBrand {") != 2:
    raise SystemExit("InstalledNonBrand caller transport drifted")
if "RawInvocationRootLineageV1::Cataloged(caller)" not in brand_text:
    raise SystemExit("Cataloged source lineage no longer feeds the child transport")
if preflight_text.count("fn prepare_cataloged_target_v1(") != 1:
    raise SystemExit("cataloged target issuer count drifted")
for token in (
    "PreparedRawOrdinaryFunctionCompletionV1::Targeted",
    "PreparedRawOrdinaryFunctionCompletionV1::Rejected",
    "BareStaticRecoveryDecisionV1::decide",
    "CallTarget::Value(value)",
):
    if token not in preflight_text:
        raise SystemExit(f"missing child target contract: {token}")

target_start = build_text.index("PreparedRawOrdinaryFunctionCompletionV1::Targeted")
resolved_start = build_text.index("PreparedRawOrdinaryFunctionCompletionV1::Resolved", target_start)
target_window = build_text[target_start:resolved_start]
if target_window.index("drive_call_arguments_v1") > target_window.index("emit_prepared_cataloged_call_v1"):
    raise SystemExit("targeted child emits before ordered argument descent")
if any(token in target_window for token in ("build_resolved_function_call", "try_unique_static_method_recovery", "make_name_const_result")):
    raise SystemExit("targeted child re-entered a late resolver/recovery/name-Const edge")

emit_start = build_text.index("fn emit_prepared_cataloged_call_v1")
emit_end = build_text.index("/// Build unified function call", emit_start)
emit_window = build_text[emit_start:emit_end]
if emit_window.count("MirInstruction::call(") != 1:
    raise SystemExit("targeted child canonical issuer count drifted")
if "MirInstruction::Call {" in emit_window or "make_name_const_result" in emit_window:
    raise SystemExit("targeted child retained a legacy Call literal or name Const")

for token in (
    "cataloged_target_preflight_applies_total_shadow_order",
    "cataloged_target_rejects_before_children_on_missing_or_wrong_arity",
    "cataloged_target_is_consumed_once_before_canonical_call_publication",
):
    if token not in tests_text:
        raise SystemExit(f"missing focused child evidence: {token}")
PY

echo "[$TAG] ok"
