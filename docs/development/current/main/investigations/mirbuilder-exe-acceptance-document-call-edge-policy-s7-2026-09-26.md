# MIRBUILDER-EXE-ACCEPTANCE-DOCUMENT-CALL-EDGE-POLICY-S7

Status: landed
Date: 2026-09-26
Parent: workstream row H (unified resume, gate 1);
  CATALOGED-CALL-EDGE-DOMAIN-D19 decision accepted
Mode: fast — one responsibility, one production edge.

## Responsibility

The cataloged same-module call-edge **domain** rule — proven
non-`Integer` argument kinds on a `Call`/`Invoke{Call}` edge,
including the `MapBox` actual/formal pair arm — is a sealed-corridor
admission contract, not canonical validity. Document publication
(`compile_normal_for_mir_json`) verifies the canonical structure of
the same module under an explicit document policy instead of being
rejected by a corridor contract its calls never claim.

## Boundary

- `invoke::check_module` gains an explicit call-edge policy
  parameter. The arity rule
  (`call.args.len() + receiver_params != callee.params.len()`) and
  every other `check_module` arm (object/field/home lifecycle
  vocabulary) stay unconditional; only the kind-domain block
  (`map_formal`/`map_actual` pair + `scalar_drift`) is consulted
  under the sealed policy.
- `MirVerifier::verify_module` keeps the sealed policy — every
  existing caller (`compile_normal_with_published` both routes,
  `module_postprocess`, `drain_terminal`, `compiler/mod.rs`, tests)
  is unchanged by construction.
- A `verify_document_module` entry delegates the same verification
  body with the document policy — one verification authority, the
  caller names the admission question.
- `compile_normal_for_mir_json` calls `verify_document_module`.
- View arm landed under this card as **reclassification**, not
  deferral: `validate_static_call` / `validate_free_function_call`
  return the resolved definition, and `try_new` pushes a
  `PublishedStaticMethodCallRef` / `PublishedFreeFunctionCallRef`
  row only when the definition returns `Integer`. A non-`Integer`
  result never enters the corridor row vocabulary — it marks
  `has_non_lifecycle_unsupported` instead, so the module classifies
  `UnsupportedBeforeObject` and the `map_projection` `Domain::I64`
  claim on checked call sites holds by construction. The
  `*RequiresIntegerReturn` error variants are removed (dead
  vocabulary).

## Fail-fast boundaries

- `verify_document_module` still rejects: arity drift on cataloged
  edges, every `check_function_in_module` structural arm (SSA,
  dominance, CFG, fault-frame, Invoke result-kind vocabulary,
  `map::check` lease invariants), object/field/home definition
  drift, `invoke::check_module` non-domain arms.
- `verify_module` is untouched: `compile_normal_with_published`
  still enforces the sealed domain for `CanonicalTyped` and
  `Unsupported`+lifecycle routes; `admit_lifecycle` and
  `hako_physical_validate_ordinary_call` keep their own gates.
- `ExplicitCompatibility` document arm unchanged
  (`reject_unretained_module` + compat outcome).

## Non-claims

- No new admitted argument domain for any backend lane — a
  `CanonicalTyped` module with a `String`/`Float`/`Box` arg still
  freezes under the sealed policy.
- No `Callee`/edge emission change; no pinned-text residence
  interpretation (a `String` record stays a document datum, never a
  residence proof).
- No claim the JSON app reaches a sealed lane — its route remains
  `UnsupportedBeforeObject`; object/instance admission is a later
  family.
- VM `ingest/1` ledger-less spine; Gates 2–4; overall completion.

## Evidence

- `invoke::check_module`/`check_call_edge` now take
  `CatalogedCallEdgePolicyV1`; `Document` returns after arity
  validation, `Sealed` keeps the Integer-domain + `MapBox` pair rule.
- `MirVerifier::verify_document_module` shares the full verification
  body with `verify_module` — only the arg-domain arm differs.
- `compile_normal_for_mir_json` calls `verify_document_module`.
- View reclassification: `try_new` pushes corridor rows only for
  `Integer`-returning definitions; non-`Integer` results set
  `has_non_lifecycle_unsupported` (route `UnsupportedBeforeObject`,
  rows excluded). `*RequiresIntegerReturn` variants removed.
- Focused tests green:
  `cataloged_call_*` 5/5, `document_policy_*` 2/2,
  `published_view_*` 4/4, `non_integer_static_result_*` 2/2,
  `arity_scope` 1/1,
  `map_literal_complete_actions_use_checked_canonical_call_and_formal`
  1/1.
- Production smoke
  (`hakorune --backend mir --emit-mir-json` on
  `apps/json-stream-aggregator/main.hako`) advances past
  `StaticMethodRequiresIntegerReturn` and
  `call-argument-type-drift`; next terminal is a different family —
  strict-verifier SSA dominance violations
  (`Value %N used in block bbM but defined in non-dominating block`,
  `Merge block ... without Phi`), which is structural lowering
  evidence, not a call-edge contract.
- Baseline debt: `named_array_source` full-suite stack overflow
  reproduces at HEAD; unrelated fmt drift pre-exists in
  `published_backend_view_tests.rs`.
