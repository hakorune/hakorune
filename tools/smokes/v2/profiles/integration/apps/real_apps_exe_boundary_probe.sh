#!/bin/bash
# Real-app EXE unsupported-shape boundary probe
#
# Contract pin:
# - VM real-app suite is the executable correctness gate.
# - BoxTorrent mini moved to `boxtorrent_mini_exe.sh`; this probe should only
#   pin the next unsupported-shape boundary for the remaining apps.
# - binary-trees moved to `binary_trees_exe.sh`.
# - mimalloc-lite moved to `mimalloc_lite_exe.sh`.
# - allocator-stress moved to `allocator_stress_exe.sh`.
# - json-stream-aggregator moved to
#   `json_stream_aggregator_exe_runtime_boundary.sh`.
# - There are currently no remaining unsupported-shape app pins. Keep this
#   smoke as the suite-level marker so future blockers have one obvious place
#   to land.
# - Do not enable compat replay as mainline proof.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="real_apps_exe_boundary_probe"

echo "[INFO] no remaining real-app EXE unsupported-shape probe pins"

test_pass "$SMOKE_NAME"
