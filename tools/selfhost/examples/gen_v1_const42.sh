#!/usr/bin/env bash
# gen_v1_const42.sh — minimal v1 JSON generator example (emit-only skeleton)
# Prints a small v1 program that returns 42.
set -euo pipefail
cat <<'JSON'
{
  "schema_version": "1.0",
  "functions": [
    {"name": "main", "blocks": [
      {"id": 0, "instructions": [
        {"op": "const", "dst": 0, "value": {"type": "i64", "value": 42}},
        {"op": "ret", "value": 0}
      ]}
    ]}
  ]
}
JSON
