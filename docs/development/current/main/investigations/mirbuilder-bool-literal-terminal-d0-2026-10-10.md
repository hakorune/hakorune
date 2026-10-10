# Bool literal terminal result D0

Status: selected design stop
Date: 2026-10-10
Scope: MIRBUILDER-BOOL-LITERAL-TERMINAL-D0
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/investigations/mirbuilder-static-currentowner-i64-eq-s0-2026-10-10.md
  - src/mir/resolved_semantics/home_new_prefix_terminal.rs
  - src/mir/normal_callable_semantic_package/result_contract.rs

## Trigger and boundary

The unchanged `SizeClassBox.accepts(size)` has two Bool literal exits:
`return false` inside the If and `return true` after it. A focused
published-view probe with the original `size_class_box.hako` and a trivial
Main reached `SizeClassBox.accepts/1` and stopped at
`[ordinary-new/local-commit/root-exit-source-missing]`. A temporary
diagnostic identified the function and was removed. The test is uncommitted
and red; Eq S0 remains unfinished.

The Home terminal scanner recognizes Bool literals as trivial, but its
`Some(_) => true` arm retains no `TerminalRelationV1`. The child callable
retention and `normal_exit_projection_v1` therefore have no exact exit row.
`root_home` correctly refuses missing source. `ExactTrivialScalarAbiV1`
currently has only I64; the unannotated result contract has no Bool physical
header. A relation-only patch cannot claim the original physical Return.

## Decision to close before construction

Source authority: resolved literal at each exact return value site,
Completion explicit exit sites, and original Home exit flow. Canonical
terminal issuer: `home_new_prefix_terminal`, retained through the existing
terminal relation index. The child projection and physical header must borrow
that result; neither may infer Bool from the emitted ValueId or AST spelling.

Choose the narrow Bool result representation and source-to-physical
correspondence for unannotated `accepts`, including:

1. one Bool-literal relation per original exit, with value, owner and site;
2. complete child retention and Normal/Fault root cleanup for both exits;
3. result contract/header or an existing equivalent physical authority for
   Bool, without treating an unannotated declaration as an arbitrary ABI;
4. one ordinary physical Return path and final MIR/backend verification.

The selected implementation must reject a missing, swapped, duplicated or
non-Bool terminal, conflicting result class, wrong owner/site, and a Return
whose physical value does not match the retained source relation. Preserve
distinct failure positions. Reuse existing return emitter and cleanup owner;
do not add a fallback or rewrite the `.hako` source.

Smallest follow-up: close this Decision with the exact existing result and
physical owners, then select one `MIRBUILDER-BOOL-LITERAL-TERMINAL-S0` slice
covering the two original `accepts` exits. The Eq S0 card retains its
uncompleted ordered Static packet/Compare/Branch acceptance and resumes only
after Bool terminal publication is verified. The whole mimalloc-lite app's
recorded first stop remains Heap-to-Page.
