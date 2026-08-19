# Brand Constructor Consumer Cutover D0

Status: closed — NoSafeSlice
Parent: `brand-constructor-source-relation-i0.md`
Row: `BRAND-CONSTRUCTOR-CONSUMER-CUTOVER-D0`
Classification: Design stop

## Execution brief

Decision: Audit one exact semantic-product-to-active-source-site handoff before
replacing the raw Brand name probe; do not infer a relation from the call name.
Source authority + canonical issuer: The landed owner/site-keyed
`VerifiedBrandCallSourceRelationV1` is the sole constructor membership issuer;
the selected raw child port may only borrow the row for its active source site.
Non-authority: `CompilationContext::brand_decls`, `is_brand_declared`, call
spelling, deferred Script statement index, resolver miss, AST, and ValueId
cannot issue, repair, or re-pair membership.
Fail-fast boundary: Before argument descent, require one exact owner/site row,
kind Constructor, arity one, and exact operand-site parity; a missing or foreign
row cannot fall back to the legacy name probe.
Smallest next slice: Name the bounded projection/query owner for callable and
Script lowering, then select one BoxShape R0 that switches the existing
constructor passthrough and deletes its raw name authority atomically.
Non-claims: No unwrap physical activation, nominal Brand value/type checking,
runtime representation, Stage1 unification, collision-policy change, fallback,
or other special-call retirement.

## Required census

- Enumerate every production admission that currently reaches raw
  `FunctionCall` lowering for callable and Script owners.
- Prove each selected admission retains an active owner plus
  `SourceExprSiteV1` that can query the landed verified product without growing
  the 760-line raw transport owner.
- Preserve the current arity-before-child and one-child-on-success ordering.
- Stop as `NoSafeSlice` if any live Brand constructor has only a name, statement
  ordinal, AST span, or mutable-map membership at the physical boundary.

## Audit result

Immediate atomic cutover is `NoSafeSlice`.

- Exact source-backed callable owners and Complete Script owners retain a
  verified semantic product and an active source site, but their lowering
  projections do not yet carry Brand disposition rows.
- Compatibility callable, Deferred Script, instance-constructor, nested/Main,
  and raw legacy admissions can still reach `FunctionCall` with only lineage
  or spelling. Deleting the global probe would silently route those calls to
  TypeOp, Math, `str`, FastMem, or ordinary call handling.
- A bare `FunctionCall` statement is deliberately converted to the unlocated
  `CallObject` portal. Therefore even a verified owner cannot consume the exact
  relation for every natural constructor site today.
- Relation absence must not mean `NonBrand`. The later consumer needs a total
  `Constructor(row) | VerifiedNonBrand` projection whose owner and site
  coverage were verified before child descent.
- `raw_invocation_source_transport.rs` is exactly 760 lines and
  `recursive_child_lowering.rs` is 794 lines. Neither may receive Brand logic.

## Ordered tasks

1. `RAW-INVOCATION-SOURCE-TRANSPORT-CLASSIFIER-SPLIT-P0`: move only the
   statement-location classifier into a private child, with behavior unchanged.
2. `BRAND-CONSTRUCTOR-ACTIVE-ADMISSION-CENSUS-D1`: close caller census for all
   relation-less production admissions.
3. `BRAND-CONSTRUCTOR-RELATION-PROJECTION-P0`: add the total verified
   disposition projection and a dedicated source-demand port outside the
   760/794-line owners.
4. If a live relation-less Brand caller remains, add one separately accepted
   exact semantic admission; never preserve a name fallback.
5. Only then run `BRAND-CONSTRUCTOR-CONSUMER-CUTOVER-R0`, switching the existing
   scalar passthrough and deleting `is_brand_declared` atomically.

Brand unwrap physical activation remains a separate BoxCount after constructor
cutover.
