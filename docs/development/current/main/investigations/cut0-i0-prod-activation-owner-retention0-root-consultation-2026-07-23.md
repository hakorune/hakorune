# CUT0-I0 OWNER-RETENTION0 ROOT Consultation

Status: **Design stop — Candidate OR-prime under review**
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

## Questions for decision

### Q1 — root input owner

Should the loose `complete_raw_root` arguments be replaced by one private
`RawRootCompletionInputV1` constructor, with token/brand/family sealed at the
same boundary?

### Q2 — collector preflight

Should `ModuleDraftCollectorV1::validate_root_batch(&self, &batch)` become the
read-only authority, with `prepare_root_batch` consuming only after validation?
The rejected owner must retain the original prepared batch and unchanged
collector indexes.

### Q3 — ledger preflight

Should the ledger gain a read-only `validate_required_root_batch` and a single
infallible `commit_preflighted` terminal, so a condition receipt failure cannot
leave a Main event behind?

### Q4 — implementation series

The smallest safe series is:

```text
ROOT-RETENTION0-PREFLIGHT
  input owner + rejected owner + collector/ledger read-only checks

ROOT-RETENTION0-COMMIT
  prepared commit + RawComplete handoff

ROOT-RETENTION0-PHYSICAL/POST
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

This consultation does not authorize canonical source changes, production
Raw ingress, physical drain, finalizer/postprocess wiring, or atomic CUT0.
Those remain disconnected until ROOT-RETENTION0 and its downstream rows are
green.

