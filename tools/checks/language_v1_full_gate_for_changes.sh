#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
MANIFEST="$ROOT_DIR/tools/checks/manifests/language_v1_full_gate_sensitive_paths.txt"
FULL_GATE="$ROOT_DIR/tools/checks/language_v1_grammar_contract_substrate_guard.sh"

usage() {
  cat <<'EOF'
Usage:
  language_v1_full_gate_for_changes.sh [--check-only] --base <git-ref>
  language_v1_full_gate_for_changes.sh [--check-only] --files <path>...
EOF
}

check_only=0
if [[ "${1:-}" == "--check-only" ]]; then
  check_only=1
  shift
fi

mode="${1:-}"
shift || true
case "$mode" in
  --base)
    [[ $# -eq 1 ]] || { usage >&2; exit 2; }
    mapfile -t changed_files < <(git -C "$ROOT_DIR" diff --name-only "$1"...HEAD)
    ;;
  --files)
    [[ $# -gt 0 ]] || { usage >&2; exit 2; }
    changed_files=("$@")
    ;;
  *)
    usage >&2
    exit 2
    ;;
esac

mapfile -t patterns < <(sed -e 's/[[:space:]]*$//' -e '/^#/d' -e '/^$/d' "$MANIFEST")
matched=""
for path in "${changed_files[@]}"; do
  for pattern in "${patterns[@]}"; do
    if [[ "$path" == $pattern ]]; then
      matched="$path"
      break 2
    fi
  done
done

if [[ -z "$matched" ]]; then
  echo "[language-v1-full-gate-for-changes] skip: no sensitive path"
  exit 0
fi

echo "[language-v1-full-gate-for-changes] required: $matched"
if [[ $check_only -eq 1 ]]; then
  exit 0
fi

LANGV1_GRAMMAR_FULL=1 exec bash "$FULL_GATE"
