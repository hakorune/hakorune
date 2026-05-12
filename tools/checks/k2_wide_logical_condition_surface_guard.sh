#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-logical-condition-surface"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

APP="apps/logical-condition-surface-proof/main.hako"
APP_TEST="apps/logical-condition-surface-proof/test.sh"
APP_README="apps/logical-condition-surface-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-203-C197-LOGICAL-CONDITION-SURFACE.md"
ORDER_CARD="docs/development/current/main/phases/phase-293x/293x-202-C197-C200-PROOF-APPLICATION-SURFACE-ORDER.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
SHORT_CIRCUIT_SSOT="docs/development/current/main/design/short-circuit-joins-ssot.md"
EBNF="docs/reference/language/EBNF.md"
PARSER_TEST="src/tests/parser_logical_condition_surface.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_logical_condition_surface_guard.sh"

echo "[$TAG] checking C197 logical condition surface"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$CARD" \
  "$ORDER_CARD" \
  "$PLAN" \
  "$SHORT_CIRCUIT_SSOT" \
  "$EBNF" \
  "$PARSER_TEST" \
  "$INDEX" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Decision: accepted' "$EBNF" "EBNF must record the C197 accepted decision"
guard_expect_in_file "$TAG" 'C197 logical condition surface hardening` \| Complete' "$PLAN" "plan must mark C197 complete"
guard_expect_in_file "$TAG" 'C198 check block surface` \| Future' "$PLAN" "check block must remain the later row"
guard_expect_in_file "$TAG" 'proof-list semantics' "$SHORT_CIRCUIT_SSOT" "short-circuit SSOT must keep proof-list semantics separate"
guard_expect_in_file "$TAG" 'all-items-evaluated contract' "$SHORT_CIRCUIT_SSOT" "short-circuit SSOT must document eager check behavior separately"
guard_expect_in_file "$TAG" 'NYASH_PARSER_TOKEN_CURSOR' "$PARSER_TEST" "parser test must cover TokenCursor route"
guard_expect_in_file "$TAG" '\&\& \(\(x = x \+ 1\) == 1\)' "$APP" "proof app must preserve AND RHS side-effect check"
guard_expect_in_file "$TAG" '\|\| \(\(x = x \+ 10\) == 10\)' "$APP" "proof app must preserve OR RHS side-effect check"
guard_expect_in_file "$TAG" 'summary=ok' "$APP" "proof app must expose stable summary"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list C197 guard"

if rg -n 'check[[:space:]]+"' "$APP" "$PARSER_TEST" >/tmp/"$TAG".check-block 2>&1; then
  echo "[$TAG] ERROR: C197 must not depend on future check-block syntax" >&2
  cat /tmp/"$TAG".check-block >&2
  rm -f /tmp/"$TAG".check-block
  exit 1
fi
rm -f /tmp/"$TAG".check-block

if rg -n 'logical-condition-surface|check block|ProofCheck' lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: C197 app/proof matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

cargo test -q parser_logical_condition_surface

tmp_dir="$(mktemp -d /tmp/hakorune_c197_logical_condition.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
vm_out="$tmp_dir/vm.out"
vm_err="$tmp_dir/vm.err"

if [[ -n "${HAKORUNE_BIN:-}" ]]; then
  HAKO_CMD=("$HAKORUNE_BIN")
else
  HAKO_CMD=(cargo run -q --bin hakorune --)
fi

NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  "${HAKO_CMD[@]}" --backend vm "$APP" >"$vm_out" 2>"$vm_err"

rg -F -q 'logical-condition-surface-proof' "$vm_out"
rg -F -q 'x=0' "$vm_out"
rg -F -q 'loop_count=3' "$vm_out"
rg -F -q 'flags=false,true,true' "$vm_out"
rg -F -q 'summary=ok' "$vm_out"

cat "$vm_out"
echo "[$TAG] ok"
