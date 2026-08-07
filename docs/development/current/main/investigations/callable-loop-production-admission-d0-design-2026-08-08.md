# Callable Loop Production Admission D0

Status: design stop after `CALLABLE-LOOP-PRODUCTION-EDGE-D0` closed as
`NoSafeSlice`.

Decision: expose one production callable admission/physicalization ingress
before attempting an I0 caller switch. This row is docs-only. It must not add
a selector, runtime fallback, retry, Generic G0 substitution, or legacy
route.

## Why this row exists

The caller-zero callable physical products currently live behind `cfg(test)`.
The closest live loop edge is the raw child-entry path:

```text
RawInvocationChildPortV1::lower_loop
  -> PreparedLocatedRawLoopChildEntryV1::lower_with_existing_route_v1
  -> lower_loop_or_freeze_v1
  -> MirBuilder::try_cf_loop_joinir / route_loop
```

That path has a pre-effect callable binding schedule but is not a callable
function physicalization owner. It returns a loop `ValueId` and lets the raw
function/module transaction own failure. It cannot be switched directly to
the new callable chain.

The named production host is
`NormalCallableSemanticLoanPortV1::lower_normal_top_level_function`
(`src/mir/builder/normal_callable_semantic_loan_port.rs:246-278`). Its current
outer contract is:

```text
success:
  LegacyFunctionPendingSessionV1
  -> commit_legacy_symbol_pending
     (DraftPublicationPolicyV1::LegacyReplaceWholePair)
  -> root collector/module drain

failure:
  ModuleLoweringPortChildErrorV1::Session
  -> pending Drop / abort_and_restore
```

This host is evidence for the future ingress boundary, not an I0 candidate.
The future adapter must replace the loop child edge while preserving the
outer function-session/discard and module publication owners until a separate
cutover decision.

## Sole authority

```text
resolved callable source
  -> PreparedCallableLoopPhysicalizationV1
  -> one fresh CanonicalFunctionLoweringSessionV1
  -> Prelude / Loop / After / Tail / Completion
  -> CanonicalSsaFunctionSessionV2::finish_for_draft_seal
  -> DraftSeal prepare/commit
  -> one CompletedFunctionDraftV1
```

The common Loop physicalizer remains unaware of callable Tail, ABI,
Completion, Return, DraftSeal, raw route names, and module publication.

## Required design products

The next design must define, without implementation:

1. the source-side production ingress that can issue or receive one exact
   `PreparedCallableLoopPhysicalizationV1` outside `cfg(test)`;
2. the outer function-session owner and its `discard_unpublished` terminal;
3. the input/output receipts crossing the ingress (owner, Prelude, Loop
   demand, After, Tail, ABI, Completion, profile-close, DraftSeal);
4. the one raw/old edge that will later be replaced, without changing the
   existing module collector or publication owner;
5. positive, owner/brand-mismatch, late-discard, and fresh-session parity
   fixtures required before I0.

The ingress may be a thin profile adapter, but it must not become a second
semantic owner or re-walk AST/source names. The adapter must accept only
sealed resolver/facts/Recipe products and fail fast on foreign or incomplete
receipts.

## Non-claims

```text
production callable physicalization = 0
I0 caller switch = 0
selector/fallback/retry = 0
Generic G0 parity = 0
collector/module publication from the canary = 0
legacy deletion = 0
```

## Acceptance for this design row

```text
one production ingress boundary is named
its session/discard owner is named
input/output receipts are listed exactly once
old edge and same-slice retirement condition are named
NoSafeSlice is explicit where a contract is absent
reference/current docs are updated in the same commit as the design task
```

No Rust or `.hako` implementation is authorized until this row is accepted.
