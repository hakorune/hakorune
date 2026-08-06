# Generic callable-semantic Loop handoff S0 (pre-effect envelope)

Status: `active implementation row`

Parent: `GENERIC-CALLABLE-SEMANTIC-LOOP-HANDOFF-D0`

## Change

Issue one AST-free, non-`Clone`, single-use
`VerifiedCallableSemanticLoopBindingScheduleV1` from the selected callable
semantic source bridge and carry it through the located raw Loop entry. Verify
the one admitted role profile (condition read, body read, body rebind) before
the existing Generic route is allowed to create Builder effects.

Old authority: the legacy Generic route remains compatibility evidence; it is
not repaired or promoted by this row.

## Contract

- The source-only projector owns site/BindingRef roles; no AST or
  `variable_map` enters the schedule. The current lowering-state view is a
  migration bridge and is not the final resolver authority.
- S0 only issues and verifies the pre-effect envelope. Portable
  Recipe/JoinSig projection, BindingRef consumption, `ValueId`/PHI
  publication, and ledger migration belong to S1 physicalization.
- Missing, foreign, duplicate, nested-profile, or incomplete coverage returns
  a typed pre-effect Freeze.  No retry, fallback, name lookup, or post-effect
  repair is permitted.
- The existing `CallableSemanticLoweringState` ValueId map remains a legacy
  bridge and is not declared the physical owner.

## Done

- Positive fixture proves the selected callable Loop schedule contains the
  exact condition/body variable and assignment sites for the admitted
  one-condition/one-body-read/one-rebind profile.
- Foreign, duplicate, missing, partial, and unsupported nested-profile
  schedules reject before `lower_loop_or_freeze_v1` is entered.
- A located raw-entry test proves a `Some` schedule is carried into the
  prepared entry; route execution and physical ledger consumption remain
  outside this row.
- Source transport and current pointer are updated in the same commit.
- The exact `docs/reference/mir/generic-loop-stage-matrix.md` entry and its
  immutable receipt are updated with the S0 boundary; later implementation
  rows must update reference docs in the same commit as their code.

S0 explicitly does not claim a `GenericSourceProjector`, portable Recipe,
JoinSig, physical ValueId/PHI consumption, production selection, or legacy
caller retirement.

## Stop

Return to design if schedule roles cannot be derived from resolver-issued
sites, if the raw entry must inspect AST after the projector closes, if a
second physical ValueId/BindingSSA owner is needed, or if the route requires
retry/fallback to pass.  S0 does not reopen production selection or legacy
deletion.
