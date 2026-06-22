#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/../.."

python3 tools/rust_lifecycle/mirbuilder_phi_and_join_id_lifecycle_inventory.py --check-reference
