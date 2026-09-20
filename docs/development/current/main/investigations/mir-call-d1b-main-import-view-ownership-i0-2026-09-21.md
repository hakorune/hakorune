---
Status: fast__2026-09-21__MainImportViewOwnership
Task: MIR-CALL-D1B-MAIN-IMPORT-VIEW-OWNERSHIP-I0
Date: 2026-09-21
Parent: mir-call-d1b-main-import-view-ownership-d0-2026-09-21.md
Implementation permission: true; one lifecycle-owned import view and owned
  Main qualified-receiver relation only
NextCard: none
---

# Main import-view ownership I0

## Six-line brief

```text
Decision: seal the invocation import view once in the normal root lifecycle,
  borrow it into Script and Main co-issuers, and retain only owned Main relation
  rows in the semantic package.
Source authority + canonical issuer: ModuleBuilderInvocationSession's
  using_import_boxes plus the package's exact declaration catalog; lifecycle
  owns the single VerifiedStaticImportAliasViewV1 seal.
Non-authority: Script AST inventory, package-local empty views, name/arity
  lookup, MIR, physical symbols, compatibility routes, and target inference.
Fail-fast boundary: alias/owner/import validation, catalog pointer, one view per
  invocation, exact Main slot/site/brand, and relation selector/arity checks.
Smallest next slice: change the two consumers to borrow the lifecycle view,
  add the owned Main relation field/transport, and add focused guards.
Non-claims: no target/loan/publication, Recipe/ABI/MIR lowering, dispatcher,
  fallback, backend parity, production switch, or legacy deletion.
```

## Finite implementation boundary

Touch only the normal root lifecycle, Script direct-static lookup issuer,
package model/install transport, and the existing Main relation owner. The
lifecycle extracts `using_import_boxes`, seals one view against the package
catalog, passes it to Script lookup and the resolver-ledger Main co-issuer,
then drops the borrowed view after the owned relation is retained.

Replace the package CoreMethod helper's empty `seal` call with an explicitly
named brand-only construction; it must not consume or recreate invocation
alias rows. Do not reuse `VerifiedQualifiedCallRouteFactsV1` or any whole-source
AST inventory.

## Required focused evidence

Cover one valid imported alias and one direct canonical owner, plus duplicate,
empty, foreign-owner, catalog-brand, Main-slot, missing/duplicate receiver
identity, lexical, selector, and arity rejection. Confirm Script and Main use
the same borrowed view, the package retains owned relation rows, and no target
or loan is issued. Record exact Cargo command/result, line counts, warning
baseline classification, pointer sync, and `git diff --check` at closeout.

## Implementation receipt (working tree, 2026-09-21)

The lifecycle now extracts `using_import_boxes` once, seals one
`VerifiedStaticImportAliasViewV1`, and borrows that view into Script lookup and
the App Main relation issuer. Script production no longer seals a competing
view. Its old row-based helper remains `#[cfg(test)]` only. The CoreMethod path
uses the explicit `brand_only` witness. The Main issuer emits only owned
receiver/site/owner/selector/arity relation rows and the package retains the
relation for later work; no target, loan, Recipe, ABI, or physical symbol is
issued here.

Focused evidence:

* `CARGO_BUILD_JOBS=4 CARGO_INCREMENTAL=1 cargo test --profile quick --lib app_main_qualified_receiver_relation` — **3 passed**: direct canonical owner, imported alias, and foreign-view rejection. The direct test also calls package retention and observes one retained row.
* `CARGO_BUILD_JOBS=4 CARGO_INCREMENTAL=0 cargo test --profile quick --lib normal_script_direct_static_lookup` — **8 passed**.
* `CARGO_BUILD_JOBS=4 CARGO_INCREMENTAL=0 cargo test --profile quick --lib normal_default_root_catalog_lifecycle` — **30 passed, 6 failed**. Five failures are the pre-existing baseline rows recorded at `ec82461377`/the map lifecycle census (`actual_string_helpers...`, `parser_scan_package...`, `source_backed_app_main_direct_call...`, `source_backed_package_failure...`, `source_bound_static_result...`). The sixth (`array_source_binding_survives_actual_compiler_finishing_with_optimization`) is an existing array-cleanup red outside this slice and remains unclassified for the next red-audit row; it does not touch the import-view owner.
* `cargo fmt --all` and `git diff --check` pass. The quick build reports the existing large warning baseline (566 warnings after the focused test build); no new warning family was selected here.
* Line counts at receipt: lifecycle 741, package model 372, install 795. The install file is below the 800 hard stop but remains a recorded 760-line debt.

This closes the bounded ownership transport only. Qualified target/loan/source
site consumption, publication, production cutover, compatibility retirement,
and the remaining red classification remain open.
