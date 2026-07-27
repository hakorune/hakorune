---
Status: design stop; consultation required
Date: 2026-07-28
Decision: pending
Closes:
  - PRELOOP-STAGEB-SELECTED-CANDIDATE-SESSION-CORRESPONDENCE0-P0
Stops:
  - PRELOOP-STAGEB-SELECTED-CANDIDATE-SESSION0-D0
Return row:
  - OWN-GRAM-REJECT0-HAKO0-S0
Related:
  - docs/development/current/main/investigations/hakorune-sparse-ownership-surface-task-2026-07-15.md
  - docs/development/current/main/investigations/preloop-stageb-carrier-f5-f9-execution-task-2026-07-27.md
  - docs/development/current/main/investigations/preloop-stageb-instance-function-session-reconciliation0-prime-r1-task-map-2026-07-27.md
---

# PRELOOP-STAGEB selected candidate session design question

## Why this stop reopened

The active ownership row remains:

```text
OWN-GRAM-REJECT0-HAKO0-S0
```

Its unchanged Hako return-type guard stops before the new reject can execute:

```text
[plan/freeze:contract] generic_loop_v1 skeleton failed:
GenericLoop carrier representation failed:
MissingTransientType { init: ValueId(28) }
```

The ownership taskboard explicitly permits reopening one parked Stage-B owner
only when this unchanged gate proves it is a direct prerequisite. This is that
case. The return row stays OWN-GRAM; the detour cannot acquire a new terminal.

## Correspondence result

The source inventory, exact candidate selection, same-allocation catalog and
alias installation, selected function capture, inner/outer Call receipts,
assignment correspondence, and success-only outer Integer publication are
already closed.

Focused evidence:

```text
PreloopStageBWholeSourceProducerV1 selection tests = 8/8 green

actual Parser selected function:
  ParserBox.static_const_parse_add/2
  collected before unchanged MissingTransientType

production selector consumer:
  0
```

The remaining gap is not another result/type owner. It is one isolated Builder
candidate session for the Legacy-arm Selected branch.

### Existing canonical session

```text
CanonicalModuleLoweringSessionV1
  candidate = MirBuilder::new()
  carries   = quiet_internal_logs only
  core IDs  = fresh
  commit    = replace live Builder
```

It does not preserve:

```text
CoreContext continuation
repl_mode
plugin_method_sigs
source hint
```

Using it for Selected would silently change Legacy Builder state.

### Existing invocation session

```text
ModuleBuilderInvocationSessionV1
  candidate/config preservation = sufficient
  CoreContext ContinueLive       = available
  failure drop / commit          = available
```

But construction requires:

```text
ModuleInvocationBrandV1
ModuleInvocationFamilyV1
ModuleInvocationTokenV1
```

Those identities belong to Raw/Canonical invocation lifecycle and downstream
publication receipts. The bounded Legacy Selected route owns neither.

### Existing selected module transaction

```text
PreparedSelectedPreloopStageBWholeSourceV1
  -> PreparedPreloopStageBModuleActivationV1
  -> atomic catalog + typed alias install
  -> InstalledPreloopStageBModuleActivationV1
  -> selected function capture
  -> CompletedPreloopStageBPreinstalledRootV1
```

This chain already operates on an isolated Builder supplied by its caller. It
does not own opening, closing, or committing that Builder.

## Required laws

```text
selection before Builder effects                  = 1
selected candidate Builder                        = isolated 1
CoreContext continuation                          = exact 1
repl_mode preservation                            = exact 1
quiet_internal_logs preservation                  = exact 1
plugin_method_sigs preservation                   = exact 1
source hint preservation                          = exact 1

typed aliases                                     = selected request only
ambient alias read during selection               = 0
catalog reseal                                    = 0

prepare_module                                    = existing 1
selected preinstalled-root lowering               = existing 1
finalize_module                                   = existing 1
compiler finish/postprocess                       = existing 1

success-only live Builder replacement             = 1
failure live Builder mutation                     = 0
selected failure -> Ordinary retry                = 0
family/route reselection                          = 0

Raw brand/token/ledger/publication in Legacy       = 0
Canonical source identity in Legacy               = 0
Builder clone/snapshot rollback                    = 0
second module lifecycle                           = 0
```

