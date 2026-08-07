# Callable Loop DraftSeal P0

Status: closed caller-zero typed finish and DraftSeal prepare/commit;
successor is `CALLABLE-LOOP-PRODUCTION-EDGE-D0`.

Decision: connect the existing callable profile-close evidence to the sole
typed function finish terminal and the existing DraftSeal transaction. This
row does not add a new semantic owner, physicalizer, selector, provider, or
module publication route.

## Scope

Consume one `ReadyCallableTailCompletionV1` from the Tail row and execute the
existing terminal sequence exactly once:

```text
TailCompletion::into_profile_close()
  -> finish_profile_close(owner, terminal_block,
       || profile_close.finish(owner, terminal_block))
  -> CanonicalSsaFunctionSessionV2::finish_for_draft_seal(...)
  -> ReadyFunctionDraftSealV1::open(...)
  -> OpenFunctionDraftSealV1::prepare()
  -> PreparedFunctionDraftSealV1::commit()
  -> one CompletedFunctionDraftV1
```

The profile-close receipt is the only input that closes the callable profile
ledger. It proves the sealed After predecessor, owner, terminal block, Bool
condition, and exact callable operation coverage (`7 = Pure4 + Read2 +
Write1`). A no-op `|| Ok(())` closure is forbidden.

## Landed implementation evidence (2026-08-07)

The caller-zero canary now consumes the profile-close receipt through the
existing validating `finish_profile_close` closure, uses the canonical session
CFG/semantic/identity/PHI/binding/Completion finish terminal, opens the
existing DraftSeal owner, prepares the detached plan, and commits one
`CompletedFunctionDraftV1`. No separate CFG session is used; the canonical
session remains the sole CFG/SSA/PHI owner. The committed draft restores the
caller context, and a fresh session can be opened afterward.
Focused evidence is green: the integrated callable canary is `1/1`, the
physicalizer suite is `20/20`, `cargo check --tests`, the current-state pointer
guard, and the in-place replacement guard all pass. The Tail source-use claim
is issued before the intentionally non-claiming `read_entry_receipt`, so the
canonical identity ledger closes at `4/4` uses.

## Required acceptance

- the consumed profile-close evidence reaches `finish_profile_close` through
  its validating closure;
- `finish_for_draft_seal` consumes CFG, semantic/If, identity, PHI, binding,
  and Completion owners exactly once;
- `ReadyFunctionDraftSealV1` is opened only from the typed finish result;
- DraftSeal `prepare` and `commit` use the existing transaction owners;
- one positive fresh-session path reaches `CompletedFunctionDraftV1`;
- one profile-close mismatch and one integrated late failure reject without
  retry or fallback; the existing F1 DraftSeal negative suite is reused for
  CFG, semantic, identity, PHI, binding, Completion, stale/signature,
  metadata, and verification mismatch coverage;
- `ReadyCallableTailCompletionV1` and its profile-close evidence are
  non-Clone and consumed exactly once; the production V2
  `ReadyFunctionDraftSealV1::new` caller count remains zero and the physical
  Return writer remains owned by DraftSeal;
- every rejected or late path discards the unpublished function and restores
  the caller exactly once; a fresh session can be opened afterward;
- the collector and module publication remain untouched;
- touched Rust source files remain below 800 lines and all required guards
  are green.

## Non-claims

This row does not claim Generic G0 parity, all-19 route coverage, production
caller selection, scheduler replacement, retry/fallback retirement, backend
parity, provider selection, performance, or legacy deletion. It does not add
a direct `ReadyFunctionDraftSealV1::new` caller or a second Return writer.

## Documentation requirement

The implementation commit must update this task, the common physical-demand
and session SSOT, the MIR reference contract, `src/mir/builder/resolved_lowering/README.md`,
the current-state pointers, and the active workstream mirror. Any later
implementation slice must update the exact relevant reference documentation
in the same commit.
