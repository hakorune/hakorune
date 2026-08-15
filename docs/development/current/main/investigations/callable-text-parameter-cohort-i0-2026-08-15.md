---
Status: complete — bounded cohort I0 landed; parent common-V2 D0 remains open
Date: 2026-08-15
Parent: `docs/development/current/main/investigations/callable-text-parameter-abi-d0-2026-08-15.md`
Authority: `src/mir/normal_callable_semantic_package/`
---

# CALLABLE-TEXT-PARAMETER-COHORT-I0

This I0 proves that the already-issued semantic `ExactTextFormalAbiV1` row
crosses the existing same-branded package/install/Port cohort. It does not
make a runtime Text handle executable and does not admit the S6C production
candidate.

## Six-line implementation brief

```text
Decision: retain the existing package/install/Port ownership path and add only focused evidence that explicit StringBox parameters remain ExactText after install and scoped lowering loan.
Source authority + canonical issuer: issue_callable_parameter_contract_v1 remains the sole formal-row issuer; package/install copies and lends the already-issued row without source re-read or reclassification.
Non-authority: raw batch slots, caller-supplied rows, S6C ingress, Main membership, MIR/runtime types, HomeDemand, selector names, result catalogs, and physical Text ABI.
Fail-fast boundary: same-brand install and selected-key loan must preserve ordinal, BindingRef, owner, and ExactText kind; foreign context, duplicate consumption, detached row, or kind drift rejects through existing typed boundaries.
Smallest next slice: add ordinary StringBox package/Port positive coverage, retain existing foreign/duplicate/incomplete negatives, update module README and reusable guard census; no production caller or Builder/session.
Non-claims: no S6C child issuer, result/header ABI, runtime handle/wire, lease/residence, common V2 envelope, physicalizer, fallback, retry, or selector switch.
```

## Acceptance

```text
ordinary static Box method with StringBox parameters enters the generic package
install/commit preserves same-brand catalog and private parameter rows
Port loan returns ExactText with exact ordinals and BindingRef identity
foreign catalog and duplicate/incomplete selected-key negatives remain green
production S6C/physical caller census remains zero
```

The S6C `Main.find_ok` scan candidate is intentionally not used here: its
selected membership and result/header ABI remain parent-D0 blockers. This I0
only proves the generic cohort transport for an ordinary selected callable.

## Evidence

```text
positive: package_scoped_loan_retains_exact_text_parameter_contract 1/1
package suite: normal_callable_semantic_package 20/20
hardening: Port missing/duplicate/owner-drift paths are typed; the old
           unwrap_or(&[]) fail-open path is removed and structurally guarded
checks: cargo check --lib (inherited warnings only), cargo fmt, diff check,
        current-state pointer, Loop physical-transfer, and Loop pre-cutover
        guards all green
caller census: production S6C/physical caller remains 0
non-claims: no runtime Text wire, S6C child, result/header ABI, Builder/session,
            fallback, retry, or selector switch
```
