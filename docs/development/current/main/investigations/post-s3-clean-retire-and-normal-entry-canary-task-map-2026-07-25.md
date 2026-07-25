# Post-S3 clean retirement and normal-entry canary task map

Decision: `POST-S3-CLEAN-CANARY-prime-r1`

Status: accepted task order. `NORMAL-ENTRY-D0` is closed by this card.
R0A, R0B, G0, PROFILE0, CANARY0, and CANARY-PARITY0/G0 are closed;
implementation authorization now stops at the fresh cutover design frontier:

```text
NORMAL-ENTRY-CUTOVER-D0
```

## Outcome

The complete S3 Raw VM-reference chain is already closed. The next work must
not create another S3 execution owner or widen an existing runner.

The selected order is:

```text
S3 closeout
  -> OLD-RAW-RETIRE0
  -> NORMAL-ENTRY-PROFILE0-S0
  -> NORMAL-ENTRY-CANARY0-S0
  -> NORMAL-ENTRY-CANARY-PARITY0/G0
  -> fresh NORMAL-ENTRY-CUTOVER-D0 decision
```

The old Raw chain is removed first because it has zero non-test callers and
still carries a hard-coded inventory, bare `MirModule` handoff, and
ledger/root-only evidence. Removing that authority before adding a runner
consumer makes the new canary unambiguous.

## Worker inventory

Four independent read-only audits covered runner/CLI, LLVM/native,
JSON/executor/selfhost/fastmem, and old-chain retirement.

### Current production-shaped Raw surface

```text
compile_raw_published_v1
  definition = 1
  consumers  = 2
    compile_raw_with_source compatibility adapter
    run_raw_vm_reference

run_raw_vm_reference
  definition             = 1
  non-test runner caller = 1 (CANARY0; pre-canary inventory was 0)
  same-file test callers = 15

compile_raw_with_source
  definition             = 1
  non-test caller        = 0
```

S3 already owns:

```text
exact sealed Main/main/0 selection
fresh MirInterpreter execution
source-result decode from retained exit evidence
CanonicalProcessExitV1 projection
typed process diagnostics
status 0 / 0..=255 / reserved fault 70
compiler reuse and decoy-entry isolation
```

It does not own source-file preparation, CLI conflict policy, stderr delivery,
or an OS process terminal.

### Normal and artifact routes remain different authorities

The normal source-hint caller manifest still contains:

```text
no-import source-hint callers = 6
explicit-import callers       = 6
normal compile adapters       = 2
direct production build_module callers = 2
```

Those callers span MIR, VM keep/fallback, VM-Hako, LLVM, WASM, bench,
Stage-1, selfhost, and the runtime AST-JSON bridge. Switching
`compile_with_source` now would be a multi-backend cutover.

JSON and Program(JSON v0) produce a bare `MirModule` without the selected
entry continuation, root-exit witness, decode plan, or Raw publication
evidence. General MIR execution still uses module entry discovery and legacy
status mappings. Neither is an adapter for S3.

LLVM/native is also not a one-caller extension. It lacks:

```text
opaque Raw-to-AOT handoff
backend-neutral source-result/decode plan
exact selected-entry transport across MIR JSON/LLVM
runtime range/unsupported-result projection
typed native fault transport
normalized-status-only NyRT ABI
```

Current LLVM/NyRT paths still perform symbol discovery, unsupported-result
zeroing, or positive-handle reinterpretation. They remain separate.

### Old Raw chain

The following old production sources remain compiled:

```text
src/mir/builder/raw_physical_finalization.rs
src/mir/compiler/raw_finalization.rs
```

They own or feed:

```text
RawPhysicalCompleteInvocationV1
RawModuleFinalizerV1
RawFinalizedModuleInvocationV1
ModulePostprocessInputV1::Raw
ModulePostprocessOwnerV1::run_raw
PostprocessEvidenceInputV1::Raw
PostprocessEvidenceSealV1::Raw
legacy external-commit Raw acceptance
```

Their non-test caller count is zero. All observed calls are old local
`#[cfg(test)]` fixtures:

```text
bind/prepare physical finalization = test-only
RawModuleFinalizer prepare/finalize = test-only
run_raw                             = test-only
old external commit                 = test-only
```

Two proofs must move before deletion:

```text
PublishedShell rejection
BuilderReadiness(CurrentModuleOpen) plus exact owner retention
```

Eleven guards mention the two old files. The retirement row removes the three
old-only guards, merges the P0-R1 proof into the current Raw lane, and removes
old-source exceptions from shared guards instead of preserving stale
authority.

## Q1 — cleanup order

