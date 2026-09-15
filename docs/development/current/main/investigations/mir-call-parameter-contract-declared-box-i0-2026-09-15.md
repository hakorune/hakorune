---
Status: selected__fast__2026-09-15
Task: MIR-CALL-PARAMETER-CONTRACT-DECLARED-BOX-I0
Date: 2026-09-15
Priority: admit declared box-typed parameters through the callable parameter contract
Parent: mir-call-resolver-if-source-result-product-issuer-d7-reentry-2026-09-15.md
NextCard: next named merged-route terminal after ParameterContract
Implementation permission: true for the callable_parameter_contract owner only
---

# Parameter contract declared-box admission I0

## Six-line brief

```text
Decision: the merged-route lane currently stops at ParameterContract/UnsupportedDeclaredType for declared box-typed parameters (ArrayBox/MapBox and other box names); admit them as opaque parameters or name the exact rejected subset — never silently widen the exact-ABI classifiers.
Source authority + canonical issuer: issue_callable_parameter_contract_v1 in src/mir/callable_parameter_contract/issuer.rs owns the declared-type contract; ExactTextFormalAbiV1/ExactTrivialParameterAbiV1 remain the sole exact-ABI classifiers.
Non-authority: declared names are transport/diagnostic data and must not mint a representation, MIR type, or binding identity; a declared box type does not authorize heap layout or ownership.
Fail-fast boundary: ordinary-transfer requirement, parameter binding identity/ownership checks, and the catalog coverage seal stay unchanged; a declared type that cannot join an admitted kind is a named rejection, not a fallback.
Smallest next slice: census the merged cohort's declared parameter types, decide OpaqueHandle-vs-new-kind for declared box names, and admit exactly that subset with focused tests.
Non-claims: parameter ABI for box values, instance dispatch, production caller switch beyond the named merged-route terminal, VM parity, and legacy retirement.
```

`Census boundary: merged entry program -> callable semantic package parameter
contract; includes every declared parameter type across the 131-declaration
merged cohort; excludes return types, local declarations, and body shapes.`

## Entry contract

The previous cards closed the static-parent seal and the resolver
expression-If issuer. This card must first census which declared parameter
types actually appear in the merged cohort, then extend
`CallableParameterContractKindV1` admission by exactly that subset. The
`OpaqueHandle` kind already covers undeclared parameters; a declared box name
is only a stronger source fact and must not imply a representation.

## Acceptance

Focused tests prove admission of the censused declared box types and rejection
of a still-unsupported name, with the batch loan and binding checks unchanged.
Run one quick-profile lib test process with at most four build jobs, classify
repository warning debt as baseline, and update the owner README and pointer
in the same closeout slice. Route evidence: the merged entry must advance past
`ParameterContract/UnsupportedDeclaredType` to the next named terminal.
