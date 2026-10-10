# MIRBUILDER-STATIC-CALL-ROOT-ENTRY-D0

Status: selected design stop; exact executable entry authority pending
Date: 2026-10-10
Scope: unchanged SizeClassBox published-view first stop after direct Static Call terminal retention
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/investigations/mirbuilder-root-exit-source-missing-d0-2026-10-10.md
  - src/mir/normal_callable_semantic_package/ordinary_new_local_commit/root_call_entry/validation.rs
  - lang/src/hako_alloc/memory/size_class_box.hako

## Observed first stop

With the original `size_class_box.hako` and protected Eq WIP in a detached
diagnostic checkout, the published-view probe moved from
`root-exit-source-missing` to `ordinary-new/local-commit/call-entry-missing`
after the direct Static Call relation was retained. Log:
`/tmp/hako-static-call-terminal-integration.log`. This is still red, and the
whole source and mimalloc-lite EXE have not passed.

`validate_root_call_entry` returns this refusal when it has a Call source but
the recorded root exit entry is `Plain`, with a legacy terminal source and no
instance-call exception. That is a source/executable-entry mismatch, not
permission to infer an invoke from the terminal relation. The prior exact
missing relation was `SizeClassBox.bin_size_usize/1` at `[Body(0)]`; this new
refusal likely concerns its `me.bin_size(bin)` return, but the generic error
alone does not prove the current owner and exit.

## Next read-only decision work

1. Identify the exact owner/exit and the existing root Call entry issuer,
   including its source, ordered actual, invoke/projection and Completion
   requirements. Compare the selected CurrentOwner Static path with existing
   Lexical and qualified Static paths.
2. Locate the canonical issuer for the executable Static call entry and the
   physical packet. Determine whether the same checked scalar cohort can be
   borrowed or a separate exact site proof is needed. Do not make a Plain
   entry executable by reclassifying it from a terminal spelling.
3. Record one bounded Decision and focused positive/negative acceptance
   before construction. Keep independent physical and final MIR checks.

The previous S0 was closed at the retained source relation boundary; the new
entry and whole-source publication are separate responsibilities.
