# rust-subset-to-hako

Status: v0 embedded-fixture, FileBox-input, and adapter-fixture `.hako` converter paths pass EXE/AOT parity

Purpose: provide a small real-app front that converts a conservative Rust subset
model into `.hako` skeleton code.

This is not a full Rust transpiler. The first useful app is:

```text
Rust source
  -> external parser adapter (syn / tree-sitter-rust / rust-analyzer)
  -> RustSubset JSON v0
  -> .hako converter app
  -> Hako skeleton
```

The Hakorune-owned part starts at `RustSubset JSON v0`. That keeps the first
front small enough to test string, array, map, record, box, enum, function, and
error handling shapes without building a Rust parser first.

## Why This App

This app is a good post-fastpath compiler construction front because it is:

- close to selfhost/compiler work
- larger than a microbench
- still scopeable
- useful for future Rust-to-Hakorune migration sketches
- strong at exercising structured data and text generation

## V0 Goal

Read a RustSubset JSON document and emit `.hako` skeleton code.

V0 accepts:

- `struct` with named fields
- simple `enum` with unit or tuple-like variants
- free `fn`
- `impl Type { fn ... }`
- simple `let`
- simple `return`
- method call / function call skeletons
- field access skeletons
- index access skeletons
- integer, string, bool, null-like placeholder literals

V0 emits:

- value-like Rust structs as `record`
- impl methods as top-level functions with `me` as the first argument
- unsupported bodies as `/* TODO */` stubs
- stable comments for unsupported Rust constructs

## Non Goals

V0 does not implement:

- borrow checking
- lifetime semantics
- macro expansion
- trait resolution
- generics beyond preserving names as comments
- full pattern matching
- async/await
- unsafe
- procedural macro support
- semantic equivalence with Rust

## Files

- `DESIGN.md`: handoff design for another AI/worker
- `schema/RustSubset-v0.md`: normalized input shape
- `schema/external-adapter-boundary-v0.md`: external parser adapter handoff contract
- `examples/simple_input.rs`: sample source for external adapter
- `examples/simple_subset.json`: sample normalized input
- `examples/simple_expected.hako`: expected skeleton output
- `converter_core.hako`: `.hako` converter core; no input route ownership
- `convert.hako`: startup wrapper for the current embedded input route
- `convert_file.hako`: startup wrapper for the current FileBox input route
- `convert_adapter_fixture.hako`: startup wrapper for host-produced adapter fixture handoff
- `convert_if_fixture.hako`: startup wrapper for the selected `If` statement fixture
- `convert_assign_fixture.hako`: startup wrapper for the selected assignment fixture
- `convert_index_fixture.hako`: startup wrapper for the selected index expression fixture
- `fixtures/simple_subset_embedded.hako`: host-generated embedded JSON fixture
- `tools/embed_fixture.py`: host tool that generates embedded fixture modules
- `tools/syn_adapter/`: external Rust parser adapter selected for v0 source
  handoff

## Current Implementation

Two converters exist:

- `convert.py`: reference implementation and parity oracle.
- `convert.hako`: native `.hako` implementation using `apps/lib/json_native`.

The Python reference currently passes:

```bash
python3 apps/rust-subset-to-hako/selftest.py
```

The `.hako` converter now emits MIR JSON and compiles/runs through EXE/AOT for
the embedded `simple_subset.json` fixture.

Current accepted slice:

```text
python_reference_selftest=ok
json_native_probe_exe=ok
hako_converter_mir_json_emit=ok
hako_converter_exe=ok
hako_converter_parity=simple_expected.hako
file_input_converter_parity=simple_expected.hako
adapter_fixture_handoff_parity=adapter_fixture_expected.hako
vm_product_route=retired
primary_route=EXE/AOT
```

Reproduce the current state:

```bash
bash apps/rust-subset-to-hako/smoke.sh
```

## Current Scope Boundary

The first AOT slice used a host-generated embedded fixture module before adding
FileBox input. Both routes now feed the same converter core and keep the
acceptance target focused on:

