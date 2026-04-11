#!/bin/bash
# phase-137x focused guard for the same-block publication-return sink

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase137x_string_publication_return_unit"

cd "$NYASH_ROOT"

if cargo test -q --lib \
    mir::passes::string_corridor_sink::tests::sinks_publication_helper_to_same_block_return_boundary \
    -- --exact --nocapture; then
    test_pass "$SMOKE_NAME: PASS (same-block publication-return sink stays green)"
else
    test_fail "$SMOKE_NAME: targeted string publication-return unit contract failed"
    exit 1
fi
