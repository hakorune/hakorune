#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 tools/rust_lifecycle/mirbuilder_mir_function_constructor_composition.py \
  --check-reference \
  --drift-probes

cat <<'REPORT'
output_contract=rust-lifecycle-mir-function-constructor-composition-v0
mir_function_constructor_composition=green
semantic_authority=MirFunction::new + BasicBlock::new
capability=MirFunctionConstructorTransport
prepared_state_install=green
separate_block_only_claim=0
function_body_lowering_claim=0
generated_hako_artifact=0
backend_behavior_changed=0
runtime_fallback=0
summary=ok
REPORT
