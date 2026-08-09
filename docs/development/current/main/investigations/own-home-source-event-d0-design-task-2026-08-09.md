---
Status: design stop; accepted source direction, issuer not implemented
Date: 2026-08-09
Parent: `docs/development/current/main/investigations/own-home-callable-body-home-flow-d0-design-task-2026-08-09.md`
Authority: `docs/development/current/main/design/ownership-home-model-ssot.md`
Reference: `docs/reference/language/ownership.md`
---

# OWN-HOME-SOURCE-EVENT-D0

## Decision

The source direction is already accepted by the ownership and lifecycle
SSOTs, but the parser/resolver source issuer is not implemented.  This card
closes only the source-event boundary; it does not open Home Flow, Builder,
MIR, or production.

The first explicit Home-end spelling is:

```hako
release root
```

It is a statement-only contextual form with one identifier root.  `release`
is not globally reserved and is not a generic ownership function.

```text
release root       = candidate Home-end source event
release(value)     = ordinary call
obj.release()      = ordinary method call
local release = 1  = ordinary binding
Build.release()    = ordinary method call
drop root          = rejected alias
unbox root         = representation extraction, never Home-end authority
unhome root        = not a language spelling
```

The source token does not prove ownership.  A later resolver Home-demand/root
issuer and Home Flow witness must prove that the identifier denotes one exact
available whole-root Home.

## Focused external design question

One narrow language-D0 review is still useful before opening the parser I0.
The review must choose syntax and precedence only; it must not redesign Home
ABI, Home Flow, or the existing `release root` decision.

Ask the reviewer to return one canonical answer for each item, including a
small EBNF/FIRST-set explanation and negative examples:

```text
1. Declaration-side demand:
   adopt(take node: Node)
   Is `take` the sole declaration-side ownership-demand spelling for this
   cohort, and is it declaration-only?

2. Explicit sharing:
   share expr
   versus
   adopt(share expr)
   Choose one canonical source form, define its precedence with calls,
   returns, fields, and parentheses, and keep `share(...)` an ordinary call.

3. Early release:
   release root
   Confirm statement-only contextual disambiguation: `release` is not a
   global reserved word, `release(value)` and `obj.release()` remain ordinary
   calls, and only one identifier whole-root is in the first parser cohort.
```

The review must also state the rejected alternatives and confirm that parser
acceptance remains out of scope until the decision is recorded here and in the
language reference.  The parser source-event issuer is already design-sealed
in this card; only its I0 implementation waits behind the syntax decision.
Until this question is answered, the current source-event card remains a
design stop and no parser carrier or Home issuer is implemented.

## Source carrier boundary

The future parser row may issue one typed, AST-free source carrier after the
ordinary parser transaction has completed:

```text
ReleaseStatementSourceV1
  parser provenance
  exact statement site
  lexical root declaration/source site
  contextual-keyword classification
```

This carrier owns spelling, source location, and lexical binding reference
lookup input only.  It does not own `HomeRoot`, availability, terminality,
fini, cleanup invalidation, or physical release.

### Issuer decision

The sole parser issuer is the existing rich body transaction boundary:

```text
ParserResolverBodyTransactionV1::with_direct_method_syntax(self, callback)
  -> parser-private borrowed syntax lease
  -> ReleaseStatementSourceV1
  -> AST-free resolver-facing carrier
```

The observer runs while the parser owns the rich body syntax, consumes the
borrowed lease, and returns only an owned carrier.  It does not add a second
parser transaction, rescan an AST after the callback, or reconstruct a row
from a method name, inventory ordinal, JSON, or post-hoc text scan.  The
carrier inherits parser provenance from the same handoff and identifies the
statement by the exact method source site plus body statement ordinal; the
root token remains lexical input, not a resolved Home root.

The issuer is design-sealed here but not implemented.  Until
`OWN-HOME-SOURCE-EVENT-I0` lands, a missing observer remains `NoSafeSlice`,
not a default empty release event.

## Event mapping

This D0 defines only the source spelling that can later request one event:

```text
ReleaseStatementSourceV1
  -> future HomeFlow event End(root, site)
```

The mapping is not executable until the following independent receipts exist:

```text
exact source root declaration
  + declared Home demand / owning-root capability
  + Home Flow Available(root) at the source point
  + no dependent handle/use after the end
  + cleanup-capture exclusion
  + terminal/non-terminal C' disposition
```

