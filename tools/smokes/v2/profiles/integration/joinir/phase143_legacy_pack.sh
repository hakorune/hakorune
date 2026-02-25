#!/bin/bash
# phase143_legacy_pack.sh - Phase 143 legacy pack (intentionally skipped)

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

test_skip "phase143_legacy_pack" "Phase143 loop(true) legacy pack is excluded from JoinIR regression SSOT (LoopBuilder removed, plugin-disabled scripts, LLVM exe expectations outdated)"
exit 0
