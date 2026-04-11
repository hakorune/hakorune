# vm_hako_caps smoke family

Capability matrix smokes for the `vm-hako` reference/conformance lane.

This family is not a product-mainline lane and not an engineering/bootstrap
default. Read it as the explicit reference bucket that keeps semantic witness
and conformance coverage visible.

Reference-lane acceptance is the phase29y gate only:

- `tools/smokes/v2/profiles/integration/vm_hako_caps/gate/phase29y_vm_hako_caps_gate_vm.sh`
- `tools/smokes/v2/suites/integration/vm-hako-caps.txt`

The phase29y gate keeps per-wrapper timeouts explicit. Several vm-hako runtime
smokes use a 60s budget because each run recompiles and executes the child
driver, so a 30s default is too tight for the reference lane.

Non-gating blocked/probe cases are archived under `tools/smokes/v2/profiles/archive/**`
and do not reopen the lane by themselves.

This family was split out of `tools/smokes/v2/profiles/integration/apps/` so the
reference capability surface can be navigated by meaning instead of by a flat
prefix bucket.

## Layout

- `app1/`: APP-1 summary and active post-open contract pins
- `args/`: `args` and `boxcall(args>1)` contract pins
- `atomic/`: atomic fence contract pins
- `compare/`: compare-op contract pins
- `env/`: environment routing contract pins
- `file/`: file/newbox/read/close/error contract pins
- `gate/`: the phase29y vm-hako capability gate
- `lib/`: shared helper layer for vm-hako capability smokes
- `mapbox/`: MapBox ported proofs still reused by collection-core suites; blocked pins were moved to archive
- `misc/`: small one-off capability pins such as `const(void)`
- `open_handle_phi/`: PHI/open-handle propagation pin
- `select_emit/`: MIR select emission blocker pin
- `tls/`: TLS last-error contract pins

`env/env_get_ported_vm.sh` is the explicit vm-hako monitor canary now. Product
ownership moved to `tools/smokes/v2/profiles/integration/core/phase2035/`
through `tools/smokes/v2/suites/integration/presubmit.txt`, while the vm-hako
row remains in `tools/smokes/v2/suites/integration/vm-hako-core.txt`.

`file/file_read_ported_vm.sh` and `file/file_close_ported_vm.sh` are also
monitor-only now; the product-facing anchors are the PLG-07 FileBox scripts,
and `file_error_vm.sh` is no longer part of the phase29y vm-hako acceptance
gate.

`mapbox/` is not part of the phase29y vm-hako acceptance gate.
`tools/smokes/v2/suites/integration/collection-core.txt` now points at
`tools/smokes/v2/profiles/integration/collection_core/mapbox_*` wrappers, so
the `collection-core` suite no longer depends on `vm_hako_caps/mapbox/*`
directly.
`app1/app1_summary_contract_ported_vm.sh` also remains referenced by
`tools/smokes/v2/suites/integration/presubmit.txt`, so `app1/` is a late
demotion family rather than an early archive target.

## Suite

- `tools/smokes/v2/suites/integration/vm-hako-caps.txt`
- `tools/smokes/v2/suites/integration/vm-hako-core.txt`
- archive monitor buckets:
  - `tools/smokes/v2/profiles/archive/vm_hako_caps/**`
  - `tools/smokes/v2/profiles/archive/core/phase2160/registry_optin_method_arraymap_direct_canary_vm.sh`
