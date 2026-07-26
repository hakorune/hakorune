---
Status: design stop
Date: 2026-07-26
Decision: NESTED-INSTANCE-RESULT-CONTRACT0-D0
Classification: BoxCount / T2 source result-contract authority
Blocked row: CALLABLE-RESULT-NESTED-REP0-S0
Related:
  - docs/development/current/main/investigations/stageb-generic-loop-transient-type-d0-design-question-2026-07-26.md
  - src/mir/callable_result_representation/README.md
  - src/mir/callable_result_representation/tests/actual_parser_add_fixture.rs
---

# Nested instance-result contract authority

## Observed S0 preflight

The current Stage-B failure needs the result of this exact source call:

```hako
me.static_const_eval_pos(ret)
```

The source site is already exact and usable:

```text
caller       = ParserBox.static_const_parse_add/2
site         = Body(3).Value.Argument(1)
receiver     = CurrentOwner
target       = ParserBox.static_const_eval_pos/1
site owner   = VerifiedSourceMethodCallSiteV1
```

The loop-refresh occurrence is separately located at
`Body(4).LoopBody(5).Value.Argument(1)`.

However, no current owner proves that the target returns `Integer`:

```text
VerifiedSameModuleCallableResultCatalogV1
  = static declarations only

current-owner source target route
  = static caller only

static_const_eval_pos/1
  = instance method
  = unannotated
```

The actual fixture's existing result gate therefore correctly reports every
ParserBox row, including the nested instance result, as `Unselected`.

## Non-authority

The following must not create an `Integer` contract:

```text
callee spelling or ParserBox identity
outer skip_ws target
GenericLoop condition, step, or carrier role
return annotation (none exists)
final MIR metadata or runtime observation
successful execution on another route
retry or fallback
```

## Decision required

### A — bounded instance-result proof (recommended)

Open `NESTED-INSTANCE-RESULT-CONTRACT0` as a new source-only owner.

```text
exact current-owner MethodCall site
  + exact instance target declaration
  + independently sealed dependency result contracts
  + bounded body-result proof
  -> ExactInteger instance-result contract
```

The proof must be compositional: it may consume existing source-target rows
and Core String result rows, but it must not key policy on `ParserBox` or a
method name.  It needs an explicit finite accepted body grammar, source-site
coverage, typed unavailable reasons, and a non-Clone failure owner.  It has no
`ValueId`, MIR, `type_ctx`, emission, or runtime authority.

### B — widen the existing static result catalog (rejected unless separately accepted)

Changing `VerifiedSameModuleCallableResultCatalogV1` from static-only to all
instance declarations rewrites an established catalog boundary and broadens
many dormant callers.  It is not an implementation detail of the current
Stage-B result.

### C — retain typed Unselected (valid park)

Leave `CALLABLE-RESULT-NESTED-REP0` without an admission.  The Stage-B probe
continues to reject `MissingTransientType` before loop effects, and the Hako
ownership transport remains parked.  This preserves current semantics but
does not advance the selected Stage-B source shape.

## Required decision product

```text
instance result authority
  = bounded new owner, existing widened owner, or no admission

source dependency authority
  = exact catalog/route contracts only

publish boundary
  = still later successful physical Call emission only

forbidden
  = name/annotation/metadata/runtime inference
  = persistent source-site -> ValueId mapping
  = GenericLoop type production
```

## Non-claims

```text
GenericLoop repair
MIR/type_ctx publication
instance-call generalization
static catalog widening
parser/Hako source edit
VM/backend/PHI/finalization change
fallback or retry
OWN-GRAM Hako transport resumption
```
