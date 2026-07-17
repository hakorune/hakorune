# Source Call Exact-Site / Route-Fact Taskboard

Date: 2026-07-17  
Status: **P0 and S0 closed; L0 lexical disposition next**
Baseline: `0988dbed31`  
Supersedes: `source-call-site-route-fact-authority-design-stop-2026-07-17.md`

## Decision

The false-seal path is closed by one durable source pipeline:

```text
canonical caller key + declaration catalog + SourceExprSiteV1
  -> shared structural source projector
  -> VerifiedSourceMethodCallSiteV1
  -> existing ShadowResolver traversal in qualified-receiver observation mode
  -> exact Bound | ProvenUnbound receiver disposition
  -> shared neutral reserved-route policy
  -> VerifiedQualifiedCallRouteFactsV1
  -> qualified/current-owner candidate factories
  -> existing source target catalog
```

Candidate B, the import-alias-only slice, remains parked. It would avoid the
missing direct-owner authority by narrowing an already selected Q0 contract.
The durable row instead finishes the exact authority without creating a
second resolver or a second reserved-route policy.

The next code-facing row is:

```text
R0-SOURCE-CALL-TARGET0-AST-BIND0-L0
```

Production source-target consumers and behavior widening remain zero through
the complete prerequisite series.

## Evidence from the worker audit

Three independent read-only audits agree on the following facts:

- `VerifiedSameModuleCallableDeclarationCatalogV1` already owns one exact
  body for each canonical caller key.
- `SourceExprSiteV1` already is the canonical function-relative structural
  path.
- compiler source projection already owns the closed path-to-AST traversal,
  but its implementation is private to the compiler layer.
- `VerifiedResolvedFunctionV1::variable_ref(site)` proves a positive Bound
  use only. A missing row is not a positive Unbound fact.
- the existing shadow resolver aborts ordinary unresolved variables and does
  not publish qualified-receiver Unbound rows.
- production reserved precedence is currently FastMem, then `__mir__`, then
  `__repl__`, then the ordinary member/static route.
- the green S0b stash needs only imported-alias and current-owner actual
  source rows, but CUT0 must still preserve the full durable Q0 contract.

## Selected ownership boundaries

### Exact source site

```text
owner:
  VerifiedSourceMethodCallSiteV1<'source>

inputs:
  declaration catalog
  canonical caller key
  SourceExprSiteV1

derived only from exact AST:
  receiver expression and receiver site
  method spelling
  checked explicit arity
```

The product is lifetime-bound and non-Clone. It never accepts a separately
supplied AST expression, receiver, method, arity, or route fact.

Structural seal law:

```text
product Clone = 0
owned AST nodes = 0
published caller/site pair = exact constructor caller/site pair
projected expression lifetime = exact catalog declaration lifetime
```

### Lexical disposition

The lexical traversal engine remains the existing `ShadowResolverV0`.
A qualified-receiver observation mode is added to that traversal:

```text
pre-verified receiver site + lookup succeeds:
  Bound(exact lexical reference)

pre-verified receiver site + lookup/ancestor lookup both fail:
  ProvenUnbound

any ordinary unresolved variable:
  existing UnresolvedName error
```

Only receiver sites supplied by `VerifiedSourceMethodCallSiteV1` may receive
the special observation. Missing map rows are never reinterpreted as
Unbound. The result is a dedicated non-Clone source-call lexical product; it
does not widen `VerifiedResolvedFunctionV1` or replace normal resolution.

Coverage is exact:

```text
pre-verified qualified receiver site set
  == published lexical disposition site set

missing rows = 0
extra rows = 0
duplicate rows = 0
product Clone = 0
```

### Reserved route

One neutral pure policy becomes the SSOT for both Builder routing and source
route facts. Tentative vocabulary:

```text
SourceMethodReservedRouteDecisionV1:
  Ordinary
  FastMem
  MirDebug
  ReplIntrinsic
  ReservedFail(reason)
```

Exact parity law:

```text
FastMem:
  inside an exact FastMemBody context + receiver mem

MirDebug:
  receiver __mir__
  method log | mark
  non-empty arguments
  first argument exact String literal

ReservedFail:
  receiver __mir__ + method log | mark + zero arguments

Ordinary __mir__ fallthrough:
  unsupported method
  OR log | mark whose first argument is not an exact String literal

ReplIntrinsic:
  receiver __repl
  method get | set

ReservedFail:
  receiver __repl with any other method
```

