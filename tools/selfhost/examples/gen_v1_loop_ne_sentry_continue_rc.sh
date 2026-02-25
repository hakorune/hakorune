#!/usr/bin/env bash
# gen_v1_loop_ne_sentry_continue_rc.sh — v1 JSON: safe '!=' sentinel → continue rc=4
set -euo pipefail
cat <<'JSON'
{
  "schema_version": "1.0",
  "functions": [
    {"name": "main", "blocks": [
      {"id":0,"instructions":[
        {"op":"const","dst":1,"value":{"type":"i64","value":1}},
        {"op":"const","dst":2,"value":{"type":"i64","value":2}},
        {"op":"compare","dst":3,"lhs":1,"rhs":2,"cmp":"Ne"},
        {"op":"branch","cond":3,"then":1,"else":2}
      ]},
      {"id":1,"instructions":[{"op":"const","dst":4,"value":{"type":"i64","value":4}},{"op":"ret","value":4}]},
      {"id":2,"instructions":[{"op":"const","dst":5,"value":{"type":"i64","value":0}},{"op":"ret","value":5}]}
    ]}
  ]
}
JSON

