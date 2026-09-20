---
Status: landed_bounded__2026-09-21__ResolvedMethodStaticTargetCoissueI0
Task: MIR-CALL-D1B-RESOLVED-METHOD-STATIC-TARGET-COISSUE-I0
Date: 2026-09-21
Parent: mir-call-d1b-resolved-method-static-target-coissue-d0-2026-09-21.md
Implementation permission: landed; owned canonical-key relation and one-shot
  pre-effect consumption only
NextCard: MIR-CALL-D1B-MAIN-RAW-EXACT-SOURCE-ISSUER-LOAN-D0
---

# Resolver MethodCall static-target co-issue I0

## Six-line brief

```text
Decision: retain the exact declaration key already verified by the Main
  resolver/catalog co-issuer and consume that relation once before effects.
Source authority + canonical issuer: the resolver-owned MethodCall row, the
  lifecycle-owned import view, and the same declaration catalog through the
  existing VerifiedNormalCallableSemanticPackageV1 issuer.
Non-authority: AST-backed Script inventory, route-facts replay, receiver/name
  lookup, raw lineage, physical symbols, and any target reconstruction later.
Fail-fast boundary: Main slot/owner, exact site, QualifiedUnbound identity,
  import/catalog brand, alias/direct precedence, static declaration key,
  selector/arity, and duplicate or second consumption.
Smallest next slice: add the canonical declaration key to the owned relation,
  transport it through install, and expose one package-port take terminal.
Non-claims: no Callee emission, affine loan, argument lowering, publication,
  production caller switch, compatibility retry, backend, or legacy deletion.
```

## Finite implementation boundary

This I0 covers only the qualified direct-variable `MethodCall` rows of the
source-backed App Main batch. It may touch the existing relation model, package
install transport, package-port accessor, and focused package tests. The
canonical key must be copied from the declaration returned by the existing
catalog lookup; it must not be reconstructed from `receiver`, `selector`, or
`arity` after the issuer returns.

The one-shot terminal may borrow or take the installed owned relation through
the existing package port, but it must reject a foreign Main site, a missing
relation, and a second take before argument descent. Empty source-backed Main
relations are valid and must be distinguishable from an unavailable Main
relation. The terminal is a transport proof only; it does not lower a call.

Keep `VerifiedSourceMethodCallSiteV1`,
`VerifiedQualifiedCallRouteFactsV1`, and
`VerifiedWholeSourceStaticCallTargetInventoryV1` out of this I0. They are
AST-backed and Script/compatibility-owned. No AST walk, alias re-resolution,
or new semantic issuer may be added.

## Required focused evidence

1. Direct canonical receiver stores and returns the exact declaration key.
2. Imported alias stores the same canonical key while preserving alias admission.
3. Foreign import/catalog, wrong namespace, selector/arity drift, missing or
   duplicate receiver identity, and duplicate relation remain named rejects.
4. The installed package-port terminal takes one exact Main relation and
   rejects a second take or an unavailable relation before any argument effect.
5. `cargo fmt --check`, `git diff --check`, touched-file line counts below
   760/800, and the existing warning baseline classification are recorded.

## Non-claims and reopen

This I0 does not create a public target catalog, `Callee`, affine source loan,
result publication, raw production switch, fallback removal, backend parity,
or old-edge deletion. The next D0 decides how this exact relation enters the
source-to-raw loan and physical target consumer. If the package port cannot
prove one-shot ownership without a second relation or an empty-as-available
fallback, close this I0 as `NoSafeSlice` and retain the typed terminal.

## Receipt (2026-09-21)

Commit `02cc09f7653df52aa4f2350551a4075089f3498d` retains the exact
`CanonicalSameModuleCallableKeyV1` returned by the existing declaration
catalog, transports the owned relation through install, and exposes the
package-port one-shot terminal before Main argument descent. The selected
input helper was moved to `install/selected_lowering_input.rs` so the touched
install owner is 664 lines and the new sibling is 163 lines; all touched Rust
files remain below the 760-line design threshold and 800-line hard stop.

Focused relation evidence is 4/4: direct canonical owner, imported alias,
foreign import view, and one-shot package consumption. The unavailable Main
relation case is 1/1 and proves `None` is distinct from a second take. The
quick lifecycle collection is 31 pass / 5 fail; the five failures are the
existing baseline rows `actual_string_helpers_general_result_row_reaches_its_first_loop_carrier`,
`parser_scan_package_passes_callable_source_handoff_without_fallback`,
`source_backed_app_main_direct_call_consumes_affine_loan`,
`source_backed_package_failure_is_terminal_before_builder_effects`, and
`source_bound_static_result_owner_reaches_the_raw_terminal`, matching the
parent census with no new failure. The quick lib build reports 565 warnings,
the established warning baseline. `cargo fmt --all -- --check` and
`git diff --check` pass.

The next design boundary is the source-to-raw exact source issuer and affine
loan lifetime. No target/Callee, argument lowering, publication, production
switch, compatibility retry, backend parity, or legacy deletion is claimed.
