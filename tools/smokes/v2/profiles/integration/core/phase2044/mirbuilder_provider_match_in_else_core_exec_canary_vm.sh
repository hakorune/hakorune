#!/usr/bin/env bash
set -euo pipefail
# Purpose: Provider route: If false → else returns Match; rc=42

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

prog_json_path="/tmp/prog_2044_match_in_else_$$.json"
cat >"$prog_json_path" <<'JSON'
{"version":0,"kind":"Program","body":[
  {"type":"Local","name":"x","expr":{"type":"Str","value":"B"}},
  {"type":"If",
    "cond":{"type":"Compare","op":"<","lhs":{"type":"Int","value":2},"rhs":{"type":"Int","value":1}},
    "then":[{"type":"Return","expr":{"type":"Int","value":0}}],
    "else":[{"type":"Return","expr":{"type":"Match","scrutinee":{"type":"Var","name":"x"},
               "arms":[{"label":"A","expr":{"type":"Int","value":0}}, {"label":"B","expr":{"type":"Int","value":42}}],
               "else":{"type":"Int","value":1}}}]
  }
]}
JSON

trap 'rm -f "$prog_json_path" || true' EXIT
run_preferred_mirbuilder_canary_and_expect_rc \
  "$prog_json_path" \
  42 \
  "provider match-in-else →" \
  "phase2044/mirbuilder_provider_match_in_else_core_exec_canary_vm"
