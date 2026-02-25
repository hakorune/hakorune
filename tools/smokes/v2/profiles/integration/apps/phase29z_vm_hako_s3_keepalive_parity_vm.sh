#!/bin/bash
# Phase29z-S3c: vm-hako S3 parity smoke (keepalive + const/add/ret on MIR JSON fixture)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
export VM_HAKO_PARITY_NAME="phase29z_vm_hako_s3_keepalive_parity_vm"
export VM_HAKO_PARITY_OPCODE="keepalive"
export VM_HAKO_PARITY_FIXTURE_REL="apps/tests/phase29z_vm_hako_s3_keepalive_const_add_return_mir_v0.json"
exec "$SCRIPT_DIR/lib/vm_hako_json_parity_common.sh" "$@"
