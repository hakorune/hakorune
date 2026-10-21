# APP-json-stream-aggregator
Status: positive Div/Mod source→Loop→Finished/native connected; JSON Text carrier and final Map/native incomplete. Date: 2026-10-22
Related: [RULES](../../../RULES.md), [current state](../CURRENT_STATE.toml), [untyped-slot D0](mirbuilder-untyped-object-storage-d0-2026-09-25.md), [pipeline](../design/mirbuilder-final-pipeline-ssot.md).

## Product evidence
- Last full suite: 7/11 PASS (EXE6 + marker1), 397.733s, no SKIP/timeout; mimalloc-lite and allocator-stress PASS. Not rerun.
- Focused JSON EXE passes the previous owner-slot-9 `source-use-completion-pending` stop, then fails at `ordinary-new/borrowed-static/source-actual-unavailable`: `StringHelpers.skip_ws` passes `substring` to `is_space`.
- `TEXT-CALL-ACTUAL-ORIGIN-D0` classifies call-result/Substring/temporary Text actuals as `RejectBeforeEffect` until a call-complete lifetime owner exists. Keep this refusal; no Core-receipt bypass, name branch, or `.hako` workaround. Bool C-return ABI stays out of scope.
- `cargo quick-build-fast` PASS (quick profile, 39.25s). Existing focused test was not runnable: lib-test compilation hit 21 API mismatches in other modified fixture files, before executing tests.
- Native Div countdown4 and Div+Mod sum10 exit0; Sub67/Fault70/Text0 unchanged. Filters and typed Loop checks pass; C parser/nonpositive divisor and two Map source refusals remain.
- ExactBool ReturnValue reads now come from retained original source plans. No app/function-name conditions or source-language edits.

## Connected source and lifecycle owners
- FieldResidence retains all Array/Dynamic/Empty fields; reverse declaration drain precedes outer End. No subset proof or scalar fallback.
- Birth retains original source, ConstructionPlan, formals, modes and Fault prefixes. Original parameter/Text-only SUM selects Text ABI, never MIR type or field spelling.
- Shared String writer materializes BorrowedText into a fresh field. Materialize/store Fault cleanup drains fresh and prior values in order; Normal transfers once; Provided stays caller-owned until Normal.
- Callee replay and independent String checker retain original plan/formal/SSA, fresh output, source order/count and Normal-only transfer. Final module, entry, physical ABI and JSON share the checks; native C remains closed.
- NewArgument retains the same New/site/ordinal/evaluation, reads, Calls, acquisitions and prefix. One writer covers the local/result and complete NewBox/Birth stack; Fault reclaims candidate/temporaries before prior Homes.
- Checked Birth keeps ordered Value/Text arrays and full source/cleanup replay, including zero-Call Text. Full-use census rejects duplicate locations; final Birth owner→symbol loan precedes function-local SSA checks.

## Map and Text boundaries
- Whole field-use walk closes declarations, aliases, capture/escape, foreign uses and mutators; unknown effects refuse. `statsFor` hit retains exact has/get/key/true Return; miss retains exact New/set/Return and disjoint Array push.
- Map holder/key remain independent. Native Map is move-only/SafeMutex-only; detached outcomes and Array children end outside locks. Commit publishes before old End with no rollback; full child End is required before install.
- Direct Map get shares the sole Invoke writer and full key/receiver/Homes. Unit receiver needs exact Prefix holder, pure original interval, complete numeric-write census and empty exit Homes. Slot replacement and callee Text-write inputs still refuse.
- Local get retains its Local receipt/Copy, Fault Homes, prefix and Finished capture. Borrowed get never acquires an owning result Home. App get/nonNull proof is unobserved; `statsFor` ingress alone does not prove it.
- SourceInstance selects original source before Completion; Unit grants no receiver, ABI or lifetime. Checked paths require the same committed ExactRead, original If/null frame, sealed Bool Branch and nonNull dominance.

## Current route and completion
- Reopen temporary Text actual-origin only after an explicit design decision and a call-completion lifetime owner covering Normal and Fault. Until then, do not change the imported helper or app source.
- Read-only BorrowedText across a Loop keeps the exact view: no descriptor PHI, Home/End, or materialization. Static/owned lifecycle remains separate.
- Positive literal Div/Mod share the same source law, evaluation census, writer, SSA and Loop schedule; Recipe and Finished/publication/C independently recheck operands and divisor. Initializer/Rebind/Return retain exact DCE roots.
- Remaining work includes other Text purposes and native formal/actual/Materialize/store/End/Fault; Dynamic/Provided/Shared; Array holder/contents/End; empty transfer; signed SUM/rebind/to_i64; generic native Bool and canonical JSON.
- Retained reds: Birth24/1, result49/3, entry64/2, actuals82/4, source33/8, arguments12/1, read-claim7/2; initializer/terminal/callback/plan/admission, brand/composition/scalar reds remain owed.
- App completion requires original smoke, EXE output and exit; goal also requires full source closure, production cutover and physical retirement. Shared safety stays; history is in Git.
