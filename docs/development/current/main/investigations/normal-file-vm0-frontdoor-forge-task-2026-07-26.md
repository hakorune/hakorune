---
Status: Accepted execution task order
Date: 2026-07-26
Decision: NORMAL-FILE-VM0-FAMILY-D0-FORGE-FRONTDOOR
Scope: forge one typed normal-file front door over the closed Raw VM-reference kernel
ceremony_tier: T2 new source/front-door authority (no production caller)
sunset_id: NORMAL-FILE-VM0-FORGE-PROOF-SUNSET-001; bind NORMAL-FILE-VM0-LEGACY-CALLER-SUNSET-001 only if D2 maps an old caller
proof_inventory_before: NoBoundedCallerFamily D0 evidence + closed Raw VM-reference compile/execution proofs
new_proofs: fixed profile, one-read source receipt, typed handoff, correspondence ledger, semantic matrix, reuse matrix, caller-zero guard
retired_or_merged_proofs: none; prior D0 evidence remains historical input
net_proof_delta: positive bounded T2 scaffold, repay at D2 or explicit forge retirement
sunset_budget: one disconnected forge proof scaffold; no production authority
sunset_row: NORMAL-FILE-VM0-FORGE-PROOF-RETIRE0-S0 (reserved; only if D2 rejects or supersedes the forge)
retire_when: D2 consumes VerifiedNormalFileVmForgeV1 or rejects the forge and proves all forge consumers are zero
budget_repayment_evidence: one reusable front-door guard, one semantic/reuse matrix, and one D2 proof product; no per-row shell guards
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/investigations/normal-file-vm0-forge-task-2026-07-25.md
  - docs/development/current/main/investigations/normal-file-vm0-family-d0-no-candidate-question-2026-07-25.md
  - docs/development/current/main/investigations/normal-entry-cutover-d1-consultation-2026-07-25.md
  - docs/reference/language/function-exit-and-entry-result.md
Supersedes:
  - normal-file-vm0-forge-task-2026-07-25.md
  - normal-file-vm0-family-d0-no-candidate-question-2026-07-25.md
---

# NormalFileNoImportVmReferenceV1 front-door forge

> Accepted and parked only while
> `LANGUAGE-DOCS-POSTFIX-CATCH-D1-CLOSEOUT` performs a short docs-only
> preemption. Resume this exact Forge0 row afterward; its scope and
> production-caller-zero law are unchanged.

## Decision

```text
Decision: NORMAL-FILE-VM0-FAMILY-D0-FORGE-FRONTDOOR
Status: accepted
Choice: A — new front-door owner; no legacy caller mapping
```

The previous D0 result remains sealed evidence:

```text
plain source-hint production sites = 6
admissible existing caller         = 0
NoBoundedCallerFamily              = sealed
```

None of the six sites may be reinterpreted as the future normal family. This
forge creates the missing front door and does not select a production caller.

## First executable row

```text
NORMAL-FILE-VM0-FRONTDOOR-FORGE0-S0
```

This row may implement the typed front-door contracts and proof fixtures, but
must keep these counts unchanged until the later D2 decision:

```text
new normal production caller = 0
normal/default cutover       = 0
legacy caller mapping        = 0
fallback/retry               = 0
```

## Owner chain

```text
NormalFileRequestV1
  -> SealedNormalEntryProfileV1
  -> PreparedNormalFileRequestV1
  -> LoadedNormalFileSourceV1
  -> PreparedNormalFileSourceV1
  -> PreparedNormalFileVmHandoffV1
  -> RawPublishedCompileRequestV1
  -> MirCompiler::compile_raw_published_v1
  -> RawPublishedInvocationV1
  -> existing exact Raw VM-reference activation
  -> SourceEntryResultV1
  -> ProcessExitProjectionV1
  -> VmReferenceProcessOutcomeV1
  -> RawVmReferenceRunReportV1
```

The front door owns only file/profile preparation and the consuming handoff.
The existing Raw compiler, exact entry execution, process projection, and
diagnostic owners remain the sole authorities for those stages.

## Fixed profile

