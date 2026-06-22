#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/../.."

python3 tools/rust_lifecycle/mirbuilder_unsafe_or_ffi_inventory.py --check-reference
