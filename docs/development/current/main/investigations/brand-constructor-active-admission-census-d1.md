# Brand Constructor Active Admission Census D1

Status: closed
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

## Census result

| Admission | Exact Brand relation at lowering | Active exact site | Disposition |
| --- | --- | --- | --- |
| Complete Script semantic source | yes | yes for already-located expressions | relation-backed |
| installed cataloged/top-level callable package | yes | yes for already-located expressions | relation-backed |
| selected Dynamic callable through the installed package | yes | yes | relation-backed |
| Deferred Script | no verified semantic product | no semantic ledger | blocking |
| callable-package Compatibility | no callable semantic ledger | lineage only | blocking |
| instance-constructor body | no Brand projection | lineage only | blocking |
| raw Main/legacy top-level/cataloged body | no verified Brand projection | lineage or compatibility site only | blocking |
| nested Box method | no verified Brand projection | nested lineage only | blocking |
| `RawLegacyChildLoweringPortV1` | none | none | blocking |

Additionally, a bare `FunctionCall` statement remains an unlocated
`CallObject` row even when its owner has a verified relation. Relation-backed
does not yet mean consumer-ready for that site.

Therefore the raw probe cannot retire. The next behavior-neutral row projects a
total exact-site `Constructor | NonBrand` disposition into the two already
verified lowering states. Relation-less admissions remain explicit blockers
and require separately accepted semantic coverage before cutover.