```text
profile                  = NormalFileNoImportVmReferenceV1
profile owner            = SealedNormalEntryProfileV1
source origin            = SingleFileUtf8V1
file read                = exactly once
parser                   = canonical parser exactly once
AST transport            = internal BareAst only
source rewrite           = forbidden
@local pre-expansion     = forbidden
local declaration strip  = forbidden
imports / using          = NoImports; source using is typed rejection
macros / plugins         = unsupported; typed rejection
REPL / JSON / arguments  = unsupported; typed rejection
optimization             = CanonicalDefaultOptimizedV1
artifact output          = no artifact contract
execution                = fresh Rust MirInterpreter
entry                    = sealed source entry + exact physical target
process profile          = CanonicalProcessExitV1::V1
fallback / retry         = zero
```

The sealed profile is a closed variant, not a bag of caller booleans:

```rust
enum NormalEntryProfileV1 { // private payload; not a second authority
    FileNoImportVmReferenceV1,
}

struct SealedNormalEntryProfileV1 {
    profile: NormalEntryProfileV1,
    downstream: RawVmReferenceSupportProfileV1,
    _seal: SealedNormalEntryProfileSealV1,
}
```

`SealedNormalEntryProfileV1` is the only profile authority; the payload enum
cannot be constructed or consumed outside its sealed constructor/terminal.

It consumes the already-supported Raw compile and VM-reference profiles by
value. The normal front door must not reconstruct Raw NarrowV1, Main policy,
fresh-VM policy, or process policy by matching the same booleans again.

## Prepared products

```rust
struct NormalFileRequestV1 {
    source_file: Box<std::path::Path>,
    profile: SealedNormalEntryProfileV1,
}

struct NormalFileSourceReceiptV1 {
    source_identity: Box<str>,
    utf8_len: usize,
    read_count: u8,  // sealed as 1
    parse_count: u8, // sealed as 1
    _seal: NormalFileSourceReceiptSealV1,
}

struct LoadedNormalFileSourceV1 {
    source_file: Box<std::path::Path>,
    source_text: Box<str>,
    receipt: NormalFileSourceReceiptV1,
    _seal: LoadedNormalFileSourceSealV1,
}

struct PreparedNormalFileSourceV1 {
    source_file: Box<std::path::Path>,
    ast: ASTNode,
    profile: SealedNormalEntryProfileV1,
    receipt: NormalFileSourceReceiptV1,
    _seal: PreparedNormalFileSourceSealV1,
}

struct PreparedNormalFileVmHandoffV1 {
    invocation: RawVmReferenceInvocationV1,
    source: NormalFileSourceReceiptV1,
    _seal: PreparedNormalFileVmHandoffSealV1,
}
```

The implementation keeps the compile and execution fields opaque inside the
existing `RawVmReferenceInvocationV1`. This is a deliberate visibility
preservation: `compile_raw_published_v1` remains MIR-internal, while the
front door consumes one already-paired Raw support profile into that invocation.
It does not widen the Raw compile kernel or reconstruct its policies.

The handoff is issued by one consuming adapter owned by the existing Raw
contract, not by a normal-side `narrow_v1` policy reconstruction:

```rust
impl PreparedNormalFileVmHandoffV1 {
    fn into_raw_vm_reference_invocation(self) -> RawVmReferenceInvocationV1;
}
```

That adapter accepts only the sealed `FileNoImportVmReferenceV1` profile and
its downstream Raw profile; arbitrary module names, entry policies, backend
profiles, or caller booleans cannot be supplied.

`NormalFileRequestV1` is constructed only from a closed, typed invocation
facts selector. It has no backend, REPL, JSON, import, macro, plugin, or
optimization booleans. The selector rejects those conflicts before I/O and
the forge module is not wired into default dispatch or any existing caller.

`PreparedNormalFileSourceV1` has one consuming exit:

```rust
fn prepare_raw_vm_handoff(self) -> PreparedNormalFileVmHandoffV1;
```

No bare AST accessor, source rewrite terminal, legacy compiler adapter, or
second Raw-policy constructor is allowed.

Source preparation failures are typed and owner-retaining:

```rust
enum RejectedNormalFileSourceV1 {
    Profile { request: NormalFileRequestV1, error: NormalFileProfileErrorV1 },
    Read { request: NormalFileRequestV1, error: NormalFileReadErrorV1 },
    Parse {
        loaded: LoadedNormalFileSourceV1,
        error: NormalFileParseErrorV1,
    },
}
```

Each rejection exposes only `stage()`, `error()`, and `discard(self)`. The
loaded parse failure retains the one-read receipt; no retry, second parser, or
legacy source preparation is permitted.

## Source preparation law

Profile-only conflicts reject before file I/O:

```text
REPL / JSON / emit mode / script arguments
using or import request / macro / @local / plugin mode
non-VM backend / unoptimized override
```

