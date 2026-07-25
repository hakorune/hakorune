---
Status: active execution task
Date: 2026-07-26
Decision: RESULT-CARRIER-NORMAL-CAPABILITY0-prime-r1
Scope: preserve existing Raw rejection facts and expose them to Forge proof
ceremony_tier: T1 bounded owner/evidence refactor inside Forge0
grammar_delta: 0
runtime_process_delta: 0
production_caller_delta: 0
Related:
  - docs/development/current/main/investigations/result-carrier-normal-capability0-design-stop-2026-07-26.md
  - docs/development/current/main/investigations/normal-file-vm0-frontdoor-forge-task-2026-07-26.md
  - docs/reference/language/function-exit-and-entry-result.md
---

# RESULT-CARRIER-NORMAL-CAPABILITY0-S0

## Accepted boundary

```text
NormalFileNoImportVmReferenceV1 carrier set:
  Unit / Integer / Bool / Float / String

Bool / Float / String:
  source result succeeds; canonical process projection reports status 70

composite or owner-bearing result:
  no decode-plan producer
  existing pre-physical Raw capability rejection
  never execute then convert to a process Fault

annotations:
  remain existing Raw/function-exit rejection
  front door does not inspect return_type_name or MirType

Null / Void:
  observed only; normal provenance credit = 0
```

The front door continues to own only one read, one canonical parse, and the
opaque Raw handoff. It gains no AST result classifier, annotation classifier,
MirType lookup, SourceEntryResult constructor, retry, fallback, or execution
authority.

## Owner changes

### FACTS0 — preserve facts already computed by Raw

No new semantic classification is allowed. Preserve only facts that the same
Raw owner has already inspected:

```text
Main metadata:
  parameter count / parameter-declaration count / return-annotation presence
  / uses / contracts / rune attributes

body unsupported statement:
  existing located path + If/Loop/LoopRange/Return/Break/Continue/ScopeBox

eligibility work rejection:
  existing statement index + RawRootWorkKindV1
```

`RawRootWorkKindV1::UnsupportedSurface` remains deliberately coarse. S0 must
not pretend that Array, Map, Record, or Call have distinct typed causes unless
their source classifier independently provides one.

Suggested implementation split:

```text
raw_root_source_facts.rs
  RawAppMainMetadataFactsV1

raw_root_body_recipe.rs
  RawUnsupportedBodyStatementKindV1

raw_root_source_facts/recipe_projection.rs
  one located-statement -> kind projection helper

raw_root_eligibility.rs
  preserve item.kind() in UnsupportedWork

raw_root_eligibility_p0.rs
  focused facts assertions
```

`raw_root_plan0.rs` is intentionally unchanged: it is already close to the
source-file boundary and already owns the work classification.

### REJECTION0 — typed Raw VM-reference evidence

Add a typed core that retains the existing compile and activation owners:

```rust
enum RejectedRawVmReferenceRunV1 {
    Compile(Box<RejectedRawPublishedCompileV1>),
    Activation(Box<RejectedRawVmReferenceActivationV1>),
}

fn run_raw_vm_reference_owned_v1(
    &mut self,
    invocation: RawVmReferenceInvocationV1,
) -> Result<RawVmReferenceRunReportV1, RejectedRawVmReferenceRunV1>;
```

It exposes only `stage()`, borrowed `evidence()`, `discard(self)`, and one
`into_public_string(self)` adapter. It never exposes an owner, retry, resume,
or fallback terminal.

The existing `run_raw_vm_reference_v1` remains the compatibility String
adapter. Its existing `repl_mode` preflight is outside this typed core because
it is a pre-invocation compatibility rejection with no Raw published owner;
its public String behavior remains unchanged.

The borrowed compile evidence view belongs in the compiler layer so Forge tests
can inspect preserved stage/cause without opening a rejection owner.

### Canonical completion

Add one canonical-only infallible projection terminal:

```text
CompletedRawVmReferenceExecutionV1
  -> complete_canonical_source_entry()
  -> VmReferenceProcessOutcomeV1
```

