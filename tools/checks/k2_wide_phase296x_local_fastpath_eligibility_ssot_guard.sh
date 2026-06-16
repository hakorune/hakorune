#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="local-fastpath-eligibility-ssot"
CARD="docs/development/current/main/phases/phase-296x/296x-895-LOCAL-FASTPATH-ELIGIBILITY-SSOT-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-894-LOCAL-I64-MAP-GET-PILOT-CLOSEOUT-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_fastpath_eligibility_ssot_guard.sh"

for file in "$CARD" "$PREV_CARD" "$INDEX"; do
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
  "output_contract=hako-local-fastpath-eligibility-ssot-v0" \
  "source_evidence=296x-894" \
  "row_kind=design" \
  "fastpath_model=observation_to_eligibility_to_fact_to_backend" \
  "local_fastpath_fact_backend_consumable=1" \
  "fallback_evidence_backend_consumable=0" \
  "fallback_fact_enabled=0" \
  "unknown_state_policy=fallback" \
  "maybe_published_policy=fallback" \
  "local_fastpath_fact_requires_unpublished=1" \
  "local_fastpath_fact_requires_alias_class=1" \
  "local_fastpath_fact_requires_routeplan=1" \
  "local_fastpath_fact_requires_objectstorageplan=1" \
  "local_fastpath_fact_requires_backend_support=1" \
  "backend_reads_local_fastpath_fact_only=1" \
  "backend_reads_fallback_evidence=0" \
  "backend_reads_helper_symbol=0" \
  "backend_reads_source_variable_name=0" \
  "full_escape_engine_required_for_v0=0" \
  "interprocedural_fixedpoint_required_for_v0=0" \
  "next_task=LOCAL-PUBLICATION-CLASSIFIER-000" \
  "summary=ok"; do
  require_card_line "$expected"
done

grep -F -q "next_task=LOCAL-FASTPATH-ELIGIBILITY-SSOT-001" "$PREV_CARD" || {
  echo "[$TAG] closeout row does not hand off to eligibility SSOT" >&2
  exit 1
}

for text in \
  "The backend may consume only a positive \`LocalFastPathFact\`." \
  "can_fast_path(site, object)" \
  "closed_world_region(site)" \
  "publication_state_before(site, alias_class) == Unpublished" \
  "Allowed:" \
  "Deferred:" \
  "Backend consumers may read:" \
  "Backend consumers must not read:" \
  "LOCAL-PUBLICATION-CLASSIFIER-000" \
  "do not make fallback evidence backend-readable"; do
  grep -F -q "$text" "$CARD" || {
    echo "[$TAG] missing SSOT text: $text" >&2
    exit 1
  }
done

echo "[$TAG] ok"
