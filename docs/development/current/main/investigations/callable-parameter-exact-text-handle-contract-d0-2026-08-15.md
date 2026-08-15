---
Status: current design_stop — bounded child of the common V2 pre-session D0
Date: 2026-08-15
Parent: `docs/development/current/main/investigations/s6c-text-eq-physical-contract-d0-2026-08-15.md`
Authority: `src/mir/callable_parameter_contract/` plus `docs/reference/language/types.md §4.3`
---

# CALLABLE-PARAMETER-EXACT-TEXT-HANDLE-CONTRACT-D0

This is the single bounded child selected from
`LOOP-S6C-COMMON-V2-PRESESSION-CONTRACT-D0`. It does not open the S6C
physical session and does not split the parent D0 into a parameter/result/
membership card family. The parent remains the umbrella stop; this child only
closes the earliest reusable formal-parameter boundary.

## Six-line brief

```text
Decision: design one profile-neutral exact Text formal parameter contract for the existing callable-parameter catalog; StringBox-as-Text is an explicitly admitted source bridge, not an OpaqueHandle default and not a runtime handle implementation.
Source authority + canonical issuer: VerifiedResolvedCallableSemanticBatchV1 parameter rows plus the normative String/Text law; the existing issue_callable_parameter_contract_v1 remains the sole issuer and later package installation consumes its same-brand catalog.
Non-authority: S6C typed input, parameter names, Main/find_ok spelling, fixture expectations, MIR MirType, EffectMask, runtime handles, StringBox::equals, selector strings, OpaqueHandle fallback, and any S6C physicalizer cannot issue the generic contract.
Fail-fast boundary: reject non-ordinary/foreign/duplicate parameters, wrong ordinal or BindingRef, wrong owner/origin, explicit types other than the exact admitted StringBox-as-Text spelling for this row, and any count/order drift before package install or Builder/session.
Smallest next slice: freeze ExactTextFormalAbiV1, its source/binding/home-demand projection, issuer API, same-brand package handoff, and positive/negative matrix only; no code, fixture edit, runtime symbol, or receipt is issued while design_stop is active.
Non-claims: no Text handle wire, retain/release/lease, ABI symbol, TextEq route, result/header ABI, Main selected membership, S6C child issuer, V2 envelope, ReadyEntry, MIR/CFG/SSA/PHI, fallback, retry, or production caller.
```

## Target contract shape

The existing catalog remains the only parameter-contract owner. The bounded
extension is a semantic formal demand, not a physical runtime handle:

```text
CallableParameterContractKindV1
  ├─ OpaqueHandle                         # only for unannotated legacy rows
  ├─ ExactTrivial(ExactTrivialParameterAbiV1)
  └─ ExactText(ExactTextFormalAbiV1)       # this child, source-backed

ExactTextFormalAbiV1
  source spelling: StringBox
  logical class:   Text
  home demand:     existing Handle projection; Text specificity stays here
  ownership:       formal declaration only; no move/retain/release claim
  physical wire:   unresolved here; later runtime/port contract only
```

`ExactTextFormalAbiV1` must not be constructible from a string, `MirType`,
runtime tag, or S6C role. Its issuer verifies the source parameter row,
`BindingRef`, owner, function origin, ordinal, ordinary-transfer status, and
the language-law bridge. The S6C typed-input product may co-check the same
binding later, but it is not a second issuer of the generic parameter row.
The type is site-free and its constructor remains issuer-private; the only
copyable part is the profile enum, never a source-bound row.

Every existing classifier arm that consumes `CallableParameterContractKindV1`
must handle `ExactText` explicitly. In particular, the Dynamic compatibility
consumer must reject or decline the text row by its existing typed boundary;
it must not reinterpret `ExactText` as `OpaqueHandle`, `Dynamic`, `I64`, or a
fallback candidate.

The formal contract deliberately says `ExactText`, not “raw runtime handle”.
The canonical handle wire, lifetime, stale-generation rule, and trap policy
remain separate physical-contract decisions.

## Owner and handoff

```text
VerifiedResolvedCallableSemanticBatchV1
  -> issue_callable_parameter_contract_v1
       -> VerifiedCallableParameterContractCatalogV1
            -> branded package/install cohort
                 -> NormalCallableSemanticPackagePortV1 loan
```

The package/port may later require the S6C profile to contain exactly two
`ExactText` rows at ordinals 0 and 1. It must borrow the already-issued
catalog row from the same branded batch; it may not accept a test-built S6C
ingress, a raw batch slot, or a second parameter ledger.

## Required acceptance matrix

Positive:

```text
two ordinary parameters, source spelling StringBox
ordinal 0/1 and exact BindingRef identity
same owner and function origin as the declaration row
same-brand parameter rows are retained by the installed package and are
borrowable through the port; the source catalog itself remains in its
CompilationContext owner
S6C typed-input Text bindings co-check without reissuing the contract
```

Negative:

```text
missing parameter row or count != source declaration count
ordinal swap, binding swap, owner/origin mismatch, duplicate BindingRef
non-ordinary/transfer parameter
explicit i64, Integer, OpaqueHandle spelling, or unknown nominal type
unannotated parameter silently promoted to ExactText
foreign catalog/batch brand or detached raw batch slot
S6C typed input used as the generic issuer
physical wire/handle/lease claim smuggled into the semantic row
Dynamic/compatibility arm silently treats ExactText as OpaqueHandle or I64
```

## Stop lines

Remain `NoSafeSlice` if the row requires any of the following:

```text
reusing OpaqueHandle as an exact Text contract
inferring Text from names, MIR, EffectMask, StringBox::equals, or runtime tags
making the generic issuer depend on S6C Recipe/Recipe keys
adding a second parameter authority in the package or S6C adapter
claiming a canonical Text handle wire or stale-generation guarantee here
opening a result/header, Main-membership, V2-envelope, Builder, or session row
```

## Boundary to the parent D0

Closing this design child only names the formal parameter contract. It does
not close the parent pre-session stop. The parent still requires, in order,

```text
Main selected membership
same-brand source result/header ABI co-seal with Completion
installed-batch S6C child issuer and parent HRTB composition
13 operations + separate If/Exit control + exact 15-placement envelope
neutral CanonicalSsaFunctionSessionV2 admission
```

After this D0 is accepted, its bounded I0 may open independently. The parent
still cannot open package/session work until its remaining source-backed
issuers are named.