Decision: retire the old Raw chain before opening a new public runner caller.

This is a bounded BoxShape retirement, not a semantic or grammar change. S3's
actual compile/VM parity replaces the obsolete prerequisite named by the old
retirement card.

```text
old prerequisite:
  PUBLIC-CUTOVER-PARITY0-S0

accepted prerequisite:
  S3 typed Raw compile + exact VM parity green
  old Raw non-test callers = 0
```

The row uses Refactor Series Mode so each commit remains buildable.

## Q2 — first controlled public consumer

Decision: after retirement, add one explicit default-off CLI canary:

```text
--backend raw-vm-reference
```

The spelling is not semantic authority. It is converted exactly once into:

```rust
struct RawVmReferenceProductionRequestV1 {
    grammar: CanonicalGrammarProfileV1,
    source: RawNarrowV1,
    imports: NoImportsV1,
    callable_main: OmittedV1,
    backend: VmReferenceCapabilityV1,
    process: CanonicalProcessExitV1,
    optimize: bool,
}
```

The exact first profile is:

```text
grammar          = Canonical
source origin    = one bare source file
source grammar   = existing Raw NarrowV1 / LinearScalar0 only
using/imports    = forbidden
macro expansion  = forbidden
REPL             = forbidden
JSON/MIR JSON    = forbidden
diagnostic flags = forbidden (`--verbose`, dump/verify/stats)
development/test = forbidden (`--dev`, `--stage3`, compiler args, test flags)
script arguments  = forbidden (arguments after `--` are retained as facts)
callable Main    = Omitted
backend          = vm-reference
process policy   = CanonicalProcessExitV1::V1
optimization     = snapshot of existing --no-optimize selection
fallback         = forbidden
```

`compat2025`, `using`, macros, helper widening, explicit Main return, control
flow, calls, objects, async, and fastmem remain capability rejections. This
does not change their language meaning.

## Q3 — early selection boundary

Decision: the canary is selected at the start of
`NyashRunner::run_refactored`, before:

```text
Stage-1 stub
using/directive preprocessing
JSON bridge
task selection
common environment mutation
plugin/runtime initialization
general backend configuration
general dispatch
```

The process-wide CLI bootstrap and Ring0 initialization that happen before
`run_refactored` remain existing CLI infrastructure and are not reclassified
as Raw source authority.

The canary must not be added as only another late `dispatch.rs` match arm.
Late dispatch would run compatibility preprocessing and plugin effects before
Raw preflight.

## Q4 — source and diagnostic authority

The runner reads one file and invokes the canonical parser exactly once. It
does not call:

```text
prepare_source_with_imports
prepare_source_minimal
macro expansion
compile_with_source_hint
compile_raw_with_source compatibility erasure
```

Unsupported source syntax is rejected by the canonical parser or the sealed
Raw eligibility/source-facts boundary. No text rewrite creates an accepted
shape.

`RawVmReferenceRunReportV1` remains the only process-result input. Add one
stable diagnostic-line projection beside
`VmReferenceProcessDiagnosticAdapterV1`; the runner does not match
`ProcessFaultV1` or reconstruct status.

```text
program success/fault:
  report.status_code() -> OS status

typed process fault:
  report diagnostic line -> stderr, best effort
  write failure does not change status

read/parse/compile/activation rejection:
  invocation failure status 1

missing feature/file or conflicting CLI mode:
  usage/capability status 2
```

A program fault remains status 70. It is not confused with failure to start
the invocation.

## Q5 — later backend order

After the explicit canary is green:

```text
NORMAL-ENTRY-CUTOVER-D0
  fresh caller/profile/capability decision
  no default-route change without explicit authorization

ENTRY-RESULT-AOT-D0
  select one AOT backend boundary
  design opaque handoff, status thunk, NyRT ABI, and fault transport

JSON-SOURCE-RESULT-D0
  select a non-forgeable entry/decode witness for JSON artifact families

GENERAL-RUNNER-STATUS-D0
  replace legacy VM/MIR status laws only after family parity

SELFHOST-ACTIVATION-D0
  only after language convergence and JSON evidence

FASTMEM-VM-ACTIVATION-D0
  only after Raw grammar and VM MemOp capability rows
```

No later row may consume a compatibility-erased `MirCompileResult` and claim
that it retained Raw entry/result authority.

## Exact task order

### 0. `OLD-RAW-RETIRE0-R0A-PROOF-MIGRATION0`

Status: closed on 2026-07-25. The two unique rejection/retention proofs now
live in the new DRAIN0/FINAL0 test modules, old-only guards (including the
merged P0-R1 dependency) are removed, and shared guards no longer require a
historical active execution row.

