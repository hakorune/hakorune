#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
exec bash "$ROOT_DIR/tools/checks/impl/k2_wide_allocator_provider_activation_safety_cli_surface_guard.sh" "$@"
