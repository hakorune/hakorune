---
Status: Historical evidence index; non-normative and non-executable
Date: 2026-07-14
Superseded: 2026-07-15
Normative source contract: ../../../../reference/language/ownership.md
Current execution board: hakorune-sparse-ownership-surface-task-2026-07-15.md
Production activation: 0
Related:
  - hakorune-ownership-v2-scoped-mutable-alias-consultation-2026-07-14.md
  - hakorune-ownership-v2-anchored-view-return-abi-task-2026-07-15.md
  - mirbuilder-dprime-binding-ssa-final-form-task-2026-07-14.md
---

# Root-Anchored Alias V2 — Historical Evidence Index

This file is a slim redirect for the superseded 2026-07-14 taskboard. It is not
a source-semantics owner and must not be executed as a parallel roadmap.

Use these authorities instead:

```text
source owner / alias / View / Shared semantics:
  docs/reference/language/ownership.md

bounded implementation order:
  hakorune-sparse-ownership-surface-task-2026-07-15.md

call-result View sub-DAG:
  hakorune-ownership-v2-anchored-view-return-abi-task-2026-07-15.md

current compiler lane:
  docs/development/current/main/CURRENT_STATE.toml
```

The original long taskboard is preserved in git history. It proposed routine
source `take` and one explicit `clone` for every added Shared owner. Those
requirements are retired and must not be revived from history.

## Accepted replacement law

```text
ordinary whole-root local:
  mutable scoped alias; owner delta 0

ordinary owner movement:
  compiler-verified forwarding; no routine move/take call-site marker

Unique -> Shared with independent owner:
  explicit share
  source root remains usable as Shared
  expression result is an independent Shared owner

inside verified Shared lane:
  compiler manages required CopyOwned / DestroyOwned

mandatory ordinary clone spelling:
  none
```

`take`, `view`, and `share` remain non-default callable-definition contracts as
specified by the normative reference. Do not create the retired proposal
`design/ownership-v2-root-anchored-alias-ssot.md`; it would be a second
authority.

## Preliminary corpus evidence

The historical bounded probe reported:

| Observation | Preliminary count |
| --- | ---: |
| tracked `.hako` / `.nyash` files | 3,294 |
| initialized locals | 31,486 |
| direct field/index projection initializers | 530 (1.68%) |
| mixed-parser MethodCall initializers | 18,353 (about 58%) |
| narrower whole-root syntax candidates | at most about 660 |
| narrower MethodCall results | about 12.7k |
| explicit `.fini()` calls | 2 |
| `fini {}` surfaces | 37 |

These counts used different filters and are not authority. They justify the
machine-generated evidence rows on the current taskboard:

```text
O2-P0a  initialized-local shape census
O2-P0r  exact whole-root eligibility
O2-P0b1 exact callable/signature evidence
O2-P0c  destination / independent-lifetime census
```

Important implications only:

- whole-root aliases look low-risk but may cover a narrow part of the corpus;
- call-result ownership/View ABI is a first-class migration seam;
- projection frequency must be measured before prioritizing a projection-loan
  surface;
- the two fini counts do not prove the alias/fini rule sound.

## Evidence constraints retained from the old board

The replacement machine reports must:

1. record input hashes, parser/profile identity, and exact scan boundaries;
2. distinguish whole root, projection, rvalue, call result, and unknown;
3. distinguish noescape use, one-owner forward, independent lifetime, and
   unknown destination;
4. count parse/resolution failures instead of dropping them;
5. reconcile totals and preserve deterministic ordering;
6. stay read-only and change no parser/Builder/runtime behavior;
7. never infer owner policy from names, runtime tags, reference counts, or
   generic `escape=true` observations.

## Historical concepts still useful as non-authority vocabulary

The old analysis separated these questions correctly:

```text
Binding identity:
  which BindingRef currently names a value

Loan permission:
  which root may not be invalidated before alias last-use

Owner availability:
  whether one token can be forwarded at a site

Independent lifetime:
  whether the explicit Shared boundary is required

Materialization:
  ValueId / CopyOwned / DestroyOwned / physical RC strategy
```

The current reference/taskboard retain that separation. Old spellings, row
dependencies, counters tied to explicit clone, and the former O2-D0
constitution-freeze row are retired.

## Nonclaims

This evidence index does not claim:

```text
current local assignment is a ScopedAlias
share/take/view syntax is parser-live
Loan Flow or Ownership SSA is production-active for Box
SharedV1 is retired
projection aliases, alias PHIs, or cross-task aliases are supported
current Arc representation has changed
the current D-prime lane has switched
```

## Next action

Do not add more prose here. When the ownership lane is explicitly selected,
materialize O2-P0a as the first machine-readable artifact named by the current
sparse ownership taskboard.
