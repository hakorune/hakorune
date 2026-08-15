---
Status: accepted design; next bounded caller-zero I0 is `CALLABLE-PHYSICAL-TEXT-PARAMETER-ABI-I0`
Date: 2026-08-15
Parent: `docs/development/current/main/investigations/callable-physical-header-transport-r0-2026-08-15.md`
Classification: closed T2 BoxShape; next T2 BoxCount I0
Authority: language Text law, ExactText formal source contract, branded callable header cohort, and the Rust-owned host-handle generation table
---

# CALLABLE-PHYSICAL-TEXT-PARAMETER-ABI-D0

The source/header transport row is complete for the existing supported formal
rows and explicit scalar result. The remaining blocker before the S6C callable
can enter the installed package is the physical owner for an explicit
`StringBox`/ExactText formal parameter. The semantic row already exists; its
physical handle/home/ownership contract does not.

## Six-line brief

```text
Decision: adopt one profile-neutral `TextFormalBorrowV1` lane: a generation-checked `{slot: u64, generation: u64}` wire owned by the Rust runtime, borrowed/no-escape at callable entry, and kept inside the complete header/Completion cohort; no per-use C call.
Source authority + canonical issuer: the normative Text law, final callable parameter source/BindingRef, `issue_callable_parameter_contract_v1`, and the host-handle generation table; `issue_text_formal_borrow_v1` is the sole runtime formal-lane issuer and the future same-brand header binder is the only compiler capability issuer.
Non-authority: raw HostHandle, `HomeDemand::Handle`, `BorrowedHandleBox`, DynamicV2 TextScan, `nyash.string.eq_hh`, MirType/StringBox spelling, TextEq/Substring, ResultCatalog/body inference, selector/name, AST/MIR re-read, fallback, and retry.
Fail-fast boundary: capture/validation requires a live Text payload and non-zero matching generation; stale, invalid, non-Text, foreign-brand, ordinal/BindingRef drift, or split parameter/result cohorts reject before body effects and map only to the later canonical Trap, never a language Fault.
Smallest next slice: implement the caller-zero Rust wire/validator, fixed C header/status projection, and focused stale/non-Text/ownership tests; keep package/S6C/Builder callers at zero.
Non-claims: no TextEq route/residence, S6C child, V2 envelope, ReadyEntry, MIR/CFG/SSA/PHI, production selector, fallback, retry, or legacy retirement.
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
classes, but it did not bind either class to an ExactText formal slot, source
`BindingRef`, callable header, or Completion cohort. `StringBox`'s Rust
equality and the kernel `eq_hh` export are implementation evidence only; the
pre-Decision state was therefore the typed `NoSafeSlice`, not an adapter
around a nearby generic handle.

The D0 now closes that design gap by naming a new owner rather than claiming
that one of those nearby representations was already sufficient. The owner is
deliberately small and profile-neutral:

```text
source ExactText row + formal BindingRef
  -> same-branded callable header/Completion cohort
  -> issue_text_formal_borrow_v1
       -> TextFormalBorrowV1 { slot, generation }
            -> Rust host-handle Text validator / no-escape read closure
                 -> later canonical Trap consumer (physical-session row)
```

The wire is two fixed-width words so slot reuse is observable. The caller
keeps the strong handle owner until the call returns; the callee neither
retains nor releases it. Rust use is closure-scoped, and the future physical
session must not detach the pair into a raw key or a second residence ledger.
The C surface is a status projection for the same validator, not a second
semantic issuer and not a hot-loop equality helper.

## Accepted wire and failure contract

```c
typedef struct NyrtTextFormalBorrowV1 {
    uint64_t slot;
    uint64_t generation;
} NyrtTextFormalBorrowV1;

// 0 = live Text; non-zero = invariant Trap candidate
uint32_t hako_text_formal_validate_v1(uint64_t slot, uint64_t generation);
```

The exact status enum is owned by the Rust validator and mirrored by the C
header. `slot == 0`, missing slot, generation mismatch, and non-Text payload
are distinct typed rejects. No status becomes a language Fault, truthy Bool,
fallback, or retry. The C validator is called only at the callable ingress or
test probe; TextEq and Loop body operations do not call it per iteration.

The existing DynamicV2 lease-generation code is a substrate for generation
tracking, not the formal-lane owner. The new issuer must co-seal the pair with
the source/header/Completion brand and must not reuse DynamicV2 lease tokens.

## Required design acceptance

The design may close only after it names:

1. one profile-neutral Text formal lane (parameter representation, Home,
   borrow/move/retain/release, and result of parameter admission);
2. the canonical runtime/FFI owner: Rust host-handle generation validation
   plus the fixed `NyrtTextFormalBorrowV1` C projection;
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
  CALLABLE-PHYSICAL-TEXT-PARAMETER-ABI-D0  (accepted design)
    -> CALLABLE-PHYSICAL-TEXT-PARAMETER-ABI-I0 (caller-zero wire/validator)
    -> CALLABLE-S6C-INSTALLED-CHILD-COMPOSITION-D0
    -> LOOP-SEMANTIC-PROGRAM-COSEAL-R0
    -> LOOP-PHYSICAL-TRANSFER-AUTHORITY-R0
    -> LOOP-COMMON-V2-PRESESSION-TRANSPORT-R0
    -> TextEq runtime contract / route admission / residence
    -> canonical MIRBuilder session
    -> parity/canary -> bounded selector -> main integration -> retirement
```

No later row may consume the S6C fixture as a physical callable until the I0
validator and same-brand header binder are green. The source/header R0 remains
closed and is not reopened. If the I0 cannot prove generation/liveness or
requires a raw HostHandle fallback, return to the typed `NoSafeSlice` token
instead of widening this contract.
