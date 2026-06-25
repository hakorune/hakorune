#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 tools/rust_lifecycle/extract_mirbuilder_allocation_policy_facts.py \
  --check-reference \
  --drift-probes

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-allocation-policy-facts-v0
mirbuilder_allocation_policy_facts=green
resolved_allocation_policy=green
directability_decision=Deny(UnsupportedDirectShape)
generated_hako_changed=0
backend_behavior_changed=0
runtime_fallback=0
summary=ok
REPORT
