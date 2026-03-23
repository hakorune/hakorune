# vm_hako_caps smoke family

Capability matrix smokes for the vm-hako lane.

Active lane acceptance is the phase29y gate only:

- `tools/smokes/v2/profiles/integration/vm_hako_caps/gate/phase29y_vm_hako_caps_gate_vm.sh`
- `tools/smokes/v2/suites/integration/vm-hako-caps.txt`

Non-gating blocked/probe cases are archived under `tools/smokes/v2/profiles/archive/**` and do not reopen the lane by themselves.

This family was split out of `tools/smokes/v2/profiles/integration/apps/` so the active capability surface can be navigated by meaning instead of by a flat prefix bucket.

## Layout

- `app1/`: APP-1 summary and active post-open contract pins
- `args/`: `args` and `boxcall(args>1)` contract pins
- `compare/`: compare-op contract pins
- `env/`: environment routing contract pins
- `file/`: file/newbox/read/close/error contract pins
- `gate/`: the phase29y vm-hako capability gate
- `lib/`: shared helper layer for vm-hako capability smokes
- `mapbox/`: MapBox ported proofs still reused by collection-core suites; blocked pins were moved to archive
- `misc/`: small one-off capability pins such as `const(void)`
- `open_handle_phi/`: PHI/open-handle propagation pin
- `select_emit/`: MIR select emission blocker pin

## Suite

- `tools/smokes/v2/suites/integration/vm-hako-caps.txt`
- archive monitor buckets:
  - `tools/smokes/v2/profiles/archive/vm_hako_caps/**`
  - `tools/smokes/v2/profiles/archive/core/phase2160/registry_optin_method_arraymap_direct_canary_vm.sh`
