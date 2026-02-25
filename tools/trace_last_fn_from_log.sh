#!/bin/bash
set -euo pipefail

if [ $# -lt 1 ]; then
  echo "usage: $0 <log_path>" >&2
  exit 2
fi

log_path="$1"
if [ ! -f "$log_path" ]; then
  echo "[ERR] log not found: $log_path" >&2
  exit 1
fi

rg -n "\\[lower_(static_)?method_as_function\\] Storing fn_body" "$log_path" | tail -n 1
