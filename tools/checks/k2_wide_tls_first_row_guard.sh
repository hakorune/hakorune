#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

TLS_CORE_FILE="lang/src/runtime/substrate/tls/tls_core_box.hako"
VM_SUBSET_FILE="src/runner/modes/vm_hako/subset_check/mod.rs"
VM_BOXCALL_FILE="lang/src/vm/boxes/mir_vm_s0_boxcall_builtin.hako"
VM_EXTERNCALL_FILE="lang/src/vm/boxes/mir_vm_s0_call_exec.hako"

echo "[k2-wide-tls-first-row] running narrow TLS first-row acceptance pack"
echo "[k2-wide-tls-first-row] --- vm-hako subset acceptance ---"
cargo test -q subset_accepts_externcall_hako_last_error -- --nocapture
cargo test -q subset_accepts_boxcall_tlscore_last_error_text_h -- --nocapture

echo "[k2-wide-tls-first-row] --- substrate/vm route lock ---"
rg -F -q 'last_error_text_h()' "$TLS_CORE_FILE"
rg -F -q 'externcall "hako_last_error"(0)' "$TLS_CORE_FILE"
rg -F -q 'externcall "nyash.box.from_i8_string"(raw)' "$TLS_CORE_FILE"
rg -F -q '[vm/adapter/tls:last_error_text_h]' "$TLS_CORE_FILE"
rg -F -q '&& box_type != "TlsCoreBox"' "$VM_SUBSET_FILE"
rg -F -q 'if func == "hako_last_error" || func == "hako_last_error/1"' "$VM_SUBSET_FILE"
rg -F -q 'if method == "last_error_text_h"' "$VM_BOXCALL_FILE"
rg -F -q 'if func == "hako_last_error" || func == "hako_last_error/1"' "$VM_EXTERNCALL_FILE"

echo "[k2-wide-tls-first-row] ok"
