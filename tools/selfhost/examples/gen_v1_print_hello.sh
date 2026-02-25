#!/usr/bin/env bash
# gen_v1_print_hello.sh — Minimal v1 JSON that extern-calls print("hello") then returns 0
set -euo pipefail
cat <<'JSON'
{
  "schema_version": "1.0",
  "functions": [
    {"name": "main", "params": [], "blocks": [
      {"id": 0, "instructions": [
        {"op":"const","dst":1,"value":{"type":"string","value":"hello"}},
        {"op":"externcall","func":"print","args":[1]},
        {"op":"const","dst":2,"value":{"type":"i64","value":0}},
        {"op":"ret","value":2}
      ]}
    ]}
  ]
}
JSON

