---
Status: active execution task
Date: 2026-07-26
Decision: NORMAL-ENTRY-CUTOVER-D2-EXPLICIT-NORMAL-FILE-VM0
Scope: one CLI-visible, default-off NormalFileNoImportVmReferenceV1 route
ceremony_tier: T2 new production caller and terminal aggregation
grammar_delta: 0
normal_default_route_delta: 0
legacy_caller_replacement: none
sunset_id: NORMAL-FILE-VM0-FORGE-PROOF-SUNSET-001
sunset_owner: NORMAL-FILE-VM0-G0
sunset_row: NORMAL-FILE-VM0-G0
retire_when: forge-only callers are merged into the production route guard
---

# NORMAL-FILE-VM0 production route

## Fixed owner chain

```text
CliConfig
  -> ExplicitReferenceRunnerSelectionV1
  -> NormalFileVmReferenceProductionRequestV1
  -> NormalFileRequestV1
  -> NormalFileVmFrontDoorV1
  -> RawVmReferenceInvocationV1
  -> existing typed Raw VM-reference execution
  -> RawVmReferenceRunReportV1
  -> ReferenceRunOutcomeV1
  -> ReferenceRunTerminalV1
```

No owner may reread the file, reparse source, recreate a Raw profile, discover
an entry, reconstruct process status, retry through Legacy, or call the
default runner.

## Route contract

```text
CLI spelling             = --backend normal-file-vm-reference
visibility               = CLI-visible / default-off / feature-gated
normal profile           = NormalFileNoImportVmReferenceV1
result carriers          = Unit / Integer / Bool / Float / String
--no-optimize            = usage rejection before I/O
source/profile failure   = usage 2 or invocation 1
program status           = existing Raw report status unchanged
existing raw-vm-reference= unchanged
default mir route        = unchanged
legacy caller retirement = none
```

`compile_with_source`, the six D0 callers, JSON, REPL, imports, macro/plugin
routes, public embedding, annotations, ordinary callables, and dynamic result
carriers remain outside this series.

## Internal order

```text
REQUEST0-S0
  -> REPORT0-S0
  -> PARITY0-P0a
  -> CALLER0-I0
  -> PARITY0-P0b
  -> G0
  -> MIRBUILDER-CORE-COMPLETE0-P0
```

## Progress ledger

```text
REQUEST0-S0 = closed by 520986b38a
REPORT0-S0  = closed by 1ab9c8aad5
PARITY0-P0a = closed by f5028112ca
CALLER0-I0  = closed by 906592cb54
PARITY0-P0b = closed by real-binary matrix on 2026-07-26
G0          = active
```

### REQUEST0-S0 — request and single selector

Create only the typed request/selection vocabulary:

```text
NormalFileVmReferenceProductionRequestV1
ExplicitReferenceRunnerRequestV1
ExplicitReferenceRunnerSelectionV1
```

The request consumes exactly one `NormalFileRequestV1`.  Selection is pure:
it reads no file, initializes no runtime, and invokes no compiler.

Extract shared explicit-reference CLI admission once rather than copying the
current Raw flag checks.  The shared owner may select the backend spelling and
common canonical/source-route constraints.  Route-specific optimization is
then fixed by the request:

```text
raw-vm-reference          -> retains its existing optimize snapshot
normal-file-vm-reference  -> rejects --no-optimize
```

Do not connect `NyashRunner::run_refactored` in this row.

Acceptance:

```text
normal selector = one
raw selector behavior = unchanged
other backends = NotSelected
missing path / profile conflict / --no-optimize = Usage
file I/O = 0
production caller = 0
```

### REPORT0-S0 — bounded report and shared terminal

Use one `ReferenceRunOutcomeV1` / `ReferenceRunTerminalV1` for Raw and normal
routes.  It is the sole `process::exit` owner:

```text
Usage       -> 2
Invocation  -> 1
Program     -> RawVmReferenceRunReportV1::status_code()
```

The runner cannot receive private MIR rejection owners.  Add a small,
MIR-owned `source_entry_vm_runner_adapter.rs` if needed:

