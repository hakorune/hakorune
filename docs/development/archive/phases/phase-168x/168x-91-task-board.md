# 168x-91: counter step_chain pure-first refresh task board

## Board

- [x] `168xA` docs lock
  - phase README
  - SSOT
  - root/current/workstream pointers
- [x] `168xB` direct contract refresh
  - refresh `Counter.step_chain/0` forwarding-shape expectation
- [x] `168xC` pure-first seed refresh
  - accept the current narrow forwarding body
  - keep leaf/body checks narrow
- [x] `168xD` exact verify
  - direct contract smoke
  - `bench_micro_aot_asm.sh kilo_micro_userbox_counter_step_chain 'ny_main' 1`
  - `bench_micro_c_vs_aot_stat.sh kilo_micro_userbox_counter_step_chain 1 1`
  - `tools/checks/dev_gate.sh quick`

## Notes

- this is a narrow backend exact-route contract refresh inside `phase-163x`
- `phase-167x` direct MIR determinism repair is already landed and is the upstream authority
- if the pure-first route still falls into generic walk after the forwarding-shape refresh, stop and reopen the seed body checks instead of widening generic walker support
