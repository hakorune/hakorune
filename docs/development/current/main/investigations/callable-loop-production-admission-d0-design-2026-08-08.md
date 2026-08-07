# Callable Loop Production Admission D0

Status: closed as `NoSafeSlice` after the production-source audit
(2026-08-08).

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

1. the future source-side production ingress boundary and the exact reason
   the current production source cannot issue one
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
future ingress boundary is named without claiming a production issuer
missing resolver/source/facts bridge is named and assigned to the next row
its session/discard owner is named
input/output receipts are listed exactly once
old edge and same-slice retirement condition are named
NoSafeSlice is explicit where a contract is absent
reference/current docs are updated in the same commit as the design task
```

## Production-source audit

The named host is confirmed, but it does not yet issue the source products
needed by the prepared physicalization contract:

```text
NormalCallableSemanticLoanPortV1::lower_normal_top_level_function
  = production host / outer orchestration only

VerifiedNormalCallableSemanticSourceV1
VerifiedNormalCallableSemanticLoanV1
  = resolver-backed source/forest/projection and semantic loan

VerifiedCallableSingleLoopRecipeProductV1
VerifiedCallableSingleLoopSourceMapV1
VerifiedCallablePreludeV1
VerifiedCallableTailV1
  = currently cfg(test)-only issuers/consumers
```

`VerifiedNormalCallableSemanticLoanV1` exposes lineage and lowering state, but
there is no production bridge that co-seals the AST-free Loop Recipe,
source/effect relations, Prelude/Tail relations, and the exact owner/frame
brand. Therefore the adapter must not be made production by simply removing
`cfg(test)`, copying a test fixture, or re-walking the AST. Until that bridge
exists, the only honest result is a typed `NoSafeSlice` before a function
session opens.

The next independent design row is
`CALLABLE-LOOP-PRODUCTION-SOURCE-FACTS-BRIDGE-D0`. It owns only the mapping
from existing resolver/source/facts authority to one AST-free, owner-branded
callable Loop relation. It does not own physical IDs, CFG/SSA/PHI, Completion,
DraftSeal, selector, retry, fallback, or publication.

## Prepared ingress contract (design only)

After the bridge is accepted, the production ingress may have this shape:

```text
NormalCallableSemanticLoanPortV1
  -> resolver/source/facts bridge
  -> PreparedCallableLoopPhysicalizationV1
  -> fresh CanonicalFunctionLoweringSessionV1
  -> CanonicalSsaFunctionSessionV2 (Completion moved exactly once)
  -> Prelude / common Loop / After / Tail
  -> finish_for_draft_seal
  -> DraftSeal prepare/commit
```

The prepared product is a relational compatibility proof, not a new
semantic owner. Its crossing receipts are exactly:

```text
owner / compilation brand / frame
resolved callable input and source/facts lineage
VerifiedCallablePreludeV1
VerifiedLoopOperationPhysicalDemandV1
VerifiedCallableTailV1
exact return ABI capability
VerifiedFunctionCompletionV1
profile-close receipt
ReadyFunctionDraftSealV1 -> Prepared/CompletedFunctionDraftV1
```

The only unpublished-function/discard owner is the existing
`CanonicalFunctionLoweringSessionV1::discard_unpublished` terminal. Adapter
failure is pre-effect rejection; any failure after a fresh session opens
discards the whole unpublished function and restores the caller once. Phi
rollback is diagnostic cleanup only. No same-session repair, retry, or
fallback is allowed.

## Next-row acceptance and implementation stop

`CALLABLE-LOOP-PRODUCTION-ADMISSION-D0` is accepted only as this design
decision and closes `NoSafeSlice`. The following are still zero:

```text
production callable physicalization
source/facts bridge
I0 caller switch
selector / retry / fallback
Generic G0 parity
collector/module publication from the canary
legacy deletion
```

The bridge D0 must provide a complete source-to-relation correspondence,
foreign/duplicate/missing rejects, owner/brand/frame receipts, and a
production caller-zero fixture. Only then may a bridge I0 and the pure
prepared-adapter/receipt tests be opened. A later caller switch must replace
the recorded raw edge and retire that edge in the same commit.

No Rust or `.hako` implementation is authorized for the production ingress
until the bridge row is accepted. When implementation eventually opens, the
same slice must update the source README, `docs/reference/**`, diagnostics,
migration note, guards, and current pointers; the final reference update is
part of the implementation acceptance, not a follow-up task.
