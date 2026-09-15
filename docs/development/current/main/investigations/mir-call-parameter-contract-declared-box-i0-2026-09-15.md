---
Status: landed__declared_handle__2026-09-15
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

## Receipt — landed 2026-09-15

**Census** (`merged entry program -> declared parameter spellings; includes
every `name: Type` parameter inside `(...)` signature positions across the
723KB merged cohort; excludes field declarations, return annotations,
comments, and string literals`): declared parameter types are exactly
`i64` (2 sites), `ArrayBox` (2), `MapBox` (5). No `StringBox`/`String`/`f64`
parameter spellings occur.

**Decision taken**: a declared box name is a source fact that the parameter
is a handle to a box value. It mints no exact ABI, so it must not collapse
into `OpaqueHandle` (which is documented as the absent-spelling class) and
must not mint an exact classifier row. New kind
`CallableParameterContractKindV1::DeclaredHandle` projects to
`HomeDemandV1::Handle` exactly like `OpaqueHandle`; the admitted spelling set
is the finite census `{ArrayBox, MapBox}` via `is_admitted_declared_box_name`
in `issuer.rs`. `f64`, `ContextBox`, and any other explicit spelling keep the
named `UnsupportedDeclaredType` rejection.

**Implementation files**: `src/mir/callable_parameter_contract/model.rs`
(new `DeclaredHandle` variant + `Handle` projection),
`src/mir/callable_parameter_contract/issuer.rs` (bounded name admission
between exact classifiers and rejection),
`src/mir/callable_parameter_contract/tests.rs` (+2 focused tests),
`src/mir/normal_callable_semantic_package/dynamic_admission.rs`
(`DeclaredHandle` -> `Dynamic` parameter class),
`src/mir/normal_callable_semantic_package/physical_signature.rs`
(`DeclaredHandle` -> `OrdinaryScalar` lane),
`src/mir/resolved_semantics/home_prefix_local_flow.rs`
(`DeclaredHandle` -> `StoredLocal::Handle`),
`src/mir/callable_parameter_contract/README.md` (vocabulary).

**Focused evidence**: `CARGO_BUILD_JOBS=4 cargo test --profile quick --lib
callable_parameter_contract` -> 10 passed / 0 failed;
`normal_callable_semantic_package` -> 185 passed / 0 failed. The ~536 warning
set is existing repository warning debt, unchanged by this slice.

**Route evidence**: `./target/quick/hakorune --backend mir
/tmp/merged_entry.hako` now advances past
`ParameterContract/UnsupportedDeclaredType` and stops at the next named
terminal `[mir/callable-semantic-package/issue] Dynamic { _batch_slot: 108,
_issue: Completion { _error: ReturnClassificationInvariant } }` — a function
whose return set mixes `return <value>` with `return null`/`return`
(`Void`/`Value` classification split in
`resolved_control_flow/function_control.rs`; `variant_payload_type`-shaped
bodies are the representative cohort). This is the next bounded slice, not
part of this card.

**Non-claims kept**: no parameter ABI for box values, no heap-layout or
ownership meaning for declared names, no caller switch, no dynamic-admission
completion change, no legacy retirement.
