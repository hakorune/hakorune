---
Status: Post-W6 graduation board; current production mutation is forbidden
Decision: staged llvmlite graduation and native-library ownership selected
Date: 2026-07-22
Scope: LLVM route truth, native library boundary, Hako LLVM-text ownership, and llvmlite retirement
Current-lane effect: none; D-prime HEADERPORT0 remains authoritative
Reserved activation: W6 final-live receipt/caller evidence is landed; G1/G2/G3
remain independently gated
Related:
  - docs/development/current/main/design/llvm-line-ownership-and-boundary-ssot.md
  - docs/development/current/main/investigations/fastmem-v1-execution-task-2026-07-22.md
  - docs/development/current/main/investigations/fastmem-v1-contracted-borrow-design-2026-07-20.md
  - crates/nyash-llvm-compiler/README.md
  - src/host_providers/llvm_codegen/README.md
  - src/llvm_py/README.md
  - docs/development/current/main/investigations/dynamic-v2-w6-production-activation-task-2026-08-13.md
---

# LLVM Native Library And llvmlite Graduation Task Board

## Decision

llvmlite is not removed now. It is frozen as an explicit reference/oracle lane
while every automatic production ingress is retired independently.

The selected long-term ownership is:

```text
MIR / sealed backend plans
  semantic representation authority
        |
        v
.hako LlTextEmitBox family
  LLVM text lowering authority
        |
        v
libhako_llvmc_ffi
  versioned target/session/error ABI
  LLVM text verification and object emission
  link transaction
        |
        +-- called from .hako through one thin host port
        |
        `-- called from ny-llvmc as a CLI adapter
```

The final library implementation uses the LLVM C API in-process. A bounded
external `opt`/`llc`/`cc` implementation may temporarily live behind the same
typed library ABI, but it is not the final no-child-process claim.

Existing `hako_llvmc_compile_json_pure_first` remains a transition owner for
already-supported MIR families. It is not expanded into a second permanent
FastMem semantic lowerer. New FastMem V1 lowering starts in the Hako LLVM-text
emitter and hands LLVM text to the native library.

`ny-llvmc --driver native` remains a const/print/ret canary. Growing it into a
second LLVM lowerer is forbidden.

## MIRBuilder W6 handoff lock (2026-08-13)

```text
Decision: W6-E Boundary route closeout is prerequisite evidence for llvmlite
graduation; the current pointer records that selected route as landed.
Source authority + canonical issuer: W6-E caller census, Boundary artifact receipt, and this board's G1/G2/G3 rows.
Non-authority: llvmlite output, harness fallback, Python route, native canary, and default build environment.
Fail-fast boundary: Boundary failure or unsupported MIR is a typed error; no harness retry/fallback; unknown Python ingress blocks G1.
Smallest next slice: after W6-E, run ROUTE0 census/identity/observe, then close G1, G2, and G3 in order.
Non-claims: no source deletion, new llvmlite semantics, or current W6 production switch is claimed here.
```

The W6 prerequisite is observable, not a prose milestone. The caller census is
scoped to the selected Dynamic lane; ordinary compatibility is not silently
retired by this board:

```text
selected Dynamic Boundary caller = 1
selected Dynamic old raw/JoinIR edge = 0
ordinary compatibility edge = 1 (allowed until its own retirement row)
Boundary artifact receipt = 1
Python/llvmlite production consumer = 0
fallback = 0, retry = 0, VM consumer = 0
```

`W6-E-C5 STATIC-RECEIPT-GATED-LIVE-INSTALL-R0` and `W6-E-C6` landed the
receipt-gated selected runner terminal, runtime-archive path, shared
LLVM-ingress materialization boundary, and scoped receipt/caller evidence.
Compiler MIR publication and backend executable publication remain two ordered
transactions; ordinary compatibility is not retired here. G1 retires automatic
production reachability, G2 removes Python/llvmlite from default build/CI/perf
gates, and G3 separately decides source/archive removal.
The explicit `--driver harness` and `NYASH_LLVM_USE_HARNESS=1` lanes remain
named keep/oracle lanes; they may not become a new production authority.

## Post-W6 graduation acceptance matrix (DOC0, 2026-08-13)

This feedback is taskized by the existing G1/G2/G3 rows; it does not open a
new backend or a new fallback lane. The rows are ordered and may not be
collapsed into “MIRBuilder finished, therefore llvmlite is deleted”.

The graduation boundary is deliberately two-dimensional:

```text
MIRBuilder / Boundary completion
  = canonical MIR is emitted and the selected native artifact path is closed

llvmlite graduation
  = every Python ingress is censused, observed, and retired in stages
```

The first line is necessary but not sufficient for the second. A MIRBuilder
green result must not be used as evidence for repository-wide llvmlite
deletion. Conversely, keeping an explicit oracle does not authorize a native
failure fallback or a second production physicalizer.

### Feedback reconciliation (DOC1, 2026-08-13)

The recommended retirement shape is accepted without opening a new task:

```text
G1  production reachability retirement
    Boundary/native failure -> typed fail-fast; Python/llvmlite fallback = 0
