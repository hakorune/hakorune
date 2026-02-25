#!/usr/bin/env bash
set -euo pipefail

ROOT=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
cd "$ROOT"

NEEDLES=(
  'instructions.push(MirInstruction::Copy'
  'add_instruction(MirInstruction::Copy'
  'add_instruction_before_terminator(MirInstruction::Copy'
)

# Allowlist exceptions (keep in sync with docs/development/current/main/design/copy-emission-ssot.md)
ALLOW_GLOBS=(
  '!src/mir/builder/emission/copy_emitter.rs'
  '!src/runner/mir_json_v0.rs'
  '!src/runner/json_v1_bridge/parse.rs'
  '!tests/**'
  '!src/**/tests/**'
  '!crates/**/tests/**'
)

RG_ARGS=(--type rust -n -S)
for g in "${ALLOW_GLOBS[@]}"; do
  RG_ARGS+=(--glob "$g")
done

tmpfile="$(mktemp)"
trap 'rm -f "$tmpfile"' EXIT

for needle in "${NEEDLES[@]}"; do
  # rg exit codes: 0=matched, 1=no match, 2=error
  set +e
  rg "${RG_ARGS[@]}" --fixed-strings "$needle" src >>"$tmpfile"
  rg_code=$?
  set -e
  if [[ $rg_code -ne 0 && $rg_code -ne 1 ]]; then
    echo "[no-direct-copy] ERROR: rg failed (code=$rg_code)."
    exit 2
  fi
done

# Filter comment-only hits (e.g. doc examples in rust files).
matches=$(
  awk -F: '
    {
      line=$0;
      content=line;
      sub(/^[^:]*:[0-9]+:/, "", content);
      if (content ~ /^[[:space:]]*\/\//) next;
      print line;
    }
  ' "$tmpfile" | sort -u
)

if [[ -n "$matches" ]]; then
  echo "[no-direct-copy] ERROR: direct MirInstruction::Copy emission is forbidden."
  echo "Fix: route through CopyEmitter or update the allowlist + SSOT."
  echo
  echo "$matches"
  exit 1
fi

echo "[no-direct-copy] OK"
