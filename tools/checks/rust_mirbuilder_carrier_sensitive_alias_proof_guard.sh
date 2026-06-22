#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/../.."

python3 tools/rust_lifecycle/mirbuilder_carrier_sensitive_alias_proof_inventory.py --check-reference
