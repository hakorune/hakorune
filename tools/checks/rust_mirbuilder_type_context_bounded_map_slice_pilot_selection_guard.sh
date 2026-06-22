#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/../.."

python3 tools/rust_lifecycle/mirbuilder_type_context_bounded_map_slice_pilot_selection.py --check-reference
