#!/bin/bash
# phase286_pattern9_legacy_pack.sh - accum_const_loop legacy pack (intentionally skipped)
# archived legacy pack stem: historical phase replay only, not part of current gate surface

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

test_skip "phase286_pattern9_legacy_pack" "phase286_pattern9_frag_poc uses plugins disabled path; provider expectations mismatch"
exit 0
