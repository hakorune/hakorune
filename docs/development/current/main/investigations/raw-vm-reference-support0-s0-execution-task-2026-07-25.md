---
Status: Active Execution Task
Date: 2026-07-25
Scope: harden and promote the existing explicit Raw VM-reference route.
Related:
  - docs/development/current/main/investigations/normal-entry-cutover-d0-consultation-2026-07-25.md
  - docs/development/current/main/investigations/post-s3-clean-retire-and-normal-entry-canary-task-map-2026-07-25.md
  - docs/reference/language/function-exit-and-entry-result.md
  - docs/tools/check-scripts-index.md
---

# RAW-VM-REFERENCE-SUPPORT0-S0 execution task

```text
Decision authority:
  NORMAL-ENTRY-CUTOVER-prime-r1

first executable row:
  RAW-VM-REFERENCE-SUPPORT0-S0

ceremony tier:
  T1 bounded BoxShape/refactor series over an already-proven T2 route

behavioral intent:
  accepted grammar, execution, and status delta = 0

docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
```

## Goal

Turn the existing explicit, feature-gated route into a durable supported
reference/conformance lane without creating another source, compiler,
execution, or process-status authority.

The row must remove temporary canary wording and decorative profile fields,
stabilize bounded diagnostics, and reclassify the existing proof family with
net proof delta zero.

## Current structural residuals

The behavior proof is already green, but the support boundary is not yet
clean:

```text
CLI help and source comments still call the lane a canary
Cargo.toml says vm-reference is default-on although it is not in default
README/backend-role docs do not name the Raw reference lane
RawVmReferenceProductionRequestV1 carries policy fields that production
  execution does not consume
compile_raw_published_v1 reconstructs NarrowV1/None/Omitted policy internally
compile rejection prints the complete Debug owner graph
proof/script/fixture vocabulary still names the temporary canary
NORMAL-ENTRY-CANARY-SUNSET-001 remains open
```

These are BoxShape and support-contract issues. This row must not expand the
accepted language or connect another caller.

## Target owner chain

```text
CLI facts
  -> RawVmReferenceProductionRequestV1
  -> one consuming support-plan projection
       parser/source plan
       Raw published-compile request
       VM-reference execution/process profile
  -> one file read
  -> one Canonical parse
  -> compile_raw_published_v1(typed request)
  -> exact VM-reference activation
  -> RawVmReferenceRunReportV1
  -> one process terminal
```

Every selected profile field must either be consumed by the named layer or be
removed. A typed field that exists only for tests is not an authority.

## Internal order

### 1. `SUPPORT-CONTRACT0`

Add one durable user/developer contract for:

```text
explicit CLI spelling
optional build feature
Canonical + Raw NarrowV1
NoImports
callable Main Omitted
fresh VM reference backend
CanonicalProcessExitV1
status 0 / exact 0..255 / 70
usage 2 / invocation 1
fallback zero
default mir unchanged
```

Update the backend role documentation and correct the `Cargo.toml`
`vm-reference` feature comment. Do not enable the feature by default.

Recommended durable entry:

```text
docs/reference/execution-backend/raw-vm-reference.md
```

Link it from the backend role section in `README.md`.

### 2. `SUPPORT-PROFILE0`

Replace parallel policy reconstruction with one consuming typed handoff.

Recommended vocabulary:

```rust
struct RawPublishedCompileRequestV1 {
    ast: ASTNode,
    source_file: Option<Box<str>>,
    module_name: Box<str>,
    source_profile: RawPublicSourceProfileV1,
    imports: RawPublicImportDispositionV1,
    callable_main: RawCallableMainSelectionV1,
}

struct RawVmReferenceExecutionProfileV1 {
    backend: RawVmReferenceBackendV1,
    process: RawVmReferenceProcessProfileV1,
}
```

Exact type names may follow existing module vocabulary, but the laws are
fixed:

```text
RawVmReferenceProductionRequestV1 is consumed once
grammar drives the parser once
source/import/callable policy drives one compile request
backend/process policy drives one execution terminal
optimize drives one compiler construction
compile_raw_published_v1 does not select a second policy
```

The existing `compile_raw_with_source` compatibility API may construct the same
private NarrowV1 request, but it remains production-caller zero.

Do not make compiler code depend on runner modules. Shared route/profile
vocabulary belongs in a neutral `src/mir/` contract module; CLI-only facts stay
in the runner request.

### 3. `SUPPORT-DIAGNOSTIC0`

Replace the complete rejected-owner `Debug` dump with one bounded typed
failure report.

```rust
struct RawPublishedCompileFailureReportV1 {
    stage: RawPublishedCompileStageV1,
    code: &'static str,
    detail: Box<str>,
}
```

Required law:

```text
inspect rejection
-> seal one report
-> discard exact owner once
-> format stable line
```

The report may expose a short typed field such as a source path or rejected
kind. It must not expose the whole owner graph, retry, reconstruct a module, or
change the status class.

### 4. `SUPPORT-SURFACE0`

Remove temporary canary language from current production surfaces:

```text
CLI backend help
runner/reference module docs
profile request module docs
Cargo feature comment
current backend role docs
```

