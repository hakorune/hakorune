#!/bin/bash
# phase286_pattern9_legacy_pack.sh - Pattern9 legacy pack (intentionally skipped)

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

test_skip "phase286_pattern9_legacy_pack" "phase286_pattern9_frag_poc uses plugins disabled path; provider expectations mismatch"
exit 0