Source-dependent rejection follows exactly one read and one parse:

```text
one UTF-8 file read
  -> LoadedNormalFileSourceV1 with a sealed read receipt
  -> one canonical parse
  -> source-profile validation
  -> typed success or discard-only rejection
```

The loaded source owner carries the exact UTF-8 payload and a non-Copy receipt;
the path alone is not evidence of a one-read boundary. No later stage may read
the path again.

Forbidden source behavior:

```text
filename fallback
source text rewrite
@local pre-expansion
local declaration stripping
prelude/import merge
second parser
legacy source preparation
```

## Existing-kernel correspondence

The only compile handoff is:

```text
PreparedNormalFileSourceV1
  -> RawPublishedCompileRequestV1
  -> compile_raw_published_v1
```

The forge must not add:

```text
compile_with_source
compile_legacy
MirBuilder::build_module
independent Return finalizer
independent postprocess
independent publication
NYASH_ENTRY or module-symbol entry search
independent status conversion
diagnostic status mutation
```

The existing `compile_raw_with_source` compatibility API remains unchanged;
it may continue to erase Raw evidence for its own contract. The forge path
retains `RawPublishedInvocationV1` until existing VM-reference activation.

## Internal task order

### `FORGE-CONTRACT0`

Add the closed normal profile, request/rejection vocabulary, source receipt,
and owner seals. No production caller.

### `FORGE-SOURCE0`

Implement profile preflight, exactly one file read, exactly one canonical parse,
and move-only `PreparedNormalFileSourceV1`. Profile and parse failures retain
the exact request/source owner and expose `stage`, typed `error`, and
`discard(self)` only.

### `FORGE-HANDOFF0`

Consume the existing Raw support profile by value and issue exactly one
`RawPublishedCompileRequestV1`. Do not duplicate Raw policy or execution.

## Execution ledger

### Closed 2026-07-26: `FORGE-CONTRACT0` -> `FORGE-SOURCE0` -> `FORGE-HANDOFF0`

The disconnected owner now lives in
`src/runner/reference/normal_file_vm_frontdoor.rs` and remains below the
source-file boundary. It seals one fixed no-import profile, performs one
UTF-8 read and one canonical parse, rejects parsed `using`/`import` before a
Raw handoff, and consumes the paired Raw support profile into one opaque
`RawVmReferenceInvocationV1`.

The neutral `RawVmReferenceSupportProfileV1` is the one compile/execution
pairing owner for both the existing supported reference request and this
forge. The front door has no execution call and no production runner caller.

Focused evidence:

```text
python3 tools/checks/lib/entry_result_projection0_s3_owner_guard.py
python3 tools/checks/lib/normal_file_vm0_frontdoor_forge_guard.py
cargo test -q --lib runner::reference::normal_file_vm_frontdoor
cargo test -q --lib runner::reference::normal_file_vm_frontdoor --features vm-reference
```

### Closed 2026-07-26: `FORGE-CORRESPONDENCE0`

The front door's sole test-only correspondence consumer moves the opaque
`RawVmReferenceInvocationV1` into the existing
`MirCompiler::run_raw_vm_reference_v1` terminal. It neither opens the Raw
published owner nor adds a front-door execution method. Two source files run
through one `MirCompiler` as `42 -> 255`, proving that the handoff reaches the
existing typed compile/publication, exact-target execution, process projection,
and diagnostic terminal without a second compiler, entry selector, or status
adapter.

The proof records the existing legacy-result deltas rather than normalizing
them away:

```text
legacy quiet runner: modulo/widening and non-numeric fallback mappings
legacy MIR interpreter: independent integer/bool/non-numeric mapping
legacy VM execution helper: independent cast/status mapping
canonical Raw lane: exact 0..255 integers, Unit=0, typed status-70 faults
```

This is a correspondence proof, not a normal-route activation or a full
reuse claim. Next internal row: `FORGE-SEMANTIC0`. `FORGE-REUSE0`, D2, and
every production caller remain open.

### Closed 2026-07-26: `FORGE-SEMANTIC0-S0`

The front-door source-text fixture now proves the first bounded Script matrix
through one file read, one canonical parse, the opaque handoff, and the
existing Raw VM-reference terminal:

```text
empty / Void                       -> Unit, status 0
Integer 0 / 255                    -> exact status
Bool / Float / String              -> unsupported-result Fault, status 70
Print / Local / Assignment / CompoundAssignment
                                   -> Unit, status 0
Integer 256                         -> range Fault, status 70
```

