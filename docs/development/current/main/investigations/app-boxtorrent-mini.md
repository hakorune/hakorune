# APP-boxtorrent-mini

Status: selected; canonical EXE incomplete. Date: 2026-10-27
Scope: unchanged boxtorrent-mini, canonical EXE, expected output and exit.
Refs: [RULES](../../../RULES.md), [CURRENT_STATE](../CURRENT_STATE.toml), [Map source contract](../../../../reference/runtime/runtime-data-dispatch.md).

## Current stop
- `HakoAllocHeap.release` owner38: `borrowed-entry/original-input-not-ready`; the input cohort is incomplete and no EXE is produced. `chunk.alloc_handle` comes from a guarded Map get, after `chunk.release()` and before `me.chunks.delete(cid)`.
- The existing source seed retains the exact get and complete field-writer census, not a live Home, nonnull value, or executable read. A source-proved class does not grant execution.
- Restored CLI: `cargo quick-build-fast -j 4` PASS in43s; selected smoke with `NYASH_BIN=target/quick/hakorune NYASH_EMIT_EXE_NYRT=target/quick` retains the stop. The last fixed suite on the earlier landed tree was9/11: 8 real EXEs plus empty marker; JSON and boxtorrent failed. A later prototype suite also reported9/11 but does not verify the restored tree.
- Prior broad focused checks had baseline failures in source-only declared cohorts, PageHeap required-integer source, JSON callee-exit, and separate physical/Text and terminal cases. The `borrowed_formal_entry` filter had63 PASS/3 FAIL with the same three failures before the provisional source check. The guarded parameter-field negative and returned-field source-input refusals remain unresolved.
- Other unresolved checks: `source_graph_instance_forward_keeps_complete_original_source_sum` (`no executable use draft`), Map field-receiver negative (`incoming-stored-pending`), and two provider cases (foreign-shape rejection and owned-array cleanup count); each reproduced before the relevant source changes. The guarded parameter-field negative rejected earlier at source-only entry and was not baseline-measured.

## Required connection
- Whole-function Map-get isolation currently rejects `chunk.release()` itself. Keep source identity separate from permission: use the original get, exact receiver/read, key and surviving nonnull guard; prevent generic Bool/argument paths from treating unproved Map payloads as ordinary handles.
- The prior `chunk.release()` is a DirectMapGet receiver with i64 result. Its original target exists, but the physical borrower handles only an OrdinaryCall receiver. Prove numeric-only effects from the complete writer sources (the existing Unit check also permits Text), exact Call/Normal/Fault cleanup, and slot liveness through the final field argument.
- A temporary opt-in trace confirmed this exact DirectMapGet call's scalar source and zero-argument projection reach Ready. This is source evidence only; Map frame, nonnull, slot liveness and physical Call/cleanup remain unproved. Trace code was removed and CLI rebuilt PASS.
- Connect the same Map-derived nullable object field to the existing field-read/ReadActual owners. Retain exact call, ordinal, receiver, get and writer; verify ObjectFieldBorrow, its Normal/Fault path, and the final outer Call operand. Parent nonnull does not prove the child nonnull; do not manufacture ReturnedField provenance or an owned Home.
- The existing expression-path owner rejects a direct take; the current scalar expression path cannot be reused for an object result without changing its value contract. Preserve its source identity and final independent validation.

## Removed experiments and verification limits
- Provisional source-only rechecks, scalar-receiver gate, field-path recheck and physical Map interval were removed: they did not connect the real consumer and could reject valid calls. The interval prototype's focused CFG test passed and its fixed suite was9/11 in617s, but no source→Finished positive reached it; those results do not verify the restored tree. Avoid reintroducing standalone proof scaffolding.
- Existing source-backed Map consumer tests cover a different route. The JSON source-scope and Map field-receiver negative tests fail before their target assertions; the recorded failures reproduce without the removed WIP. Do not count them as positive or negative coverage of this connection.
- The VM `--emit-mir-json` invocation on this app with plugins disabled stops at `birth-global-legacy-stopped`; it is not an available oracle under those conditions.
- Previous landed changes and detailed results remain in Git history. JSON, required production connections, R6/R7, Loop and finite-product closeout remain incomplete.
