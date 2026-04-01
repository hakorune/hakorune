#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"

echo "[phase2120/compat] vm-adapter legacy cluster"

ADAPTER_CANARIES=(
  'core/phase2120/s3_vm_adapter_array_len_canary_vm.sh'
  'core/phase2120/s3_vm_adapter_array_length_alias_canary_vm.sh'
  'core/phase2120/s3_vm_adapter_array_size_alias_canary_vm.sh'
  'core/phase2120/s3_vm_adapter_array_len_per_recv_canary_vm.sh'
  'core/phase2120/s3_vm_adapter_map_size_struct_canary_vm.sh'
  'core/phase2120/s3_vm_adapter_register_userbox_length_canary_vm.sh'
  'core/phase2120/s3_vm_adapter_map_len_alias_state_canary_vm.sh'
  'core/phase2120/s3_vm_adapter_map_length_alias_state_canary_vm.sh'
)

CHECK_FILE="/tmp/hako_inline_using_check_$$.hako"
trap 'rm -f "$CHECK_FILE" || true' EXIT
cat > "$CHECK_FILE" <<'HCODE'
using "selfhost.vm.helpers.mir_call_v1_handler" as MirCallV1HandlerBox
static box Main { method main(args) { return 0 } }
HCODE

set +e
NYASH_DISABLE_NY_COMPILER=1 HAKO_DISABLE_NY_COMPILER=1 NYASH_FEATURES=stage3 \
  NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 "$ROOT/target/release/hakorune" --backend vm "$CHECK_FILE" >/dev/null 2>&1
USING_OK=$?
set -e

if [ "$USING_OK" -ne 0 ]; then
  echo "[phase2120] SKIP adapter reps (inline using unsupported)" >&2
  exit 0
fi

for filter in "${ADAPTER_CANARIES[@]}"; do
  bash "$ROOT/tools/smokes/v2/run.sh" --profile integration --filter "$filter"
done

echo "[phase2120] vm-adapter legacy cluster done."
