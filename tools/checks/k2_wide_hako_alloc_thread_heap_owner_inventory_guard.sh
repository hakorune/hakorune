#!/usr/bin/env bash
set -euo pipefail

exec bash "$(dirname "$0")/impl/k2_wide_hako_alloc_thread_heap_owner_inventory_guard.sh" "$@"
