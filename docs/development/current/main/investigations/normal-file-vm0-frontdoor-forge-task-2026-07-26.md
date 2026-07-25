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

Next internal row: `FORGE-CORRESPONDENCE0`. `FORGE-SEMANTIC0`,
`FORGE-REUSE0`, D2, and every production caller remain open.

### `FORGE-CORRESPONDENCE0`

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

### `FORGE-SEMANTIC0`

Build a matrix where every row is accepted or a typed profile/capability
rejection before the relevant effects:

```text
ordinary function: explicit value, explicit Unit, Unit fallthrough
Main.main/0: explicit return, Unit fallthrough, no implicit final-expression return
Script: final expression=Value; Print/Local/Assignment/CompoundAssignment=Unit
empty body; void/null; annotated and unannotated results
Integer / Bool / Float / String
Object / Box / Array / Future / WeakRef = typed first-profile rejection
helper plus Main; non-Main entry candidates never scanned or retried
```

Canonical authorities remain `ExplicitReturnOnly`,
`ScriptLastExpressionOrUnit`, and `ProcessExitProjectionV1`; a Builder-last
`ValueId` is never source return authority.

The following are not credited by existing Raw fixtures and must be either
proven by forge-specific fixtures or recorded as typed NarrowV1 rejection:

```text
Main explicit return
ordinary non-empty helper-function result
Null-versus-Void source/result distinction
process-fault -> same-compiler success
VM-execution-fault -> same-compiler success
front-door profile/source/parse rejection -> same-compiler success
```

### `FORGE-REUSE0`

Prove same-`MirCompiler` reuse for:

```text
success -> success
profile/source/parse rejection -> success
compile rejection -> success
canonical process Fault -> success
VM execution Fault -> success
```

Existing Raw tests are not credited automatically. Forge-specific fixtures
must prove the profile/source rejection and process/VM-fault reuse rows; until
those fixtures are green, the reuse section is incomplete.

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

## Proof product for D2

```rust
struct VerifiedNormalFileVmForgeV1 {
    profile: SealedNormalEntryProfileV1,
    source: VerifiedNormalFileSourceContractV1,
    correspondence: VerifiedRawVmCorrespondenceV1,
    semantics: VerifiedNormalFileSemanticMatrixV1,
    reuse: VerifiedCompilerReuseMatrixV1,
    callers: ZeroProductionCallerReceiptV1,
    _seal: VerifiedNormalFileVmForgeSealV1,
}
```

`NORMAL-ENTRY-CUTOVER-D2` consumes this single proof product. D2 may authorize
one production caller only after the proof is complete. D2 must not implement
missing language or result capabilities; incomplete rows return to their
exact capability owner instead.

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
