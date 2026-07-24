# DECLACCESS COVERAGE0 execution task

Status: **Ready**  
Date: 2026-07-24  
Decision: **DECLACCESS-IMPLEMENTATION-prime-r1**

## Progress

The coverage witness slice is implemented locally and verified:

```text
RawRootCoverageV1 is the non-Clone route witness.
Script declaration/work kinds are rejected explicitly.
App plain-static-Main validation remains the sole App catalog authority.
```

The manifest and physical owners remain disconnected; this row adds no
physical or production consumer.

## Goal

Strengthen `RawRootEligibilityV1` with an exact first-slice coverage witness
before the manifest producer is introduced. DECLACCESS must not become a
second eligibility authority.

## Scope

```text
Script:
  admit only declaration-free ScalarControl0 statements
  reject DeclarationFact, StaticBox, InstanceBox, TopLevelFunction,
  MainRoot, StaticData, and every unsupported work item explicitly

App:
  admit exactly one plain static Main box
  retain complete helper/callable correspondence
  reject non-Main boxes, top-level functions, declarations, fields,
  constructors, static init, records, interfaces, and partial catalog
```

Add a non-Clone/Copy-free coverage witness to `RawRootEligibilityV1`; the
manifest builder consumes the witness and does not repeat these policy
decisions. Keep PLAN0 and classifier files below 800 lines.

## Required changes

```text
src/mir/compiler/raw_root_eligibility.rs
  explicit Script/App coverage checks
  typed coverage witness and rejection stage

src/mir/compiler/raw_root_eligibility_p0.rs
  success/rejection matrix

tools/checks/lib/cut0_i0_root0_raw_lane_guard.py
  one shared lane guard profile (if not yet present)
```

Do not add the exact manifest, physical open, Builder/shell install, BODY0,
or a production consumer in this row.

## Acceptance

```text
invalid Script declaration -> typed eligibility rejection
invalid App declaration/catalog -> typed eligibility rejection
valid empty Script -> coverage witness
valid plain static Main App -> coverage witness
manifest producer = 0
physical/session/shell/collector/ledger construction = 0 in new tests
current_module / AST re-scan = 0
retry/fallback = 0
all touched files < 800 lines
```

Verification:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
cargo check -q --lib
cargo test -q raw_root_eligibility --lib -- --test-threads=1
```
