# Pure-First MIR Artifact and Diagnostics SSOT

Status: SSOT
Decision: accepted
Date: 2026-05-16
Scope: pure-first/selfhost MIR artifact exactness, route preflight, and
no-output diagnostics before returning to allocator rows.

## Problem

The current pure-first guard path can look like one route while still using two
MIR emissions:

```text
guard:
  emit MIR JSON
  inspect that MIR JSON

selfhost EXE build:
  receive the same path as --mir
  emit MIR JSON again
  build EXE from the second emission
```

That means the preflight and the executable build are not guaranteed to consume
the exact same MIR JSON bytes. The environment can also differ between the guard
emission and the selfhost build route, which makes route failures hard to
classify.

The second issue is observability. When a pure-first build produces little or no
output, the caller cannot quickly tell whether it is:

- a slow compile
- a compiler work explosion
- a missing lowering route
- an unsupported same-module route contract
- a linker/backend failure

## Decision

Before continuing `MIMAP-029B`, insert a compiler/selfhost BoxShape sidecar
series. The sidecar fixes the route shape, not allocator behavior.

The durable rules are:

```text
same-artifact rule:
  pure-first preflight and EXE build must consume the exact same MIR JSON
  artifact when a guard says it is checking an EXE route.

route-preflight rule:
  pure-first guards must fail before ny-llvmc / C shim emission when MIR
  metadata already proves a missing or unsupported lowering route.

progress rule:
  selfhost/pure-first wrappers must identify the active phase so "slow",
  "stuck", and "unsupported route" are not confused.
```

## CLI Semantics

Use explicit MIR direction names:

```text
--mir-out FILE:
  source -> MIR JSON output path

--mir-in FILE:
  existing MIR JSON input path for EXE build

--mir FILE:
  compatibility alias for --mir-out during the transition
```

`--mir-in` must not re-emit MIR from source. It consumes the supplied file as
the executable input artifact.

## Canonical External Emit Tool

Existing tool surface:

```text
tools/smokes/v2/lib/emit_mir_route.sh
  operational route SSOT for smoke/check callers today

tools/hakorune_emit_mir.sh
  internal compat-capsule implementation
  direct callers are guarded by tools/checks/hakorune_emit_mir_direct_caller_guard.sh

tools/hakorune_emit_mir_compat.sh / tools/hakorune_emit_mir_mainline.sh
  thin preset wrappers around the compat capsule
```

Therefore `MIR-EMIT-SSOT-002` must not blindly add a competing large wrapper.
It must first decide whether the canonical external entry is:

```text
preferred:
  tools/smokes/v2/lib/emit_mir_route.sh

or thin facade:
  tools/hako_emit_mir_json.sh -> tools/smokes/v2/lib/emit_mir_route.sh
```

If a thin facade is still useful, the long-term external entry may be:

```bash
tools/hako_emit_mir_json.sh --in app.hako --out app.mir.json
```

but it must be a small facade over the existing route SSOT, not a replacement
for `tools/hakorune_emit_mir.sh`. Direct CLI flags may remain for developer
diagnostics, but guards should not each reinvent the emit environment.

## Route Preflight

Add a MIR JSON preflight before ny-llvmc / C shim emission:

```text
tools/checks/pure_first_route_preflight.py
```

The preflight reads `functions[].metadata.lowering_plan` and classifies call
sites before backend emission. Required reason vocabulary:

```text
lowering_plan_missing
unsupported_tier
typed_user_box_method_contract_missing
typed_global_call_contract_missing
target_body_supported=false
target_exists=false
arity_mismatch
return_shape_missing
value_demand_mismatch
```

Schema reality:

```text
metadata.lowering_plan exists today.

owner:
  docs/development/current/main/design/lowering-plan-json-v0-ssot.md

producer:
  src/runner/mir_json_emit/metadata.rs
  src/runner/mir_json_emit/route_json.rs

existing consumers:
  lang/c-abi/shims/hako_llvmc_ffi_* lowering-plan views
  tools/checks/k2_wide_* guards that inspect metadata.lowering_plan
```

The reason names above are preflight classifications, not a promise that every
name is a literal JSON field. `MIR-ROUTE-PREFLIGHT-001` must map them from the
actual LoweringPlan JSON v0 fields.

Scope:

```text
default:
  main-reachable pure-first EXE route

rule:
  follow supported direct-function LoweringPlan edges from main and fail only
  on route rows that the pure-first emitter can reach.

why:
  MIR JSON may contain diagnostic unsupported rows for library functions that
  are not emitted into the current EXE. Those rows remain useful metadata, but
  they are not a same-artifact pure-first build failure until a reachable call
  path touches them.
```

Initial mapping target:

| Reason | Source field / condition |
| --- | --- |
| `lowering_plan_missing` | no entry for the MIR call site in `metadata.lowering_plan` |
| `unsupported_tier` | `tier == "Unsupported"` or `emit_kind == "unsupported"` |
| `typed_user_box_method_contract_missing` | `source == "user_box_method_routes"` with `reason` not accepted or missing direct contract fields |
| `typed_global_call_contract_missing` | `source == "global_call_routes"` with `reason` not accepted or missing direct contract fields |
| `target_body_supported=false` | `target_body_supported == false` on user-box method route |
| `target_exists=false` | `target_exists == false` on global/user-box route |
| `arity_mismatch` | `arity_matches == false` or `target_arity` disagrees with `arity` |
| `return_shape_missing` | route needs a result but `return_shape == null` |
| `value_demand_mismatch` | `value_demand` is absent or incompatible with `return_shape` / route kind |

