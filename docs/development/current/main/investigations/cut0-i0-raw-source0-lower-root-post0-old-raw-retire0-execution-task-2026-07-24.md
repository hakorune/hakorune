# OLD-RAW-RETIRE0 execution task

Decision: `RAW-PUBLIC-CUTOVER-prime-r1`

Status: R0A complete; R0B selected as the next post-S3 executable row. The former
`PUBLIC-CUTOVER-PARITY0-S0` prerequisite is superseded by the guarded S3
typed Raw compile plus exact VM parity and the measured zero non-test caller
census.

## R0A closeout — 2026-07-25

The proof-migration slice is closed without Rust behavior or public-route
change:

```text
PublishedShell rejection -> new DRAIN0 focused fixture
BuilderReadiness/retention -> new FINAL0 focused fixture
old-only production guards and P0-R1 guard -> removed
shared DRAIN0/FINAL0/POST0/COMMIT0/PUBLICATION0 guards -> reusable across rows
cfg(test) caller counting -> brace-aware production-scope filtering
```

The old source files remain intentionally for R0B; this closeout only removes
their proof ownership and historical guard coupling.

One BoxShape semantic row uses Refactor Series Mode. Each commit must build.

Internal rows:

```text
OLD-RAW-RETIRE0-R0A-PROOF-MIGRATION0
-> OLD-RAW-RETIRE0-R0B-SOURCE-EVIDENCE0
-> OLD-RAW-RETIRE0-G0
```

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

Merge the old P0-R1 dependency into the current S3 Raw proof family and
remove old-source exceptions from shared guards. Do not create replacement
per-row guards for deleted sources.

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
cargo check --lib --features vm-reference
cargo test --lib raw_ -- --test-threads=1
cargo test --lib --features vm-reference source_entry_vm_execution -- --test-threads=1
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Proof budget

```text
ceremony_tier = T1 bounded retirement
sunset_id = RAW-PUBLICATION-SUNSET-001
retirement_owner = OLD-RAW-RETIRE0
sunset_row = OLD-RAW-RETIRE0-G0

new_proofs =
  two migrated focused fixtures in existing new-chain test modules

retired_or_merged_proofs =
  old local fixtures
  + three old-only guards
  + old P0-R1 chain dependency

net_proof_delta <= 0
sunset_budget = 0

retire_when =
  S3 typed Raw compile/VM parity green
  + old Raw non-test callers zero
  + PublishedShell proof migrated
  + BuilderReadiness retention proof migrated
  + old guard dependencies removed
  + new Raw and canonical gates green
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
It does not own or close function-exit compatibility evidence;
`RAW-BODY-RETURN-COMPAT-SUNSET-001` owns that independent scaffold.
