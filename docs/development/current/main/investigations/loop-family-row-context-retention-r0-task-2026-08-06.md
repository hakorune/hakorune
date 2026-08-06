---
Status: Landed caller-zero BoxShape refactor / assembler next
Date: 2026-08-06
Decision: FAMILY-ROW-CONTEXT-RETENTION-R0
Authority: docs/development/current/main/design/loop-family-observation-policy-ssot.md
---

# Loop-family row context-retention R0

## Purpose

The common admission assembler must co-seal five already-issued family rows.
That requires every row, including `Declined`, `Unresolved`, and `Rejected`,
to retain the identity, mode, and coverage evidence that produced it. The
current observer enums retained context only inside `Candidate`; the R0
refactor now closes that D0 contract without changing accepted source shapes.

This is a behavior-neutral BoxShape refactor. It did not add an accepted
source shape, a selector, a Recipe/JoinSig product, a Builder/MIR caller, or a
production route.

## Decision

Each of the five family observers keeps a move-only evidence envelope for all
four dispositions:

```text
FamilyObservationEvidence {
  expected context: owner/origin/source-kind/site/frame + mode + coverage
  observed source: owner/origin/source-kind/site/frame + mode + coverage
}

Candidate { observation payload, evidence }
Declined { reason, evidence }
Unresolved { reason, evidence }
Rejected { reason, evidence }
```

The expected context is the sealed observer context. The observed source is
the source-attempt identity/mode/coverage moved out exactly once. A foreign,
mode-mismatched, incomplete, or unsealed attempt therefore retains both sides
of the mismatch instead of losing provenance. Candidate payloads retain their
existing typed product; the envelope is not a second source authority.

The five family-specific envelopes remain typed. No `Box<dyn Trait>`, string
family tag, clone, source relookup, AST reconstruction, or retry is allowed.
The later common assembler may project these typed envelopes into its own
five-tag move-only row enum, but it must not rerun an observer or infer a
missing context.

## Acceptance matrix

| Case | Required evidence |
| --- | --- |
| Candidate | typed candidate payload plus expected/observed identity, mode, coverage |
| Declined | typed decline reason plus the same evidence envelope |
| Unresolved | typed unresolved reason plus the same evidence envelope |
| Rejected | typed reject reason plus expected and observed mismatch evidence |
| foreign owner/frame | both expected and observed identity survive |
| mode mismatch/unsealed | expected and observed mode survive, including `None` |
| incomplete coverage | expected and observed coverage survive |

## Implementation boundary

Only the five caller-zero observer modules, their focused tests, and the
shared guard change in the implementation slice.
Use the existing shared family-observer guard; do not add a large row-specific
guard. The guard must reject bare reason-only `Declined/Unresolved/Rejected`
constructors in the five observer modules and keep every file below 800 lines.

Reference documents and current mirrors are updated in the same commit. The
implementation receipt states that all five dispositions retain evidence and
that the common assembler is now the next open row.

## Non-claims

```text
common admission assembly = 0
selector / overlap policy  = 0
Recipe / JoinSig / BindingKey = 0
Builder / MIR / physical route = 0
production caller = 0
retry / fallback / legacy deletion = 0
```

## Exit criteria

```text
five observer enums carry evidence on all four variants
candidate APIs remain source-compatible where practical
focused C/D/U/R tests cover evidence retention and mismatch pairs
shared caller-zero/line guard is green
current pointer names the common admission assembler next
reference docs and READMEs record the prerequisite receipt; only then does the
common admission assembler become the next open row
```

## Implementation receipt

R0 is landed as a caller-zero, behavior-neutral BoxShape refactor. All five
family observers now decompose each source attempt exactly once before any
early-return validation and attach a typed evidence envelope to every
`Candidate`, `Declined`, `Unresolved`, and `Rejected` disposition. The envelope
retains expected and observed identity, mode, and coverage; the outer family
observation exposes one lossless `evidence()` accessor for the future
assembler. No clone, source relookup, AST reconstruction, retry, or fallback
was added.

The focused observation suite is green (`89 passed`), the shared
`mirbuilder_inplace_replacement_guard.sh` is green, all changed observer files
remain below 800 lines, and `git diff --check` is clean. The common admission
assembler is the next open row; selector, Recipe, Builder/MIR, production, and
legacy retirement remain closed.
