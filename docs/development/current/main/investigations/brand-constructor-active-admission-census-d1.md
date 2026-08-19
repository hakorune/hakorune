# Brand Constructor Active Admission Census D1

Status: selected
Parent: `brand-constructor-consumer-cutover-d0.md`
Row: `BRAND-CONSTRUCTOR-ACTIVE-ADMISSION-CENSUS-D1`
Classification: Design stop

## Execution brief

Decision: Census every production raw `FunctionCall` admission and classify
whether an exact verified Brand owner/site product reaches it; do not add a
partial consumer or preserve a name fallback.
Source authority + canonical issuer: The landed
`VerifiedBrandCallSourceRelationV1` and each admission's verified semantic
owner/site are the only evidence for relation-backed constructor consumption.
Non-authority: lineage names, AST spelling/span, deferred statement ordinal,
`CompilationContext::brand_decls`, RawLegacy behavior, tests, and caller absence
from one fixture cannot prove exact admission.
Fail-fast boundary: Any live Brand constructor that reaches raw lowering
without exact owner/site coverage blocks atomic cutover and requires a separate
semantic admission decision before effect.
Smallest next slice: Enumerate Complete Script, Deferred Script, cataloged and
top-level callable, Compatibility callable, instance constructor, nested/Main,
and RawLegacy edges; prove caller-zero or name the missing issuance for each.
Non-claims: No code, fixture, Brand projection, newly located call, consumer
cutover, unwrap activation, compatibility retirement, fallback, or runtime.

## Exit

The row exits only with one exhaustive table and either:

- all live natural Brand constructors relation-backed, selecting the total
  disposition projection P0; or
- one exact missing admission selected as a separate BoxCount, while the raw
  probe remains unchanged.
