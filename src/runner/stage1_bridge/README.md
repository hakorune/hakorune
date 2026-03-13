# Stage1 Bridge

Scope: Rust-side Stage-1 bridge glue in `src/runner/stage1_bridge/`.

## Status

- future retire target
- not current MIR-direct authority
- keep bridge logic thin and explicit

## Program JSON Rule

- Stage1 bridge mode classification is fixed in `args.rs::Stage1ArgsMode`
- backend CLI hint extraction is fixed in `args.rs::Stage1Args::backend_cli_hint()`
- bridge entry child/enable guard + trace logging live in `entry_guard.rs`
- stub capture-vs-delegate contract is fixed in `args.rs::Stage1Args::stub_exec_plan()`
- exact execution plan selection is fixed in `plan.rs::Stage1BridgePlan`
- `route_exec.rs` is a thin route-to-executor facade
- binary-only direct route execution and direct-route exit-code mapping live in `route_exec/direct.rs`
- Stage1 stub route facade lives in `route_exec/stub.rs`
- binary-only direct route facade lives in `direct_route/mod.rs`
- binary-only direct-route MIR compile lives in `direct_route/compile.rs`
- binary-only direct-route MIR output-path policy and JSON write live in `direct_route/emit.rs`
- bridge-local emit output-path resolution lives in `emit_paths.rs`
- bridge-local Program(JSON v0) entry cluster lives in `program_json_entry/`
- `program_json_entry/mod.rs` is now a thin facade for the bridge-local `emit-program-json-v0` route and owns the exact success/error process-exit formatting
- `program_json_entry/request.rs` owns the bridge-entry request predicate used by `runner/mod.rs` for `skip_stage1_stub`, source-path precedence (`stage1::input_path()` aliases first, CLI input fallback second), and exact out-path extraction from the explicit CLI flag
- outer callers should use the `program_json_entry` module helpers directly; this contract is no longer rebound as `NyashRunner` methods
- `emit_program_json_v0(...)` must use `stage1::program_json_v0::emit_program_json_v0_for_stage1_bridge_emit_program_json(...)`
- Stage1 stub entry resolution + child command/env assembly + prepare-failure mapping live in `stub_child.rs`
- Stage1 stub plain delegate-status execution + child-spawn-failure mapping live in `stub_delegate.rs`
- stub emit facade lives in `stub_emit.rs`
- stub emit stdout parse / validation live in `stub_emit/parse.rs`
- stub emit writeback policy lives in `stub_emit/writeback.rs`
- child env policy stays behind `env.rs` and `env/README.md`
- runtime defaults live in `env/runtime_defaults.rs`
- Stage-1 alias propagation lives in `env/stage1_aliases.rs`
- parser / using toggle propagation lives in `env/parser_stageb.rs`
- Stage-B module payload generation + child-env apply live in `modules.rs`
- bridge-local file read/write for this route lives in `program_json/mod.rs`
- `program_json/mod.rs` is a thin facade; bridge-local read->emit->write orchestration and owner-1 payload emission live in `program_json/pipeline.rs`, source-text read lives in `program_json/read_input.rs`, and bridge-local writeback policy lives in `program_json/writeback.rs`
- next Rust-only retire slices stay inside `program_json_entry/` and `program_json/`; treat `src/runner/mod.rs` and `src/runner/emit.rs` as `must-stay thin callers`
- do not call `source_to_program_json_v0_strict(...)` from this directory
- do not add new bridge-local Program(JSON v0) parsing policy here

## Allowed Responsibilities

- route planning for stub/direct bridge behavior
- bridge entry guard delegated out of `mod.rs`
- stub capture-vs-delegate classification delegated out of `route_exec/stub.rs`
- exact execution plan selection delegated out of `route_exec.rs`
- route execution facade delegated out of `mod.rs`
- bridge-local Program(JSON v0) entry delegated out of `mod.rs`
- bridge-local Program(JSON v0) branch selection and success/error formatting delegated out of `src/runner/emit.rs`
- bridge-local Program(JSON v0) success/error process-exit delegated out of `src/runner/emit.rs`
- direct-route compile / emit policy delegated out of `direct_route/mod.rs`
- emit output-path policy delegated out of `stub_emit.rs` and `direct_route/emit.rs`
- stub plain delegate-status execution delegated out of `route_exec/stub.rs`
- stub emit parse / writeback delegated out of `stub_emit.rs`
- child-process / embedded-entry orchestration
- file read/write around bridge-specific CLI surfaces
- thin delegate modules such as `entry_guard.rs` / `route_exec.rs` / `route_exec/*` / `direct_route/*` / `emit_paths.rs` / `stub_child.rs` / `stub_delegate.rs` / `program_json_entry/` / `program_json/mod.rs` / `program_json/*` / `stub_emit.rs` / `stub_emit/*` that keep bridge-only command prep, dispatch, route-local exit handling, compile/output policy, plain delegate-status execution, parse/writeback policy, I/O, and stub emit output handling out of `mod.rs`

## Forbidden Responsibilities

- authority/compat route policy
- build-route selection policy
- source-shape interpretation
- parse/lower internals from `stage1/program_json_v0`
- mode inference outside `args.rs`
- backend CLI hint parsing outside `args.rs`
- mixed child-env policy inside `env.rs`
- child/enable bridge entry guard inside `mod.rs`
- route branching / route-local exit-code policy inside `mod.rs`
- direct-route compile/output policy inside `direct_route/mod.rs`
- duplicate emit output-path policy inside `stub_emit.rs` or `direct_route/emit.rs`
- stub emit parse/writeback policy inside `stub_emit.rs`
- child command/env assembly inside `mod.rs`
- JSON line parsing / emit output path policy inside `mod.rs`
