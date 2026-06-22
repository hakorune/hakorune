#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 tools/rust_lifecycle/mirbuilder_metadata_context_deferred_inventory.py --check-reference

cat <<'REPORT'
output_contract=rust-mirbuilder-metadata-context-deferred-v0
metadata_context_deferred_recorded=1
subject=MetadataContext
consultation_only=1
route_selection=0
nightly_rustc_adapter=0
summary=ok
REPORT
