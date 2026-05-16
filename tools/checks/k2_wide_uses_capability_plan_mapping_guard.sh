#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-uses-capability-plan-mapping"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-293x/293x-537-USES-002A-DECLARED-USES-CAPABILITY-PLAN-MAPPING.md"
DESIGN="docs/development/current/main/design/declared-uses-capability-plan-mapping-ssot.md"
USES_SSOT="docs/development/current/main/design/uses-metadata-capsule-ssot.md"
CAPABILITY="docs/development/current/main/design/mimalloc-hakorune-capability-surface-ssot.md"
RUNTIME_MANUAL="docs/reference/runtime/substrate-capabilities.md"
PLAN_OWNER="src/mir/effect_capability_plan.rs"
TESTS="src/tests/mir_effect_capability_plan.rs"
EXTERN_PLAN="src/mir/extern_call_route_plan.rs"
INC_DIR="lang/c-abi/shims"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_uses_capability_plan_mapping_guard.sh"

echo "[$TAG] checking USES-002A declared uses capability mapping"

guard_require_command "$TAG" rg
guard_require_command "$TAG" cargo
guard_require_files \
  "$TAG" \
  "$CARD" \
  "$DESIGN" \
  "$USES_SSOT" \
  "$CAPABILITY" \
  "$RUNTIME_MANUAL" \
  "$PLAN_OWNER" \
  "$TESTS" \
  "$EXTERN_PLAN" \
  "$INDEX" \
  "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "USES-002A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "declared uses mapping SSOT must be accepted"
guard_expect_in_file "$TAG" 'no Stage0 capability checker' "$USES_SSOT" "USES-001 must remain metadata-only"
guard_expect_in_file "$TAG" '`uses osvm`' "$CAPABILITY" "capability surface must keep osvm declared uses"
guard_expect_in_file "$TAG" '`uses atomic`' "$CAPABILITY" "capability surface must keep atomic declared uses"
guard_expect_in_file "$TAG" '`uses rawbuf`' "$CAPABILITY" "capability surface must keep rawbuf declared uses"
guard_expect_in_file "$TAG" 'metadata-only `uses osvm` / `uses atomic` / `uses rawbuf` / `uses random`' "$RUNTIME_MANUAL" "runtime manual must document declared uses CapabilityPlan ids"
guard_expect_in_file "$TAG" 'source_uses_capability_allow' "$PLAN_OWNER" "MIR owner must own source uses mapping"
guard_expect_in_file "$TAG" 'Some\("hako\.osvm"\)' "$PLAN_OWNER" "MIR owner must map osvm"
guard_expect_in_file "$TAG" 'Some\("hako\.atomic"\)' "$PLAN_OWNER" "MIR owner must map atomic"
guard_expect_in_file "$TAG" 'Some\("hako\.rawbuf"\)' "$PLAN_OWNER" "MIR owner must map rawbuf"
guard_expect_in_file "$TAG" 'Some\("hako\.random"\)' "$PLAN_OWNER" "MIR owner must preserve random"
guard_expect_in_file "$TAG" 'source_declared_uses_emit_canonical_capability_plan_ids' "$PLAN_OWNER" "unit test must cover canonical mapping"
guard_expect_in_file "$TAG" 'mir_transports_low_level_declared_uses_as_capability_plan_ids' "$TESTS" "MIR integration test must cover declared uses"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list USES-002A guard"

if rg -n '"tls"[[:space:]]*=>[[:space:]]*Some\("hako\.tls"\)' "$PLAN_OWNER" >/tmp/"$TAG".tls_mapping 2>&1; then
  echo "[$TAG] ERROR: USES-002A must not open source-level tls mapping" >&2
  cat /tmp/"$TAG".tls_mapping >&2
  rm -f /tmp/"$TAG".tls_mapping
  exit 1
fi
rm -f /tmp/"$TAG".tls_mapping

if rg -n 'cap[[:space:]]*\{|capability_policy_solver|verify_capability|helper_name_capability|uses_capability_backend_gate' \
  "$PLAN_OWNER" "$TESTS" >/tmp/"$TAG".policy_widening 2>&1; then
  echo "[$TAG] ERROR: USES-002A must not add cap blocks or broad capability solver" >&2
  cat /tmp/"$TAG".policy_widening >&2
  rm -f /tmp/"$TAG".policy_widening
  exit 1
fi
rm -f /tmp/"$TAG".policy_widening

if rg -n 'hako_random|hako_entropy|random_source|entropy_source|/dev/urandom|getrandom' \
  "$EXTERN_PLAN" "$INC_DIR" >/tmp/"$TAG".random_route 2>&1; then
  echo "[$TAG] ERROR: random/entropy route leaked into runtime/backend code" >&2
  cat /tmp/"$TAG".random_route >&2
  rm -f /tmp/"$TAG".random_route
  exit 1
fi
rm -f /tmp/"$TAG".random_route

cargo test -q --lib source_declared_uses_emit_canonical_capability_plan_ids
cargo test -q --lib mir_transports_low_level_declared_uses_as_capability_plan_ids

echo "[$TAG] ok"
