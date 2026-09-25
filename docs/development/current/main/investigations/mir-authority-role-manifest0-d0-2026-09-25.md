# MIR-AUTHORITY-ROLE-MANIFEST0-D0 — authority-role census manifest

Parent: `repo-physical-structure-cleanup-ssot.md` step 4 (native .hako
authority migration entry) and parked final-convergence task cells.

## Scope boundary

```text
起点: repository source roots at HEAD
終点: one machine-readable role per in-scope root (classification only)
includes: src/** (src/mir expanded to second-level dirs), lang/**,
          explicitly named compat/bootstrap/tooling roots
excludes: target/, build/, dist/, tmp/, logs/, artifacts/, sessions/,
          docs/ (own docs topology owner), VCS/config dot dirs —
          recorded as exclusions inside the manifest header
```

## Manifest

`docs/development/current/main/design/fixtures/mir-authority-role-manifest-d0-v1.tsv`

165 rows. Columns: `path`, `role`, `owner`, `evidence`.

Role enum: `meaning` | `substrate` | `host/backend` | `oracle` |
`quarantine`.

Counts: 109 meaning, 25 substrate, 14 host/backend, 12 oracle,
5 quarantine (165 total).

- meaning: src semantic roots + all `src/mir/*` second-level dirs +
  lang semantic roots + frontend/mir crates + grammar/, spec/.
- substrate: runtime/ABI/box/transport/plugin surfaces (src abi, bid,
  box_callable, box_factory, boxes, core, debug, messaging, providers,
  ring0, runtime, stdlib, transport, type_abi; lang c-abi, externs,
  runtime, vm; include/, plugins/, nyash_* crates).
- host/backend: codegen/entry/config/tooling surfaces (src backend,
  bin, cli, config, host_providers, llvm_py, runner; lang llvm_ir,
  runner, tools, build; engines/, backend crates).
- oracle: tests, benchmarks, examples, dev harnesses, check tooling
  (src/tests, src/benchmarks, src/wasm_test; tests/, local_tests/,
  apps/, benchmarks/, examples/, dev/, hello_windows/, wasm_demo/,
  tools/).
- quarantine: compat/bootstrap/generated roots with explicit identity
  (src/stage1, lang/src/compat, lang/generated, archive/, projects/).

## Rules recorded in the manifest

- Every in-scope root has exactly one role; unnamed middle ownership
  is zero by construction (coverage verified: all `src/*/`,
  `src/mir/*/`, `lang/*`, `lang/src/*/`, `crates/*/` dirs have a row).
- Unknown external callers remain a non-claim, never an excuse for a
  compatibility edge; no row implies a default fallback.

## Constraints honored

Classification only — no source route, resolver, lowering, or runtime
behavior changed. `reference_delta = 0`.

## Next

`MIR-LEGACY-JOINMODULE-DISPOSITION0-D0` (parked cell), then
`REPO-FINAL-CONVERGENCE-AUDIT0-G0` links this manifest for audit item 3
(authority roles).
