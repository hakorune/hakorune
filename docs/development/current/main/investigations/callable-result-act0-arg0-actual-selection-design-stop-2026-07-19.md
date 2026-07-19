---
Status: Resolved by direction-2 rebase
Date: 2026-07-19
Parent: callable-result-act0-arg0-source-gate-task-2026-07-19.md
Scope: ACT0-ARG0 actual Parser source-gate premise
---

> Resolved by `callable-result-act0-arg0-rebase-task-2026-07-19.md`.
> The design-stop evidence remains authoritative for the actual source shape.

# ACT0-ARG0 actual-selection design stop

## Observed contradiction

`CALLABLE-RESULT-ACT0-ARG0-S0` is closed. Its disconnected source gate is
sound: an exact-i64 static target is selected only when the result catalog has
an exact call-result row for the same `(caller, SourceExprSiteV1)`.

P0 tested the actual `ParserBox.static_const_parse_add` source without
activation rows, Builder state, ledger claims, or Loop lowering. The source
does **not** contain the decision card's presumed pre-Loop shape
`skip_ws(text, pos)`.

```hako
pos = ParserStringUtilsBox.skip_ws(
    text,
    me.static_const_eval_pos(ret)
)
```

The Loop cleanup has the same nested current-owner call shape, with `rhs`.
Both outer `skip_ws` calls require argument ordinal 1. The nested
`me.static_const_eval_pos(...)` calls are outside the current static target and
Core-string source-proof profiles, so their source facts are `Unknown`. No
exact call-result row is recorded for either outer target candidate.

```text
actual method-call rows:       15
static target candidates:       2
exact call-result rows at them: 0
Candidate 1′ selected rows:     0
Candidate 1′ unselected rows:  15
```

This is not a parameter-proof producer failure. A direct formal parameter is
already represented as an exact source fact; the actual required expression is
instead a nested instance-call result.

## Why P0 cannot silently continue

The selected ARG0 card locks a different actual outcome:

```text
selected:   1
unselected: 14
```

and later requires one successful selected emission in resumed L0. Changing
the P0 expectation to `0 / 15` would make the focused test pass, but would
silently revoke both of those claims. It is therefore forbidden.

Candidate 1′ itself correctly classifies the two outer calls as `Unselected`.
It must not compensate by writing an `Integer` type, rewalking AST syntax,
matching a method name, or retrying raw execution after selected-terminal
failure.

## Preserved boundaries

```text
S0 source-gate classifier:             retained, disconnected
ARG0 I0 activation-row connection:     blocked
ARG0 G0 closeout:                       blocked
clean LOOP0-L0 resumption:              blocked
actual claim schedule / ledger:         unchanged
CoreCallSource / located-plan identity: unchanged
stash restoration:                      forbidden
```

The evidence-only P0 attempt is preserved as:

```text
stash@{0}: wip/act0-arg0-p0-actual-zero-selected (design stop)
```

It is not an implementation authority.

## Required decision

Choose exactly one direction before another code-facing row:

1. Open the already parked `CALLABLE-RESULT-NESTED-REP0` widening. It would
   need a separately sealed current-owner/instance-call result authority and
   an emission-local source-site-to-final-result witness. This restores the
   possibility of an actual selected outer call, but is explicitly outside
   Candidate 1′'s original L0 scope.
2. Revise the actual-profile contract to accept `0 selected / 15 Unselected`,
   and separately nominate a different explicit source fixture for selected
   terminal coverage. This no longer claims that the actual Parser caller
   proves selected emission.

No third option may infer a nested instance result from its spelling, runtime
class, final metadata, or raw success.

## Raw-primary-path audit

The existing claimed CorePlan emission port already distinguishes an
`Unselected` claim before selected-terminal work:

```text
Unselected claim
  -> one direct existing raw-effect emission

SelectedExactI64 claim
  -> selected terminal and its final ValueId type gate
```

Thus a future `0 selected / 15 Unselected` actual profile would not mean
"try selected, catch the failure, then use raw." It would remain a planned
primary raw route for every row.

This audit is structural evidence only. The repository does not yet prove the
complete actual 15-row all-Unselected lowering and execution path, because the
source gate is still disconnected from activation-row construction. If direction
2 is selected, `P0-REBASE0` must first prove that path with no selected-terminal
attempt, no claim-schedule change, and no retry. A raw-path failure there opens
a separate raw plan/emission parity stop; it must not reopen nested-result type
inference implicitly.

## Next code-facing owner after a decision

```text
if direction 1:
  CALLABLE-RESULT-NESTED-REP0-D0

if direction 2:
  CALLABLE-RESULT-ACT0-ARG0-P0-REBASE0
```

Until then, the active blocker is this contradiction rather than a type or
Loop-local lowering bug.