Use:

```text
supported opt-in Raw VM-reference lane
reference/conformance scope
default-off build capability
```

Do not rename the CLI spelling.

### 5. `SUPPORT-PROOF0`

Rename/reclassify the existing real-binary proof as the stable supported-lane
conformance proof.

Rules:

```text
reuse all sixteen semantic fixtures
reuse feature-enabled/disabled binaries
reuse decoy/default/conflict/rejection cases
do not create a second subprocess family
update docs/tools/check-scripts-index.md
use existing S3 owner guard
new per-row shell guard = 0
net proof delta = 0
```

The proof file and fixture directory may be renamed in one mechanical change.
Do not retain both canary and supported copies.

### 6. `CANARY-SUNSET0`

Close:

```text
NORMAL-ENTRY-CANARY-SUNSET-001
```

Budget repayment:

```text
temporary selector/runner status -> durable supported reference role
temporary subprocess proof       -> durable conformance proof
duplicate proof                   -> 0
new production consumer           -> 0
```

### 7. `SUPPORT-G0`

Extend the existing reusable owner/caller guard rather than adding a new shell
guard.

## Structural guard contract

```text
raw-vm-reference CLI spelling                           = 1
CLI backend default mir                                 = 1
early selector production caller                        = 1
run_raw_vm_reference non-test caller                    = 1
compile_raw_with_source non-test caller                  = 0

Raw VM-reference selected profile producer              = 1
selected request consuming projection                   = 1
compile_raw_published_v1 second policy reconstruction    = 0
unconsumed production profile fields                    = 0

file reads in supported route                            = 1
Canonical parser entry in supported route                = 1
process terminal in supported route                      = 1

legacy fallback                                          = 0
NYASH_ENTRY lookup                                       = 0
execute_module entry discovery                           = 0
status reconstruction outside ProcessExitProjectionV1    = 0

complete rejected-owner Debug formatting                 = 0
stable bounded compile failure report producer           = 1

new normal/default/JSON/LLVM/WASM/Stage1/selfhost caller  = 0
vm-reference default feature enablement                   = 0

supported-lane subprocess proof families                 = 1
new per-row shell guards                                  = 0
net proof delta                                           = 0

all modified/new source/check files                      < 800 lines
```

## Acceptance matrix

### Profile and routing

```text
no backend override             -> default mir unchanged
--backend mir/vm/vm-hako/llvm   -> supported selector NotSelected
--backend raw-vm-reference      -> exact supported profile
feature disabled                -> status 2 before file read
using/macro/REPL/JSON/emit/etc. -> status 2 before file read
script arguments                -> status 2 before file read
```

### Program results

Reuse the closed matrix:

```text
empty/void/print/local/assignment/compound/App fallthrough -> 0
Integer 0 / 255                                             -> 0 / 255
Integer -1 / 256                                            -> 70 + range
Bool / Float / String                                       -> 70 + unsupported
division fault                                              -> 70 + source fault
```

### Failures

```text
parse rejection      -> status 1 + stable parse tag
compile rejection    -> status 1 + bounded stage/code report
missing source       -> status 2
profile conflict     -> status 2
diagnostic I/O error -> original semantic status
fallback output      -> absent
```

### Isolation and reuse

```text
NYASH_ENTRY decoy does not change sealed target
default route remains byte-for-byte on the legacy owner
same compiler success/rejection reuse tests remain green
```

## Verification commands

Use the row's final file names after proof reclassification:

```bash
cargo check --lib
cargo check --lib --features vm-reference
cargo test --lib raw_vm_reference
python3 tools/checks/lib/entry_result_projection0_s3_owner_guard.py
python3 <supported-reference-proof> \
  --binary <vm-reference-binary> \
  --disabled-binary <default-binary>
bash tools/checks/current_state_pointer_guard.sh
```

Run the two real binaries in separate `CARGO_TARGET_DIR` directories as in the
closed canary proof.

## Refactor series contract

Refactor Series Mode is authorized for two to five buildable commits because
this is one BoxShape goal:

```text
contract/docs first
profile handoff
diagnostic boundary
surface/proof rename
sunset/guard closeout
```

Do not mix accepted-shape, backend, or normal caller changes into the series.

## Parked task order after support closeout

```text
RAW-MINIMAL-MIR-JSON-PROFILE-D0
  nearest possible future bounded compile-only caller

ENTRY-RESULT-AOT-D0
  opaque Raw-to-AOT handoff, exact entry/status thunk, NyRT ABI

JSON-SOURCE-RESULT-D0
  non-forgeable selected-entry/decode evidence for JSON artifacts

PROGRAM-JSON-IMPORT-BUNDLE-D0
  separate imported bundle authority

GENERAL-RUNNER-STATUS-D0
  legacy VM/MIR status-law migration

NORMAL-ENTRY-CUTOVER-D1
  only after one exact candidate is green
```

## Non-claims

```text
normal/default cutover
compile_with_source change
Raw grammar or helper widening
using/macro/REPL/JSON activation
general VM/MIR runner change
LLVM/native/ny_main activation
WASM/Stage1/selfhost/fastmem activation
legacy status retirement
CUT0
```
