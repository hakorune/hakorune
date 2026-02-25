#!/usr/bin/env bash
# gen_v1_typeop_check_integer.sh — minimal v1 JSON generator (const+typeop check integer)
set -euo pipefail
cat <<'JSON'
{
  "schema_version": "1.0",
  "functions": [
    {"name": "main", "blocks": [
      {"id": 0, "instructions": [
        {"op": "const", "dst": 0, "value": {"type": "i64", "value": 7}},
        {"op": "typeop", "operation": "check", "src": 0, "target_type": "integer", "dst": 1},
        {"op": "ret", "value": 1}
      ]}
    ]}
  ]
}
JSON