```text
RejectedRawVmReferenceRunV1
  -> RawVmReferenceInvocationFailureReportV1
  -> runner Invocation report
```

It must expose stage/code/bounded detail only.  It must not widen the private
owner, leak AST/Builder state, or infer status from a String prefix.

`normal_file_vm.rs` consumes the request once, runs the existing front door,
constructs a fresh compiler, calls the typed adapter, and returns an outcome.
It is not yet selected by CLI in this row.

Closed evidence: `1ab9c8aad5` adds the bounded adapter, a shared terminal
used by the existing Raw lane without changing its status law, and the
unconnected normal run owner. Focused feature-on/off tests and existing
S3/Forge guards are green; the normal CLI caller remains zero.

### PARITY0-P0a — production-shaped, caller-zero proof

Exercise only the normal request/run owner.  Require the existing forge matrix
plus usage=2, invocation=1, program fault=70, status/diagnostic parity with
the Raw reference in their common subset, and `rejection -> success` reuse.
This is not a Legacy parity claim.

Closed evidence: `f5028112ca` runs the normal owner directly and compares the
common scalar/unit process snapshots against the supported Raw reference lane.
It also fixes missing/read, parse/using, and Raw compile rejection as
Invocation outcomes, followed by a successful normal run. The pre-existing
front-door reuse matrix remains the same-compiler proof; this run owner is
intentionally fresh per invocation.

### CALLER0-I0 — one production connection (closed)

Only here replace the first `run_refactored` branch with one central call:

```rust
if let Some(outcome) = reference::select_and_run(&self.config) {
    outcome.finish();
}
```

The selector may return Raw or normal request outcomes.  Do not add ordered
independent `if let` selectors.  Update CLI help and one reference document.

Closed evidence: `906592cb54` adds one central `reference::select_and_run`
caller, dispatches Raw and NormalFile requests through the shared terminal,
keeps the default route untouched, and adds CLI/reference documentation.

### PARITY0-P0b — real binary evidence (closed)

With `vm-reference`, verify the exact matrix in the built binary, including
one-line diagnostics and default/raw route isolation.  Without the feature,
verify feature-unavailable status 2 with no file read.

Closed evidence from `target/release/hakorune`:

```text
normal-file-vm-reference 42       -> status 42, no diagnostic
raw-vm-reference 42               -> status 42, no diagnostic
default mir 42                     -> status 42, no diagnostic
normal Bool                       -> status 70, [process/unsupported-result] kind=Bool
normal missing file               -> status 1, file-not-found
normal --no-optimize              -> status 2, non-default-optimization-requested
feature-disabled normal route     -> status 2, feature-unavailable before I/O
```

The fixture source was temporary and removed after the run. No default route,
Raw route, or `compile_with_source` behavior changed.

### G0 — route guard and proof repayment (active)

Promote the existing Forge guard into the reusable route guard.  It must prove
one central selector, Raw caller=1, normal caller=1, default delta=0,
fallback/retry=0, one terminal, existing compile/execution/result authority,
and all touched source/check files below 800 lines.  Do not add per-row shell
guards.

## File budget

Keep each source/check file below 800 lines.  Prefer small modules:

```text
src/runner/reference/request.rs
src/runner/reference/normal_file_vm_request.rs
src/runner/reference/terminal.rs
src/runner/reference/normal_file_vm.rs
src/mir/compiler/source_entry_vm_runner_adapter.rs
```

Existing large `source_entry_vm_execution.rs` must not grow past its boundary.

## Verification

```bash
cargo check --lib
cargo check --lib --features vm-reference
cargo test -q --lib runner::reference
cargo test -q --lib source_entry_vm_execution --features vm-reference
python3 tools/checks/lib/normal_file_vm0_frontdoor_forge_guard.py
python3 tools/checks/lib/entry_result_projection0_s3_owner_guard.py
python3 tools/checks/lib/entry_result_projection0_s3_execution_guard.py
bash tools/checks/current_state_pointer_guard.sh
```

## Next boundary

After G0, `MIRBUILDER-CORE-COMPLETE0-P0` may declare the new compiler core
complete.  It must not claim default or full normal-compiler completion.
