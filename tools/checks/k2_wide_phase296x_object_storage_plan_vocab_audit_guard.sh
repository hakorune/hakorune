#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="object-storage-plan-vocab-audit"
CARD="docs/development/current/main/phases/phase-296x/296x-1055-REASON-DOMAIN-REPORT-VOCABULARY-001.md"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/hako_check/object_storage_plan_vocab_audit.py"
TEST="tools/hako_check/tests/test_object_storage_plan_vocab_audit.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_object_storage_plan_vocab_audit_guard.sh"

for file in "$CARD" "$INDEX" "$TOOL" "$TEST"; do
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
  "output_contract=hako-object-storage-plan-vocab-audit-v0" \
  "source_evidence=296x-1054" \
  "row_kind=inventory" \
  "keep_separate_count=6" \
  "merge_candidate_count=2" \
  "immediate_merge_allowed=0" \
  "vocabulary_merge_count=0" \
  "fact_fallback_separation_preserved=1" \
  "public_api_reexport_preserved=1" \
  "guard_path_compat_landed=1" \
  "reason_enum_merge_enabled=0" \
  "reason_domain_report_enabled=1" \
  "reason_domain_count=3" \
  "first_safe_followup=OBJECT-SITE-LOCATION-VOCABULARY-DESIGN-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

python3 -m unittest tools.hako_check.tests.test_object_storage_plan_vocab_audit >/tmp/"$TAG".unittest.out
python3 "$TOOL" >/tmp/"$TAG".kv

for expected in \
  "output_contract=hako-object-storage-plan-vocab-audit-v0" \
  "keep_separate_count=6" \
  "merge_candidate_count=2" \
  "immediate_merge_allowed=0" \
  "vocabulary_merge_count=0" \
  "reason_enum_merge_enabled=0" \
  "reason_domain_report_enabled=1" \
  "reason_domain_count=3" \
  "first_safe_followup=OBJECT-SITE-LOCATION-VOCABULARY-DESIGN-001" \
  "row_3_name=local_fastpath_fact" \
  "row_3_action=keep" \
  "row_6_name=site_location_fields" \
  "row_6_action=defer"; do
  grep -F -x -q "$expected" /tmp/"$TAG".kv || {
    echo "[$TAG] missing audit output: $expected" >&2
    exit 1
  }
done

echo "[$TAG] ok"
