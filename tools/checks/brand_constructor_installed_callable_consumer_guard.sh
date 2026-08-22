#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

require_text() {
  local file="$1"
  local text="$2"
  grep -Fq "$text" "$file" || {
    echo "[brand-installed-callable-consumer] missing '$text' in $file" >&2
    exit 1
  }
}

PORT=src/mir/builder/calls/function_call_brand_source_demand.rs
DISPATCH=src/mir/builder/raw_expression_dispatch/mod.rs
PREFLIGHT=src/mir/builder/calls/function_call_preflight_route.rs
STATE=src/mir/builder/normal_callable_semantic_lowering_state.rs
CARD=docs/development/current/main/investigations/brand-instance-constructor-source-relation-d0.md

require_text "$PORT" "RelationlessCompatibility"
require_text "$PORT" "InstalledNonBrand"
require_text "$PORT" "take_brand_constructor"
require_text "$PORT" "operand-site-drift"
require_text "$DISPATCH" "brand_call_authority_v1"
require_text "$PREFLIGHT" "prepare_with_brand_authority"
require_text "$PREFLIGHT" "InstalledNonBrand"
require_text "$PREFLIGHT" "is_brand_declared"
require_text "$STATE" "consumed_brand_constructors"
require_text "$STATE" "constructor_count"
require_text "$CARD" "Installed callable consumer I0"
require_text "$CARD" 'exact NonBrand never calls `is_brand_declared`'

if rg -n "brand_call_authority_v1.*is_brand_declared|InstalledNonBrand.*is_brand_declared" \
  "$PORT" "$DISPATCH" "$PREFLIGHT"; then
  echo "[brand-installed-callable-consumer] installed exact authority may not re-probe the mutable Brand map" >&2
  exit 1
fi

for file in "$PORT" "$DISPATCH" "$PREFLIGHT" "$STATE"; do
  lines="$(wc -l < "$file")"
  if (( lines >= 760 )); then
    echo "[brand-installed-callable-consumer] source split required: $file has $lines lines" >&2
    exit 1
  fi
done

echo "[brand-installed-callable-consumer] OK"
