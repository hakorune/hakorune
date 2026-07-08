#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
exec python3 "$ROOT/tools/rust_lifecycle/current_active_rust_lifecycle_guard_resolver.py" --run
