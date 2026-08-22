# Brand Constructor Relation Projection P0

Status: landed
Parent: `brand-constructor-active-admission-census-d1.md`
Row: `BRAND-CONSTRUCTOR-RELATION-PROJECTION-P0`
Classification: BoxShape

## Execution brief

Decision: Project the existing verified Brand relation into callable and
Complete Script lowering states as a total exact-site disposition; add no raw
consumer and change no route.
Source authority + canonical issuer: Each verified resolved owner remains the
sole issuer. Its exact expression-site inventory plus
`VerifiedBrandCallSourceRelationV1` derives `Constructor(row) | NonBrand` for
that same owner and site.
Non-authority: `Option::None`, call spelling, mutable Brand maps, AST, lineage,
statement index, ValueId, and raw preflight cannot issue `NonBrand` or repair a
missing site.
Fail-fast boundary: State construction rejects foreign owners, relation sites
outside the expression inventory, duplicate projections, and missing site
coverage; queries outside coverage return an error, never `NonBrand`.
Smallest next slice: Add one bounded private projection model, expose read-only
expression-site iteration from the verified owner, install it in callable and
Script lowering states, and test exact Constructor/NonBrand/error outcomes.
Non-claims: No raw source-demand port, newly located call, preflight switch,
name-probe retirement, relation-less admission, unwrap activation, nominal
Brand value/type, runtime, or backend.

## Acceptance

- The callable and Script projections consume rows from their exact verified
  owner without AST or name lookup.
- Same-name Brand and non-Brand sites remain distinct by site.
- Exact non-Brand FunctionCall sites return `NonBrand`; absent/foreign sites
  reject rather than default.
- Constructor rows preserve declaration ID, name, underlying type, call site,
  and operand site without `ValueId`.
- Existing lowering behavior and raw `is_brand_declared` callers remain
  unchanged in this P0.
- Focused tests, reusable guard, compile check, and the 760/800 boundaries are
  green.

## Landed evidence

- One 238-line request-local projection copies only exact expression-site
  coverage and verified Constructor rows from the same resolved owner.
- Exact covered sites return Constructor or NonBrand; absent, foreign, and
  relation-outside-inventory rows reject.
- Callable and Complete Script lowering states build and retain the projection.
  No raw port or preflight reads it yet, and `is_brand_declared` is unchanged.
- Two focused positive/negative tests, `cargo check --profile quick --lib`, and
  `brand_constructor_relation_projection_guard.sh` are green.
- Touched production owners remain below 760 lines.
