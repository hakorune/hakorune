#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="trivial-canonical-analyzer-mode-guard"
PROFILE="$ROOT_DIR/src/mir/resolved_value_profile"
ANALYZER="$PROFILE/analyzer.rs"
MODE="$PROFILE/analyzer_mode.rs"
FACADE="$PROFILE/mod.rs"
CAPABILITY="$ROOT_DIR/src/mir/compiler/capability.rs"

fail() {
  echo "[$TAG] $*" >&2
  exit 1
}

require() {
  local file="$1"
  local token="$2"
  rg -F -q -- "$token" "$file" || fail "missing $token in ${file#$ROOT_DIR/}"
}

[[ -f "$MODE" && -f "$ANALYZER" && -f "$FACADE" && -f "$CAPABILITY" ]] ||
  fail "analyzer mode owner files are missing"

require "$MODE" "pub(crate) enum TrivialCanonicalAnalysisModeV1"
for variant in \
  "OrdinaryClosed" \
  "OrdinaryFiniteDirectCalls" \
  "NormalMainClosed" \
  "NormalMainFiniteDirectCalls"; do
  require "$MODE" "$variant"
  require "$ANALYZER" "TrivialCanonicalAnalysisModeV1::$variant"
done

[[ "$(rg -c '^pub\(crate\) fn analyze_trivial_canonical_with_mode_v1\(' "$FACADE")" == 1 ]] ||
  fail "canonical mode entry definition count is not one"
[[ "$(rg -c '^pub\(super\) fn analyze_trivial_canonical_with_mode_impl_v1\(' "$ANALYZER")" == 1 ]] ||
  fail "analyzer mode implementation count is not one"

if rg -n 'analyze_trivial_canonical_(owner|main_owner)' \
  "$PROFILE" "$CAPABILITY"; then
  fail "retired policy wrapper remains in production/test owner"
fi

for variant in \
  "OrdinaryClosed" \
  "OrdinaryFiniteDirectCalls" \
  "NormalMainClosed" \
  "NormalMainFiniteDirectCalls"; do
  require "$CAPABILITY" "TrivialCanonicalAnalysisModeV1::$variant"
done

for file in "$ANALYZER" "$MODE" "$FACADE" "$CAPABILITY" "$0"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  (( lines < 800 )) || fail "800-line hard stop reached: ${file#$ROOT_DIR/} ($lines)"
done
analyzer_lines="$(wc -l < "$ANALYZER" | tr -d '[:space:]')"
(( analyzer_lines < 760 )) || fail "analyzer requires a responsibility split: $analyzer_lines"

echo "[$TAG] ok"
