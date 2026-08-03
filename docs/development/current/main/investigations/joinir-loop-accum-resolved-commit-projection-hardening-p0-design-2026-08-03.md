# Resolved DirectAccum Commit/Projection Hardening P0 Design

Date: 2026-08-03
Status: accepted; H0/H1/H2 implemented, H3 guard closeout in progress.

## Context

The first resolved DirectAccum family is now connected through the existing
`compile_resolved` owner and the source-bound candidate chain. The production
physicalizer and the `CanonicalSsaFunctionSessionV2` / `BindingSsaBuilderV1` /
`CanonicalCfgSessionV1` / `PhiTxn` lifecycle are already single-owner. The
remaining proof gap is at the late-failure and result-publication boundary,
not in PHI/SSA design.

Two facts must not be silently conflated:

```text
old compile_resolved_first_family:
  final verification is a required barrier
  successful MirCompileResult.verification_result = Ok(())

source-bound external commit today:
  Canonical evidence retains { pre_transform, final_verified }
  LegacyPublicationPayload projects pre_transform
```

If pre-transform verification fails but final verification succeeds after the
canonical postprocess repair, the two paths expose different public meanings.
That is a contract decision, not a test-detail.

## Worker decision

For the canonical `BindingSsaTrivial` DirectAccum family, use the existing
explicit canonical policy:

1. `RequireFinal` remains the publication barrier.
2. Any final-verification failure is a typed postprocess failure and performs
   zero external commit.
3. A successful canonical publication exposes `Ok(())` as
   `MirCompileResult::verification_result`, matching the old resolved
   `finish_built_canonical_module` contract.
4. Raw publication keeps its separate `ReportPreTransformOnly` contract; this
   row must not generalize the canonical decision to Raw or normal/default.

The source-bound external commit projection must therefore consume the sealed
canonical `final_verified` evidence for the returned result, rather than
re-projecting `pre_transform`. The evidence remains retained internally for
diagnostics/parity tests; it is not silently deleted.

## Accepted hardening order

### H0 — contract and fixture freeze

Add tests/docs that name the exact failure stages and observable boundary:

```text
preflight rejection: no candidate, no commit
late lower/collector/completion/finalizer/postprocess rejection:
  live Builder fingerprint unchanged, current module unpublished, commit=0
success:
  commit=1, final verification required, result verification_result=Ok(())
same compiler after failure:
  valid DirectAccum request succeeds
```

The fingerprint is the existing candidate fingerprint surface; do not add a
second Builder snapshot or an undo journal.

### H1 — ingress-level late-failure seam

Add one `#[cfg(test)]` fault seam around the existing source-bound postprocess
owner (reuse the mutation pattern from `module_postprocess_failure_p0.rs`).
The seam is test-only and must not add a public API, environment toggle,
second compiler ingress, retry, or new SSA/PHI owner. The fixture must prove:

```text
DirectAccum compile -> injected postprocess/final-verifier failure
-> no external commit + live fingerprint unchanged
-> same MirCompiler -> valid DirectAccum compile succeeds
```

Candidate-only seal-failure and reopen tests remain valid evidence for the
lowering boundary, but do not substitute for this production-ingress proof.

### H2 — canonical result projection parity

Change only the canonical projection in the existing external-commit owner so
that a successful `RequireFinal` canonical result reports `Ok(())`. Add a
focused source-bound parity fixture covering:

- pre-transform verifier rejection that is repaired before final verification;
- final-verification rejection that commits nothing;
- final-success result equal to the old resolved canonical contract.

Raw `pre_transform` reporting and normal/default parity remain untouched.

### H3 — static closeout guard

Extend the existing shared guards to assert:

- exactly one DirectAccum production caller and one prepare/commit chain;
- canonical projection uses final verification evidence;
- no `pre_transform` projection escapes into canonical `MirCompileResult`;
- no retry/fallback/route registry/legacy PHI writer is reachable from this
  family;
- all touched Rust files remain below 800 lines.

## Implementation result (2026-08-03)

H1 is now covered at the real resolved DirectAccum ingress boundary. The
test-only seam runs the complete source-bound prepare chain, drops the
prepared publication product immediately before commit, and returns a typed
injected error. The fixture verifies the existing Builder fingerprint and
unpublished-module state are unchanged, then compiles a valid request on the
same `MirCompiler`.

H2 is implemented in the existing external-commit owner. Canonical evidence
that has crossed `RequireFinal` projects `verification_result = Ok(())`; the
sealed `pre_transform` evidence remains available to the owner but is no
longer exposed as the public canonical result. Raw publication remains on its
separate pre-transform-reporting path.

The projection parity unit fixture is intentionally synthetic: it models a
canonical final seal with a retained pre-transform error. Existing natural
postprocess fixtures separately cover final-verification rejection and
discard-only ownership. A natural DirectAccum fixture where pre-transform
verification fails and final verification succeeds is not claimed here.

The shared candidate-scope guard now fixes the one production prepare/commit
edge, the test-only late-failure proof, and the canonical final-barrier
projection helper. No PHI/SSA owner or route scheduler was added.

## Non-claims

This design does not activate Generic V0/V1, other Loop families, normal or
default compile, `route_loop`, a second candidate, symbolic MIR, undo journal,
or repository-wide legacy PHI-writer retirement. PHI/SSA ownership is already
SSOT'd and remains unchanged.

## Stop conditions

Return to design if implementing H1/H2 requires a public fault toggle, a new
production ingress, a second postprocess/projection owner, a Builder snapshot
outside the existing candidate session, or changing Raw/normal result
semantics. Only after H0 is documented and the canonical result decision is
accepted may H1/H2 code changes begin.
