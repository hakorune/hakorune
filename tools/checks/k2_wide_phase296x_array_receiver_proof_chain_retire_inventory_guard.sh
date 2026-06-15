#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-835-ARRAY-RECEIVER-RESIDENCE-PROOF-CHAIN-RETIRE-INVENTORY-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-834-BACKEND-METHOD-NAME-PROOF-AUDIT-001.md"
LEGACY_MODULE="src/array_receiver_representation_source.rs"
LIVE_MODULE="src/mir/array_receiver_proof.rs"
LIB="src/lib.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_array_receiver_proof_chain_retire_inventory_guard.sh"

[[ -f "$CARD" ]] || { echo "[array-receiver-proof-chain-retire-inventory] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[array-receiver-proof-chain-retire-inventory] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$LEGACY_MODULE" ]] || { echo "[array-receiver-proof-chain-retire-inventory] legacy module must remain before retire gate: $LEGACY_MODULE" >&2; exit 1; }
[[ -f "$LIVE_MODULE" ]] || { echo "[array-receiver-proof-chain-retire-inventory] live proof module missing: $LIVE_MODULE" >&2; exit 1; }
[[ -f "$LIB" ]] || { echo "[array-receiver-proof-chain-retire-inventory] missing lib.rs" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || {
  echo "[array-receiver-proof-chain-retire-inventory] card must be Landed" >&2
  exit 1
}
grep -q '^Status: Landed$' "$PREV_CARD" || {
  echo "[array-receiver-proof-chain-retire-inventory] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[array-receiver-proof-chain-retire-inventory] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[array-receiver-proof-chain-retire-inventory] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-array-receiver-proof-chain-retire-inventory-v0" \
  "source_evidence=296x-833,296x-834" \
  "array_receiver_representation_source_consumers_classified=1" \
  "array_receiver_representation_source_line_count=719" \
  "array_receiver_representation_source_src_reference_file_count=2" \
  "array_receiver_representation_source_tools_reference_file_count=14" \
  "array_receiver_representation_source_docs_reference_file_count=18" \
  "legacy_residence_vocabulary_total_reference_count=565" \
  "legacy_residence_vocabulary_tools_file_count=31" \
  "legacy_residence_vocabulary_docs_file_count=36" \
  "legacy_residence_vocabulary_src_file_count=2" \
  "live_array_receiver_proof_module=src/mir/array_receiver_proof.rs" \
  "live_array_receiver_proof_line_count=148" \
  "live_array_receiver_proof_must_keep=1" \
  "retire_gate_required=1" \
  "implementation_started=0" \
  "code_deleted=0" \
  "product_default_changed=0" \
  "selected_next=MIMALLOC-FRESH-FRONT-SELECTION-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

[[ "$(wc -l < "$LEGACY_MODULE" | tr -d ' ')" == "719" ]] || {
  echo "[array-receiver-proof-chain-retire-inventory] legacy module line count drifted" >&2
  exit 1
}
[[ "$(wc -l < "$LIVE_MODULE" | tr -d ' ')" == "148" ]] || {
  echo "[array-receiver-proof-chain-retire-inventory] live module line count drifted" >&2
  exit 1
}
grep -F -q "pub mod array_receiver_representation_source;" "$LIB" || {
  echo "[array-receiver-proof-chain-retire-inventory] legacy export must remain before retire gate" >&2
  exit 1
}

for token in \
  "ArrayReceiverRepresentationSource" \
  "ArrayReceiverResidenceInputSource" \
  "ArrayReceiverResidenceProofChain" \
  "array_receiver_representation_source_report_fields"; do
  grep -F -q "$token" "$LEGACY_MODULE" || {
    echo "[array-receiver-proof-chain-retire-inventory] missing legacy vocabulary token: $token" >&2
    exit 1
  }
done

for token in \
  "do not delete src/array_receiver_representation_source.rs in this inventory row" \
  "do not touch live src/mir/array_receiver_proof.rs" \
  "do not remove src/lib.rs export before retire gate exists" \
  "ARRAY-RECEIVER-PROOF-CHAIN-RETIRE-GATE-001"; do
  grep -F -q "$token" "$CARD" || {
    echo "[array-receiver-proof-chain-retire-inventory] missing retire stop/task token: $token" >&2
    exit 1
  }
done

echo "[array-receiver-proof-chain-retire-inventory] ok"
