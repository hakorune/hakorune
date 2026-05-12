#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

tmp_dir="$(mktemp -d /tmp/hakorune_c200_guard_else_app.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

out="$tmp_dir/out.txt"
err="$tmp_dir/err.txt"

if [[ -n "${HAKORUNE_BIN:-}" ]]; then
  HAKO_CMD=("$HAKORUNE_BIN")
else
  HAKO_CMD=(cargo run -q --bin hakorune --)
fi

NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  "${HAKO_CMD[@]}" --backend vm apps/guard-else-surface-proof/main.hako >"$out" 2>"$err"

rg -F -q 'guard-else-surface-proof' "$out"
rg -F -q 'first-guard=passed' "$out"
rg -F -q 'second-guard=blocked' "$out"
rg -F -q 'summary=ok' "$out"

if rg -F -q 'unexpected' "$out"; then
  cat "$out" >&2
  exit 1
fi

cat "$out"
