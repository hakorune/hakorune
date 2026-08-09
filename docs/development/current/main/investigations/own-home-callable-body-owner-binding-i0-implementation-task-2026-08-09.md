---
Status: closed bounded implementation
Date: 2026-08-09
Parent: `docs/development/current/main/investigations/own-home-callable-body-owner-binding-d0-design-task-2026-08-09.md`
Authority: `docs/reference/language/callable-contracts.md`
---

# CALLABLE-BODY-OWNER-BINDING-I0

## Goal

Issue the one non-`Clone` owner-link product over the two already sealed
catalogs. This row is a relational co-seal only; it issues no owner and does
not infer body behavior.

```text
VerifiedDeclaredQueryBodySourceCatalogV1<'body,'contract>
+ VerifiedInstanceMethodFunctionCarrierCatalogV1
    -> VerifiedInstanceMethodBodyOwnerCatalogV1<'body,'contract>
```

The carrier row already owns the exact root `VerifiedResolvedFunctionV1` and
its `FunctionOwnerIdV1`. The owner link borrows that root; it must not accept
a separate function array, copy/reissue an owner, or call the resolver again.

## Allowed scope

* one ordinary direct Rust instance-method cohort;
* one selected Query body projection;
* one resolver-issued carrier/catalog;
* one catalog-level owner-link issuer and its focused tests.

The output row borrows the selected body row, carrier row, and carrier root
function/owner. It remains AST-free and has no public arbitrary constructor.

## Exact matching contract

The issuer checks once before issuing the catalog:

```text
body.parser_provenance == carrier.parser_provenance
body.resolver_brand == carrier.resolver_brand
selected body identity == exactly one carrier identity:
  nominal Box type
  Box source statement site
  direct method source site
body item coverage == carrier ordered coverage
```

`name` is a diagnostic consistency check only. `FunctionOriginV1`, owner
numbers, inventory ordinals, names, arity, vector position, and compilation
brands are not pairing authority. Non-Query carrier rows from the complete
direct cohort may remain valid and unselected.

Empty coverage (`[]`) is valid. Missing/duplicate selected rows, foreign
brands/provenance, source-site mismatch, nominal-type mismatch, and coverage
gap/duplicate/reorder/count mismatch are rejected. Static/generated/
selected-gate rows are rejected upstream by their source/carrier cohort, not
manufactured as owner-link test products.

## Acceptance tests

Positive:

```text
one selected Query body ↔ one carrier root
empty body coverage
non-Query carrier extra remains unselected
```

Negative, using real sealed products only:

```text
foreign parser provenance
foreign resolver brand
nominal/source-site mismatch
body coverage mismatch
selected body duplicate or missing row
```

Impossible states rejected upstream by parser/body/carrier issuers must be
recorded as upstream guards; no forged `Verified*` test constructor is added.

## Non-claims

```text
body AST or behavior Facts
Query/Pure/Home/signature conformance
resolver target or source-bound call relation
Recipe / CallSlot
Builder / MIR / CFG / PHI / physical ABI
runtime/provider dispatch
fallback/retry
```

After this row closes, stop at `CALLABLE-BODY-FACTS-QUERY-D0` and update the
module README, language reference, task map, and current mirrors in the same
implementation slice.

## Verification

```text
focused owner-binding tests
cargo check --lib
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Keep every touched source file below the 760-line split trigger and below
800 lines.

## Closeout receipt (2026-08-09)

Implemented `InstanceMethodBodyOwnerBindingIssuerV1` as the sole catalog-level
relational co-seal over the selected Query body projection and the existing
resolver-issued function carrier/catalog. The output is non-`Clone`, borrows
the selected body row, carrier row, and exact carrier-root
`VerifiedResolvedFunctionV1`, and issues no `FunctionOwnerIdV1` or second
function product.

The focused matrix is green for sparse Query selection, empty body coverage,
unselected non-Query carrier extras, and foreign parser provenance. Exact
parser provenance, resolver brand, nominal/source site, diagnostic name, and
ordered body coverage checks are enforced; body facts, conformance, target,
Recipe/CallSlot, Builder/MIR, and runtime remain closed.

Verification receipt:

```text
cargo test --lib instance_method_body_owner -- --nocapture  # 3 passed
cargo check --lib
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The next design stop is `CALLABLE-BODY-FACTS-QUERY-D0`; do not implement body
facts in this row.