Output must be stable and actionable:

```text
[pure-first-route][fail]
function=<name>
site=<block.instruction>
callee=<source callee when available>
reason=<reason>
owner=<route owner when available>
suggestion=<narrow next action>
```

The C shim remains the final defense, but pure-first guards should catch known
metadata route misses before backend codegen starts.

## Progress Diagnostics

Selfhost and pure-first wrappers should emit stable phase progress lines, and
later optional JSONL events, around the long steps:

```text
[selfhost] phase=selfhost.emit_mir start
[selfhost] phase=selfhost.emit_mir done elapsed_ms=<n>
[selfhost] phase=selfhost.route_preflight start
[selfhost] phase=selfhost.route_preflight done elapsed_ms=<n>
[selfhost] phase=selfhost.nyllvmc start
[selfhost] phase=selfhost.nyllvmc done elapsed_ms=<n>
```

When a timeout or no-output closeout happens, the wrapper should print the last
known phase and, after JSONL exists, the last route/function event. This is a
diagnostic layer only; it must not change lowering behavior.

`selfhost.link` is reserved for the future point where the shell wrapper owns a
separate link command. Today `ny-llvmc --emit exe` is a combined compiler/linker
phase from the shell wrapper's perspective, so `selfhost.nyllvmc` is the
highest-fidelity phase boundary.

## Task Order

### MIR-EMIT-SSOT-001: same MIR artifact

Status: landed

Purpose:

```text
Split selfhost MIR arguments into --mir-in / --mir-out and make pure-first
guards build EXE from the exact MIR artifact they preflighted.
```

Required work:

- Add `--mir-in` to `tools/selfhost/selfhost_build.sh` routing.
- Add `--mir-out` as the explicit source-to-MIR output spelling.
- Keep `--mir` as a temporary compatibility alias for `--mir-out`.
- Change pure-first guards to:
  1. emit MIR once
  2. run preflight on that file when available
  3. invoke selfhost EXE build with `--mir-in`
- Add an artifact exactness guard that proves the file is not re-emitted during
  EXE build.

Return condition:

```text
preflight MIR SHA == EXE input MIR SHA
```

### MIR-ROUTE-PREFLIGHT-001: lowering-plan preflight

Status: landed

Purpose:

```text
Classify missing/unsupported pure-first routes from MIR metadata before
ny-llvmc or the C shim tries to emit them.
```

Required work:

- Confirm the current LoweringPlan JSON v0 shape against
  `docs/development/current/main/design/lowering-plan-json-v0-ssot.md` and at
  least one generated MIR artifact.
- Add `tools/checks/pure_first_route_preflight.py`.
- Add a guard fixture for at least one supported route and one missing route.
- Integrate the preflight into pure-first guards after same-artifact routing is
  available.

Return condition:

```text
unsupported route fails with a stable reason before backend emission
```

### SELFHOST-PROGRESS-001: phase progress diagnostics

Status: landed

Purpose:

```text
Make selfhost/pure-first build output distinguish no-output hang, slow compile,
route unsupported, and backend/link failure.
```

Required work:

- Add phase start/done lines to selfhost shell wrappers.
- Add timeout closeout that prints the last known phase.
- Keep logging stable, single-line, and quiet enough for CI.
- Add JSONL only after the text phase contract is stable.

Return condition:

```text
timeout/no-output failure includes the active phase and last known step
```

### MIR-EMIT-SSOT-002: canonical emit wrapper

Status: selected current after progress diagnostics

Purpose:

```text
Make the source-to-MIR external authority explicit without competing with the
existing compat capsule.
```

Return condition:

```text
CI/guard/selfhost callers use one source-to-MIR route entry; direct calls to
tools/hakorune_emit_mir.sh remain restricted to allowed capsule wrappers
```

### RETURN-CONTRACT-001: expected return type propagation

Status: parked future sidecar

Purpose:

```text
When a function declares a scalar return type, propagate that expected type into
return expressions such as ArrayBox.get before route selection.
```

This is not required for `MIR-EMIT-SSOT-001`. MIMAP-029A already used explicit
source-level return contracts and typed locals, which are valid contracts for
the current compiler.

## Stop Lines

- Do not add allocator behavior in these rows.
- Do not widen MIMAP-029A or MIMAP-029B.
- Do not add C shim / `.inc` name matchers as a shortcut.
- Do not hide route failures behind fallback execution.
- Do not activate provider hooks, host allocator replacement, or
  `#[global_allocator]`.
- Do not implement broad source-language return inference inside the artifact
  exactness row.

## Acceptance

Minimum docs/current guard:

```bash
bash tools/checks/current_state_pointer_guard.sh
```

Expected future row guards:

```bash
bash tools/checks/pure_first_mir_artifact_exactness_guard.sh
bash tools/checks/pure_first_route_preflight_guard.sh
bash tools/checks/selfhost_progress_diagnostics_guard.sh
```

After `MIR-EMIT-SSOT-001`, rerun the allocator guard that exposed the issue:

```bash
bash tools/checks/k2_wide_mimalloc_facade_huge_decommit_exe_guard.sh
```

## Return To MIMAP

After the same-artifact route and route preflight are stable, return to:

```text
MIMAP-029B post-huge-decommit allocator row selection
```

`MIMAP-029B` then selects the next allocator behavior row, likely duplicate
decommit fail-fast diagnostics if no new route blocker appears.
