# Resolver syntax-facts authority D0

Status: `Decision stop; MAP-S1 is NoSafeSlice until this authority is sealed`

Parent: `GENERIC-CALLABLE-SINGLE-LOOP-SOURCE-RECIPE-MAP-D0`

## Change

Choose one sanctioned owner for the syntax facts that the resolver ledger
does not currently publish: operator, RHS literal, initializer value,
prefix-call-to-binding relation, and terminal tail shape. The recommended
boundary is a dedicated source-syntax observer at the Observe stage:

```text
FunctionSyntaxView + resolver owner/typed sites
  -> VerifiedSourceSyntaxFactsV1
  -> AST-free MAP-S1 join
```

The observer may inspect source syntax only at its boundary. It must publish
an immutable, owner-branded product; AST, names, and paths must not cross into
Recipe, Verify, Lower, or physical code.

## Contract

- Resolver remains the sole owner of BindingRef, owner/origin/source-kind,
  Loop frame, Scope/Region, and control-transfer identity.
- The syntax observer owns only as-written shape observations: operator kind,
  literal shape, initializer shape, call boundary shape, and return expression
  shape. It does not claim type/range/overflow/monotonicity policy; those stay
  with the numeric substrate/route policy. It does not resolve call targets or
  BindingRefs; it joins a CallShape to resolver direct-call receipts by typed
  site. It does not allocate ValueId/PHI/CFG, produce Recipe, or select a route.
- The product must distinguish mapped, unsupported, opaque, missing, duplicate,
  and foreign rows. No name lookup, path-suffix inference, AST rewrite, or
  second resolver is allowed.
- The nine syntax rows (initial carrier; condition Lhs/Rhs/operator; step
  Lhs/Rhs/operator/assignment target; terminal tail) plus one separate prefix
  boundary must be represented exactly. Whole-callable declaration/reference/
  assignment/exit coverage is sealed by the MAP-S1 join and outer plan, not by
  this observer alone. Negative cases include nested/second Loop and
  non-terminal tails.
- The product is joined by owner-branded typed sites. A partial syntax view is
  not sufficient to reopen MAP-S1.
- The resolver must issue one co-sealed Loop source/frame/Scope/Region token (or
  an equivalent one-shot lookup) for MAP-S1. Raw paths and `exact_scope`
  reconstruction are forbidden.

## Done

- Authority and lifecycle are fixed: one observer, one sealed product, one
  MAP-S1 join; no resolver AST schema expansion is introduced implicitly.
- A machine-readable fixture proves the nine syntax rows plus the separate
  prefix boundary and their exact dispositions for `StringHelpers.int_to_str/1`.
- Negative fixtures cover wrong operator/type/literal, missing/extra rows,
  duplicate/foreign site, prefix mismatch, non-terminal/opaque tail, and
  whole-callable coverage failure.
- The reference entry, current pointer, MAP-S1 task, and workstream are
  synchronized. Implementation is still caller-zero and docs must be updated
  in the same commit as the later code.

## Stop

Return to design if the observer must become a second semantic resolver, if
AST or names cross the observation boundary, if resolver identity is rebuilt
from paths, or if the product requires Recipe/ValueId/CFG/PHI policy. Until
this row is closed, MAP-S1, RECIPE-S2, PHYS-S3, production cutover, and legacy
retirement remain closed. After this D0, do not add row-specific D0 suffixes:
the next implementation row is `SyntaxFacts-S1`, then `MAP-S1` directly.