## Options

### A-prime — extract one brand-free candidate-session core

Extract the transport/lifecycle-neutral part of
`ModuleBuilderInvocationSessionV1`:

```text
BuilderCandidateSessionCoreV1
  - MirBuilder candidate
  - BuilderInvocationConfigV1
  - quiescence/readiness
  - consuming success-only replacement
```

Keep family wrappers:

```text
ModuleBuilderInvocationSessionV1
  = brand + family + brand-free core

LegacySelectedBuilderCandidateSessionV1
  = selected Legacy owner + brand-free core
```

The Legacy wrapper alone drives:

```text
existing prepare_module
existing selected preinstalled-root transaction
existing finalize_module
existing compiler finish/postprocess
```

The core owns no source classification, catalog, aliases, family identity, or
publication policy.

Recommended opening configuration:

```text
BuilderInvocationConfigV1::snapshot_for_legacy_selected(...)
  CoreContext       = ContinueLive
  repl/quiet/plugin = preserved
  source hint       = request hint
  aliases           = initially empty

typed request aliases
  -> existing atomic selected activation install only
```

This is the recommendation.

### B — add a Legacy family and mint an invocation token

Extend:

```text
ModuleInvocationFamilyV1::LegacySelected
```

and reuse the complete branded invocation session.

Reject unless there is an independently required Legacy publication identity.
The current route needs isolation and commit, not a new source/publication
brand. Minting one would give a bounded repair unnecessary downstream
authority.

### C — widen CanonicalModuleLoweringSessionV1

Teach the canonical session to preserve all Legacy configuration and add a
ContinueLive mode.

Reject unless canonical and Legacy source lifecycles are intentionally merged.
It would make the canonical Fresh-ID session policy-dependent and would move a
Legacy compatibility concern into a canonical owner.

### D — lower on the live Builder and restore on failure

Reject.

This requires a complete mutable Builder rollback inventory and changes the
already accepted failure law. The repository has no complete clone/restore
authority for this path.

## Questions

```text
Q1:
  Is A-prime the correct owner boundary?

Q2:
  Should the brand-free core own only candidate/config/readiness/replace,
  leaving prepare/lower/finalize/postprocess in thin family wrappers?

Q3:
  Should the Legacy Selected config start with aliases empty and permit the
  existing atomic selected activation transaction to be their sole installer?

Q4:
  Is ContinueLive the correct CoreContext seed for this Legacy route?

Q5:
  What exact product must cross compiler finish before live replacement:
    MirCompileResult + closed candidate session
  or
    a stronger prepared Legacy publication owner?
```

## Recommended implementation order after acceptance

```text
BUILDER-CANDIDATE-SESSION-CORE0-S0
  behavior-neutral extraction from ModuleBuilderInvocationSessionV1

PRELOOP-STAGEB-SELECTED-CANDIDATE-SESSION0-S0
  Legacy Selected wrapper + exact config

PRELOOP-STAGEB-SELECTED-CANDIDATE-SESSION0-P0
  preservation/failure/success/reuse matrix

PRELOOP-STAGEB-SELECTED-CANDIDATE-SESSION0-G0
  sole core + no brand/fallback/catalog reseal

PRELOOP-STAGEB-LEGACY-WHOLE-SOURCE-REQUEST0-I0
  owned request plumbing

PRELOOP-STAGEB-COMPILE-REQUEST-INGRESS0-I0/P0/G0
  sole Legacy-arm production consumer

CALLABLE-RESULT-NESTED-PRELOOP-STAGEB0-P0
  unchanged real progression guard

return:
  OWN-GRAM-REJECT0-HAKO0-S0
```

## Non-claims

```text
ownership grammar activation
general Legacy transaction cutover
direct MirBuilder::build_module activation
Raw/canonical lifecycle merge
new invocation/publication identity
loop-refresh activation
GenericLoop-side type publication
fallback/retry
default backend or compiler route change
```