The pure policy receives an explicit verified context. Source observation
derives FastMem enclosure from the exact source path; the legacy Builder
adapter derives it from its already-active FastMem session. The policy owns
the decision, not either context carrier.

### Route co-seal

`VerifiedQualifiedCallRouteFactsV1` co-keys exact site, lexical disposition,
reserved decision, and the existing immutable import-alias view.

```text
reserved decision != Ordinary:
  qualified static route rejected before alias/catalog lookup

ImportedAlias:
  verified alias evidence wins before lexical binding
  Bound remains an observation, not a rejection

DirectCanonicalOwner:
  exact ProvenUnbound + Ordinary required
```

Target identity remains owned only by the existing source target catalog.

## Exact task order

### `R0-SOURCE-CALL-TARGET0-AST-BIND0-P0`

Behavior-neutral structural prerequisite.

Status: **closed**.

```text
new semantic authority:
  0

work:
  extract the existing closed SourcePathSegment projector into a neutral
  resolved-semantics substrate
  keep compiler source projection as a thin consumer

production behavior delta:
  0
```

Tests prove compiler projection parity for every admitted path segment,
including MethodCall receiver/arguments, nested body roles, and malformed
segments. No second path vocabulary or AST inventory is added.

Closeout evidence:

```text
neutral structural projector owners = 1
SourcePathSegmentV1 vocabularies = 1
compiler projector consumers = 1
parked segment kinds explicitly rejected = 15
neutral projector tests = 6/6
compiler source-view tests = 7/7
resolved callable-module tests = 6/6
production behavior delta = 0
```

The sole match owner now lives in
`resolved_semantics/source_projection.rs`; compiler source projection only
maps a failed projection to its existing typed navigation error. The shared
view borrows nodes/bodies, creates no AST inventory, and is visible only
inside `crate::mir`.

### `R0-SOURCE-CALL-TARGET0-AST-BIND0-S0`

Disconnected exact-site product.

Status: **closed**.

```text
new authority:
  exact caller declaration/body/site/MethodCall identity

production consumers:
  0
```

Closeout evidence:

```text
VerifiedSourceMethodCallSiteV1 definitions = 1
Clone implementations = 0
owned AST/body rows = 0
catalog caller lookups = 1
neutral catalog-body projector consumers = 1
production consumers = 0
focused exact-site tests = 9/9
```

One catalog caller key selects the only body that may satisfy the site. The
product derives the borrowed MethodCall expression, receiver expression/site,
method, arguments, and checked arity from that body. Two callers with the same
relative site remain bound to their own catalog bodies. Sites crossing a
nested FunctionDeclaration or Lambda owner boundary reject typed instead of
being attributed to the outer callable. Actual ParserStringUtilsBox.skip_ws
and StringHelpers.to_i64 sites are green, while the old handwritten
StringHelpers site rejects. The family guard also counts same-module consumers
so a future disconnected-to-production drift cannot hide inside
`source_call_target`.

Suggested files:

```text
src/mir/source_call_target/source_method_call_site.rs
src/mir/source_call_target/source_method_call_site_tests.rs
```

### `R0-SOURCE-CALL-TARGET0-AST-BIND0-L0`

Disconnected lexical disposition product using the existing shadow traversal.

Status: **sole next code-facing row**.

```text
new authority:
  positive Bound | positive ProvenUnbound at exact qualified receiver sites

lexical traversal engines:
  1

production consumers:
  0
```

The declaration catalog supplies a borrowed neutral function lexical view
(parameters, body, receiver policy) to the existing traversal. It does not
reconstruct an AST FunctionDeclaration or introduce a second Binding identity.

### `R0-SOURCE-CALL-TARGET0-AST-BIND0-R0`

Behavior-neutral reserved-policy extraction plus disconnected route co-seal.

```text
new policy owners:
  one neutral reserved-route classifier

new sealed products:
  VerifiedQualifiedCallRouteFactsV1

Builder behavior delta:
  0

source-target production consumers:
  0
```

Builder special handlers and source-call observation must share the same
classifier before the row closes. There is no temporary duplicated policy.

### `R0-SOURCE-CALL-TARGET0-AST-BIND0-CUT0`

Atomic disconnected authority cutover.

