# vm_hako_caps smoke family

Capability matrix smokes for the vm-hako lane.

This family was split out of `tools/smokes/v2/profiles/integration/apps/` so the active capability surface can be navigated by meaning instead of by a flat prefix bucket.

## Layout

- `app1/`: APP-1 summary and post-open stack overflow contract pins
- `args/`: `args` and `boxcall(args>1)` contract pins
- `compare/`: compare-op contract pins
- `env/`: environment routing contract pins
- `file/`: file/newbox/read/close/error contract pins
- `gate/`: the phase29y vm-hako capability gate
- `lib/`: shared helper layer for vm-hako capability smokes
- `mapbox/`: MapBox capability and blocker pins
- `misc/`: small one-off capability pins such as `const(void)`
- `open_handle_phi/`: PHI/open-handle propagation pin
- `select_emit/`: MIR select emission blocker pin

## Suite

- `tools/smokes/v2/suites/integration/vm-hako-caps.txt`
