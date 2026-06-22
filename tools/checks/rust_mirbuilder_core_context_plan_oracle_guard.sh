#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/../.."

python3 tools/rust_lifecycle/mirbuilder_core_context_plan_oracle_inventory.py --check-reference
