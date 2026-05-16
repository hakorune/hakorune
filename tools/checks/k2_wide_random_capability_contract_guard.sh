#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-random-capability-contract"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-293x/293x-532-RANDOM-CAP-001-USES-RANDOM-CAPABILITY-DECISION.md"
DESIGN="docs/development/current/main/design/random-capability-failfast-ssot.md"
CAPABILITY="docs/development/current/main/design/mimalloc-hakorune-capability-surface-ssot.md"
RUNTIME_MANUAL="docs/reference/runtime/substrate-capabilities.md"
PLAN_OWNER="src/mir/effect_capability_plan.rs"
METADATA_TYPES="src/mir/function/types.rs"
BUILDER_METADATA="src/mir/builder/builder_metadata.rs"
TESTS="src/tests/mir_effect_capability_plan.rs"
SECURE_POLICY="lang/src/hako_alloc/memory/secure_free_list_policy_box.hako"
EXTERN_PLAN="src/mir/extern_call_route_plan.rs"
KERNEL_EXPORTS="crates/nyash_kernel/src/exports"
RUNTIME_SUBSTRATE="lang/src/runtime/substrate"
INC_DIR="lang/c-abi/shims"
INDEX="docs/tools/check-scripts-index.md"

echo "[$TAG] checking RANDOM-CAP-001 uses random contract"

guard_require_command "$TAG" rg
guard_require_command "$TAG" cargo
guard_require_files \
  "$TAG" \
  "$CARD" \
  "$DESIGN" \
  "$CAPABILITY" \
  "$RUNTIME_MANUAL" \
  "$PLAN_OWNER" \
  "$METADATA_TYPES" \
  "$BUILDER_METADATA" \
  "$TESTS" \
  "$SECURE_POLICY" \
  "$EXTERN_PLAN" \
  "$INDEX" \
  "$0"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "RANDOM-CAP-001 card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "random capability SSOT must be accepted"
guard_expect_in_file "$TAG" 'hako.random' "$DESIGN" "SSOT must name hako.random metadata id"
guard_expect_in_file "$TAG" 'execution remains unsupported' "$CAPABILITY" "capability surface must keep random execution unsupported"
guard_expect_in_file "$TAG" 'source=source_uses' "$RUNTIME_MANUAL" "runtime manual must document source_uses capability plan"
guard_expect_in_file "$TAG" 'source_uses_capability_allow' "$PLAN_OWNER" "MIR owner must own source uses capability mapping"
guard_expect_in_file "$TAG" 'hako.random' "$PLAN_OWNER" "MIR owner must map source uses random to hako.random"
guard_expect_in_file "$TAG" 'declared_capability_uses' "$METADATA_TYPES" "FunctionMetadata must carry source uses"
guard_expect_in_file "$TAG" 'set_current_function_declared_capability_uses' "$BUILDER_METADATA" "builder metadata must own uses transport"
guard_expect_in_file "$TAG" 'mir_transports_source_uses_random_as_metadata_only_capability_plan' "$TESTS" "MIR test must cover source uses random"
guard_expect_in_file "$TAG" "$0" "$INDEX" "check script index must list RANDOM-CAP-001 guard"

if rg -n 'hako_random|hako_entropy|random_source|entropy_source|/dev/urandom|getrandom' \
  "$EXTERN_PLAN" "$KERNEL_EXPORTS" "$RUNTIME_SUBSTRATE" "$INC_DIR" \
  >/tmp/"$TAG".random_route 2>&1; then
  echo "[$TAG] ERROR: random/entropy route leaked into runtime/backend code" >&2
  cat /tmp/"$TAG".random_route >&2
  rm -f /tmp/"$TAG".random_route
  exit 1
fi
rm -f /tmp/"$TAG".random_route

if rg -n 'hako_random|hako_entropy|random_source|entropy_source|runtime_entropy|/dev/urandom|getrandom' "$SECURE_POLICY" \
  >/tmp/"$TAG".secure_policy 2>&1; then
  echo "[$TAG] ERROR: secure free-list policy must not change for RANDOM-CAP-001" >&2
  cat /tmp/"$TAG".secure_policy >&2
  rm -f /tmp/"$TAG".secure_policy
  exit 1
fi
rm -f /tmp/"$TAG".secure_policy

cargo test -q --lib mir_transports_source_uses_random_as_metadata_only_capability_plan

echo "[$TAG] ok"
