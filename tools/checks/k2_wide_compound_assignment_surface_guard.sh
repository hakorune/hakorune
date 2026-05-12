#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-compound-assignment-surface"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

APP="apps/compound-assignment-surface-proof/main.hako"
APP_TEST="apps/compound-assignment-surface-proof/test.sh"
APP_README="apps/compound-assignment-surface-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-205-C199-COMPOUND-ASSIGNMENT-SURFACE.md"
ORDER_CARD="docs/development/current/main/phases/phase-293x/293x-202-C197-C200-PROOF-APPLICATION-SURFACE-ORDER.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
EBNF="docs/reference/language/EBNF.md"
PARSER="src/parser/mod.rs"
PARSER_TEST="src/tests/parser_compound_assignment_surface.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_compound_assignment_surface_guard.sh"

echo "[$TAG] checking C199 compound assignment surface"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$CARD" \
  "$ORDER_CARD" \
  "$PLAN" \
  "$EBNF" \
  "$PARSER" \
  "$PARSER_TEST" \
  "$INDEX" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" '### C199 Compound Assignment Surface' "$EBNF" "EBNF must record the C199 decision"
guard_expect_in_file "$TAG" 'Decision: accepted' "$EBNF" "EBNF must mark C199 accepted"
guard_expect_in_file "$TAG" "compound_assign_op := '\\+=' \\| '-=' \\| '\\*=' \\| '/='" "$EBNF" "EBNF must define compound assignment operators"
guard_expect_in_file "$TAG" 'C199 compound assignment surface` \| Complete' "$PLAN" "plan must mark C199 complete"
guard_expect_in_file "$TAG" 'C200 guard else surface` \| Complete' "$PLAN" "C200 must now be the completed later row"
guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "C199 card must be complete"
guard_expect_in_file "$TAG" 'ASTNode::Index' "$PARSER" "parser must accept index compound assignment targets"
guard_expect_in_file "$TAG" 'NYASH_SYNTAX_SUGAR_LEVEL' "$PARSER_TEST" "parser test must cover sugar gate behavior"
guard_expect_in_file "$TAG" 'array\[0\] \+=' "$APP" "proof app must cover index compound assignment"
guard_expect_in_file "$TAG" 'counter.value \+=' "$APP" "proof app must cover field compound assignment"
guard_expect_in_file "$TAG" 'summary=ok' "$APP" "proof app must expose stable summary"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list C199 guard"

if rg -n 'compound-assignment-surface|CompoundAssignment|PlusAssign|MinusAssign|MulAssign|DivAssign' lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: C199 syntax/proof matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

cargo test -q parser_compound_assignment_surface

tmp_dir="$(mktemp -d /tmp/hakorune_c199_compound_assignment.XXXXXX)"
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

rg -F -q 'compound-assignment-surface-proof' "$vm_out"
rg -F -q 'x=3' "$vm_out"
rg -F -q 'counter=13' "$vm_out"
rg -F -q 'array0=7' "$vm_out"
rg -F -q 'summary=ok' "$vm_out"

cat "$vm_out"
echo "[$TAG] ok"
