#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

# B3 closeout contract:
# - compat_invoke_core is isolation-only.
# - functional calls must stay inside plugin/invoke_core.rs.

violations=""
while IFS= read -r line; do
  file="${line%%:*}"
  if [[ "$file" != "crates/nyash_kernel/src/plugin/invoke_core.rs" ]]; then
    violations+="$line"$'\n'
  fi
done < <(rg -n "compat_invoke_core::" crates/nyash_kernel/src/plugin -g'*.rs' || true)

if [[ -n "$violations" ]]; then
  echo "[phase29cc-kernel-b3-compat-guard] violation: compat_invoke_core call leaked outside invoke_core.rs" >&2
  printf "%s" "$violations" >&2
  exit 1
fi

if ! rg -n "mod compat_invoke_core;" crates/nyash_kernel/src/plugin/mod.rs >/dev/null; then
  echo "[phase29cc-kernel-b3-compat-guard] violation: mod compat_invoke_core declaration missing" >&2
  exit 1
fi

echo "[phase29cc-kernel-b3-compat-guard] ok"
