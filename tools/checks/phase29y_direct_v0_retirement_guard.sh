#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/design/de-rust-runtime-meaning-decision-red-inventory-ssot.md"
LANE_DOC="$ROOT_DIR/docs/development/current/main/phases/phase-29y/50-LANE-GATE-SSOT.md"
ROUTE_CONTRACT="$ROOT_DIR/src/runner/modes/common_util/selfhost/runtime_route_contract.rs"
DISPATCH="$ROOT_DIR/src/runner/dispatch.rs"
RUNNER_FLAGS="$ROOT_DIR/src/config/env/runner_flags.rs"
CLI_ARGS="$ROOT_DIR/src/cli/args.rs"
ENV_DOC="$ROOT_DIR/docs/reference/environment-variables.md"
DIRECT_V0_SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/integration/apps/phase29y_direct_v0_bridge_guard_vm.sh"
GUARD_TAG="phase29y-direct-v0-retirement-guard"

source "$(dirname "$0")/lib/guard_common.sh"

cd "$ROOT_DIR"
echo "[$GUARD_TAG] checking direct-v0 retirement boundary"

guard_require_command "$GUARD_TAG" rg
guard_require_files "$GUARD_TAG" "$DOC" "$LANE_DOC" "$ROUTE_CONTRACT" "$DISPATCH" "$RUNNER_FLAGS" "$CLI_ARGS" "$ENV_DOC" "$DIRECT_V0_SMOKE"

# direct-v0 freeze boundary message remains as historical contract.
guard_expect_in_file "$GUARD_TAG" 'status=retired' "$ROUTE_CONTRACT" "runtime_route_contract missing retired status marker"

# Removed parser entrypoints must stay removed.
if rg -q 'Arg::new\("parser"\)' "$CLI_ARGS"; then
  guard_fail "$GUARD_TAG" "cli parser flag unexpectedly present in args.rs"
fi
if rg -q 'NYASH_USE_NY_PARSER' "$CLI_ARGS"; then
  guard_fail "$GUARD_TAG" "cli still exports NYASH_USE_NY_PARSER"
fi
if rg -q 'use_ny_parser_legacy|NYASH_USE_NY_PARSER' "$RUNNER_FLAGS"; then
  guard_fail "$GUARD_TAG" "runner_flags still contains legacy NY parser env route"
fi
if rg -q 'direct_v0_policy::route_requested|direct_v0_policy::execute_or_exit' "$DISPATCH"; then
  guard_fail "$GUARD_TAG" "dispatch still has direct-v0 parser route gate"
fi
guard_expect_in_file "$GUARD_TAG" 'unexpected argument.*--parser' "$DIRECT_V0_SMOKE" "direct-v0 smoke missing removed-cli-flag assertion"
guard_expect_in_file "$GUARD_TAG" 'NYASH_USE_NY_PARSER=1' "$DIRECT_V0_SMOKE" "direct-v0 smoke missing legacy env ignored case"
guard_expect_in_file "$GUARD_TAG" 'legacy env still activates retired direct-v0 route' "$DIRECT_V0_SMOKE" "direct-v0 smoke missing env ignored failure assertion"

# direct-v0 route must stay rollback-only in docs.
guard_expect_in_file "$GUARD_TAG" 'RDM-2' "$DOC" "red inventory missing RDM-2 entry"
guard_expect_in_file "$GUARD_TAG" 'retired route' "$ENV_DOC" "environment reference missing retired-route wording"
guard_expect_in_file "$GUARD_TAG" 'phase29y_direct_v0_bridge_guard_vm.sh' "$LANE_DOC" "lane gate SSOT missing direct-v0 guard reference"

echo "[$GUARD_TAG] ok"
