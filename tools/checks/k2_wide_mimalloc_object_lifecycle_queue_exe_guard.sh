#!/usr/bin/env bash
set -euo pipefail

exec bash "$(dirname "$0")/impl/k2_wide_mimalloc_object_lifecycle_queue_exe_guard.sh" "$@"
