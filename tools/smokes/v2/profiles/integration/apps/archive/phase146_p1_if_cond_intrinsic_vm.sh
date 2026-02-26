#!/bin/bash
# Phase 146 P1: If condition with whitelisted intrinsic (VM)
set -euo pipefail

HAKORUNE="${HAKORUNE:-./target/release/hakorune}"
TEST_FILE="apps/tests/phase146_p1_if_cond_intrinsic_min.hako"

HAKO_ANF_DEV=1 HAKO_ANF_ALLOW_PURE=1 "$HAKORUNE" --backend vm "$TEST_FILE"
