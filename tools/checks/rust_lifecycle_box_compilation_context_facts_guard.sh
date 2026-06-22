#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 tools/rust_lifecycle/extract_box_compilation_context_facts.py --check-reference

cat <<'REPORT'
output_contract=rustc-semir-box-compilation-context-facts-extraction-v0
box_compilation_context_facts_extraction_green=1
output_kind=RustLifecycleFacts
subject=BoxCompilationContext
lightweight_body_facts=1
nightly_rustc_adapter=0
backend_behavior_changed=0
summary=ok
REPORT
