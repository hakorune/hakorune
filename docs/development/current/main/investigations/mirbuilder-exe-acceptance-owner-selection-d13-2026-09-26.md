# MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D13

Status: design_stop__2026-09-26
Date: 2026-09-26
Parent: workstream row H (unified resume, gate 1); D12/S0 landed
Mode: design_stop — census and Decision only.

## Blocking observation

```text
EXE (pure-first, debug binary):
  [plan/freeze:contract] loop winner selection declined:
  zero selected family candidates
  fn=StringHelpers.trim/1
  site=SourceStmtSiteV1(SourceNodeSiteV1([Body(2)]))
  cond=BinaryOp{And, Less{i,n}, Or{Equal{substring," "},
        Equal{substring,"\t"}}}

VM (debug binary):
  same decline
  fn=JsonStreamAggregator.ingest/1
  site=[Body(2)]
  cond=BinaryOp{Less, start, n}
```

`StringHelpers.starts_with/3` (return-in-body) now lowers via
the composite LoopBreak owner (S0). The next terminals are
exit-free `loop(cond)` bodies — no `Break`, `Continue`, or
`Return` anywhere under the root.

## Census questions

1. Exact loop statements that decline (trim/ingest confirmed;
   enumerate any other exit-free `loop(cond)` reachable in
   the app compile order).
2. Which lane each function enters — `trim` is ledger-bound
   (callable lane, `RouteNotFrontSelected` expected);
   `ingest` is ledger-less (`lower_non_callable_loop_route_v1`
   → `route_loop` → winner spine).
3. Whether an existing owner can be reached:
   - composite LoopBreak: requires explicit exit evidence —
     `RootExitMissing` for these (fail-fast boundary pinned
     by S0, must not be silently widened).
   - callable sole-family vocabulary: `LoopCondBreakContinue`
     + `LoopTrueBreakContinue` only — no-exit facts extractor
     (`try_extract_loop_cond_no_exit_facts`) accepts
     `Stmt`/`ProgramBlock`/`GeneralIf` items but rejects
     `ConditionalUpdateIf`; is it reachable from either lane
     today, or retired with `route_loop`?
   - winner spine: 5 families — all shape-declined for
     `loop(cond){stmts}` with no exits.
4. Whether `loop(cond){stmts}` exit-free is a genuinely new
   bounded family slice or an existing designed row
   (`LOOP-PHYSICAL-{ALWAYS,IF,EXIT}-COVERAGE-I0` lines were
   recorded closed — audit what was actually landed).
5. Whether `ingest`'s ledger absence (instance method on the
   non-callable port) is itself a separate ownership question
   or shared with the exit-free problem.

## Boundary

design_stop only — census, reject reasons, owner audit, one
bounded S-card or NoSafeSlice. Do not extend
`CallableLoopSoleFamilyV1`, the winner spine, or the
composite ledger admission in this card.
