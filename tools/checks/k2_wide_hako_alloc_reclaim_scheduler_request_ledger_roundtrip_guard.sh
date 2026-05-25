#!/usr/bin/env bash
set -euo pipefail

exec bash "$(dirname "$0")/impl/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_roundtrip_guard.sh" "$@"
