---
Status: current design stop; no physical Text receipt is open
Date: 2026-08-15
Parent: `docs/development/current/main/investigations/callable-physical-header-transport-r0-2026-08-15.md`
Classification: T2 BoxShape
Authority: language Text law, ExactText formal source contract, and one branded callable header cohort
---

# CALLABLE-PHYSICAL-TEXT-PARAMETER-ABI-D0

The source/header transport row is complete for the existing supported formal
rows and explicit scalar result. The remaining blocker before the S6C callable
can enter the installed package is the physical owner for an explicit
`StringBox`/ExactText formal parameter. The semantic row already exists; its
physical handle/home/ownership contract does not.

## Six-line brief

```text
Decision: design one profile-neutral ExactText formal-parameter physical contract that remains part of the complete callable header cohort; do not mint a parameter-only receipt or wire.
Source authority + canonical issuer: the normative Text law, final callable parameter source/BindingRef, and issue_callable_parameter_contract_v1; a single future header-cohort binder is the only physical formal issuer.
Non-authority: MirType/StringBox spelling alone, HomeDemand::Handle, runtime StringBox helpers, TextEq/Substring, ResultCatalog/body inference, selector/name, AST/MIR re-read, fallback, and retry.
Fail-fast boundary: reject missing canonical Text handle wire/home/lease owner, formal ordinal/BindingRef/owner drift, Text-vs-opaque downgrade, foreign package brand, and any parameter/result split; remain NoSafeSlice when the owner is absent.
Smallest next slice: close the Text formal lane, ownership, failure/trap policy, and same-cohort API only; no code, fixture, runtime symbol, or Builder/session.
Non-claims: no C ABI implementation, stale-generation proof, TextEq route/residence, S6C child, V2 envelope, ReadyEntry, MIR/CFG/SSA/PHI, selector, production caller, fallback, or legacy retirement.
```

## Current authority facts

```text
ExactTextFormalAbiV1
  -> semantic/formal source classification only
  -> does not own a runtime Text wire

issue_callable_parameter_contract_v1
  -> owns ordinal, BindingRef, owner, and formal-kind rows
  -> currently rejects explicit StringBox as a physical admission

CALLABLE-PHYSICAL-HEADER-TRANSPORT-R0
  -> transports supported formal rows + explicit i64 result + Completion
  -> lends None when a selected row has no explicit source header

S6C typed input / TextEq site
  -> owns logical Text relation and source identity
  -> cannot be promoted to a physical handle or residence capability
```

The current S6C fixture is intentionally not a positive physical-header
fixture yet: its `StringBox` parameters are semantic ExactText rows, while
the package has no source-backed physical Text formal owner. Adding a default
opaque handle, borrowing the existing `HomeDemand::Handle`, or reading
`MirType::String` would be an authority change and is forbidden here.

## Existing-candidate census

The current repository has several nearby representations, but none is the
missing formal-parameter owner:

```text
ExactTextFormalAbiV1
  = source spelling / semantic formal classification only
HomeDemandV1::Handle
  = one-way Home demand projection; no Text wire or ownership policy
include/nyrt_host_api.h HostHandle
  = generic reverse-call handle and TLV surface; no Text formal lane
include/nyrt_dynamic_call_slot_v2.h HostHandle
  = generic CallSlot value/fault/lease transport; not callable-header ABI
include/nyrt_dynamic_text_scan_v1.h
  = TextScan-specific symbolic facts; no ordinary StringBox formal issuer
TextReadSession / BorrowedHandleBox
  = runtime read/cache helpers; no compiler-owned lifetime receipt
nyash.string.eq_hh
  = hookable compatibility export with fallback and invalid-handle policy
```

The public ABI inventory has `handle_owned` and `handle_borrowed_string`
classes, but it does not bind either class to an ExactText formal slot,
source `BindingRef`, callable header, or Completion cohort. `StringBox`'s
Rust equality and the kernel `eq_hh` export are implementation evidence only.
Until one owner closes those gaps, the correct state is the existing typed
`NoSafeSlice`, not a new adapter around a nearby generic handle.

## Required design acceptance

The design may close only after it names:

1. one profile-neutral Text formal lane (parameter representation, Home,
   borrow/move/retain/release, and result of parameter admission);
2. the canonical runtime/FFI owner, or an explicit `NoSafeSlice` if no
   canonical Text handle wire exists;
3. invalid/stale/non-Text behavior and the trap/failure boundary without
   language Fault, fallback, or retry;
4. exact co-seal with the existing formal rows, source result/header cohort,
   and `VerifiedFunctionCompletionV1` under the same package/Port brand;
5. a site-free reusable contract separated from a per-callable non-Clone
   formal evidence row; and
6. an API that cannot accept a caller-supplied parameter row, raw key, AST,
   MIR type, ResultCatalog, or independent route.

## Negative matrix

```text
StringBox -> OpaqueHandle downgrade       -> FormalKindMismatch
missing Text handle/home owner             -> NoSafeSlice
stale or invalid handle policy unspecified -> NoSafeSlice
foreign package/catalog brand             -> ForeignCohort
ordinal/BindingRef/owner drift             -> FormalBindingMismatch
parameter-only physical receipt            -> IncompleteCallableHeader
TextEq/Substring used as parameter owner  -> ForeignAuthority
MirType or fixture expectation             -> InferenceForbidden
raw AST/MIR/selector/name caller           -> API unavailable / reject
fallback or retry                          -> structural rejection
```

## Ordered DAG

```text
CURRENT
  CALLABLE-PHYSICAL-TEXT-PARAMETER-ABI-D0  (design stop)
    -> CALLABLE-S6C-INSTALLED-CHILD-COMPOSITION-D0
    -> LOOP-SEMANTIC-PROGRAM-COSEAL-R0
    -> LOOP-PHYSICAL-TRANSFER-AUTHORITY-R0
    -> LOOP-COMMON-V2-PRESESSION-TRANSPORT-R0
    -> TextEq runtime contract / route admission / residence
    -> canonical MIRBuilder session
    -> parity/canary -> bounded selector -> main integration -> retirement
```

No later row may consume the S6C fixture as a physical callable until this
design stop has either named its canonical owner or recorded the typed
`NoSafeSlice` token. The source/header R0 remains closed and is not reopened.