```text
JSON parse -> JsonNode traversal -> RustSubset skeleton emission -> EXE/AOT
```

Stdin and external adapter invocation remain separate follow-up rows. The
minimal FileBox new/open/read/close route and the FileBox-backed converter
input wrapper are green on EXE/AOT. The adapter fixture handoff route is also
green and uses the same converter core. The converter core is already separated
from the current input routes:

```text
convert.hako:
  startup wrapper only

convert_file.hako:
  FileBox input wrapper only

convert_adapter_fixture.hako:
  host-produced adapter fixture wrapper only

converter_core.hako:
  RustSubset JSON -> .hako skeleton conversion

fixtures/simple_subset_embedded.hako:
  host-generated embedded JSON handoff
```

`json_native` now uses generic object-key entry-table equality for
scanner-derived dynamic keys. The former RustSubset critical-key dictionary has
been retired; converter call sites do not carry schema-specific lookup logic.

App-front structure follows:

```text
docs/development/current/main/design/hako-app-front-boundary-template-ssot.md
```

## Suggested Next Implementation

Keep converter core separate from input route work.

```text
closed:
  embedded fixture handoff
  probes layout
  status-code schema normalizer probe
  bool-returning schema helper shape
  generic unknown-key materialization fallback
  FileBox minimal EXE/AOT probe
  converter file input through FileBox, separate from converter core
  external adapter boundary contract
  adapter fixture handoff probe

active next:
  COREPLAN-RECURSIVE-RECIPE-REAL-SHAPE-INTAKE-001
    current_task=COREPLAN-CONTINUE-PARTIAL-CARRIER-PHI-001
    next_after_current=COREPLAN-READ-NEXT-NUMBER-LITERAL-MULTI-STAGE-LOOP-ACCEPTANCE-001
    note=RustSubset while / Vec literal transport rows are already closed; this is compiler Recipe/CorePlan loop-exit intake. The current failing evidence is partial-carrier continue PHI, not a broad read_next_number_literal rewrite.

hardening:
  object-key equality regression guard
  serial smoke/regression rebuild guard
```

The first external adapter route is a small `syn`-based host producer:

```bash
cargo run --manifest-path apps/rust-subset-to-hako/tools/syn_adapter/Cargo.toml -- \
  apps/rust-subset-to-hako/examples/adapter_fixture_input.rs \
  --module adapter_fixture \
  -o /tmp/adapter_fixture_subset.json
```

It remains outside the Hakorune-owned converter core and produces the same
RustSubset JSON v0 schema.

The dedicated adapter handoff gate is:

```bash
bash apps/rust-subset-to-hako/smoke_adapter.sh
```

This is a thin wrapper around `smoke.sh` with `RUST_SUBSET_RUN_ADAPTER=1`; it
does not give `converter_core.hako` ownership of Rust parsing or input routing.

Do not bypass `json_native` with a native JSON DLL in this row. The app is useful
because it exercises real `.hako` JSON/tree traversal pressure.

The detailed task board lives in `STATUS.md`.

## Acceptance

```text
rust_source_parser_owned_by_hako=0
rust_subset_json_schema_defined=1
hako_converter_scope=v0_skeleton
full_rust_transpiler_claim=0
borrow_checker_claim=0
macro_expansion_claim=0
semantic_equivalence_claim=0
summary=ok
```

## Current AOT Acceptance Target

```text
python_reference_selftest=ok
hako_converter_mir_json_emit=ok
json_native_probe_mir_json_emit=ok
json_native_probe_exe=ok
hako_converter_exe=ok
hako_converter_parity=ok
embedded_fixture_handoff=ok
file_input_enabled=1
file_input_converter_parity=ok
adapter_fixture_handoff_parity=ok
index_fixture_parity=ok
syn_adapter_smoke=ok
schema_key_dictionary_enabled=1
generic_unknown_key_fallback_enabled=1
json_object_key_materialization=entry_table_plus_temporary_critical_key_bridge
summary=ok
```