The canonical projection helper belongs to `ProcessExitProjectionV1`; do not
duplicate its match or use `expect` in VM execution. Generic legacy-profile
projection remains disconnected/test-only.

## Internal execution order

```text
RESULT-CARRIER-NORMAL-CAPABILITY0-S0-FACTS0
  -> RESULT-CARRIER-NORMAL-CAPABILITY0-S0-REJECTION0
  -> FORGE-SEMANTIC0-S2
  -> FORGE-REUSE0-S0
  -> FORGE-REUSE0-S1
  -> FORGE-G0
  -> NORMAL-ENTRY-CUTOVER-D2
```

Use 3–5 buildable commits. Every modified/new source or check file remains
below 800 lines.

## Forge S2 / Reuse acceptance

`FORGE-SEMANTIC0-S2` uses a child test module under
`src/runner/reference/normal_file_vm_frontdoor/`; the parent production owner
must not exceed its boundary or gain test-only result logic.

Required observations:

```text
credited Script results:
  empty, Print, Local, Assignment, CompoundAssignment, Integer 0/255,
  Bool, Float, String, Integer 256

observed but uncredited:
  explicit Void, explicit Null

function-exit exclusions:
  Main/helper annotations, Main explicit Return, ordinary top-level function

source/slot exclusions:
  existing UnsupportedWork and UnsupportedProcessGlobalSlot source rows

entry:
  non-Main static box never becomes a retry target
```

Unannotated scalar success is Script-only evidence. This task makes no
ordinary callable or Main annotation admission claim.

Reuse rows use the typed core where a Raw owner exists:

```text
front-door profile/read/parse/using rejection -> later success
Raw compile rejection -> later success
canonical process Fault -> later success
VM execution Fault -> later success
```

## D2 evidence

Forge0 keeps a test-only evidence bundle, not a production dispatch product:

```rust
#[cfg(test)]
struct VerifiedNormalFileVmForgeV1 {
    carrier_matrix: VerifiedNormalResultCarrierMatrixV1,
    function_exit_exclusions: VerifiedFunctionExitExclusionMatrixV1,
    null_void_evidence: NullVoidEvidenceDispositionV1,
    reuse: VerifiedNormalCompilerReuseMatrixV1,
    caller_zero: ZeroNormalProductionCallerReceiptV1,
}
```

It is a D2 fixture/guard aggregation only. It has no runtime consumer and no
production constructor.

## Structural law

```text
Raw App metadata facts producer                 = 1
Raw body unsupported statement kind producer    = 1
Raw eligibility work-kind preservation          = 1

front-door result / annotation / MirType classifier = 0

run_raw_vm_reference_owned_v1                   = 1
run_raw_vm_reference_v1 String adapter           = 1
second Raw compile kernel                        = 0

decode plan new Object/Array/Future/WeakRef      = 0
SourceEntryResult::Object production producer    = 0
composite result -> execute -> Fault              = 0

Main annotation / explicit Return admission      = 0
ordinary callable admission                      = 0
normal production caller / default route delta   = 0
fallback / entry scan / JSON / LLVM delta         = 0
new per-row shell guard                           = 0
```

Extend only `normal_file_vm0_frontdoor_forge_guard.py` at G0.

## Verification

```bash
cargo check --lib
cargo check --lib --features vm-reference
cargo test -q --lib raw_root_source_facts
cargo test -q --lib raw_root_eligibility
cargo test -q --lib source_entry_vm_execution --features vm-reference
cargo test -q --lib runner::reference::normal_file_vm_frontdoor --features vm-reference
python3 tools/checks/lib/normal_file_vm0_frontdoor_forge_guard.py
python3 tools/checks/lib/entry_result_projection0_s3_owner_guard.py
python3 tools/checks/lib/entry_result_projection0_s3_execution_guard.py
bash tools/checks/current_state_pointer_guard.sh
```

## Non-claims

```text
normal/default cutover or production caller
Main explicit-return or annotation admission
ordinary callable admission
dynamic/object result carrier
Null/Void provenance parity
general VM/MIR status-law replacement
JSON, LLVM/native, REPL, executor, selfhost, fastmem, CUT0
```
