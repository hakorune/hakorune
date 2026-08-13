---
Status: Parked post-W6 handoff board; current production mutation is forbidden
Decision: staged llvmlite graduation and native-library ownership selected
Date: 2026-07-22
Scope: LLVM route truth, native library boundary, Hako LLVM-text ownership, and llvmlite retirement
Current-lane effect: none; D-prime HEADERPORT0 remains authoritative
Reserved activation: after `DYNAMIC-V2-AOT-ACTIVATION-I0-W6` C5 receipt-gated
selected-caller evidence
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
Decision: W6-E must close the Boundary route before llvmlite graduation begins.
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

`W6-E-C5 STATIC-RECEIPT-GATED-LIVE-INSTALL-R0` has landed the existing
receipt-gated selected runner terminal, including the explicit runtime-archive
path required by the Boundary link ABI. The remaining `W6-E-C6
FINAL-LIVE-PUBLICATION-D0` design stop must name the existing root owner/API
and scoped before/after census before G1 starts. Neither row creates a new
transaction or retires ordinary compatibility. Until C6 is accepted, this
board remains parked. Once C6 and its scoped receipt/caller evidence exist, G1
retires automatic production reachability, G2 removes Python/llvmlite from
default build/CI/perf gates, and G3 separately decides source/archive removal.
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

```text
W6-E receipt and caller census
  -> LLVMLITE-PROD0-G0 (G1: automatic production retirement)
  -> LLVMLITE-AUTO0      (G2: default build/CI/perf independence)
  -> LLVMLITE-KEEP0-RET0 (G3: source/archive retirement, separate approval)
```

### G1 — remove production reachability

Owner: W6-E Boundary artifact receipt plus route/child-process census.

Acceptance is all of the following: selected Dynamic Boundary caller `= 1`,
selected Dynamic old raw/JoinIR edge `= 0`, ordinary compatibility edge may
remain `= 1`, Boundary artifact receipt/fence `= 1`, automatic
Python/llvmlite consumer `= 0`, native failure -> harness retry `= 0`,
generic C export -> implicit harness `= 0`, and unsupported native input
becomes a typed fail-fast. The explicit harness/oracle roots remain available
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

### `LLVMLITE-ROUTE0-IDENTITY0`

Separate actual selector from descriptive or stale flags.

```text
actual llvmlite selector:
  --driver harness
  explicit llvmlite provider
  explicit compat_replay=harness

not sufficient evidence:
  NYASH_LLVM_USE_HARNESS=1
  --harness PATH without --driver harness
  filename/comment containing llvmlite or harness
```

Fix tool names, comments, README examples, and gates so every claimed route is
backed by an actual selection receipt. Behavior changes belong to later
retirement rows, not this truth-sync row.

### `LLVMLITE-ROUTE0-OBSERVE0`

Add one route receipt at the selected boundary. It records:

```text
request id
entry family
driver
library export
recipe
compat replay
Python child started: yes/no
artifact result
```

The receipt is test/diagnostic observation, not semantic authority. It may not
select a backend, retry, alter an error, or read a second route after failure.

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

Required terminal counts:

```text
automatic production Python ingresses = 0
default production llvmlite reachability = Unreachable
unknown watched reachability = 0
native/library failure -> llvmlite retry = 0
generic C export -> implicit harness = 0
stage1 forced harness replay = 0
default build Python requirement = 0
default perf/acceptance Python child = 0

explicit llvmlite keep roots = closed enumerated set
explicit keep route receipts = one per execution
llvmlite source deletion claim = 0
```

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
