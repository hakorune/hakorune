#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/../../.." && pwd)

# Use lang Stage-B compiler entry when NY mode is requested.
NYASH_JSON_ONLY=1 NYASH_DISABLE_PLUGINS=1 NYASH_CLI_VERBOSE=0 \
  "${ROOT_DIR}/target/release/nyash" "${ROOT_DIR}/lang/src/compiler/entry/compiler_stageb.hako" \
  | awk 'BEGIN{printed=0} { if (!printed && $0 ~ /^\s*\{/){ print; printed=1 } }'
