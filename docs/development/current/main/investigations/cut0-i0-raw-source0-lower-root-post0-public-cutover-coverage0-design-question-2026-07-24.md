# RAW public cutover COVERAGE0 repair design question

Decision: `COVERAGE0-REPAIR-prime-r1`

Status: selected design. The first COVERAGE0 implementation is preserved in
`stash@{0}` as WIP and may resume only through the bounded repair task after
the decision lock below.

## Decision lock

All four recommended choices are accepted:

```text
Q1 = PLAN0 is the sole execution-order authority. The StaticHelper0 witness
     corroborates exact locator parity and never replaces the PLAN0 schedule.
Q2 = NarrowV1 is a public-only branded profile. General Raw eligibility and
     internal fixtures do not inherit the public helper restriction.
Q3 = helper coverage/profile handoff moves into a small sibling module;
     raw_root_eligibility.rs does not receive further orchestration growth.
Q4 = closeout guards validate durable closed markers and remain rerunnable
     after the current pointer advances to PARITY0 or a later row.
```

The next executable task is `COVERAGE0-REPAIR-S0`; PARITY0 remains parked
until its guard and focused evidence are green.

## Why this is a design stop

Worker audit found that the WIP implementation has a useful exact
`StaticHelper0` witness and pre-physical rejection, but it currently mixes
four contracts:

1. PLAN0 produces the ordered helper schedule, while CHILDREN0 currently
   discards the plan locators and executes a cloned witness schedule.
2. `RawPublicIngressPolicyV1::NarrowV1` is public-only, but the WIP narrows
   general `RawRootEligibilityV1`, including internal Raw fixtures.
3. Eligibility grows from an already large owner, although the COVERAGE0 card
   requires a small sibling for helper coverage.
4. The closeout guard requires an active row/status and cannot be rerun after
   the current pointer advances to PARITY0.

The WIP was stashed so the design boundary could be reviewed on a clean tree.
The repair row below is now authorized; normal-entry cutover, JSON, executor,
old Raw retirement, and CUT0 remain out of scope.

## Q1 — order authority

Choose one authority and state the non-authority explicitly.

### Candidate A: PLAN0 execution authority (recommended)

`RawRootPlanV1::into_pre_root_children` remains the only producer of the
ordered helper locator sequence. `RawStaticHelperCoverageV1` validates the
same locator rows and seals exact source facts, but is corroboration only.
Eligibility must compare the two sequences by exact locator identity before
physical open. CHILDREN0 consumes the PLAN0 locators; it does not call
`RawStaticHelperCoverageV1::into_locators()` as its schedule.

Required law:

```text
PLAN0 ordered locators = exactly coverage witness locators
CHILDREN0 schedule authority = PLAN0 locators
sorted_method_entries / HashMap iteration in CHILDREN0 = 0
```

### Candidate B: coverage witness authority

The witness owns the schedule and PLAN0 becomes a discarded summary. This is
rejected unless a new decision explicitly moves the existing Q1 order
authority, because it would create a second order producer in COVERAGE0.

## Q2 — public scope

Choose whether `StaticHelper0` is a public NarrowV1 policy or a global Raw
eligibility law.

### Candidate A: policy-branded public profile (recommended)

Keep general `RawRootEligibilityV1` reusable for internal Raw fixtures. Add a
small policy/profile handoff from `RawPublicIngressPolicyV1::NarrowV1` to a
public-only eligibility preparation boundary. The public profile admits only
the exact empty `StaticHelper0` rows; internal Raw fixtures retain their
existing explicit scope and do not silently inherit the public restriction.

The policy is passed as an owned/branded witness, not reread from an ambient
flag. No caller-selected helper policy is allowed.

### Candidate B: global first-slice narrowing

Apply the exact-empty helper rule to every Raw eligibility caller. This is
only valid if the decision explicitly accepts that internal Raw fixtures and
future non-public routes share the same restricted grammar. No such widening
or narrowing may be inferred from the public ingress name alone.

## Q3 — eligibility size and ownership

The existing `raw_root_eligibility.rs` is already near the 800-line source
limit. The repair must not add more orchestration there.

Recommended shape:

```text
raw_root_helper_coverage.rs
  source-only StaticHelper0 facts and typed errors

raw_root_helper_coverage_prepare.rs (or equivalent small sibling)
  policy/profile handoff
  PLAN0↔coverage exact parity
  consuming witness installation

raw_root_eligibility.rs
  existing eligibility authority only
```

The helper coverage module must not become a second catalog or order
authority. It owns only exact source facts and a sealed corroboration witness.
Eligibility rejects before `open_physical`; CHILDREN0 performs no grammar
revalidation.

## Q4 — closeout guard lifecycle

The guard must remain rerunnable after the pointer advances. It must not
require the row to remain `active`.

Recommended lifecycle contract:

```text
task Status: closed
CURRENT_STATE contains "COVERAGE0 are closed"
current row is the selected successor (PARITY0 or later active row)
```

The guard checks the durable closeout marker and the closed card, while the
successor row is checked only through `current_state_pointer_guard.sh`. This
prevents a historical guard from blocking a valid pointer advancement.

## Required fixtures before implementation resumes

```text
App with zero helpers
App with one empty helper
App with two helpers in reverse source insertion order
PLAN0 locator order != witness order -> typed pre-physical rejection
non-static helper -> typed coverage error, not only source bind Err
override helper -> typed coverage error
non-empty helper -> typed pre-physical rejection
metadata/params/uses/attrs/contracts helper -> typed pre-physical rejection
all rejection snapshots: physical/Builder/collector/ledger delta = 0
```

The two-helper fixture must prove lexical order without re-running
`sorted_method_entries` in CHILDREN0. The plan/witness mismatch fixture must
prove that the witness corroborates the plan rather than replacing it.

## Guard contract after decision

```text
PLAN0 schedule producer = 1
coverage witness producer = 1
PLAN0↔coverage exact parity check = 1
CHILDREN0 executes PLAN0 locators = 1
CHILDREN0 coverage grammar re-run = 0
HashMap/sorted_method_entries in CHILDREN0 = 0
public NarrowV1 policy handoff = 1
general eligibility hidden-policy narrowing = 0
eligibility source/check files < 800 lines = 1
closed task and durable closeout marker = 1
normal entry / JSON / executor / old Raw retirement / CUT0 = 0
```

## Implementation stop line

After this consultation is accepted, the next executable card is a bounded
`COVERAGE0-REPAIR-S0` row containing only the chosen authority/profile
handoff, the small sibling extraction, the required fixtures, and a
rerunnable guard. PARITY0 remains parked until that row is green.
