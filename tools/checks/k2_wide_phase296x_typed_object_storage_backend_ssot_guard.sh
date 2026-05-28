#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-190-TYPED-OBJECT-STORAGE-BACKEND-SSOT.md"

grep -q '^Decision: accepted$' "$DOC"
grep -q '^Default typed-object storage remains SafeMutexStore\.$' "$DOC"
grep -q '^SingleThreadExactStore is a diagnostic/exact-EXE fast lane only\.$' "$DOC"
grep -q '^The exported typed-object ABI stays unchanged\.$' "$DOC"
grep -q '^storage_backend_ssot=accepted$' "$DOC"
grep -q '^default_backend=SafeMutexStore$' "$DOC"
grep -q '^selected_fast_lane_backend=SingleThreadExactStore$' "$DOC"
grep -q '^exported_abi_unchanged=1$' "$DOC"
grep -q '^exact_exe_gate_required=1$' "$DOC"
grep -q '^silent_fallback_allowed=0$' "$DOC"
grep -q '^summary=ok$' "$DOC"

echo "typed_object_storage_backend_ssot_guard=ok"
