#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 tools/rust_lifecycle/mirbuilder_mir_module_minimal_shell_transport.py \
  --check-reference \
  --drift-probes

cat <<'REPORT'
output_contract=rust-lifecycle-mir-module-minimal-shell-transport-v0
mir_module_minimal_shell_transport=green
semantic_authority=MirModule::new
capability=MirModuleMinimalShellTransport
source_file_assignment_claim=0
function_insertion_claim=0
metadata_publication_claim=0
generated_hako_artifact=0
backend_behavior_changed=0
runtime_fallback=0
summary=ok
REPORT
