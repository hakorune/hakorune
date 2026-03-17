# Phase 2170 Current Pack

このディレクトリは runtime-zero / collection adapter canary の current pack だよ。

## Current Pack

- `array_push_size_5_vm.sh`
- `array_push_size_10_vm.sh`
- `array_len_alias_vm.sh`
- `array_length_alias_vm.sh`
- `per_recv_global_canary_vm.sh`
- `per_recv_per_canary_vm.sh`
- `map_set_dup_key_size_canary_vm.sh`
- `map_value_state_get_has_canary_vm.sh`
- `flow_across_blocks_array_size_canary_vm.sh`

公式 entry は [run_all.sh](/home/tomoaki/git/hakorune-selfhost/tools/smokes/v2/profiles/integration/core/phase2170/run_all.sh)。

## Weaker Duplicates

- `hv1_mircall_array_push_size_state_canary_vm.sh`
- `hv1_mircall_map_set_size_state_canary_vm.sh`

これらは Rust `hv1_inline` 側の legacy proof で、current `.hako` owner proof ではないよ。default pack には含めない。

## Role

- current owner proof の本体は `phase29cc_runtime_v0_adapter_fixtures_vm.sh`
- phase2170 はその補助として、array/map adapter behavior の rc/flow canary を束ねる current pack
- historical compat pack は phase2120 に分けて管理する

## Non-goals

- `.hako` collection owner の source-contract lock を置き換えること
- backend-zero proof pack を兼ねること
- `hv1_inline` historical proofs を default pack に戻すこと
