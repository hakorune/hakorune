# 169x-91: substring concat stable-length phi task board

## Board

- [x] `169xA` docs lock
  - phase README
  - SSOT
  - root/current/workstream pointers
- [x] `169xB` string relation metadata
  - add `stable_length_scalar`
  - emit/print the witness payload
  - pin unit coverage on the current loop-carried route
- [x] `169xC` sink widening
  - consume the new relation in complementary substring-length fusion
  - collapse the current `substring_len_hii + const + substring_len_hii` loop path
- [x] `169xD` exact-contract refresh
  - refresh direct/post-sink smoke
  - refresh guard fixture
  - refresh pure-first exact seed
- [x] `169xE` verify
  - focused cargo tests
  - `phase137x_direct_emit_substring_concat_post_sink_shape.sh`
  - `phase137x_direct_emit_substring_concat_phi_merge_contract.sh`
  - `phase29x_backend_owner_daily_substring_concat_loop_min.sh`
  - `bench_micro_aot_asm.sh kilo_micro_substring_concat 'ny_main' 1`
  - `bench_micro_c_vs_aot_stat.sh kilo_micro_substring_concat 1 3`
  - `tools/checks/dev_gate.sh quick`

## Notes

- this is the next string metadata-contract phase after the landed `phase-137x` stop-line
- the current exact front is still `kilo_micro_substring_concat`
- if the stable-length witness cannot stay narrow and benchmark-local, stop and reopen the contract instead of widening generic merged-window carry