`Create`, `Consume`, `Share`, `Forward`, and `Escape` are not issued by this
row.  `share` remains the only source operation that may add an independent
Home, but its grammar and Shared representation are separate D0 rows.

## Exact accepted cohort for a future parser I0

```text
one direct statement: release IDENT
one lexical identifier root
no field/index/container/projection
no generic argument
no call-site take
no consuming receiver
no branch/loop/closure/capture/suspension interaction
```

Parser acceptance of this cohort must still reject malformed contextual use
and must not claim that the source compiles or consumes a Home.  The first
semantic Home Flow fixture remains a later owning-local/parameter linear row.

## Fail-fast matrix

```text
parser syntax error:
  missing identifier, extra arguments, invalid statement placement,
  `release(...)`, or a globally reserved-word interpretation

NoSafeSlice:
  source-event observer implementation absent, Home-demand/root issuer absent,
  or the selected source path cannot preserve parser provenance and exact site

Rejected:
  field/index/container root, generic root, foreign source brand, duplicate
  carrier, forged Home root, alias-only root, or a forbidden alias spelling

Candidate:
  source carrier only after the parser issuer is landed; semantic Candidate
  remains unavailable until Home Flow provides the complete witness
```

`NoSafeSlice` is a development state, not a source disposition.  The parser
must not return an empty/default `Verified*` product for an unavailable Home
issuer.

## Ownership table

| Layer | Owns | Must not own |
| --- | --- | --- |
| Lexer/parser | contextual token sequence and syntax diagnostics | Home availability or terminality |
| Parser source seal | exact statement/root source provenance | HomeRoot, Home state, fini |
| Resolver declaration/Home issuer | exact lexical root and declared Home demand | source spelling reclassification, MIR |
| Home Flow | `BindingRef -> HomeRoot/state`, End event, path/CFG witness | `BindingRef -> ValueId`, parser AST, physical layout |
| C' DropPlan | terminal hook/field/native teardown order | source parsing or root re-resolution |

Home Flow state is separate from Binding SSA:

```text
Binding SSA: BindingRef -> current ValueId
Home Flow:   BindingRef -> HomeRoot/state
```

`HomeRelationBrandV1` remains relation-batch provenance and must not be reused
as the Home root identity.

## Ordered follow-up tasks

```text
1. OWN-HOME-SOURCE-EVENT-D0
   this design stop; no code yet

2. OWN-HOME-SYNTAX-D0
   close the focused `take` / `share` grammar and precedence question above;
   update the language reference only after one canonical choice is accepted

3. OWN-HOME-SOURCE-EVENT-I0
   parser-private observer inside
   `ParserResolverBodyTransactionV1::with_direct_method_syntax`; issue one
   AST-free carrier for `release IDENT`, add grammar/contextual negatives,
   and update reference/EBNF in the same slice

4. OWN-HOME-CAPABILITY-TAXONOMY-D0
   freeze Create/Consume/Share/Forward/Escape/End capability meanings and
   dispositions before issuing Home-demand products

5. OWN-HOME-ABI-HOME-DEMAND-I0
   source-backed owning parameter/local root capability; no body inference

6. CALLABLE-BODY-HOME-FLOW-LINEAR-I0
   private body_home_flow issuer for Available -> End/Consumed -> Unit return

7. CALLABLE-BODY-HOME-FLOW-CFG-D0
   branch/loop/backedge/Maybe-join and transfer authority

8. CALLABLE-BODY-EXECUTION-COSEAL
   only after all four axis issuers are complete
```

The current Home Flow D0 remains the parent boundary.  This source-event D0
may become the next current design card only after the pointer update in the
same docs closeout commit.  If the grammar requires a new ownership concept,
stop and open a separate language D0; do not widen this card.

## Explicit non-claims

This row does not implement or authorize:

```text
parser acceptance in production
Home-demand classification
HomeRoot/state issuance
take/share/forward/escape events
field/index/container/composite release
branch/loop/capture/await/qmark/throw flow
fini dispatch or DropPlan execution
Ownership SSA, Binding SSA changes, FunctionOwner, target, Recipe/CallSlot
Builder, MIR, runtime, backend, fallback, retry, or production selection
```