G2  default build/CI/perf independence
    Python/llvmlite remains only in explicitly named compat/oracle/monitor jobs
G3  source/archive retirement
    separate approval after an independent oracle and fixture/archive census
```

MIRBuilder or Boundary completion alone closes none of G1/G2/G3. W6-E must
first provide the selected caller, old-edge, artifact-receipt, and no-fallback
evidence. Until then the board remains parked, `--driver harness` and
`NYASH_LLVM_USE_HARNESS=1` remain explicit non-production keep roots, and no
llvmlite source deletion or new llvmlite semantic lowering is authorized.

```text
W6-E receipt and caller census
  -> LLVMLITE-PROD0-G0 (G1: automatic production retirement)
  -> LLVMLITE-AUTO0      (G2: default build/CI/perf independence)
  -> LLVMLITE-KEEP0-RET0 (G3: source/archive retirement, separate approval)
```

### G1 — remove production reachability (`LLVMLITE-PROD0-G1-REACHABILITY-R0`)

Owner: W6-E Boundary artifact receipt plus route/child-process census;
`G1-ROUTE-BOUNDARY-R0` is landed (default BoundaryPureFirst, named Stage1
ExplicitHarnessCompat), while `G1-RUNNER-FAILFAST-R0` remains separate.
Acceptance: selected Dynamic Boundary caller `= 1`, old raw/JoinIR edge `= 0`,
ordinary compatibility may remain `= 1`, Boundary artifact receipt/fence `= 1`,
automatic Python/llvmlite consumer `= 0`, native retry `= 0`, generic C ->
implicit harness `= 0`, selected automatic mock fallback `= 0`, and unsupported
native input is typed fail-fast. Explicit harness/oracle roots remain available
but are not production callers.

### G2 — remove default dependency

Owner: `LLVMLITE-AUTO0` ingress rows plus build/CI/perf census.

Acceptance is default build Python requirement `= 0`, default execution and
perf gates Python child count `= 0`, and every surviving llvmlite job is
explicitly named `compat`, `oracle`, `monitor`, or `historical`. No new
production lowering is added to the keep lane.

### G3 — archive or delete the keep lane

Owner: `LLVMLITE-KEEP0-RET0`, only after an independent semantic oracle,
fixture/golden preservation, and a zero-or-archived consumer census exist.
Source deletion is a separate approval and must preserve the source/artifact
archive needed for historical bug reproduction. G1 and G2 never imply G3.

The keep lane is frozen during G1/G2: no new semantic lowering, ABI authority,
fallback, or production caller may be added to `src/llvm_py/**`, the harness,
or the explicit `--driver harness` path. Its only permitted outputs are
named compatibility/oracle evidence and preserved fixtures. A later G3
decision must record the archive location, fixture/golden inventory, and the
zero-or-archived consumer census before any source removal.

All three rows require positive route receipts, negative fallback/retry
evidence, and a documented red classification. Until G1 starts, the current
MirBuilder W6 lane and its `new=0 / old=1` production state remain unchanged.

## Corrected current truth

The current repository has three distinct ny-llvmc routes:

```text
ny-llvmc default Boundary
  -> libhako_llvmc_ffi
  -> C pure-first MIR(JSON)-to-LLVM lowering

ny-llvmc --driver harness
  -> Python
  -> llvmlite

ny-llvmc --driver native
  -> Rust bootstrap canary
  -> const(i64) / print / ret only
```

Therefore these statements are false:

```text
"ny-llvmc itself only supports const/ret/print"
"all llvmlite use is already explicit"
"libhako_llvmc_ffi is already an in-process LLVM library"
"--harness PATH alone selects the llvmlite driver"
"NYASH_LLVM_USE_HARNESS=1 alone proves llvmlite execution"
```

### Remaining automatic Python reachability

The audit found at least these production/compat routes:

```text
compiled-stage1 surrogate
  compat_replay = harness

env.codegen.emit_object with no explicit recipe
  -> generic C export
  -> hako_aot
  -> ny-llvmc --driver harness

hako_llvmc_compile_json generic export
  -> hako_aot_compile_json
  -> ny-llvmc --driver harness

hako_aot_compile_json default command
  -> ny-llvmc --driver harness
```

Default `ny-llvmc` with `recipe=pure-first` and `compat_replay=none` does not
automatically fall back. That local fact must not be generalized to every
public or compatibility ingress.

### Current native-library reality

`libhako_llvmc_ffi` already exists, but its current implementation is a large
C MIR(JSON)-to-text lowerer plus external tool orchestration.

```text
versioned public ABI header: absent
explicit target request: absent
owned compile session/context: absent
structured error object: absent
hidden export discipline: incomplete
in-process LLVM C API object emission: optional host/X86 probe only
default object emission: textual LLVM + external opt/llc/cc
```

The optional target-machine path parses already-written LLVM text and may
return to external tools. It is not the final canonical provider.

### FastMem reality

```text
Rust MemOp vocabulary: 17/17
llvmlite lowering functions: 17/17
focused positive llvmlite proofs: 11/17
C pure-first MemOp support: 0/17
Rust MirInterpreter MemOp execution: 0/17
```

The six MemOps without focused positive llvmlite proof are:

```text
AddrOf LogicalShr BitAnd Add Sub FreeHeadPop
```

`CurrentAllocOwnerId` has an LLVM declaration/call path but no confirmed
linked runtime definition. An IR construction test is not executable proof.

## Graduation has three separate meanings

### G1 — production reachability retirement

```text
automatic production path to Python/llvmlite = 0
unsupported canonical route = typed fail-fast
native failure -> llvmlite retry = 0
```

This is `LLVMLITE-PROD0-G0`.

### G2 — default build and CI independence

```text
default compiler build requires Python = 0
default execution/perf gates require llvmlite = 0
explicit oracle/monitor jobs remain separately named
```

This may close with llvmlite source still present.

### G3 — source/archive retirement

```text
reference semantic oracle replacement exists
all Python unit/check/tool consumers are zero or archived
llvmlite source deletion is separately approved
```

This is the later `LLVMLITE-KEEP0-RET0`. G1 never implies G3.

## Macro task order

The lane remains parked behind the current MirBuilder/finalization work. When
activated, the order is:

```text
FASTMEM-BASELINE0
-> LLVMLITE-ROUTE0-CENSUS0
-> LLVMLITE-ROUTE0-IDENTITY0
-> LLVMLITE-ROUTE0-OBSERVE0

-> LLVM-NATIVELIB0-CENSUS0
-> LLVM-NATIVELIB0-ROLELOCK0
-> LLVM-NATIVELIB0-ABI0
-> LLVM-NATIVELIB0-TARGETPORT0
-> LLVM-NATIVELIB0-ERROR0
-> LLVM-NATIVELIB0-VISIBILITY0
-> LLVM-NATIVELIB0-SESSION0
-> LLVM-NATIVELIB0-TOOLTX0
-> LLVM-NATIVELIB0-LLTEXT0
-> LLVM-NATIVELIB0-HAKOPORT0

-> FASTMEM-SSOT-DRIFT0
-> FASTMEM-VOCAB-FREEZE0
-> FASTMEM-LLVMLITE-REF0
-> FASTMEM V1 representation foundation
-> FASTMEM-FIELDLOAD-VERTICAL0

-> LLVM-NATIVELIB0-LLVMAPI0
-> LLVM-NATIVELIB0-PARITY0

-> HAKOLL-COVERAGE0 family rows
-> LLVMLITE-AUTO0 ingress retirements
-> LLVMLITE-TOOL0-TRUTH0
-> LLVMLITE-DEADAPI0-RET0
-> LLVMLITE-PROD0-G0

later:
  LLVM-NATIVELIB0-LEGACYJSON-RET0
  LLVMLITE-KEEP0-RET0
```

Only one code-facing row is active at a time. FastMem subrows are owned by the
FastMem execution board; this board owns their backend dependencies and
retirement gates.

Every durable row expands only as far as needed through this protocol:

```text
D0  exact decision/owner/non-owner/failure boundary; code delta = 0
S0  disconnected vocabulary or extractor; production consumers = 0
M0  caller/mutation/route census when existing behavior is not yet closed
P0  positive, negative, failure-atomic, reuse, and route proof
I0  one production owner cutover
G0  old owners/fallbacks/unknown reachability = 0
```

An I0 may not begin because its S0 compiles. Its exact P0 must be green, and
one semantic behavior delta may belong to only one I0.

## 1. Route truth series

### `LLVMLITE-ROUTE0-CENSUS0`

#### `LLVMLITE-ROUTE0-CENSUS0-D0-IDENTITY-OBSERVE` (closed D0 receipt; next stop is `LLVMLITE-ROUTE0-IDENTITY0`)

```text
Decision: classify executable LLVM ingress by actual driver/provider/replay, not names.
Source authority + canonical issuer: driver dispatch, provider selection, and child-command observation.
Non-authority: comments, NYASH_LLVM_USE_HARNESS alone, generic-export fallback, or historical command names.
Fail-fast boundary: unknown Python reachability, pure-first -> generic fallback, or unclassified ingress stays stopped.
Smallest next slice: source-site matrix, child-command observation, and one reusable route guard; behavior and retirement remain closed.
Non-claims: no G1/G2/G3 retirement, source deletion, new backend, or fallback change.
```

Current census leads are `hako_aot_compile_json` hardcoding `--driver harness`,
`provider_keep.rs` directly selecting llvmlite, `capi_transport.rs` falling
from pure-first to a generic export, and `fast-smoke` explicitly using
`compat_replay=harness`. The default `ny-llvmc` Boundary and stage1
`pure-first/replay=none` routes are separate observations, not proof that all
other ingresses are Python-free.

Initial source-site matrix (D0 observation, not a production decision):

| source site | actual selector | Python reachability |
| --- | --- | --- |
| `ny-llvmc` default | Boundary driver | Unreachable observed |
| `ny-llvmc --driver harness` | `harness_driver` | Reachable |
| `ny-llvmc --driver native` | native canary | Unreachable observed |
| `hako_aot_compile_json` | hard-coded `--driver harness` | Reachable |
| `provider_keep.rs` | `HAKO_LLVM_EMIT_PROVIDER=llvmlite` | Reachable |
| `route.rs` CAPI/default | CAPI selection; generic export may enter `hako_aot`/harness | Reachability is route-dependent; symbol fallback remains a drift stop |
| `env.codegen.emit_object` with CAPI + generic recipe | generic C export -> `hako_aot_compile_json` -> `--driver harness` | Reachable |
| `env.codegen.emit_object` with CAPI + `pure-first`, replay=none | pure-first C export | Unreachable observed; unsupported shape stops |
| `env.codegen.emit_object` with explicit `HAKO_LLVM_EMIT_PROVIDER=llvmlite` and CAPI disabled | provider_keep -> Python harness | Reachable keep |
| `env.codegen.emit_object` with explicit `HAKO_LLVM_EMIT_PROVIDER=ny-llvmc` and CAPI disabled | `ny-llvmc` Boundary | Unreachable observed |
| `env.codegen.compile_ll_text` | external `opt`/`llc` tool seam | Unreachable observed (not a llvmlite route) |
| stage1 mainline | `pure-first`, `replay=none` | Unreachable observed |
| `tools/ny_mir_builder.sh` llvmlite branch | explicit backend flag | Reachable keep |
| `fast-smoke` compat job | `compat_replay=harness` | Reachable keep |
| perf AOT helpers | reject harness/llvmlite/replay | Unreachable by policy |

The D0 implementation may only turn `Unknown` into an observed classification
or a typed stop; it may not silently reinterpret a generic C export as the
Boundary route or change any production behavior. In particular, a CAPI flag
does not prove Boundary-only execution: the generic C export can deliberately
enter the compatibility `hako_aot_compile_json -> --driver harness` route.

### Worker route-observation receipt (D0, 2026-08-13)

The read-only route audit closed the previously unknown plugin ingress as
route-dependent rather than Boundary-only:

```text
compat_codegen_receiver::emit_object
  -> mir_json_text_object::route
  -> CAPI generic export (when CAPI flags are enabled and recipe is absent)
  -> hako_llvmc_compile_json
  -> hako_aot_compile_json
  -> ny-llvmc --driver harness
  -> Python/llvmlite
```

The same ingress with `pure-first` and `compat_replay=none` remains a pure
lane and rejects unsupported shapes; `replay=harness` is the only explicit
replay into Python. `compile_ll_text` is a separate Rust thin seam over
external `opt`/`llc` and must not be counted as llvmlite reachability.

Two identity hazards remain open for the next bounded guard, without changing
behavior in D0:

1. `capi_transport.rs` can try a generic compile symbol after the requested
   pure-first symbol is absent. This is a route-identity drift and must be
   observed or typed-stopped before G1; it is not evidence of a successful
   Boundary route.
2. `HAKO_LLVM_EMIT_PROVIDER=llvmlite` is not a direct-provider receipt when
   CAPI flags win earlier in `route.rs`; the actual selected C export and
   child command are the authority.

The plugin receiver currently converts `emit_object`/`compile_ll_text` errors
to `Ok(None)`. Whether that compatibility-facing loss of typed failure is
accepted or changed is a separate route-contract decision; it does not count
as Python reachability evidence and remains outside D0 behavior changes.

Create a source-derived route inventory. One row represents one exact ingress
and carries:

```text
ingress symbol and source site
build/cfg profile
selected driver
selected C export
recipe
compat replay
Python child reachability: Reachable | Unreachable | Unknown
fallback owner
failure disposition
test/production/diagnostic domain
retirement owner
```

Required roots include:

```text
ny-llvmc default / harness / native
env.codegen.emit_object
env.codegen.compile_ll_text
hako_llvmc_compile_json
hako_llvmc_compile_json_pure_first
hako_aot_compile_json
compiled-stage1 surrogate
tools/build_llvm.sh
tools/ny_mir_builder.sh
monitor and CI jobs
```

Acceptance:

```text
unclassified executable LLVM ingress = 0
unknown Python reachability = 0
manual aggregate route count = 0
source-site reverse census is bijective
production behavior delta = 0
```

### D0 bounded guard task (`LLVMLITE-ROUTE0-CENSUS0-IDENTITY-GUARD-S0`)

```text
Task: LLVMLITE-ROUTE0-CENSUS0-IDENTITY-GUARD-S0
Owner: existing route/driver/provider source census; no new backend owner
Shape: one reusable static guard plus source-derived matrix, no runtime change
Guard inputs: CAPI flags, recipe, compat replay, provider, selected C export,
              driver, child command, and plugin ingress
Positive: pure-first+none has Python=0; explicit harness/provider is Reachable;
          compile_ll_text is external-tool-only; generic C export is compat
          harness reachability, not Boundary-only
Negative: missing requested pure-first symbol, implicit generic fallback,
          provider/CAPI precedence drift, unknown child command, and untyped
          ingress remain a typed stop / red census
Non-claims: no G1/G2/G3 retirement, source deletion, fallback removal,
            provider precedence change, or plugin error-policy change
```

S0 receipt (2026-08-13): `tools/checks/llvm_codegen_route_identity_guard.sh`
is green. Identity0/OBSERVE0 then fixed selector labels and child evidence;
G1-D1 now fences selected-Dynamic route inheritance before child spawn. F0
closed the pure-first CAPI symbol fallback with focused route/guard evidence.
The remaining source-backed hazards are generic C -> hako_aot -> harness
reachability and plugin `Err=>Ok(None)`; no provider order changed.

### `LLVMLITE-ROUTE0-IDENTITY0` (closed)

Identity0/OBSERVE0 are closed behavior-free selector/child-evidence batches.
The route guard remains the regression gate; explicit keep/oracle lanes remain
source-scoped and no llvmlite source deletion is implied.

### `LLVMLITE-AUTO0-GENERIC-CAPI-RET0` (closed by R0)

```text
Decision: design the retirement of automatic generic-C/API -> hako_aot ->
  harness reachability; keep explicit `--driver harness`, provider_keep, and
  replay lanes unchanged.
Source authority + canonical issuer: route plan/driver/provider/replay choice,
  exact CAPI symbol, hako_aot child command, plugin result, and child evidence.
Non-authority: `.or_else(generic)` fallback, provider names, NYASH hint alone,
  llvmlite output, or plugin `Err=>Ok(None)` as a success signal.
Fail-fast boundary: pure-first symbol loss, recipe-unset generic export,
  implicit hako_aot harness, CAPI-unavailable, plugin error, or unsupported
  input must stop with typed failure; no native retry or second lookup.
Non-claims: no G2/G3 retirement, explicit keep removal, selected-Dynamic change,
  generic backend expansion, or llvmlite source deletion.
```

R0 receipt (2026-08-14): recipe-unset generic C and non-replay direct AOT fail
before a child/object; pure-first/replay and named keep lanes remain unchanged.
C build, missing-gate ctypes smoke, explicit compat probe, route guard, fmt, and
diff check are green. No fallback, provider reorder, G2/G3, or source deletion.

F0 receipt (2026-08-14, closed): one requested CAPI symbol lookup; missing pure-first symbols return typed dlsym failure.
Plugin receipt (2026-08-14, closed): `compile_ll_text`, `emit_object`, and
`link_object` preserve success paths and map backend errors to
`BidError::PluginError`; focused tests, route guard, fmt, pointer guard, and
diff check are green. Generic C/hako_aot ingress remains design-only; no
provider order or source deletion changes here.
### Boundary C ABI role lock (DOC2)
```text
Decision: versioned C ABI is a thin transport bridge, never semantic authority.
Source authority + issuer: MIR/site plan plus sole Rust lease owner.
Non-authority: C/LLVM, raw handle/drop, provider lookup, Fault meaning, lease table,
  generation checks, and release semantics. Fail-fast: bad status/wire/ABI/suspension
  traps before a semantic successor; no Python fallback. No alternate ABI/publication.
```
### `LLVMLITE-AUTO0-STAGE1-REPLAY-RET0` (closed by R0)
```text
Decision: `build_stage1.sh --compat-replay <none|harness>` is the sole Stage1
  replay admission; default is `pure-first/none`; inherited `harness` without
  the option fails before bootstrap child/object creation.
Source authority + issuer: build invocation policy plus `stage1_contract` validation;
  artifact metadata records replay mode/admission. Non-authority: Python output,
  `NYASH_LLVM_USE_HARNESS` alone, or env text. Fail-fast: invalid/mismatched/
  inherited replay stops before Python child.
Receipt: helper, build/stage3 wiring, positive/negative tests, metadata,
  route/pointer guards, shell syntax, and diff checks are green.
Non-claims: no provider reorder, G1/G2/G3 retirement, or source deletion.
```
### `LLVMLITE-AUTO0-ENV-CODEGEN-RET0` (closed R0; fast)
Decision: ordinary `env.codegen.emit_object` is fixed Boundary/pure-first and
  rejects inherited replay; only named `emit_object_compat_harness` admits the
  explicit harness keep lane. Source authority: named receiver plus route request.
Non-authority: hints, Python output, provider labels, generic C fallback, plugin `None`.
Fail-fast: ambient replay other than `none` rejects before route/child creation.
Receipt (2026-08-14): focused Rust tests=7; ordinary child=0, named child=1,
  inherited replay child=0 observed by opt-in strace; route/pointer guards, fmt,
  and diff check are green. Non-claims: no G1/G2/G3 retirement or source deletion.
### `LLVMLITE-AUTO0-HAKO-AOT-FFI-ADMISSION-F0` (closed, 2026-08-14)
Decision: generic AOT/FFI now rejects inherited harness replay; only the versioned
  named `*_compat_harness` C ABI may enter the frozen keep lane. Source authority is
  the named export, not `HAKO_AOT_USE_FFI` or ambient replay.
Non-authority: inherited replay, `NYASH_LLVM_USE_HARNESS`, provider names, fallback,
  and Python output. Fail-fast is before FFI `dlsym`, child spawn, or object creation.
Receipt: direct generic reject, named direct keep, generic FFI replay reject, and named
  FFI keep all pass `llvm_hako_aot_ffi_admission_smoke.sh`; build, route/pointer guards,
  and diff check are green. Tracked daily hako_aot caller census remains zero.
Non-claims: no provider reorder, G1/G2/G3 retirement, source deletion, or driver switch.
### `LLVMLITE-ROUTE0-OBSERVE0-R0` (closed)
Decision: reuse `NYASH_LLVM_ROUTE_TRACE` as diagnostic events; no durable route
  receipt or second selector. Source authority: route/driver dispatch emits one
  selection event; child-command owner emits one child event.
Non-authority: trace text, Python output, paths, names, `NYASH_LLVM_USE_HARNESS`,
  or lookup after failure. Fail-fast: contradictory selectors/child/artifact
  evidence stop. Acceptance: route guard pins both producers, trace default-off,
  selectors, and field contract
  (`request_id`, `entry_family`, `driver`, `export`, `recipe`, `compat_replay`,
  `python_child`, `artifact_result`).
Evidence (2026-08-14): opt-in guard records ordinary=0, named compat=1,
  inherited replay=0 Python children; explicit replay has no child/object.
Non-claims: no G1/G2/G3 retirement, source deletion, provider reorder,
  fallback/retry change, or new semantic/backend authority.
## 2. Native library foundation
### `LLVM-NATIVELIB0-CENSUS0`

Inventory every exported symbol, caller, allocator/free convention, dynamic
load site, environment dependency, temporary file, child process, target
default, and error channel of `libhako_llvmc_ffi`.

No API or behavior changes occur here.

### `LLVM-NATIVELIB0-ROLELOCK0`

Lock these roles:

```text
.hako emitter:
  sealed MIR/backend plan -> LLVM text

native library:
  LLVM text -> verified module -> object
  object(s) -> executable

ny-llvmc:
  CLI argument/file adapter to the same library

legacy C pure-first:
  temporary MIR JSON compatibility lowerer
```

The native library may validate LLVM IR and target compatibility. It may not
infer source semantics, method identity, FastMem field meaning, or access-plan
facts.

### `LLVM-NATIVELIB0-ABI0`

Introduce one versioned header, for example `hako_llvmc_v1.h`, with capability
queries and opaque types.

```text
hako_llvmc_api_version_v1
hako_llvmc_capabilities_v1
hako_llvmc_session_open_v1
hako_llvmc_compile_ll_v1
hako_llvmc_link_v1
hako_llvmc_error_view_v1
hako_llvmc_session_close_v1
```

No pointer returned by the library may require callers to guess whether
`free`, `hako_mem_free`, or another allocator owns it.

### `LLVM-NATIVELIB0-TARGETPORT0`

Use one explicit target product:

```text
triple
cpu
features
pointer width
endianness
relocation model
code model
optimization level
data-layout fingerprint
```

FastMem `FASTMEM-TARGET0` must co-seal this product. A second host-derived
target authority is forbidden.

### `LLVM-NATIVELIB0-ERROR0`

Replace integer-plus-string ambiguity with a stable error domain:

```text
InvalidRequest
UnsupportedTarget
InvalidLlvmText
VerificationFailed
ObjectEmissionFailed
LinkFailed
Cancelled
InternalInvariant
```

Errors preserve stage and bounded diagnostics. `None`, empty path, warning, or
alternate route is not failure handling.

### `LLVM-NATIVELIB0-VISIBILITY0`

Hide all non-ABI symbols, including bundled JSON implementation symbols.
Export only the versioned ABI surface. Add an export-list/version-script gate
for supported platforms.

### `LLVM-NATIVELIB0-SESSION0`

One opaque session owns:

```text
target machine/config
diagnostic buffer
cancellation flag
temporary/artifact namespace
LLVM context/module lifetime
```

No per-call ambient environment mutation, process-global current request, or
PID-only temporary naming is permitted. Parallel sessions must not collide.

### `LLVM-NATIVELIB0-TOOLTX0`

Contain the temporary external-tool implementation behind one transaction:

```text
owned input
-> unique private work directory
-> argv execution without shell interpolation
-> bounded stdout/stderr
-> atomic artifact rename
-> cleanup
```

`system()` and caller-provided shell fragments are retired. Failure leaves no
published object/executable and does not invoke llvmlite.

### `LLVM-NATIVELIB0-LLTEXT0`

Connect the typed `compile_ll` ABI to the transaction. Required proof:

```text
valid in-memory LLVM text -> object
invalid text -> typed verification error
target mismatch -> typed error
output collision -> no partial artifact
parallel compile -> isolated results
Python child = 0
MIR/source inference = 0
```

### `LLVM-NATIVELIB0-HAKOPORT0`

Move `env.codegen.compile_ll_text` behind one thin, typed host port that calls
the new library ABI. Remove direct external-tool ownership from the Rust host
provider. The `.hako` caller observes one result or one typed failure; empty
string/`None` swallowing and alternate-provider retry are forbidden.

Add a sibling ny-llvmc LLVM-text input mode that consumes the same ABI. It is
an adapter, not an independent code generator.

## 3. First FastMem vertical

FastMem owns its source, proof, representation, site, access-plan, and escape
products. The LLVM series owns only the last steps:

```text
sealed FastMem FieldLoad access plan
-> HAKOLL-FASTMEM-FIELDLOAD0
-> NATIVELIB-LLTEXT-OBJECT0
-> NYLLVMC-ROUTE-PROOF0
```

### `FASTMEM-LLVMLITE-REF0`

Before new lowering, freeze the 17-row llvmlite reference matrix:

```text
implementation present
positive IR construction proof
link/execution proof
runtime symbols
target assumptions
proof-token assumptions
known semantic gaps
```

Add missing positive fixtures only when they describe current V0 behavior. Do
not add new V1 semantics or make llvmlite an authority.

### `HAKOLL-FASTMEM-FIELDLOAD0`

Extend the Hako LLVM-text emitter for exactly:

```text
TableIndex:
  RawTableView<PageMapV1> x proven usize -> LayoutRef<PageMeta>

FieldLoad:
  LayoutRef<PageMeta> x sealed field id -> scalar usize
```

The emitter consumes the producer-sealed access plan. It does not parse field
names, reconstruct offsets, accept Integer as a pointer, or read final MIR to
invent facts.

### `NATIVELIB-LLTEXT-OBJECT0`

Compile that emitted LLVM text through `hako_llvmc_compile_ll_v1`. The first
slice may use the contained external-tool transaction. It must not use the
legacy MIR JSON C lowerer or Python.

### `NYLLVMC-ROUTE-PROOF0`

Prove the same LLVM-text library operation through ny-llvmc's CLI adapter.
This is route parity, not a second lowering implementation.

First-vertical acceptance:

```text
.hako emitter instruction shape = expected
native-library executable result = C fixture result
llvmlite oracle result = same
Python child on selected production route = 0
legacy C MIR JSON lowerer on selected route = 0
automatic fallback/replay = 0
helper/boxing/dynamic field lookup in hot load = 0
```

## 4. In-process LLVM promotion

### `LLVM-NATIVELIB0-LLVMAPI0`

Replace the external-tool implementation behind the stable library ABI with
the LLVM C API:

```text
memory buffer parse
module verification
target lookup
target-machine creation
data-layout match
object emission
diagnostic ownership
```

No generated `.ll` file, `opt`, `llc`, compiler child, or shell is used. Link
may remain a separately declared native tool transaction until a link-library
owner is selected.

Acceptance:

```text
same ABI and .hako caller
same object semantics
host and one synthetic non-host target proof
parallel sessions green
tool child count for object emission = 0
llvmlite child count = 0
external-tool failure fallback = 0
```

### `LLVM-NATIVELIB0-PARITY0`

Compare external-tool keep, in-process LLVM C API, and llvmlite oracle on a
closed fixture set. Compare result, verifier outcome, target layout, selected
IR shape, and executable behavior. Exact object bytes are not required.

After this row, in-process LLVM is the sole object-emission owner for the
selected library ABI.

## 5. Hako emitter coverage and automatic-route retirement

Do not port all Python instructions blindly. Open one Hako emitter coverage
row only from an exact production unsupported-shape receipt.

Initial family buckets are:

```text
HAKOLL-COVERAGE0-CORECFG0
HAKOLL-COVERAGE0-PHI0
HAKOLL-COVERAGE0-CALL0
HAKOLL-COVERAGE0-OBJECTFIELD0
HAKOLL-COVERAGE0-COLLECTIONSTRING0
HAKOLL-COVERAGE0-OWNERSHIPWEAK0
```

Each row proves one closed input family through:

```text
MIR/plan authority
-> Hako LLVM text
-> native library
-> executable
```

It must not widen the legacy C JSON lowerer and llvmlite simultaneously.

Retire automatic Python ingresses independently:

```text
LLVMLITE-AUTO0-STAGE1-REPLAY-RET0
LLVMLITE-AUTO0-ENV-CODEGEN-RET0
LLVMLITE-AUTO0-GENERIC-CAPI-RET0
LLVMLITE-AUTO0-HAKO-AOT-RET0
```

For each ingress:

```text
supported input -> selected Hako/native-library route
unsupported input -> typed unsupported error
Python retry -> 0
alternate C generic export -> 0
failure timing and artifact atomicity -> fixed
```

No row may claim success parity by silently dropping formerly accepted inputs.
Required production families must close first; non-required legacy inputs get
an explicit compatibility/retirement decision.

## 6. Tool truth and production graduation

### `LLVMLITE-TOOL0-TRUTH0`

Fix every tool, README, CI job, and smoke whose name claims llvmlite without an
actual selector. Explicit keep jobs must pass an exact driver/provider/replay
selection and assert the route receipt. Non-keep jobs must stop using harness
names and assert Python child count zero.

### `LLVMLITE-DEADAPI0-RET0`

Retire zero-consumer direct Python helpers and stale wrapper APIs only after a
source and cfg census. Do not delete shared oracle code here.

### `LLVMLITE-PROD0-G0`
#### `LLVMLITE-PROD0-G0-D1-ROUTE-REQUEST-BOUNDARY` (closed)
Decision: ordinary Boundary, explicit llvmlite keep, and selected Dynamic Boundary
  are separate requests; `llvm_use_harness()` alone never selects production.
Source authority: route plan, driver/provider/replay request, child command, and
  selected Boundary artifact receipt. Non-authority: labels, Python output, hints,
  or a second lookup. Fail-fast: unknown ingress, inherited replay, native retry,
  generic implicit harness, plugin `Err=>Ok(None)`, or untyped unsupported input.
Receipt: route policy and named C/API admissions are closed; no G1/G2/G3 or
  source deletion is claimed.

#### `LLVMLITE-PROD0-G0-CENSUS-R0` (fast; active)
Decision: build one source-backed matrix for every active Python/llvmlite ingress,
  retry edge, C/AOT export, runner/tool, CI/smoke/perf root, and explicit keep root.
Source authority: caller/child-process owner plus the existing W6 Boundary receipt.
Non-authority: environment names, static labels, Python output, and archive-only code.
Fail-fast: any unclassified caller, native-failure retry, or automatic child path
  blocks G1. Acceptance: `llvmlite-production-ingress-census-v0.json` has
  production=0, explicit keep roots enumerated, positive/negative evidence per row,
  and one reusable guard; child observation ordinary=0, compat=1, replay=0;
  direct keep/debug/smoke roots are enumerated; `LegacyAmbientKeep` and
  harness-or-fallback remain explicit G1 inputs; no route behavior, source
  deletion, G2/G3, or new semantic/backend authority.
## 7. Later retirement

### `LLVM-NATIVELIB0-LEGACYJSON-RET0`

Retire C MIR(JSON)-to-LLVM semantic lowering only after every surviving daily
family has a Hako LLVM-text producer and all generic JSON callers are gone.
The native library keeps LLVM-text/object/link responsibilities.

### `LLVMLITE-KEEP0-RET0`

Delete/archive llvmlite only after an independent semantic oracle exists.
That oracle may be a checked interpreter plus LLVM differential fixtures; it
must not be the same production emitter compared with itself.

## Stop conditions

Stop the current row if it requires:

1. automatic fallback from native/C/Hako failure to llvmlite;
2. growing `ny-llvmc --driver native` beyond its canary responsibility;
3. implementing one semantic lowering rule in both Hako and new C code;
4. adding FastMem V1 semantics to the legacy C JSON lowerer;
5. treating llvmlite output as source, layout, or representation authority;
6. using host target defaults for a sealed non-host request;
7. calling `system()` with caller-derived text;
8. returning empty/`None` instead of a typed library failure;
9. adding process-global mutable compile state or PID-only temp identity;
10. claiming llvmlite retirement from the default ny-llvmc route alone;
11. deleting a Python keep path before its reference/monitor consumer census;
12. changing the current MirBuilder lane before the reserved handoff;
13. combining a FastMem semantic row with a native-library lifecycle row;
14. extending an existing source/check file at 800 lines or more.

## Decision lock

> **llvmlite graduation is staged, not immediate. The default ny-llvmc
> Boundary route is already C pure-first and non-replay, but that local fact
> does not erase the compiled-stage1, env.codegen, generic C export, hako_aot,
> tool, monitor, and FastMem dependencies that still reach Python. Every
> ingress is source-censused and route-observed before retirement. llvmlite is
> frozen as an explicit oracle and receives no new production authority.
> Long-term lowering ownership is `sealed MIR/backend plan -> .hako LLVM-text
> emitter -> versioned libhako_llvmc_ffi LLVM-text/object API`; ny-llvmc is a
> CLI adapter to that same library, and its Rust native driver remains a
> canary. The existing C pure-first MIR JSON lowerer remains transitional and
> is not widened for FastMem V1. The native library first receives a typed
> target/session/error/artifact boundary, then a safe LLVM-text transaction,
> then an in-process LLVM C API implementation with no object-emission child
> process. FastMem's first TableIndex/FieldLoad vertical is emitted in Hako,
> compiled by that library, compared against llvmlite and C, and admitted only
> with Python, replay, fallback, and legacy JSON lowering all absent from the
> selected production route. Production graduation means automatic Python
> reachability zero; source deletion remains a later independently gated
> oracle-retirement row.**
