#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-check-block-surface"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

APP="apps/check-block-surface-proof/main.hako"
APP_TEST="apps/check-block-surface-proof/test.sh"
APP_README="apps/check-block-surface-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-204-C198-CHECK-BLOCK-SURFACE.md"
ORDER_CARD="docs/development/current/main/phases/phase-293x/293x-202-C197-C200-PROOF-APPLICATION-SURFACE-ORDER.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
EBNF="docs/reference/language/EBNF.md"
PARSER_TEST="src/tests/parser_check_block_surface.rs"
AST="src/ast/mod.rs"
PARSER_PRIMARY="src/parser/expr/primary.rs"
PARSER_CURSOR="src/parser/expr_cursor.rs"
BUILDER="src/mir/builder/exprs.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_check_block_surface_guard.sh"

echo "[$TAG] checking C198 check block surface"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$CARD" \
  "$ORDER_CARD" \
  "$PLAN" \
  "$EBNF" \
  "$PARSER_TEST" \
  "$AST" \
  "$PARSER_PRIMARY" \
  "$PARSER_CURSOR" \
  "$BUILDER" \
  "$INDEX" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" '### C198 Check Block Surface' "$EBNF" "EBNF must record the C198 decision"
guard_expect_in_file "$TAG" 'Decision: accepted' "$EBNF" "EBNF must mark C198 accepted"
guard_expect_in_file "$TAG" 'check_expr := ' "$EBNF" "EBNF must define check_expr"
guard_expect_in_file "$TAG" 'C198 check block surface` \| Complete' "$PLAN" "plan must mark C198 complete"
guard_expect_in_file "$TAG" 'C199 compound assignment surface` \| Complete' "$PLAN" "C199 must remain a separate completed row"
guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "C198 card must be complete"
guard_expect_in_file "$TAG" 'CheckExpr' "$AST" "AST must carry CheckExpr"
guard_expect_in_file "$TAG" 'parse_check_expr' "$PARSER_PRIMARY" "default parser must parse check blocks"
guard_expect_in_file "$TAG" 'parse_check_or_variable' "$PARSER_CURSOR" "TokenCursor parser must parse check blocks"
guard_expect_in_file "$TAG" 'build_check_expression' "$BUILDER" "MIR builder must lower check blocks"
guard_expect_in_file "$TAG" 'MirInstruction::Select' "$BUILDER" "check lowering must use eager scalar accumulation"
guard_expect_in_file "$TAG" 'NYASH_PARSER_TOKEN_CURSOR' "$PARSER_TEST" "parser test must cover TokenCursor route"
guard_expect_in_file "$TAG" 'check "c198 eager proof"' "$APP" "proof app must use check syntax"
guard_expect_in_file "$TAG" 'still eager after failure' "$APP" "proof app must prove eager evaluation"
guard_expect_in_file "$TAG" 'summary=ok' "$APP" "proof app must expose stable summary"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list C198 guard"

if rg -n 'check-block-surface|check block|CheckExpr|parse_check' lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: C198 syntax/proof matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

cargo test -q parser_check_block_surface

tmp_dir="$(mktemp -d /tmp/hakorune_c198_check_block.XXXXXX)"
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

rg -F -q 'check-block-surface-proof' "$vm_out"
rg -F -q 'observed=2' "$vm_out"
rg -F -q 'failed=0' "$vm_out"
rg -F -q 'passed=1' "$vm_out"
rg -F -q 'summary=ok' "$vm_out"

cat "$vm_out"
echo "[$TAG] ok"
