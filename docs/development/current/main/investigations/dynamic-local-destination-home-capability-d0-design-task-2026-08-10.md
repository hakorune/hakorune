# DYNAMIC-LOCAL-DESTINATION-HOME-CAPABILITY-D0

Status: accepted census; Home-capability implementation is `NoSafeSlice`
Date: 2026-08-10
Depends on: `DYNAMIC-FAULT-CUTPOINT-CATALOG-I0` closed
Parent Decision:
`dynamic-fault-exit-transaction-d0-design-task-2026-08-10.md`

## Decision

Three independent audits agree that the unchanged `skip_while/4` source does
not have a canonical source-backed classifier for the normal I6 result. The
existing source/Recipe relation proves exact V10-to-`ch` identity, Loop-body
scope, one borrowed I7 use, and zero rebind/capture. It does not prove whether
the opaque carrier is `Trivial`, `Unique`, `Shared`, or `Weak`.

Therefore this row closes with:

```text
Home capability for unchanged V10/ch:
  NoSafeSlice

HomeRoot / Available / cleanup issuance:
  0
```

`Dynamic`, `SelfContainedDynamicCarrier`, selector `substring`, runtime Box
tags, provider metadata, `MirType`, and physical `ValueId` are explicit
non-authorities. No `VerifiedChHomeV1` or unconditional owner-bearing receipt
may be introduced.

## Existing exact input

```text
VerifiedDynamicFullLoopSemanticProgramV2
  -> exact I6 producer / normal V10 key
  -> exact ch declaration / BindingRef / Loop-body scope
  -> exact I7 borrowed read
  -> complete Dynamic invocation envelope
```

This is a neutral lifetime/source relation only. It proves no Home install,
availability, release, cleanup, terminality, or field ownership.

## Authority census

| Existing owner | Source-backed | Classifies one Dynamic result | Decision |
|---|---:|---:|---|
| source-bound Dynamic target | yes | no | exact call/result-site identity only |
| Dynamic execution envelope | yes | no | closed three-category carrier set |
| iteration-local V10/ch view | yes | no | destination/source identity only |
| resolver semantic value type | yes | no | current cohort is I64/Unit only |
| passive Home relation vocabulary | partial | no | not a classifier |
| Query Home ABI | yes | no | declaration-level I64/Unit cohort only |
| MIR/runtime/provider representation | no | physical only | forbidden authority |

## Missing general Home boundary

A future source-visible Home path requires three separate authorities:

```text
source/import-backed normal-result value capability
  Trivial | Unique | Shared | Weak | Unknown

source-backed local destination capability
  HandleOnly | OwningHome | SharedHome | WeakSlot | TrivialSlot | Unknown

atomic normal-only value-to-destination publication relation
  exact invocation/result/source/local/frame/scope
```

Only a later CFG-complete Home Flow may turn an admitted owner-bearing
publication into `Available`. Trivial and Weak results create no Home state.
The unchanged untyped Dynamic call has no such result contract today.

## Normal/Fault boundary

```text
I6 Normal:
  V10 is published, but no Home classification is available

I6 Fault:
  V10 is not published
  destination install = 0
```

## Disposition

```text
repository development state:
  NoSafeSlice (canonical classifier absent)

future issuer, exact shape but missing result capability:
  Unresolved(MissingDynamicNormalResultCapability)

foreign/duplicate/cross-wired source relation:
  Rejected

fully observed non-local destination family:
  Declined
```

## Separate alternative

Every self-contained carrier may have a representation-neutral structural
`forward-or-end exactly once` obligation even when it is not a source-visible
Home. That is not a repair for this Home row and must not be smuggled into
Home Flow. It is evaluated by the separate
`DYNAMIC-CARRIER-LEXICAL-DISPOSITION-D0` Decision.

## Closeout

This D0 opens no Home implementation row. If a future source/imported Dynamic
message contract supplies an exact result capability, the general taxonomy
and value-to-destination relation are reopened outside this profile.

## Nonclaims

```text
Home Available/Consumed state
CFG-complete Home Flow
cleanup obligation / release / fini / DropPlan
Fault execution or primary outcome
JoinSig transfer changes
Completion consumption
Builder/MIR/CFG/PHI/physical IDs
provider/runtime tag classification
production activation / retry / fallback
```

## Hard stops

```text
no Dynamic implies Home
no SelfContainedDynamicCarrier implies one Home
no runtime Box/tag/pointer inspection
no method-name or selector classification
no Recipe key as source identity
no standalone forged VerifiedCh/Home product
no test-only arbitrary capability constructor
no source fixture narrowing
```

All proposed Rust files must split near 650-700 lines, stop additions at 760,
and remain below the 800-line hard limit.