```text
move PublishedShell rejection to new DRAIN0 fixture
move BuilderReadiness/retention to new FINAL0 fixture
retire old-only guards and their check-index entries
remove old-file exceptions from shared guards
behavior/grammar/public caller delta = 0
```

### 1. `OLD-RAW-RETIRE0-R0B-SOURCE-EVIDENCE0` — closed 2026-07-25

```text
delete the two old source files
remove registrations/re-exports/dead_code allowances
remove old run_raw and ModulePostprocessInputV1::Raw
remove ledger/root-only Raw evidence variants
remove legacy external-commit Raw acceptance
preserve new run_raw_ready and complete Raw evidence
```

Closeout: both caller-zero legacy source files and their module/evidence
registrations are gone. The canonical RawDirect chain remains the only Raw
production vocabulary; no public/default/JSON/backend route was widened.

### 2. `OLD-RAW-RETIRE0-G0` — closed 2026-07-25

```text
old scoped symbols/callers/files = 0
new DRAIN0/FINAL0/POST0/PUBLICATION/S3 guards green
default and vm-reference cargo checks green
focused Raw/canonical tests green
close RAW-PUBLICATION-SUNSET-001
```

Closeout: all scoped symbols/files/callers are zero; the new Raw chain,
focused VM-reference execution, and both cargo library lanes are green. No
default, JSON, legacy-runner, or backend route changed.

### 3. `NORMAL-ENTRY-PROFILE0-S0` — closed

```text
add passive RawVmReferenceProductionRequestV1 and one selector
convert CLI facts to the request exactly once; non-target backend = NotSelected
reject conflicting modes before file I/O/source effects
canonical grammar and NarrowV1 remain independent typed fields
request is move-only; production runner caller = 0
default route delta = 0
```

The passive selector also retains the arguments after `--` as `CliConfig`
facts and rejects them for NarrowV1 before any source-file effect. This keeps
profile selection pure: the selector does not read, parse, compile, run, or
modify process state. `--verbose` is treated as a diagnostic-route conflict,
not as an implicit profile setting.

Closeout evidence:

```text
cargo check --lib                                      = PASS
cargo check --lib --features vm-reference              = PASS
cargo test --lib raw_vm_reference_request              = 5/5 PASS
entry_result_projection0_s3_owner_guard.py             = PASS
current_state_pointer_guard.sh                         = PASS
production profile callers                             = 0
default route behavior                                 = unchanged
```

This row also consolidates the admitted Script/App witness matrix into the
existing entry-result proof family. The parked Legacy any-statement
observation is not materialized as an executable profile.

### 4. `NORMAL-ENTRY-CANARY0-S0` — closed

Internal order:

```text
CANARY-DIAGNOSTIC0
  stable compiler-owned diagnostic-line projection

CANARY-RUNNER0
  src/runner/reference/raw_vm_reference.rs
  file read -> canonical parse -> run_raw_vm_reference

CANARY-SELECT0
  one early exact selector before compatibility runner effects

CANARY-I0
  --backend raw-vm-reference
  feature-disabled and conflicting-mode fail-fast
```

Closeout evidence:

```text
run_refactored first selector                    = PASS
default backend NotSelected fallthrough          = PASS
feature-disabled canary before file read         = status 2
feature-enabled Script `0`                       = status 0
feature-enabled Integer `-1`                     = status 70 + range report
profile/diagnostic/runner structural guard       = PASS
canonical/source-entry focused tests              = PASS
```

### 5. `NORMAL-ENTRY-CANARY-PARITY0/G0` — closed 2026-07-25

Run the real binary and cover:

```text
empty Script                          -> 0
Integer 0 / 255                       -> 0 / 255
Integer -1 / 256                      -> 70 plus range diagnostic
Bool / Float / String                 -> 70 plus unsupported diagnostic
print(1)                              -> side effect plus status 0
Local / Assignment / Compound         -> status 0
empty and non-empty fallthrough App   -> status 0
division fault                        -> 70 plus source-fault diagnostic
compile/eligibility rejection         -> 1 and no fallback
feature unavailable / mode conflict   -> 2
decoy NYASH_ENTRY                     -> sealed Main remains selected
default mir/vm/vm-hako/llvm routes    -> unchanged
```

Caller and structure census:

