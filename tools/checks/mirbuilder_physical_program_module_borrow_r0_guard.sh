#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mirbuilder-physical-program-module-borrow-r0-guard"
STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
PROGRAM="$ROOT_DIR/src/mir/compiler/normal_default_pipeline/published_backend_view/physical_program.rs"
JSON="$ROOT_DIR/src/mir/compiler/normal_default_pipeline/published_backend_view/physical_program_json.rs"
ABI="$ROOT_DIR/src/mir/compiler/normal_default_pipeline/published_backend_view/physical_abi.rs"

fail() {
  echo "[$TAG] result_class=current-change failure status=fail: $*" >&2
  exit 1
}

for file in "$STATE" "$PROGRAM" "$JSON" "$ABI"; do
  [[ -f "$file" ]] || fail "required owner missing: ${file#$ROOT_DIR/}"
done

grep -Fq 'current_execution_row = "MIRBUILDER-PHYSICAL-PROGRAM-MODULE-BORROW-RETIRE-R0"' "$STATE" \
  || fail "physical program module-borrow row is not selected"
grep -Fq 'struct PublishedLifecyclePhysicalProgramV1' "$PROGRAM" \
  || fail "physical program product is missing"
grep -Fq 'handoff: &' "$PROGRAM" || fail "physical program handoff ownership is missing"
grep -Fq 'pub(crate) fn functions(' "$PROGRAM" || fail "physical function product accessor is missing"
grep -Fq 'module: Option<&' "$PROGRAM" \
  || fail "issuance-time module input disappeared; only product borrow may be removed"
if rg -n 'module: &'"'"'module MirModule|pub\(crate\) fn module\(&self\)' "$PROGRAM"; then
  fail "published physical product still exposes a module borrow/getter"
fi
if rg -n 'program\.module\(\)' "$JSON" "$ABI"; then
  fail "transport/layout consumer returned to product module lookup"
fi

for file in "$PROGRAM" "$JSON" "$ABI"; do
  lines="$(wc -l <"$file")"
  (( lines < 800 )) || fail "owner crossed the 800-line hard stop: ${file#$ROOT_DIR/}=$lines"
done

echo "[$TAG] result_class=current-change failure status=pass caller_zero=confirmed product_module_borrow=removed issuance_module_input=retained"
