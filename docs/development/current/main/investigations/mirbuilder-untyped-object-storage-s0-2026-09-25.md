# MIRBUILDER-UNTYPED-OBJECT-STORAGE-S0

Status: landed__2026-09-25
Date: 2026-09-25
Parent: MIRBUILDER-UNTYPED-OBJECT-STORAGE-D0 (accepted)
Owner: workstream row H / unified resume gate 1
Authority: final-pipeline-ssot acceptance recheck branch (a)
  (source-backed storage migrated to explicit supported type) +
  MIRBUILDER-INVOKE-LIFECYCLE-PHYSICAL-V2-CUTOVER-I0 (landed
  4a96073343) — lifecycle probes run on `--emit-exe`, not the retired
  `selfhost_build.sh --mir` route.

## Slice

One app + one smoke script:

1. `apps/typed-object-birth-param-min/main.hako`:
   `init { page_id, capacity }` -> declared `page_id: i64` /
   `capacity: i64` members; `sum()` -> `sum(): i64` (declared result
   row required by the root-call contract). Update the header comment
   to describe declared storage (drop the inference claim).
2. `tools/smokes/v2/profiles/integration/apps/typed_object_birth_param_min_exe.sh`:
   move from `selfhost_build.sh --mir` + `ny-llvmc` to the
   `--backend mir --emit-exe` lifecycle caller shape used by
   `typed_object_method_min_exe.sh` (V4 trace + llvm-c-api toolchain
   pins + runtime archive + run + exit-30 assertion). Contract pin
   text updated to declared-storage wording.

## Fail-fast boundary

Unchanged named stops: `root-call-entry-missing`,
`artifact-source-unavailable`, `FieldType` destruction, MIR-JSON
`Invoke` rejection (the retired route stays rejected — that is the
contract, the probe just stops using it as an artifact producer).

## Verification

- Direct:
  `bash tools/smokes/v2/profiles/integration/apps/typed_object_birth_param_min_exe.sh`
  — expected PASS with `stage=lifecycle-v4-measure result=ok` +
  `toolchain=llvm-c-api` + exit 30.
- Suite rerun: `real-apps-exe-boundary` expected 4 pass / 7 fail;
  no other entry's terminal must change (binary_trees etc. untouched).
- `git diff --check`; pointer guard.

## Evidence

- `typed_object_birth_param_min_exe` direct run: PASS (lifecycle V4
  trace `result=ok`, `toolchain=llvm-c-api`, exit 30).
- Suite rerun `real-apps-exe-boundary` (quick binary): **4 pass /
  7 fail** — birth_param_min flipped green; all other terminals
  unchanged (untyped_field panic, binary_trees birth stop, usize x2,
  NamedArray, selected-header-missing, newbox toolchain).
- `git diff --check` clean; pointer guard green.

## Exit

- [x] birth_param_min green on the lifecycle route (exit 30).
- [x] Suite receipt recorded; other terminals unchanged.
- [x] Card/pointer/workstream synced; commit+push.
