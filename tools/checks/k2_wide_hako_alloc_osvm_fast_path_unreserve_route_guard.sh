#!/usr/bin/env bash
set -euo pipefail

exec bash "$(dirname "$0")/impl/k2_wide_hako_alloc_osvm_fast_path_unreserve_route_guard.sh" "$@"
