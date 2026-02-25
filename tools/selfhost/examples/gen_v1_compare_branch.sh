#!/usr/bin/env bash
# gen_v1_compare_branch.sh — minimal 2-block v1 JSON generator (compare/branch)
# Program: if (3 < 5) return 1 else return 2
set -euo pipefail
cat <<'JSON'
{
  "schema_version": "1.0",
  "functions": [
    {"name": "main", "blocks": [
      {"id": 0, "instructions": [
        {"op":"const","dst":0,"value":{"type":"i64","value":3}},
        {"op":"const","dst":1,"value":{"type":"i64","value":5}},
        {"op":"compare","dst":2,"lhs":0,"rhs":1,"cmp":"Lt"},
        {"op":"branch","cond":2,"then":1,"else":2}
      ]},
      {"id": 1, "instructions": [
        {"op":"const","dst":3,"value":{"type":"i64","value":1}},
        {"op":"ret","value":3}
      ]},
      {"id": 2, "instructions": [
        {"op":"const","dst":4,"value":{"type":"i64","value":2}},
        {"op":"ret","value":4}
      ]}
    ]}
  ]
}
JSON
