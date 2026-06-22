#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/../.."

python3 tools/rust_lifecycle/mirbuilder_non_ascii_ordered_key_inventory.py --check-reference
