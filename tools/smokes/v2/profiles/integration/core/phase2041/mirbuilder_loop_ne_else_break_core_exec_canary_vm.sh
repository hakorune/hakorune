#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_json="/tmp/program_loop_ne_else_break_core_$$.json"
cat > "$tmp_json" <<'JSON'
{"version":0,"kind":"Program","body":[
  {"type":"Local","name":"i","expr":{"type":"Int","value":0}},
  {"type":"Local","name":"s","expr":{"type":"Int","value":0}},
  {"type":"Loop",
    "cond":{"type":"Compare","op":"<","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Int","value":5}},
    "body":[
      {"type":"If","cond":{"type":"Compare","op":"!=","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Int","value":3}},
        "then":[{"type":"Local","name":"s","expr":{"type":"Binary","op":"+","lhs":{"type":"Var","name":"s"},"rhs":{"type":"Int","value":1}}},
                 {"type":"Local","name":"i","expr":{"type":"Binary","op":"+","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Int","value":1}}}],
        "else":[{"type":"Break"}]
      }
    ]
  },
  {"type":"Return","expr":{"type":"Var","name":"s"}}
]}
JSON

trap 'rm -f "$tmp_json" || true' EXIT

run_verify_canary_and_expect_rc \
  run_verify_program_via_core_default_to_core \
  "$tmp_json" \
  3 \
  "mirbuilder_loop_ne_else_break_core_exec_canary_vm" \
  "mirbuilder_loop_ne_else_break_core_exec_canary_vm"
