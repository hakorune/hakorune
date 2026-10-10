# MIRBUILDER-SEARCH-LOOP-EXIT-RELATION-D0

Status: selected design stop
Date: 2026-10-11
Scope: Normal exit source for the two ready returns of the original `SizeClassBox.size_to_bin`
Related: `mirbuilder-search-loop-early-return-home-s0-2026-10-11.md`

## Observed frontier

The unchanged `lang/src/hako_alloc/memory/size_class_box.hako` with a trivial
Main now crosses the selected Home and Static packet transport. Its physical
input stops at
`[freeze:contract][ordinary-new/local-commit/root-exit-source-missing]`
(`/tmp/hako-search-loop-home-s0-transport-physical-20261011.log`). This is
not a whole-source or EXE pass.

`validate_root_home_exit` requires a `normal_exit_projection_v1` for every
ready explicit exit. That projection requires a retained terminal relation.
`ordinary_new_coseal_issue.rs` currently filters child terminal relations
through `retain_child_terminal_relation`, with `I64Scalar` retained only when
`has_integer_return` is true. This may explain the new stop; it is a
read-only hypothesis, not a chosen implementation.

## Decision required before implementation

- Identify the exact missing exit and its original terminal relation and
  disposition; compare the before-filter and retained rows.
- Name the source authority and canonical issuer for an I64 local return at
  both early and tail exits. Determine whether the existing result-contract
  cohort already proves the class or a bounded source proof is missing.
- Keep unrelated child returns and unproved local I64 returns fail-closed.
  Do not retain all `I64Scalar` rows or infer a public contract from callers.
- Choose one bounded implementation slice with positive original-source
  first-stop and negative changed-class/exit evidence. Use a read-only worker
  before selecting the Decision.

Non-claims: no Home change, packet issuer change, generic search-loop support,
or mimalloc-lite EXE completion is selected here.
