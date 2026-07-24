# OLD-RAW-RETIRE0 execution task

Decision: `RAW-PUBLIC-CUTOVER-prime-r1`

Status: queued after `PUBLIC-CUTOVER-PARITY0-S0`.

One BoxShape semantic row uses Refactor Series Mode. Each commit must build.

## R0a — proof migration

```text
migrate old PublishedShell rejection -> new DRAIN0 fixture
migrate old BuilderReadiness/retention -> new FINAL0 fixture
remove historical guard dependence on the old source files
behavior/grammar/public caller delta = 0
```

Retire the three old-only guards and update shared guards to the new chain:

```text
cut0_i0_prod_activation_post0_raw_guard.py
cut0_i0_prod_activation_post0_raw_finalizer_guard.py
cut0_i0_prod_activation_post0_raw_postprocess_guard.py
```

## R0b — source and variant retirement

Delete:

```text
src/mir/builder/raw_physical_finalization.rs
src/mir/compiler/raw_finalization.rs
their registrations and re-exports
ModulePostprocessInputV1::Raw
ModulePostprocessOwnerV1::run_raw
PostprocessEvidenceInputV1::Raw
PostprocessEvidenceSealV1::Raw { ledger, root }
legacy external-commit Raw-family acceptance
```

Retain:

```text
RawCompleteInvocationV1::into_parts
ModuleVerificationEvidenceV1::Raw
ModulePostprocessScheduleV1::for_family(Raw)
run_postprocess_stages / run_raw_ready
raw_finalization_contract.rs / raw_root_finalization.rs
canonical publication authority
```

## G0 — closeout

```text
old source files = absent
old registrations/re-exports = 0
old Raw finalizer/run_raw/evidence symbols = 0
new DRAIN0/FINAL0/POST0/COMMIT0/PUBLICATION/INGRESS guards = green
normal/JSON/compiler legacy authorities = unchanged
all modified source/check files < 800 lines
```

## Required gates

```bash
cargo check --lib
cargo test --lib raw_ -- --test-threads=1
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Non-claims

```text
compile_legacy retirement
MirBuilder::build_module retirement
normal-entry cutover
JSON/executor/selfhost/fastmem/CUT0
```

`RAW-PUBLICATION-SUNSET-001` closes only for the old Raw-specific chain.
Generic Legacy and JSON compatibility authorities remain.
