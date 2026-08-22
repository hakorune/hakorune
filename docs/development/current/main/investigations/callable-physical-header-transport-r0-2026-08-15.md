---
Status: completed caller-zero transport row; next pointer is CALLABLE-PHYSICAL-TEXT-PARAMETER-ABI-D0
Date: 2026-08-15
Parent: `docs/development/current/main/investigations/callable-physical-header-abi-d0-2026-08-15.md`
Classification: T1 BoxShape transport refactor with one bounded source/header product
Authority: final callable source, callable parameter issuer, verify_function_completion_v1, and same-brand package/Port cohort
---

# CALLABLE-PHYSICAL-HEADER-TRANSPORT-R0

This row implements only the accepted source/header transport boundary. It
adds the explicit `: i64` source annotation to the canonical scan fixture and
co-seals the existing formal parameter rows with a source-backed result/header
and `VerifiedFunctionCompletionV1` inside the same normal-callable package.
It does not implement a runtime handle wire or enter MIRBuilder physical
lowering.

## Four-block implementation brief

```text
Change: add one non-splittable source/header cohort to the existing package and Port; retain exact formal rows, explicit result spelling, owner/batch identity, and Completion proof without a second catalog or raw AST escape.
Contract: source annotation is explicit `i64`; ExactText/i64/OpaqueHandle parameter kinds remain owned by issue_callable_parameter_contract_v1; verify_function_completion_v1 is the only exit/cleanup issuer; the new view is borrow-only and non-physical.
Done: selected/package focused positives and negatives prove same-brand transport, exact source/result/Completion parity, foreign/missing/duplicate rejection, and no parameter-only/result-only path; source and touched Rust files stay below 760/800 lines; existing pointer/Loop guards stay green.
Stop: no body/MIR/ResultCatalog inference, no runtime Text handle/wire, no C symbol, no lease/retain/release, no S6C Recipe/physical session, no Builder IDs, fallback, retry, or production caller; return to NoSafeSlice on any missing source/header owner.
```

## Ownership shape

```text
VerifiedResolvedCallableSemanticBatchV1
  -> issue_callable_parameter_contract_v1
  -> explicit source result annotation
  -> verify_function_completion_v1
       -> VerifiedCallablePhysicalHeaderCohortV1
            -> VerifiedNormalCallableSemanticPackageV1
                 -> InstalledNormalCallableSemanticPackageV1
                      -> NormalCallableSemanticPackagePortV1
                           -> SelectedCallableLoweringInputRefV1
                                -> narrow header borrow view
```

The cohort is owned by the package and moved through install. The catalog
still remains in `CompilationContext`; the Port proves the same catalog brand
before lending the cohort. It accepts no caller-supplied row, key, batch slot,
AST node, ResultCatalog, or Completion.

The new product stores one row per resolved static/instance callable covered
by the existing formal parameter catalog. Each row retains only:

```text
batch slot + owner
ExactTrivialScalarAbiV1 result (currently exact i64 only)
VerifiedFunctionCompletionV1
```

The formal parameter rows remain in the package's existing owned contract
catalog; the issuer checks exact batch-slot/owner/cardinality parity before
the cohort is sealed. No source site, Recipe key, MIR type, ValueId, or
physical handle is duplicated.

## Implementation slices

1. Add `: i64` to `find_ok` in
   `apps/tests/scan_with_init_typed_ok_min.hako`; no body or control change.
2. Add a small package-local `physical_header` owner that issues the cohort
   from the existing batch, formal parameter rows, and Completion verifier.
3. Move the cohort through package install and expose a narrow borrow view from
   the selected lowering input. Keep the raw `VerifiedFunctionCompletionV1`
   owned by the cohort; expose only named result/owner/target/exit/cleanup
   projections needed by later D0.
4. Add a new focused test module rather than growing the 737-line package
   test file. Reuse existing parser/package helpers and existing Loop guard.

## Focused acceptance matrix

Positive:

```text
explicit `: i64` scan fixture -> Annotated(i64) source result
existing supported formal rows -> exact ordinal/BindingRef/owner retained
Completion -> exact source exit set, same owner/target, empty cleanup
same-brand install + Port loan -> one narrow header view, no detached parts
ordinary i64-only package -> existing rows remain accepted
```

The current package parameter issuer remains the authority for semantic
`ExactText(StringBox-as-Text)` rows, but this R0 does not mint a physical Text
parameter wire. The canonical S6C fixture therefore exercises the source
annotation change and remains outside the package's physical-header positive
until the later Text formal ABI row is accepted.

Negative:

```text
missing annotation                         -> no header cohort (`None`); later physical consumer stops
declared non-i64                           -> ResultHeaderMismatch
foreign package/catalog brand              -> ForeignCatalog / ForeignCohort
formal row missing/duplicate/owner drift   -> HeaderCoverage / FormalBindingMismatch
Completion target/exit/cleanup drift       -> typed completion reject
body/MIR/ResultCatalog-only inference      -> API unavailable / reject
parameter-only or result-only projection   -> IncompleteCallableHeader
raw AST/key/batch-slot caller              -> API unavailable / reject
Text handle/wire/ABI symbol requested      -> ScopeViolation / not represented
fallback/retry                             -> structural guard failure
```

## Guard and file budget

Reuse `loop_physical_transfer_authority_guard.sh` and extend only its existing
S6C/package surface checks if necessary. Do not create a top-level guard.
Keep the new owner in a separate file; do not grow `install.rs` or the package
test file across the 760-line design trigger/800-line hard stop. The source
fixture change is the only fixture change in this row.

## Evidence receipt (2026-08-15)

```text
cargo check --lib -q                                  PASS
cargo test --lib normal_callable_semantic_package     27 passed / 0 failed
cargo test --lib s6c_                                  30 passed / 0 failed
cargo test --lib normal_callable_semantic_package::physical_header_tests
                                                       3 passed / 0 failed
loop_physical_transfer_authority_guard.sh             PASS
current_state_pointer_guard.sh                        PASS
cargo fmt --all -- --check                            PASS
git diff --check                                      PASS
```

The latest full lib suite reports `6692 passed / 134 failed / 29 ignored`.
Those failures are the inherited broad baseline tracked by the parent/current
state (the new package/header and S6C focused rows are green); no production
caller or Builder path was opened by this R0. The broad red remains a parked
baseline-cleanup task, not a transport-row regression claim.

## Non-claims and next DAG

```text
not claimed here:
  runtime Text handle/wire, C ABI, stale-generation/liveness, lease,
  TextEq route/residence, S6C ingress/Recipe, V2 operation/control envelope,
  ReadyEntry, Builder/MIR/CFG/SSA/PHI, production caller, selector, fallback,
  retry, main integration, or legacy retirement

after this row:
  S6C-INSTALLED-BATCH-CHILD-COMPOSITION-D0
    -> LOOP-SEMANTIC-PROGRAM-COSEAL-R0
    -> LOOP-PHYSICAL-TRANSFER-AUTHORITY-R0
    -> LOOP-COMMON-V2-PRESESSION-TRANSPORT-R0
```
