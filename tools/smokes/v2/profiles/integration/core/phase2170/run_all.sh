#!/bin/bash
set -euo pipefail

DIR="$(cd "$(dirname "$0")" && pwd)"

bash "$DIR/array_push_size_5_vm.sh"
bash "$DIR/array_push_size_10_vm.sh"
bash "$DIR/array_len_alias_vm.sh"
bash "$DIR/array_length_alias_vm.sh"
bash "$DIR/per_recv_global_canary_vm.sh"
bash "$DIR/per_recv_per_canary_vm.sh"
bash "$DIR/map_set_dup_key_size_canary_vm.sh"
bash "$DIR/map_value_state_get_has_canary_vm.sh"
bash "$DIR/flow_across_blocks_array_size_canary_vm.sh"

# hv1_mircall_* wrappers remain available as legacy hv1_inline proofs,
# but they are weaker duplicates of the stronger rc/flow canaries above and
# are no longer part of the default phase2170 pack.

# dup-key non-increment now enforced

echo "[PASS] phase2170 all"