The same compiler executes the matrix, but this row does not close the
separate reuse proof: its ordered fixture is semantic evidence, not the
named rejection/fault-after-success matrix required by `FORGE-REUSE0`.
`null` remains an explicitly uncredited observation because current Raw
source facts collapse it with Void. Next internal row:
`FORGE-SEMANTIC0-S1`.

### Closed 2026-07-26: `FORGE-SEMANTIC0-S1`

The front-door Function/Main boundary fixture records the current NarrowV1
contract without broadening it:

```text
Main final expression / fallthrough -> AppFixedVoid, status 0
empty static helper plus Main       -> admitted, status 0
Main explicit return value / Unit   -> eligibility rejection
ordinary top-level function         -> eligibility rejection
non-Main static-box candidate       -> eligibility rejection; no entry retry
```

This is not an `ExplicitReturnOnly` implementation claim. `AppFixedVoid` is
current Raw compatibility evidence only. A normal-profile requirement for
Main explicit Return or ordinary callable bodies must stop at
`FUNCTION-EXIT-F1-NORMAL-CAPABILITY0-D0`; the front door may not repair the
result from a lowered value or select a replacement entry. Next internal row:
`FORGE-SEMANTIC0-S2`.

### `FORGE-CORRESPONDENCE0` (closed)

Prove that the forge uses the existing Raw compile/publication and exact
VM-reference execution chain, including source-entry continuation, route
pairing, process projection, and diagnostic ownership.

The front door does not re-own the source-entry continuation; it only proves
that the downstream Raw handoff consumes the existing continuation and exact
target. Existing Raw route/profile evidence remains the downstream authority.

The proof also records, without normalizing away, the legacy result deltas:

```text
legacy quiet runner: modulo/widening and non-numeric fallback mappings
legacy MIR interpreter: independent integer/bool/non-numeric mapping
legacy VM execution helper: independent cast/status mapping
canonical Raw lane: exact 0..255 integers, Unit=0, typed status-70 faults
```

This is a difference ledger, not a parity claim. D2 requires the selected
front-door behavior to use the canonical Raw projection only.

### `FORGE-SEMANTIC0-S0` — Script source-text matrix

The first executable semantic slice uses only source text through the
front-door path:

```text
file source
  -> one-read / one-parse front door
  -> opaque Raw VM-reference invocation
  -> existing exact Raw terminal
```

It may add no source semantic repair. Its rows are:

```text
Script empty / explicit Void                 -> Unit, status 0
Script final Integer 0 / 255                 -> exact process status
Script final Bool / Float / String            -> typed process Fault, status 70
Script final Print                            -> Unit, status 0
Script final Local                            -> Unit, status 0
Script final Assignment / CompoundAssignment -> Unit, status 0
out-of-range Integer                          -> typed range Fault, status 70
```

Every row is credited only when it is observed through the front door. Direct
AST Raw fixtures remain downstream regressions, not normal-file evidence.

`null` is deliberately not included as a successful Unit row: the current
Raw recipe folds Null and Void into one origin. `FORGE-SEMANTIC0-S0` records
that observation but cannot claim the normative Null-versus-Void distinction.
It must stop at `SCRIPT-RESULT-NORMAL-CAPABILITY0-D0` before adding a
successful Null claim.

### `FORGE-SEMANTIC0-S1` — Function/Main boundary matrix

This row is an evidence-and-routing boundary, not permission to broaden Raw.
It classifies:

```text
Main empty/fallthrough and final expression
Main explicit return value / Unit
helper inventory plus Main and non-Main entry decoys
ordinary function explicit value / Unit / fallthrough
```

Current Raw NarrowV1 admits only an empty helper plus an `AppFixedVoid` Main
route. It does not admit Main explicit Return or ordinary callable bodies.
The row therefore records those as named pre-physical capability rejections;
it must not translate them into a last lowered ValueId or repair them in the
front door.

Any D2-required admission gap returns to:

```text
FUNCTION-EXIT-F1-NORMAL-CAPABILITY0-D0
```

before this forge resumes.

### `FORGE-SEMANTIC0-S2` — Annotation and result-carrier matrix

Blocked pending `RESULT-CARRIER-NORMAL-CAPABILITY0-D0`. The front door may not
classify annotations or composite results until that decision selects the
source/profile rejection authority.

Keep annotations and result carriers separate from S0/S1:

