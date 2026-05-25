#!/usr/bin/env bash
set -euo pipefail

exec bash "$(dirname "$0")/impl/k2_wide_hako_alloc_segment_allocation_readiness_scalar_guard.sh" "$@"
