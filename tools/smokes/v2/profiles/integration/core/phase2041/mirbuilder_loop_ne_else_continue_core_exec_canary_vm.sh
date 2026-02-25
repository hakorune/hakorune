#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_json="/tmp/program_loop_ne_else_continue_core_$$.json"
cat > "$tmp_json" <<'JSON'
{"version":0,"kind":"Program","body":[
  {"type":"Local","name":"i","expr":{"type":"Int","value":0}},
  {"type":"Local","name":"s","expr":{"type":"Int","value":0}},
  {"type":"Loop",
    "cond":{"type":"Compare","op":"<","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Int","value":5}},
    "body":[
      {"type":"If","cond":{"type":"Compare","op":"!=","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Int","value":2}},
        "then":[{"type":"Local","name":"s","expr":{"type":"Binary","op":"+","lhs":{"type":"Var","name":"s"},"rhs":{"type":"Int","value":1}}},
                 {"type":"Local","name":"i","expr":{"type":"Binary","op":"+","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Int","value":1}}}],
        "else":[{"type":"Continue"}]
      }
    ]
  },
  {"type":"Return","expr":{"type":"Var","name":"s"}}
]}
JSON

set +e
HAKO_VERIFY_PRIMARY=core HAKO_VERIFY_BUILDER_ONLY=1 verify_program_via_builder_to_core "$tmp_json" >/dev/null 2>&1
rc=$?
set -e
rm -f "$tmp_json" || true

# Presence-only (builder-only structure) for continue sentinel
if [ $rc -ne 0 ]; then
  echo "[FAIL] mirbuilder_loop_ne_else_continue_core_exec_canary_vm (builder-only structure) rc=$rc" >&2
  exit 1
fi
echo "[PASS] mirbuilder_loop_ne_else_continue_core_exec_canary_vm (builder-only)"
exit 0
