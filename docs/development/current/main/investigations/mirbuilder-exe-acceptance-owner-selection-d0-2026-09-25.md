# MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D0 — pick one EXE fail class

Status: open__census__2026-09-25
Date: 2026-09-25
Parent: MIRBUILDER-UNIFIED-SELFHOST-RESUME-D0 (accepted 2026-09-25)
Owner: workstream row H / unified resume order gate 1
  (docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md:49)
Authority: docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
             acceptance table (:1509-1521)

## Question

The `real-apps-exe-boundary` suite last recorded 2 pass / 9 fail, each
red a typed fail-fast terminal with an owner class already mapped.
Select exactly ONE fail class whose (source authority, canonical
issuer, live production caller, fail-fast terminal, exclusive
delete-set, acceptance) tuple can close — i.e. the class's fix lands
the missing authority path AND retires the old edge, so the suite
entry flips green on a rerun.

## Census boundary

- Start: fresh `real-apps-exe-boundary` rerun receipt at current HEAD
  (`tools/smokes/v2/run.sh --profile integration --owner-profile
  integration --suite real-apps-exe-boundary`).
- End: one selected class with a complete tuple + one bounded
  execution card (I0/S0).
- Includes: the four named owner classes (exact-usize parameter
  contract, callable-loop-handoff, untyped-storage ordinary-new
  commit, selfhost emit lane residuals).
- Excludes: gates 2-4 (parked/queued), the retired MapStore queue,
  baseline cargo-lib debt (manifests).

## Evidence (fresh receipt + worker 80d0de41 audit, spot-checked)

Fresh HEAD rerun of `real-apps-exe-boundary`: **2 pass / 9 fail**
(same as recorded). Class map:

| class | entries | verdict |
|---|---|---|
| `qualified-preflight` `New` in initializer | method_min, birth_param_min | route-scope regression — see Decision |
| `ParameterContract` `UnsupportedDeclaredType` (`usize` params in cataloged `selfhost.hako_alloc` decls, not called) | boxtorrent, mimalloc_lite | needs-design-authority (usize is a distinct exact type; multi-owner BoxCount) |
| `callable-loop/facts-absent` (method-call loop bodies) | binary_trees | needs-design-authority (no Facts family) |
| `NamedArray(TextSourceMissing)` | allocator_stress | needs-design-authority (semantic boundary decision) |
| `unsupported terminator Invoke` | untyped_field_min | needs-design-authority (Invoke D0 excludes generic emit; untyped-storage parked) |
| `no_lowering_variant` + mem2reg opaque-ptr | newbox_min | environment/toolchain debt (resolved `opt` too old) |
| `main-import-view/selected-header-missing` | json_stream | needs-design-authority (non-i64 result ABI widening) |

The qualified-preflight class is a **route-scope regression**:
`main_root.rs:223` diverts any app-main containing ANY method call —
including lexical `pair.sum()` — into the canonical qualified-methods
route whose trivial-SSA grammar excludes `New`. The route's own
design scope is qualified receivers only
(`mir-call-d1b-main-raw-qualified-method-acceptance-d0-2026-09-21.md`),
and lexical `Main.main -> pair.sum()` is assigned to the
ordinary_new/lifecycle path by `MIRBUILDER-INVOKE-LIFECYCLE-ROOT-METHOD-CALL-D1`.
method_min passed `--emit-exe` at `4a96073343`.

## Six-line Decision (accepted 2026-09-25)

```text
Decision: select the qualified-preflight route-scope class; fix the
          diversion predicate at main_root.rs:223 to engage the
          canonical qualified-methods route only when at least one
          method call has a QualifiedUnbound receiver — lexical-only
          mains return to inner.lower_body (their design-sanctioned
          owner).
Source authority + canonical issuer:
          VerifiedResolvedMethodCallSourceV1::receiver() ->
          ResolvedMethodCallReceiverSourceV1::QualifiedUnbound
          (resolver-sealed; no new Facts/Recipe).
Non-authority: the qualified recipe port — it structurally cannot
          serve lexical receivers (qualified relation issues no rows;
          lowerer recipe_missing).
Fail-fast boundary: capability.rs:734 + the main_root wrap stay
          unchanged for genuinely out-of-scope shapes; mixed mains
          still route canonical and fail fast on the lexical call.
Smallest next slice:
          MIRBUILDER-EXE-ACCEPTANCE-QUALIFIED-PREFLIGHT-ROUTE-SCOPE-S0 —
          predicate narrowing at main_root.rs:223 + regression pin.
Non-claims: does not admit New into the qualified grammar; does not
          fix birth_param_min (it moves to its real untyped-storage
          boundary — correct); no usize contract, loop facts,
          NamedArray, Invoke, or main-import-view changes.
```

## Exit

- [x] One EXE fail class selected with complete tuple —
      qualified-preflight route scope; next card
      `MIRBUILDER-EXE-ACCEPTANCE-QUALIFIED-PREFLIGHT-ROUTE-SCOPE-S0`.
- [x] Guard/pointer/workstream synced.

## Post-S0 receipt (2026-09-25, quick binary)

S0 landed. `real-apps-exe-boundary` rerun: **3 pass / 8 fail**
(method_min now green). Updated terminal map:

| entry | terminal | class |
|---|---|---|
| typed_object_method_min | PASS | S0 fixed |
| typed_object_birth_min | PASS | — |
| real_apps_exe_boundary_probe | PASS | — |
| typed_object_birth_param_min | `ordinary-new/local-commit/root-call-entry-missing` | moved boundary: real untyped-storage/ordinary-new local-commit boundary (predicted) |
| binary_trees | `ordinary-new/birth-global-legacy-stopped` | moved boundary: `Main.main` = lexical `bench.run()` now on ordinary-new lane; `new BinaryTreesBench()` -> `BinaryTreesBench.birth/0` global call hits the S1 named stop before the loop-Facts gap. Was `callable-loop/facts-absent` (both terms are named stops; app remains red) |
| typed_object_untyped_field_min | phase84-5 panic `Type inference failed for ValueId(15)` | untyped storage / Invoke family — needs owner audit |
| boxtorrent_mini | `ParameterContract UnsupportedDeclaredType` (`usize` decls) | parameter ABI contract — needs design authority |
| mimalloc_lite | same as boxtorrent_mini | same |
| allocator_stress | `NamedArray(TextSourceMissing)` | semantic boundary — needs design authority |
| json_stream_aggregator | `main-import-view/selected-header-missing` | non-i64 qualified result ABI widening |
| typed_object_newbox_min | `no_lowering_variant` (+ stale `opt` mem2reg) | environment/toolchain debt |

Next selection round: `MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D1`
picks one bounded class from this map; ordinary-new lane blockers now
cover two entries (birth_param, binary_trees) and are the highest-weight
candidate family.