```text
run_raw_vm_reference non-test runner consumer = 1
raw-vm-reference early selector               = 1
new-route process terminal                    = 1

compile_raw_with_source production consumer   = 0
compile_with_source caller delta              = 0
execute_mir_module_quiet_exit new caller      = 0
run_vm_compiled_module new caller             = 0
execute_vm_family_route new caller            = 0

Stage-1/using/plugin/new-route consumer        = 0
JSON/LLVM/ny_main/general runner widening      = 0
status reconstruction                         = 0
fallback                                      = 0
```

Closeout evidence:

```text
CARGO_TARGET_DIR=/tmp/hakorune-canary-default cargo build --bin hakorune
CARGO_TARGET_DIR=/tmp/hakorune-canary-feature cargo build --features vm-reference --bin hakorune

python3 tools/checks/lib/entry_result_projection0_s3_canary_parity.py \
  --binary /tmp/hakorune-canary-feature/debug/hakorune \
  --disabled-binary /tmp/hakorune-canary-default/debug/hakorune
  = PASS: cases=16 decoy=1 conflict=2 default=1 disabled=1

entry_result_projection0_s3_owner_guard.py = PASS
raw-vm-reference focused tests = PASS
default and vm-reference cargo checks = PASS
```

The proof family also covers parse/compile rejection status 1, missing-source
status 2, feature-disabled pre-I/O status 2, exact typed diagnostics, and
default `mir` preservation using an out-of-range legacy probe. No new per-row
shell guard was added; the reusable proof script and owner guard are the sole
subprocess/structural evidence.

The next row is a fresh design stop, not an implementation row:

```text
NORMAL-ENTRY-CUTOVER-D0
docs/development/current/main/investigations/
  normal-entry-cutover-d0-consultation-2026-07-25.md
```

### 6. `NORMAL-ENTRY-CUTOVER-D0`

Disambiguate whether cutover means opt-in canary support, one bounded caller
family, `compile_with_source`, the default CLI backend, or an explicit park.
Do not create `NORMAL-ENTRY-CUTOVER0-S0` until a new decision names one exact
target and grants implementation authorization.

## Task-map closeout repair

Moving `current_execution_row` beyond S3 exposed one stale temporal condition
in `entry_result_projection0_s3_entry_carry_guard.py`. The guard previously
accepted only four hard-coded S3 row names, so a closed proof became red as
soon as any successor row was selected.

The guard is now historical/reusable as required by the current checks
policy: it validates the closed task contract and code structure without
requiring S3 to remain the active row. This changes no Rust behavior and adds
no proof surface.

## Proof budget and sunset

### Old Raw retirement

```text
ceremony_tier = T1 bounded retirement
sunset_id = RAW-PUBLICATION-SUNSET-001
retirement_owner = OLD-RAW-RETIRE0
sunset_row = OLD-RAW-RETIRE0-G0

proof_inventory_before =
  old local fixtures + three old-only guards + P0-R1 old-chain proof

new_proofs =
  two migrated focused fixtures in existing new-chain test modules

retired_or_merged_proofs =
  old local fixtures + old-only guards + old P0-R1 old-chain dependency

net_proof_delta <= 0
sunset_budget = 0

retire_when =
  S3 compile/VM parity green
  + old non-test callers zero
  + both unique proofs migrated
  + old guard dependencies removed
  + new Raw/canonical gates green
```

### Explicit canary

```text
ceremony_tier = T2 production policy boundary
sunset_id = NORMAL-ENTRY-CANARY-SUNSET-001
retirement_owner = NORMAL-ENTRY-CUTOVER0
sunset_row = NORMAL-ENTRY-CANARY-RETIRE0

proof_inventory_before =
  S3 in-process compile/VM parity family

new_proofs =
  one real-binary subprocess fixture family
  + assertions in the reusable entry-result lane proof

retired_or_merged_proofs =
  no new per-row guard; reuse S3 process cases where identical

net_proof_delta =
  +1 production-boundary subprocess family

sunset_budget =
  one explicit canary selector, one runner shell, one subprocess family

retire_when =
  normal-entry cutover is explicitly accepted and green
  + default route owns the same typed profile
  + explicit canary has no unique diagnostic/capability role

budget_repayment_evidence =
  selector/runner/subprocess consumers zero and deleted,
  or a new accepted reference-runner decision removes them from canary status
```

## First executable row

```text
OLD-RAW-RETIRE0-R0B-SOURCE-EVIDENCE0
```

## Non-claims

```text
normal compile_with_source cutover
compile_with_source_and_imports cutover
general VM/MIR status-law replacement
LLVM/native ny_main activation
JSON / Program(JSON v0) changes
executor / selfhost / fastmem activation
Raw grammar widening
helper widening
explicit Main return capability
compat2025 activation
REPL or macro activation
interpreter reuse
CUT0
```
