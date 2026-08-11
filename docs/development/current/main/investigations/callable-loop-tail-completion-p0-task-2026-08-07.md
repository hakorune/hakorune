# Callable Loop Tail / Completion P0

Status: closed caller-zero Tail/Completion handoff and move-only profile-close
evidence; successor is `CALLABLE-LOOP-DRAFT-SEAL-P0`.

Decision: implement only the callable Tail-to-Completion boundary. The
common Loop physicalizer, Generic G0, DraftSeal, production selection, and
legacy retirement remain closed.

## Scope

Consume the one-shot `ReadyLoopAfterContinuationV1` produced by the A slice
and prove, in a fresh unpublished function session, that:

1. the exact callable Tail source binding is read through canonical identity;
2. the resulting `ValueId` matches the existing declared return ABI;
3. `mark_return` is issued exactly once for the selected After block;
4. the verified function Completion is claimed exactly once;
5. a move-only profile-close receipt preserves the sealed After witness and
   exact callable coverage (`7 = Pure4 + Read2 + Write1`, including the Bool
   condition) for the later typed function-finish terminal;
6. any failure discards the whole unpublished session and restores the caller
   once.

The implementation must use the already sealed CFG/identity state. It must
not reopen or re-seal After, invent a second return path, or infer a type from
the physical value.

## Landed implementation evidence (2026-08-07)

The test-only callable canary now consumes `ReadyLoopAfterContinuationV1`,
reads the Tail binding through `ResolvedSsaIdentityStateV2`, publishes the
exact `i64` ABI only when the sealed PHI is still `Unknown`, claims the
existing Completion once, and marks the same return site once. A second
Completion claim rejects, and the unpublished function session is explicitly
discarded after the canary. The common operation physicalizer remains
unaware of Tail, ABI, Completion, Return, and DraftSeal.
The common After receipt remains neutral. The existing `#[cfg(test)]`
Callable Tail adapter owns the non-Clone `ReadyCallableLoopProfileCloseV1`; it
moves through the Tail/Completion receipt and revalidates owner, sealed After
predecessor, exact operation family counts, and terminal block without
introducing a second semantic owner. Its `finish` method is the required
non-no-op closure input for the later `finish_profile_close` step.

## Required contracts

```text
ReadyLoopAfterContinuationV1 -> one-shot Tail consumer
VerifiedCallableTailV1       -> exact source binding / terminal site
ExactReturnAbiV1              -> declared result compatibility
VerifiedFunctionCompletionV1  -> one claim, then consumed
```

The Tail contract is distinct from Loop After. The common operation
physicalizer never sees Tail, ABI, Completion, Return, DraftSeal, or profile
identity.

## Acceptance

- positive callable fixture reaches `mark_return` once and claims Completion
  once;
- wrong owner, wrong block, missing binding, and incompatible return type
  reject before return publication;
- second Tail consumption and second Completion claim reject;
- profile-close evidence is one-shot, validates owner/block/predecessor and
  exact `7/4/2/1` coverage, and cannot be replaced with `|| Ok(())`;
- failure after return preparation uses `discard_unpublished` and fresh
  session reuse; no retry or fallback is added;
- current callable operation coverage remains 7 (`Pure=4`, `Read=2`,
  `Write=1`) and the real Prelude receipt remains the only entry source;
- touched source files remain below 800 lines and all fast guards stay green.

## Non-claims

This row does not claim `finish_for_draft_seal`, DraftSeal publication,
module collection, Generic G0 parity, production caller selection, retry or
fallback removal, backend parity, or legacy deletion.

## Documentation requirement

The implementation commit must update this task, the callable/physical
session SSOT, the MIR reference contract, the current-state pointers, and
the relevant workstream/readme mirrors. Any later implementation row must
also update its reference documentation in the same commit.
