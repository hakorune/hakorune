#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"

python3 tools/checks/source_ast_vocabulary_inventory_v1.py

echo "[source-ast-vocabulary-v1-guard] ok"
