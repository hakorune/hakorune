# vm_hako_caps smoke family

Capability matrix smokes for the `vm-hako` reference/conformance lane.

This family is not a product-mainline lane and not an engineering/bootstrap
default. Read it as the explicit reference bucket that keeps semantic witness
and conformance coverage visible.

Reference-lane acceptance is the phase29y gate only:

- `tools/smokes/v2/profiles/integration/vm_hako_caps/gate/phase29y_vm_hako_caps_gate_vm.sh`
- `tools/smokes/v2/suites/integration/vm-hako-caps.txt`

The phase29y gate is now a compatibility stub only. Active vm-hako shadow and
monitor rows live in `tools/smokes/v2/suites/integration/vm-hako-core.txt`.

The phase29y gate keeps per-wrapper timeouts explicit. Several vm-hako runtime
smokes use a 60s budget because each run recompiles and executes the child
driver, so a 30s default is too tight for the reference lane.

Non-gating blocked/probe cases are archived under `tools/smokes/v2/profiles/archive/**`
and do not reopen the lane by themselves.

This family was split out of `tools/smokes/v2/profiles/integration/apps/` so the
reference capability surface can be navigated by meaning instead of by a flat
prefix bucket.

## Layout

- `app1/`: retained APP-1 vm-hako parity witnesses; no longer suite-owned
- `args/`: retained seam witnesses after the narrow `args_vm` retirement
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

`open_handle_phi/open_handle_phi_ported_vm.sh` is a non-blocking shadow only
now. It remains in `tools/smokes/v2/suites/integration/vm-hako-core.txt`, but
it is no longer part of `vm-hako-caps.txt` or
`phase29y_vm_hako_caps_gate_vm.sh`.

`select_emit/select_emit_block_vm.sh` is no longer suite-owned. The non-vm-hako
owner is now
`tools/smokes/v2/profiles/integration/phase29y/hako/emit_mir/phase29y_hako_emit_mir_select_exec_contract_vm.sh`
via `tools/smokes/v2/suites/integration/phase29y-hako-emit-mir.txt` and
`tools/smokes/v2/suites/integration/selfhost-core.txt`.

`mapbox/` is not part of the phase29y vm-hako acceptance gate.
All 7 live `MapBox.*` owner rows now live in the dedicated non-vm_hako
emit+exec smokes under
`tools/smokes/v2/profiles/integration/phase29y/hako/emit_mir/`.
`tools/smokes/v2/suites/integration/collection-core.txt` no longer depends on
`collection_core/mapbox_*` or `vm_hako_caps/mapbox/*` directly.
`app1/` and `args/boxcall_args_gt1_ported_vm.sh` are no longer suite-owned.
The product owner for APP-1 summary behavior is now
`tools/smokes/v2/profiles/integration/apps/gate_log_summarizer_vm.sh` via
`tools/smokes/v2/suites/integration/presubmit.txt`.

## Suite

- `tools/smokes/v2/suites/integration/vm-hako-caps.txt`
- `tools/smokes/v2/suites/integration/vm-hako-core.txt`
- archive monitor buckets:
  - `tools/smokes/v2/profiles/archive/vm_hako_caps/**`
  - `tools/smokes/v2/profiles/archive/core/phase2160/registry_optin_method_arraymap_direct_canary_vm.sh`
