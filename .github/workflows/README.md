# CI feedback tiers

`min-gate.yml` and `portability-ci.yml` use the pull-request draft state as
the boundary between the edit loop and merge-readiness validation.

## Decision

- Draft pull requests run the fast `rust-check` steps on every update.
- The Language v1 sensitive-change FULL gate does not run for a draft.
- Moving a pull request to ready-for-review triggers both workflows again.
- Moving it back to draft triggers a fast run and cancels superseded work in
  the same workflow concurrency group.
- Non-draft pull-request updates run the FULL gate when the sensitive-path
  selector requires it, and run the portability jobs.
- Manual `workflow_dispatch` keeps the portability jobs available.

The FULL gate remains a merge-readiness check. Draft skipping is a feedback
tier, not a weakening of the language contract.

## Authority boundary

- Workflow event and draft policy: `.github/workflows/*.yml`
- Sensitive-path selection: `tools/checks/language_v1_full_gate_for_changes.sh`
- Static drift guard: `tools/checks/ci_feedback_tier_policy_guard.sh`

The workflow must not infer language sensitivity from job names, commit
messages, or runtime results. The existing sensitive-path manifest remains the
only selector once a non-draft pull request reaches the FULL-gate step.

## Non-claims

- The portability matrix is not yet path-filtered.
- The FULL gate is not split into a parallel job in this row.
- Scheduled/nightly coverage is not introduced here.
