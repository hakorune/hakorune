#!/usr/bin/env bash
# gen_v1_threeblock_collect.sh — v1 JSON: 3-block collect → rc=44 on then-path
set -euo pipefail
cat <<'JSON'
{
  "schema_version": "1.0",
  "functions": [
    {"name": "main", "blocks": [
      {"id":0,"instructions":[
        {"op":"const","dst":1,"value":{"type":"i64","value":3}},
        {"op":"const","dst":2,"value":{"type":"i64","value":5}},
        {"op":"compare","dst":3,"lhs":1,"rhs":2,"cmp":"Lt"},
        {"op":"branch","cond":3,"then":1,"else":2}
      ]},
      {"id":1,"instructions":[
        {"op":"const","dst":4,"value":{"type":"i64","value":44}},
        {"op":"jump","target":3}
      ]},
      {"id":2,"instructions":[
        {"op":"const","dst":4,"value":{"type":"i64","value":40}},
        {"op":"jump","target":3}
      ]},
      {"id":3,"instructions":[{"op":"ret","value":4}]}
    ]}
  ]
}
JSON

