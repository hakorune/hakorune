# Source Call Site / Route-Fact Authority Design Stop

Date: 2026-07-17  
Status: **design consultation required; implementation authority = 0**  
Baseline: `a63da46431`  
Parent: `source-call-target-and-core-result-authority-design-stop-2026-07-17.md`

## Stop reason

`R0-CALLABLE-RESULT-I64-CATALOG0-S0b` reached a green disconnected
implementation, but final review found a false-seal path in the source target
substrate. `QualifiedStaticCallCandidateV1::new` can currently combine an
arbitrary caller key, source site, receiver/method/arity spelling, and route
facts. The target sealer verifies catalog membership and uniqueness, but does
not prove that the exact site inside that exact caller body is the MethodCall
described by the candidate.

Replacing the raw constructor with `from_method_call(caller, site, expr)` is
necessary but insufficient: the caller/site can still be paired with an AST
expression borrowed from another declaration or site. Raw Bound/Unbound and
reserved-route dispositions would also remain unsealed inputs.

Therefore S0b is parked before commit. Its green work is saved only as:

```text
wip/callable-result-s0b (blocked by source-call AST/site co-seal)
```

Do not apply that stash until the prerequisite below is green.

## Required prerequisite

Working name:

```text
R0-SOURCE-CALL-TARGET0-AST-BIND0
```

The prerequisite must provide one lifetime-bound, non-Clone exact-site
product, tentatively:

```rust
VerifiedSourceMethodCallSiteV1<'source>
```

It must:

- co-seal the supplied canonical caller key with its exact catalog
  declaration and body;
- locate the exact expression identified by `SourceExprSiteV1` inside that
  caller's exact body;
- require that expression to be `ASTNode::MethodCall`;
- derive receiver expression, method spelling, and checked arity from that
  exact AST node;
- be the common source for qualified-static and current-owner candidates;
- expose no second AST body inventory and no mutable source authority.

## Remaining design question

Exact-site lexical and reserved-route facts still need a durable owner.
`VerifiedResolvedFunctionV1::variable_ref(site)` can prove a bound variable
use, but absence is not by itself a positive unbound fact and it does not own
all reserved-route dispositions.

### Candidate A-prime — recommended

Add the exact AST-site product plus a second co-keyed sealed product:

```rust
VerifiedQualifiedCallRouteFactsV1<'source>
```

Both products are keyed by the same exact caller and source site. Route facts
are produced only from existing or newly authorized neutral source observers;
the target catalog remains the sole target owner. Candidate factories consume
the products and raw constructors are retired.

The existing Q0 precedence law must remain explicit:

```text
ImportedAlias:
  verified import-alias evidence wins before lexical binding;
  Bound remains an observation and is not a rejection by itself

DirectCanonicalOwner:
  exact positive Unbound plus Ordinary reserved-route disposition required
```

Missing lexical evidence is never converted into positive Unbound evidence.

This keeps the boundaries separate:

```text
exact AST/site identity       -> VerifiedSourceMethodCallSiteV1
lexical/reserved disposition -> VerifiedQualifiedCallRouteFactsV1
target identity              -> existing source target catalog
ABI/effect/result            -> unchanged separate authorities
```

### Candidate B — narrow imported-alias slice

Admit only import-alias qualified calls first, where positive alias evidence
already exists. This is smaller but would narrow the claims of Q0 and leave
direct qualified calls parked. It is not preferred unless A-prime cannot be
sealed without a new lexical authority.

### Candidate C — catalog-owned call/lexical inventory

Make the declaration catalog own a second body/call/lexical inventory.
Rejected: this mixes header/declaration identity with expression resolution.

### Candidate D — trust raw candidate facts

Keep the existing raw constructors and rely on tests/callers.
Rejected: it preserves the false-seal path.

## Proposed task order after selection

```text
R0-SOURCE-CALL-TARGET0-AST-BIND0-S0
  exact caller-body/site/MethodCall view, production consumers 0

R0-SOURCE-CALL-TARGET0-AST-BIND0-R0
  exact-site lexical and reserved-route fact seal

R0-SOURCE-CALL-TARGET0-AST-BIND0-CUT0
  qualified/current-owner factories consume only sealed views;
  raw constructors and raw fact inputs become zero

resume:
  R0-CALLABLE-RESULT-I64-CATALOG0-S0b
```

All prerequisite rows are behavior-neutral and keep production consumers at
zero.

## Required fixtures

Pass:

- direct qualified call;
- imported-alias call;
- current-owner `me.method(...)` call;
- nested argument and loop-initializer sites;
- two same-spelled calls at different sites;
- declaration reorder parity;
- actual `StringHelpers` and `ParserStringUtilsBox` source fixtures.

Reject:

- nonexistent site;
- site containing a non-call expression;
- caller/body/site foreign pairing;
- AST receiver/method/arity mismatch in a malformed private draft test;
- missing positive route fact;
- duplicate caller/site publication;
- a missing lexical row treated as proven unbound.

## Counters and guards

```text
exact source MethodCall site product definitions = 1
raw candidate constructors after CUT0 = 0
qualified/current-owner candidate factories = 2
candidate caller/site/AST independent inputs = 0

second AST body/call inventory = 0
Builder/current_module/current_static_box reads = 0
function-name or MIR-symbol parsing = 0
runtime-tag reads = 0
ABI/effect/result authority delta = 0
production consumers = 0
fallback/retry = 0
source/check files >= 800 lines = 0
```

## Stop conditions

Stop if implementation requires:

1. a second AST body or call-site inventory;
2. replaying result analysis or Lower to identify the source call;
3. Builder mutable maps or lowering order as source authority;
4. treating missing lexical evidence as a positive Unbound proof;
5. physical symbol, function-name, runtime tag, or HMI-name inference;
6. ABI, effect, result representation, runtime, or backend widening;
7. fallback or route retry.

## Decision request

Select the exact owner of lexical Bound/Unbound and reserved-route facts for
one already co-sealed caller/body/source-site MethodCall. Candidate A-prime is
recommended. No code-facing prerequisite implementation is authorized until
that owner and its fail-fast boundary are selected.
