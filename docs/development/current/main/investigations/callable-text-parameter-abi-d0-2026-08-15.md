---
Status: accepted design — bounded cohort I0 complete; parent common-V2 D0 remains open
Date: 2026-08-15
Parent: `docs/development/current/main/investigations/s6c-text-eq-physical-contract-d0-2026-08-15.md`
Authority: `src/mir/callable_parameter_contract/` and the existing normal package/install/Port cohort
---

# CALLABLE-TEXT-PARAMETER-ABI-D0

This is the next bounded design slice after the completed
`CALLABLE-PARAMETER-EXACT-TEXT-HANDLE-I0`. It closes the same-branded package
cohort boundary for the semantic `ExactTextFormalAbiV1` row. Despite the
historical `ABI` name, this row is still a formal source/parameter demand; it
does not invent a runtime handle wire.

## Six-line brief

```text
Decision: extend the existing Normal callable package/install/Port cohort so an already-issued ExactText(StringBox-as-Text) parameter row survives the same branded loan without becoming OpaqueHandle or a physical runtime handle.
Source authority + canonical issuer: VerifiedResolvedCallableSemanticBatchV1 -> issue_callable_parameter_contract_v1 remains the sole formal-row issuer; package installation copies only the already-issued private row while the source catalog remains owned by the branded CompilationContext.
Non-authority: S6C typed input, Main/find_ok names, raw batch slots, fixture expectations, MIR types, runtime handles, TextEq/Substring, result catalogs, StringBox::equals, Dynamic fallback, selector strings, and any detached Port argument cannot issue or repair the row.
Fail-fast boundary: before a Port loan, reject foreign catalog/package brand, missing or duplicate parameter rows, ordinal/BindingRef/owner drift, explicit StringBox downgraded to OpaqueHandle, absent annotation promoted to ExactText, i64 reclassified as Text, and any external S6C parameter product.
Smallest next slice: freeze the branded context-catalog + Installed package + Port HRTB ownership graph, copied-row lifetime, exact row projection, and negative matrix only; no package code, fixture edit, runtime symbol, or S6C production caller opens while design_stop is active.
Non-claims: no physical Text handle/wire, stale-generation/liveness, retain/release, result/header ABI, Main membership, S6C Facts/Recipe/Completion, common V2 envelope, Builder/MIR/CFG/SSA/PHI, ReadyEntry/session, fallback, retry, or selector switch.
```

## Ownership shape

The existing catalog remains the sole formal parameter authority. The package
does not take ownership of the catalog after install; installation moves the
catalog into its existing `CompilationContext` owner and retains the catalog
brand plus private copied contract rows needed by the Port loan.

```text
VerifiedResolvedCallableSemanticBatchV1
  -> issue_callable_parameter_contract_v1
       -> VerifiedCallableParameterContractCatalogV1
            -> VerifiedNormalCallableSemanticPackageV1
                 -> consuming install/commit
                      -> branded CompilationContext catalog
                      + InstalledNormalCallableSemanticPackageV1
                        (brand, selected coverage, private parameter rows)
                           -> NormalCallableSemanticPackagePortV1
                              HRTB loan of those same rows
```

The Port must not accept a caller-supplied `VerifiedCallableParameter...`,
raw batch slot, `BindingRef` tuple, or S6C ingress. The only valid input is the
already installed same-brand cohort. The generic catalog may contain any
ordinary parameter ordinal; S6C may later co-check exactly two rows at
ordinals `0` and `1`, but that is a consumer constraint, not a second issuer.

`ExactTextFormalAbiV1` is site-free and carries no ordinal or binding. The
already-issued `VerifiedCallableParameterContractV1` owns ordinal, BindingRef,
owner, function origin, and declaration mode. `HomeDemandV1::Handle` is only a
one-way projection; Text specificity stays in `ExactTextFormalAbiV1` and no
physical Home/lease/wire is inferred.

## Required acceptance

Positive:

```text
explicit StringBox -> ExactTextFormalAbiV1
explicit i64       -> ExactTrivial(i64)
absent annotation  -> OpaqueHandle
same-brand install -> copied private rows retain exact ordinal/BindingRef/owner
Port HRTB loan     -> borrows only the installed cohort; no detached row escapes
arbitrary generic ordinals remain valid; S6C 0/1 parity is deferred to S6C
```

Negative:

```text
foreign CompilationContext/catalog brand
foreign Installed package or selected map
missing/duplicate parameter row
ordinal swap, BindingRef swap, owner/origin drift
StringBox downgraded to OpaqueHandle
unannotated row promoted to ExactText
i64 reclassified as ExactText
external raw batch slot, fixture-built S6C ingress, or caller-supplied row
physical Text wire/handle inferred from HomeDemand or MIR type
Dynamic compatibility fallback/reclassification
```

The formal-row issuer remains the only place that recognizes the source
spelling `StringBox`. Package/install/Port code only checks same-brand
identity and projects the retained row; it must not parse source syntax or
reclassify a kind.

## Stop conditions

Keep the parent stop open and return `NoSafeSlice` if this boundary requires:

```text
adding a second parameter issuer;
accepting a detached row and comparing keys after the fact;
keeping the whole catalog inside Installed when the existing owner is Context;
using OpaqueHandle as an ExactText substitute;
deriving Text from MIR/fixture/name/selector data;
introducing a runtime handle/wire, stale-generation, lease, or trap claim;
opening Main membership, result/header ABI, S6C ingress, or common V2 transport;
```

## Later DAG

```text
CALLABLE-TEXT-PARAMETER-ABI-D0  (this design stop)
  -> CALLABLE-TEXT-PARAMETER-COHORT-I0
     same-branded install + Port loan, caller-zero and Builder-free
  -> source-backed result/header ABI D0
  -> installed-batch S6C child/composition D0
  -> common V2 transport R0/I0
```

No implementation is authorized by this card while `work_mode=design_stop`.
After acceptance, the I0 must remain a bounded package/Port cohort change and
must not be combined with result/header, Main membership, S6C Recipe, or
physical session work.

## Design acceptance

The existing source-backed issuer, consuming install transition, branded
`CompilationContext` catalog, installed private parameter rows, and scoped Port
loan already provide the required owner graph. No new parameter authority or
catalog owner is needed for this child. The bounded implementation is opened
at `callable-text-parameter-cohort-i0-2026-08-15.md` and is limited to proving
that an ordinary selected `StringBox` callable retains `ExactText` through
install and Port borrowing. The S6C scan candidate remains outside this I0:
its Main membership, result/header ABI, and child composition are parent-D0
blockers.
