---
Status: closed__2026-09-21__MainQualifiedMethodSourceHandoff
Task: MIR-CALL-D1B-MAIN-RAW-QUALIFIED-METHOD-HANDOFF-I0
Date: 2026-09-21
Parent: mir-call-d1b-main-raw-qualified-method-source-coseal-d0-2026-09-21.md
Implementation permission: true for one Main qualified StaticBoxMethod handoff
NextCard: MIR-CALL-D1B-MAIN-RAW-QUALIFIED-METHOD-ACCEPTANCE-D0
---

# Main raw qualified MethodCall handoff I0

## Six-line brief

```text
Decision: consume the existing Main qualified-receiver relation exactly once
  and reuse the existing target-only physical terminal for its finite rows.
Source authority + canonical issuer: resolver MethodCall source rows joined by
  VerifiedNormalCallableSemanticPackageV1::issue_app_main_qualified_receiver_catalog_relation
  with the lifecycle-owned import view and same-module declaration catalog.
Non-authority: Script/AST target inventory, raw name lookup, DirectCall loans,
  CoreMethod rows, compatibility routes, result ABI/publication, and VM.
Fail-fast boundary: Cataloged App Main root, exact source site, receiver and
  selector/arity, same-brand declaration key, ordered argument sites, one-shot
  take, and residual-row completion.
Smallest next slice: retain argument sites, install the relation in the Main
  adapter, add an unarmed/ready port hook, validate argument sites before
  descent, and call the existing target-only bridge.
Non-claims: no other caller, result typing, Script cutover, fallback repair,
  backend parity, legacy deletion, or source-to-exe acceptance.
```

## Implementation boundary

The relation row remains the only source product. Extend it with the resolver's
ordered `argument_sites`; do not rescan AST nodes or derive argument locations
from indexes. The installed Main adapter owns one move-only relation for the
duration of the App Main lowering callback. A narrow port hook returns either an
explicitly unarmed state or one owned handoff containing the already-validated
declaration key and argument-site slice.

The member-call route may use the handoff only for a matching receiver spelling,
selector, and arity at the current Cataloged Main source site. The physical
consumer must validate each argument site through the existing expected-site
descent, then invoke `lower_target_only_static_result_publication_v1`. This
bridge emits the existing physical call; it does not publish a new result ABI.

## Required guards

Focused tests must cover:

* one direct canonical receiver and one import alias reaching the existing
  target-only terminal;
* missing, foreign, duplicate, and second relation takes;
* receiver/selector/arity mismatch before argument descent;
* argument-site order or count mismatch before any argument effect;
* unarmed compatibility/raw and non-Main scopes remaining on their prior route;
* successful Main completion rejecting residual relation rows.

Keep the existing relation brand and declaration-key checks. A failure after the
handoff is terminal for the current lowering transaction; no legacy retry or
Script publication re-entry is allowed.

## Execution receipt

The bounded implementation landed at `d6ee865368`:

* the Main adapter now owns the relation for the lowering callback and exposes
  an explicitly unarmed compatibility hook;
* the exact source row is taken once, retaining the resolver-provided ordered
  argument sites; and
* the existing target-only physical terminal consumes the declaration key after
  expected-site argument descent. No result ABI or publication row was added.

Focused relation evidence is green:
`app_main_qualified_receiver_relation_takes_exact_row_and_argument_sites_once`
and the existing direct, alias, foreign-view, and package one-shot tests. The
new guard checks ordered two-argument sites, receiver mismatch before taking,
exact take, residual completion, and second-take rejection. `cargo fmt --check`
and `cargo check --profile quick --lib` pass. The quick check reports the
repository's existing warning baseline (1,845 warnings); no new warning family
was introduced by this slice.

The full source-backed Main call acceptance remains outside this I0. The
existing `source_backed_app_main_direct_call_consumes_affine_loan` family still
reproduces the known cleanup/state-imbalance baseline, so it is recorded as a
next-card acceptance dependency rather than claimed as a regression fixed here.

## Closeout

Run the focused package/route tests and `cargo fmt --check` on touched files.
Record the stable quick check and known baseline reds separately. Keep all
touched files below the 760-line split threshold and 800-line hard stop. This
I0 proves a physical handoff only; result typing, source-to-MIR acceptance,
production caller switch, and old-edge deletion stay open for the next card.
