# Stage1 Bridge Route Exec

Scope: route execution helpers under `src/runner/stage1_bridge/route_exec/`.

## Sections

- `../route_exec.rs`: facade (`execute(...)`) only
- `direct.rs`: binary-only direct route execution, route-local logging, bridge direct-route exit-code mapping
- `stub.rs`: stage1 stub route facade (prepare + capture-vs-delegate only)
- `../stub_delegate.rs`: plain stage1 stub delegate-status execution, spawn-failure mapping, stub delegation log

## Contract

- exact execution plan selection stays in `../plan.rs::Stage1BridgePlan`
- `direct.rs` must not branch on route enums again
- stub capture-vs-delegate classification stays in `../args.rs::Stage1Args::stub_exec_plan()`
- `stub.rs` must not re-plan direct vs stub routes or re-infer emit-vs-run from raw `Stage1ArgsMode`
- stub prepare-failure mapping stays in `../stub_child.rs`
- plain stub delegate-status execution/log/child-spawn-failure mapping stay in `../stub_delegate.rs`

## Forbidden

- route planning here
- child env policy here
- direct-route compile/output policy here
- stub emit parse/writeback policy here
