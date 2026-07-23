# CUT0-I0 OWNER-RETENTION0 ROOT Consultation

Status: **Decision locked — Candidate OR-prime selected; ROOT-RETENTION0-PREFLIGHT is next**
Date: 2026-07-23
Scope: retain the complete unpublished Raw root owner across every root-batch
preflight failure without adding a production consumer.

Related:

- `docs/development/current/main/investigations/cut0-i0-prod-activation-execution-task-2026-07-23.md`
- `docs/development/current/main/investigations/cut0-i0-prod-activation-p0-r1-failure-consultation-2026-07-23.md`
- `src/mir/builder/raw_root_completion.rs`
- `src/mir/builder/module_draft_collector/root_batch.rs`
- `src/mir/builder/raw_expansion_receipt_ledger.rs`

## Current gap

`complete_raw_root` currently accepts seven loose owned arguments:

```text
token
collector
ledger
prepared root batch
main reservation
condition reservation
callable-main disposition
```

The collector root preflight consumes the batch, and later ledger completion
can still fail after collector receipts have been produced. Errors are mapped
to a bare `RawRootCompletionErrorV1`, so the complete unpublished owner cannot
be retained for discard or diagnosis.

The existing late-admission fixture proves publication-zero, but not the
strong OWNER-RETENTION0 law. This is a real ownership seam, not a missing test
only branch.

## Candidate OR-prime

Use one non-Clone root input and a two-phase preflight/commit boundary:

```text
RawRootCompletionInputV1
  owns token / branded collector / mutable ledger /
       PreparedRootDraftBatch / reservations / disposition

-> prepare(self)
     immutable family/brand/policy/reservation/root-body checks
     collector admission validation
     ledger slot/history validation

success -> PreparedRawRootCompletionV1
failure -> RejectedRawRootCompletionV1 { owner, typed error }

-> commit(self)
     one infallible collector receipt + root evidence + ledger publication
     -> RawCompleteInvocationV1
```

The prepared product is the only mutation terminal. No `Option::take`, clone,
Arc, retry, fallback, or bare error mapping may discard the owner.

## Questions and decision

### Q1 — root input owner

**Yes.** The loose `complete_raw_root` arguments are replaced by one private
`RawRootCompletionInputV1` constructor, with token/brand/family sealed at the
same boundary?

### Q2 — collector preflight

**Yes.** `ModuleDraftCollectorV1::validate_root_batch(&self, &batch)` becomes the
read-only authority, with `prepare_root_batch` consuming only after validation?
The rejected owner must retain the original prepared batch and unchanged
collector indexes.

### Q3 — ledger preflight

**Yes.** The ledger gains a read-only `validate_required_root_batch` and a single
infallible `commit_preflighted` terminal, so a condition receipt failure cannot
leave a Main event behind?

### Q4 — implementation series

**Yes, with TOKEN-HANDOFF split out explicitly.** The smallest safe series is:

```text
ROOT-RETENTION0-PREFLIGHT
  one input owner + rejected owner + collector/ledger read-only checks

ROOT-RETENTION0-COMMIT
  prepared commit + RawComplete handoff

ROOT-RETENTION0-TOKEN-HANDOFF
  RawComplete owns the token; bind_physical(self, session, shell)

OWNER-RETENTION0-PHYSICAL/POST
  downstream rejected-owner retention
```

Each commit must build, keep production consumers at zero, and remain below
800 lines per touched source/check file.

## Acceptance and non-claims

```text
failure at every preflight stage retains the full input owner
collector/ledger/root-body state is unchanged before commit
live Builder and external publication remain untouched
success has exactly one PreparedRawRootCompletionV1::commit(self)
retry/recovery/fallback terminals = 0
same invocation token/brand survives the handoff
```

## OR-prime closeout

Candidate OR-prime is selected.

```text
Q1  one non-Clone RawRootCompletionInputV1 owns the complete unpublished root
    input; token/family/brand are sealed once and loose arguments disappear
    from the production terminal.

Q2  collector validation borrows the collector and PreparedRootDraftBatch;
    it does not consume the batch or mutate indexes before all checks pass.

Q3  ledger validation borrows the ledger and both reservations/expected rows;
    only PreparedRawRootCompletionV1::commit(self) may mutate collector,
    ledger, and root witness, and that terminal is infallible.

Q4  successful RawCompleteInvocationV1 owns the original non-Clone token.
    TOKEN-HANDOFF later removes the loose token argument from bind_physical.
```

Every preflight failure returns `RejectedRawRootCompletionV1 { owner, error }`.
The rejected product exposes error inspection and discard only: no retry,
replacement, parts extraction, or recovery-to-complete terminal. The first
row starts after `CompletedRootBodyV1` and both reservations already exist.

This consultation does not claim retention for root-body completion,
reservation allocation, canonical routes, finalizer/postprocess failures,
production Raw ingress, physical drain, or atomic CUT0. Those remain separate
rows.

This consultation does not authorize canonical source changes, production
Raw ingress, physical drain, finalizer/postprocess wiring, or atomic CUT0.
Those remain disconnected until ROOT-RETENTION0 and its downstream rows are
green.
