#!/usr/bin/env bash
set -euo pipefail

exec bash "$(dirname "$0")/impl/k2_wide_hako_alloc_page_source_policy_exe_guard.sh" "$@"
