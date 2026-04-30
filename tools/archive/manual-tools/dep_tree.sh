#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")/../../.." && pwd)
ENTRY=${1:-lang/src/compiler/entry/compiler_stageb.hako}

# Phase‑20.33: legacy apps/selfhost/tools/dep_tree_main.hako has been retired.
# Intentionally fail fast to surface stale references. TTL: replace with lang tool.
LEGACY_TOOL="$ROOT_DIR/apps/selfhost/tools/dep_tree_main.hako"
if [ -f "$LEGACY_TOOL" ]; then
  echo "[warn] Using legacy dep_tree tool (apps/selfhost). Migrate to lang tool soon (TTL)." >&2
  NYASH_DISABLE_PLUGINS=0 NYASH_CLI_VERBOSE=0 NYASH_USE_PLUGIN_BUILTINS=1 \
    "$ROOT_DIR/target/release/nyash" --backend interpreter \
    "$LEGACY_TOOL" <<<"$ENTRY"
else
  echo "[error] Legacy dep_tree tool not found: $LEGACY_TOOL" >&2
  echo "[hint] Replace this script to call a lang/ tool when available. See CURRENT_TASK.md (Phase‑20.33)." >&2
  exit 2
fi
