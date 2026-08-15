---
Status: complete — bounded semantic I0 landed; parent common-V2 D0 remains open
Date: 2026-08-15
Parent: `docs/development/current/main/investigations/callable-parameter-exact-text-handle-d0-2026-08-15.md`
Authority: `src/mir/callable_parameter_contract/`
---

# CALLABLE-PARAMETER-EXACT-TEXT-HANDLE-I0

This I0 implements only the accepted semantic formal-parameter contract. It
does not make StringBox callable lowering executable and does not open the
parent S6C package/session boundary.

## Six-line implementation brief

```text
Decision: add one source-spelling ExactText parameter kind to the existing callable-parameter catalog and keep all physical handle/wire claims outside this slice.
Source authority + canonical issuer: VerifiedResolvedCallableSemanticBatchV1 parameter rows; issue_callable_parameter_contract_v1 is the sole classifier/issuer.
Non-authority: MIR types, runtime handles, S6C Facts, Recipe keys, selector names, StringBox::equals, and Dynamic compatibility inference.
Fail-fast boundary: exact StringBox ordinary rows become ExactText; i64 and absent annotations retain their existing kinds; unknown explicit types, foreign/duplicate bindings, and non-ordinary transfers reject.
Smallest next slice: add the profile type, enum arm, issuer match, explicit Dynamic compatibility reject, focused positive/negative tests, and module README/guard census.
Non-claims: no physical Text ABI, result/header, Main membership, S6C child, V2 envelope, Builder/MIR/session, fallback, retry, or production caller.
```

## Acceptance

```text
ExactTextFormalAbiV1::classify("StringBox") = Some(...)
all near spellings reject
generic parameter catalog preserves arbitrary source ordinals and BindingRef
i64 remains ExactTrivial; no annotation remains OpaqueHandle
Dynamic compatibility matches ExactText explicitly and returns its existing
typed ParameterContractMismatch instead of reclassifying or falling back
focused tests cover positive StringBox, near spelling, binding identity, and
compatibility rejection
```

The installed package remains caller-zero for this slice. The copied private
parameter rows are not a new catalog authority; the source catalog stays in
its existing CompilationContext owner. Parent S6C co-check is later.

## Evidence

```text
implemented: src/mir/exact_text_parameter_abi.rs,
            callable_parameter_contract/{issuer,model,tests,README,mod}.rs,
            normal_callable_semantic_package/dynamic_admission.rs
positive:   callable_parameter_contract tests 8/8
negative:   exact_text_is_rejected_before_dynamic_recipe_reclassification 1/1
profile:    exact_text_parameter_abi tests 1/1
checks:     cargo check --lib (1826 inherited warnings, no compile failure),
            cargo fmt --all -- --check, git diff --check,
            current_state_pointer_guard, loop_physical_transfer_authority_guard,
            loop_precutover_authority_guard
caller census: production ExactText/S6C/physical callers remain 0
non-claims: no runtime ABI, result/header, S6C package, Builder/session, or
            fallback/retry route was opened
```