```text
qualified factory input:
  exact site + co-sealed route facts

current-owner factory input:
  exact site only

raw candidate constructors:
  0

raw caller/site/AST/fact combinations:
  0

production consumers:
  0
```

After CUT0, review the saved S0b diff against the new APIs and reapply only
the needed patch. The stash is evidence, not authority; do not wholesale
`stash apply` it.

### Resume

```text
R0-CALLABLE-RESULT-I64-CATALOG0-S0b
```

## Required fixtures

Pass:

- direct canonical qualified call with ProvenUnbound;
- imported alias colliding with a local binding, alias wins;
- current-owner `me.method(...)`;
- MethodCall nested in an argument;
- MethodCall in a loop initializer;
- two same-spelled calls at distinct sites;
- branch and nested-scope Bound observations;
- declaration reorder parity;
- exact `ParserStringUtilsBox.skip_ws -> StringHelpers.skip_ws` source;
- exact `StringHelpers.to_i64 -> me._digit_value` source.

Reserved-policy parity:

- FastMem `mem.*` inside and outside FastMem context;
- accepted and fallthrough `__mir__` shapes;
- zero-argument `__mir__.log/mark` reserved fail-fast;
- `__repl__.get/set` and unsupported-method fail-fast;
- reserved route wins before import alias.

Reject:

- nonexistent or non-expression site;
- non-MethodCall site;
- foreign caller/body/site pairing;
- receiver/method/arity supplied independently;
- missing receiver observation row;
- missing lexical evidence treated as Unbound;
- ordinary unresolved variable tolerated by observation mode;
- duplicate receiver/site disposition;
- mismatched route-fact/site co-seal.

## Guards and counters

```text
structural SourcePath projector owners = 1
SourcePath vocabularies = 1
second AST/body/call inventories = 0
new body clones = 0

VerifiedSourceMethodCallSiteV1 definitions = 1
VerifiedSourceMethodCallSiteV1 Clone implementations = 0
VerifiedSourceMethodCallSiteV1 owned AST nodes = 0
exact caller/site correspondence failures = 0
independent caller/site/AST candidate inputs = 0 after CUT0
raw qualified/current-owner candidate constructors = 0 after CUT0

lexical scope traversal engines = 1
qualified receiver disposition owners = 1
qualified receiver requested/published site-set mismatches = 0
qualified receiver missing/extra/duplicate rows = 0
qualified receiver disposition Clone implementations = 0
missing variable_ref -> Unbound conversions = 0
ordinary unresolved-variable behavior delta = 0
second Binding identity authorities = 0

reserved-route classifier owners = 1
old by-name reserved route-decision owners outside neutral classifier = 0
reserved classifier Builder/source consumers = exact expected count
Builder/source reserved-policy divergence = 0
source observer Builder-state reads = 0

source target catalogs = 1
ABI/effect/result authority delta = 0
source-target production consumers = 0
fallback/retry = 0
source/check files >= 800 lines = 0
```

## Implementation may claim

```text
one canonical caller/body/source-site MethodCall identity
positive Bound and positive ProvenUnbound receiver dispositions
one shared reserved-route decision policy
alias-before-local and reserved-before-alias parity
qualified and current-owner target rows cannot be forged from unrelated AST
all prerequisite rows remain disconnected from production source lowering
```

## Implementation must not claim

```text
general unresolved-name support
general external-name resolution
new lexical resolver semantics
general call routing
bare-call widening
ABI/effect/result representation ownership
Builder/MIR/runtime/backend activation
fallback or retry
```

## Stop conditions

Stop if any row requires:

1. a second lexical traversal engine;
2. a second SourcePath vocabulary or persistent AST/call index;
3. `variable_ref(site) == None` as Unbound evidence;
4. Builder `variable_map`, mutable import map, or FastMem stack as a sealed
   source fact;
5. different reserved policies in Builder and source observation;
6. function-name, MIR-symbol, runtime-tag, or HMI-name inference;
7. a second Binding identity;
8. ABI, effect, result, runtime, backend, or ownership widening;
9. fallback, retry, or wholesale stash restoration;
10. a source/check file reaching 800 lines.

## Estimate

```text
P0 shared projector extraction: 1-2 working days
S0 exact-site product:          1-2 working days
L0 lexical observation:        2-4 working days
R0 reserved SSOT + co-seal:    2-4 working days
CUT0 and guards:               1-2 working days

total prerequisite:
  roughly 7-14 working days
```
