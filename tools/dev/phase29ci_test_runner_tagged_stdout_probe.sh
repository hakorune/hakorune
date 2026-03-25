#!/usr/bin/env bash
set -euo pipefail

ROOT="${NYASH_ROOT:-$(cd "$(dirname "$0")/../.." && pwd)}"
export NYASH_ROOT="$ROOT"

source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

TMP_GOOD="$(mktemp --suffix .phase29ci.tagged_stdout.good)"
TMP_REPAIR="$(mktemp --suffix .phase29ci.tagged_stdout.repair)"
PROG_JSON='{"version":0,"kind":"Program","body":[]}'
TAG_PATTERN_BASIC='\[mirbuilder/min:return.binop.intint\]'
TAG_PATTERN_EXTENDED='\[mirbuilder/(min|registry):return.binop.intint\]'

cleanup() {
  rm -f "$TMP_GOOD" "$TMP_REPAIR" 2>/dev/null || true
}
trap cleanup EXIT

cat > "$TMP_GOOD" <<'EOF'
[mirbuilder/min:return.binop.intint]
[MIR_BEGIN]
{"functions":[{"name":"main","blocks":[{"id":0,"instructions":[{"op":"ret","value":0}]}]}]}
[MIR_END]
EOF

if ! ensure_phase2160_tagged_stdout_contract \
  0 \
  basic \
  "$TAG_PATTERN_BASIC" \
  "$PROG_JSON" \
  builder \
  "$TMP_GOOD" \
  1; then
  echo "[phase29ci/probe] expected valid tagged stdout to remain accepted" >&2
  exit 1
fi

if ! stdout_file_matches_tagged_mir_contract basic "$TAG_PATTERN_BASIC" "$TMP_GOOD" 1; then
  echo "[phase29ci/probe] valid tagged stdout no longer matches contract" >&2
  exit 1
fi

printf 'noise only\n' > "$TMP_REPAIR"
if ensure_phase2160_tagged_stdout_contract \
  1 \
  extended \
  "$TAG_PATTERN_EXTENDED" \
  "$PROG_JSON" \
  registry \
  "$TMP_REPAIR" \
  1; then
  REPAIR_RC=0
else
  REPAIR_RC=$?
fi

if [ "$REPAIR_RC" -ne 1 ]; then
  echo "[phase29ci/probe] expected repair path rc=1, got $REPAIR_RC" >&2
  exit 1
fi

if ! stdout_file_matches_tagged_mir_contract extended "$TAG_PATTERN_EXTENDED" "$TMP_REPAIR" 1; then
  echo "[phase29ci/probe] repaired tagged stdout is malformed" >&2
  exit 1
fi

REGISTRY_TAG="$(normalize_phase2160_tag_pattern "$TAG_PATTERN_EXTENDED" registry)"
if [ "$REGISTRY_TAG" != "[mirbuilder/registry:return.binop.intint]" ]; then
  echo "[phase29ci/probe] registry tag normalization drifted: $REGISTRY_TAG" >&2
  exit 1
fi

echo "[phase29ci/probe] PASS"
