#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="local-publication-classifier"
CARD="docs/development/current/main/phases/phase-296x/296x-896-LOCAL-PUBLICATION-CLASSIFIER-000.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-895-LOCAL-FASTPATH-ELIGIBILITY-SSOT-001.md"
CODE="src/object_storage_plan.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_publication_classifier_guard.sh"

for file in "$CARD" "$PREV_CARD" "$CODE" "$INDEX"; do
  [[ -f "$file" ]] || { echo "[$TAG] missing file: $file" >&2; exit 1; }
done

grep -q '^Status: Landed$' "$CARD" || {
  echo "[$TAG] card must be Landed" >&2
  exit 1
}

grep -F -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[$TAG] check index missing guard entry" >&2
  exit 1
}

require_card_line() {
  local expected="$1"
  if ! grep -F -x -q "$expected" "$CARD"; then
    echo "[$TAG] missing card line: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-local-publication-classifier-v0" \
  "source_evidence=296x-895" \
  "row_kind=passive_vocabulary" \
  "publication_state_vocabulary_defined=1" \
  "publication_state_unpublished_fastpath_allowed=1" \
  "publication_state_published_fastpath_allowed=0" \
  "publication_state_maybe_published_fastpath_allowed=0" \
  "local_fastpath_fallback_reason_vocabulary_defined=1" \
  "local_fastpath_fact_vocabulary_defined=1" \
  "local_fastpath_fact_backend_consumable=1" \
  "fallback_evidence_backend_consumable=0" \
  "fallback_fact_enabled=0" \
  "full_escape_engine_required_for_v0=0" \
  "interprocedural_fixedpoint_required_for_v0=0" \
  "object_storage_plan_execution_enabled=0" \
  "object_plan_execution_enabled=0" \
  "backend_new_lowering_enabled=0" \
  "next_task=LOCAL-ALIAS-CLASS-MVP-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

grep -F -q "next_task=LOCAL-PUBLICATION-CLASSIFIER-000" "$PREV_CARD" || {
  echo "[$TAG] eligibility row does not hand off to publication classifier" >&2
  exit 1
}

for code_text in \
  "pub enum PublicationState" \
  "Unpublished" \
  "Published" \
  "MaybePublished" \
  "pub enum LocalFastPathFallbackReason" \
  "PublishedBeforeSite" \
  "MaybePublishedBeforeSite" \
  "pub struct LocalFastPathFact" \
  "pub fn permits_local_fast_path" \
  "pub fn fallback_reason" \
  "pub fn known_receiver_direct_call" \
  "(\"local_fastpath_fact_backend_consumable\", \"1\")" \
  "(\"fallback_fact_enabled\", \"0\")"; do
  grep -R -F -q "$code_text" src/object_storage_plan.rs src/object_storage_plan || {
    echo "[$TAG] missing code evidence: $code_text" >&2
    exit 1
  }
done

for text in \
  "No lowering or backend execution is enabled by this row." \
  "\`Unpublished\` is the only publication state that permits a local fast path." \
  "\`Published\` and \`MaybePublished\` produce fallback reasons." \
  "no fallback facts"; do
  grep -F -q "$text" "$CARD" || {
    echo "[$TAG] missing card text: $text" >&2
    exit 1
  }
done

echo "[$TAG] ok"
