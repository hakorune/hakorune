---
Status: closed
Date: 2026-08-09
Row: `GENERAL-STATIC-CALL-RESULT-PUBLICATION-I0`
Parent: `HAKO-PARSER-RICH-BODY-S0-DEPENDENCY-CARRIER-D0`
Mode: BoxShape / existing owner connection
---

# GENERAL-STATIC-CALL-RESULT-PUBLICATION-I0

## Change

Project exact general `SameModuleStatic` call-result rows into the existing
`VerifiedStaticCallResultPublicationOwnerV1`. Delete the selected-row-to-None
gap atomically; do not create another result proof or publisher.

## Contract

The canonical body/result proof and exact source target remain semantic
authority. One exact caller/site/target handoff is consumed once, one physical
Call receipt precedes publication, and the existing local Copy only propagates
the published type. Selected missing/duplicate/foreign/target-mismatch rows
freeze instead of falling through. Ordinary unselected calls retain their
existing terminal.

## Done

The actual `StringHelpers.int_to_str/1` initializer call publishes Integer to
the call result and local `v`, and its first GenericLoop passes the existing
carrier verifier. Focused tests cover general-row selection, no double row,
single consumption, mismatch rejection, post-success-only publication, and
the clean imported `sh_core` dependency canary.

Update `src/mir/callable_result_representation/README.md`, the owning Builder
README/reference receipt, and current pointers in the same implementation
commit.

## Stop

Stop and return to design if the actual source row lacks an exact target or
ExactI64 disposition, if the repair needs a name lookup/source annotation,
or if it requires GenericLoop/local/Completion inference, retry, fallback,
or a second publication owner.

## Closeout receipt

The general exact row now enters the existing publication owner and is
consumed by a typed `Selected` disposition. `Unselected` is reserved for
ordinary calls with no exact result row; target mismatch, foreign/missing
owner, and second consumption are terminal contract errors. The selected
physical Call disables legacy signature annotation, then the existing
source-bound publisher performs the sole post-success Integer write.

The actual unmodified `lang/src/shared/common/string_helpers.hako` lifecycle
canary is green through `StringHelpers.int_to_str/1` and its first Loop. The
focused general-row, bounded-row, mismatch, single-consumption, unselected,
physical-receipt, post-success publication, and lifecycle tests are green.
No `.hako` annotation or GenericLoop/local/Completion inference was added.