```text
: void + Unit                         -> current admitted behavior or named reject
: void + non-Unit                     -> named contract/capability reject
non-Void annotation + fallthrough     -> named missing-result reject
unannotated Integer/Bool/Float/String -> source-result observation
Object/Box/Array/Future/WeakRef        -> precise first-profile rejection
```

If the current lane cannot name the required rejection before publication,
stop at `RESULT-CARRIER-NORMAL-CAPABILITY0-D0`; do not use a front-door
fallback or a dynamic carrier workaround.

Canonical authorities remain `ExplicitReturnOnly`,
`ScriptLastExpressionOrUnit`, and `ProcessExitProjectionV1`; a Builder-last
`ValueId` is never source return authority.

### `FORGE-REUSE0-S0` — pre-compiler rejection non-poisoning

Use one already-created `MirCompiler`, perform each front-door rejection, then
run a known success through the same compiler:

```text
empty-path profile rejection -> success
using/import source-profile rejection -> success
parse rejection -> success
```

These cases do not consume the compiler. The claim is intentionally named
*front-door rejection does not poison a pre-existing compiler*, not compiler
reuse by an unused compiler.

### `FORGE-REUSE0-S1` — compiler and execution reuse

The actual compiler/VM reuse rows are:

```text
Raw compile rejection -> success
canonical process Fault -> success
VM execution Fault -> success
```

All cases must use the front-door opaque handoff and the existing Raw terminal.
Existing Raw-only tests are downstream regression evidence but do not close
Forge-specific rows.

### `FORGE-G0`

Extend the existing Forge guard once, after the matrix is complete. It owns
only structural facts:

```text
front-door production execution method        = 0
front-door production runner/default caller   = 0
test-only existing Raw terminal consumer      = 1
source-text semantic harness                  = 1
all semantic/reuse cases use the front-door path = 1
all touched source/check files                < 800 lines
```

The fixture matrix, not a count of source spellings in a Python guard, is the
semantic SSOT.

### `FORGE-G0`

Require:

```text
sealed profile producer             = 1
prepared source producer            = 1
Raw compile handoff producer        = 1
forge Raw handoff producer           = 1 (route-scoped)
forge Raw compile consumer           = 1 (route-scoped)
global Raw kernel consumers          = existing compatibility/reference routes plus this forge route
new production caller               = 0
legacy fallback/retry               = 0
source rewrite                      = 0
second compiler/finalizer/status    = 0
all touched source/check files      < 800 lines
```

## Proof bundle for D2

Forge0 deliberately creates no production Rust proof carrier. Its D2 input is
one documented evidence bundle, backed by focused front-door fixtures and the
existing structural guard:

```text
sealed fixed profile
+ source receipt
+ Raw-terminal correspondence
+ complete semantic matrix (accepted or named rejection)
+ reuse matrix
+ zero-production-caller receipt
```

The bundle has three separately reported states:

```text
matrix_complete:
  every row is observed as an accepted outcome or a named rejection

required_normal_admission:
  every D2-required Function/Main/Script row is green

production_activation:
  false until D2
```

`NORMAL-ENTRY-CUTOVER-D2` consumes this evidence bundle and may authorize one
production caller only after its required rows are green. It must not implement
missing language or result capabilities; incomplete rows return to their exact
capability owner instead. Real-binary parity belongs after D2, in
`NORMAL-FILE-VM0-PARITY0-P0`, because caller-zero Forge0 cannot prove a normal
production route.

## Retirement and sunset

This forge replaces no existing caller and therefore issues no legacy sunset.
If D2 later selects an exact old caller, bind retirement only to that caller:

```text
sunset_id          = NORMAL-FILE-VM0-LEGACY-CALLER-SUNSET-001
retirement owner   = NORMAL-FILE-VM0-CALLER-RETIRE0
retirement row     = NORMAL-FILE-VM0-LEGACY-CALLER-RETIRE0-S0
```

`compile_with_source` retirement remains a separate `MIRBUILDER-LEGACY-FENCE0`
decision. Do not require repo-wide token zero.

## Non-claims

```text
existing six callers selected
new production caller
normal/default compile cutover
compile_with_source change
RAW-MINIMAL-MIR-JSON activation
using/import/macro/plugin support
source rewriting
general VM/MIR status-law change
LLVM/native/ny_main
JSON / Program(JSON v0)
REPL / Stage1 / WASM
executor / selfhost / fastmem
old Raw retirement
App AnyStatement promotion
CUT0
```
