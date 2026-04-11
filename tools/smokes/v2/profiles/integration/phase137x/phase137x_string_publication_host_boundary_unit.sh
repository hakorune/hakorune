#!/bin/bash
# phase-137x focused guard for the same-block publication host-boundary sink

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase137x_string_publication_host_boundary_unit"

cd "$NYASH_ROOT"

if cargo test -q --lib \
    mir::passes::string_corridor_sink::tests::sinks_publication_helper_to_same_block_runtime_data_set_boundary \
    -- --exact --nocapture; then
    test_pass "$SMOKE_NAME: PASS (same-block publication host-boundary sink stays green)"
else
    test_fail "$SMOKE_NAME: targeted string publication host-boundary unit contract failed"
    exit 1
fi
