# Phase 82x SSOT

## Intent

`82x` selects the next source lane after `81x` confirmed there are still no true archive-ready wrappers.

## Facts to Keep Stable

- `81x` landed with a no-op archive sweep.
- top-level selfhost wrappers remain explicit façade or proof surfaces, not archive payload.
- top-level `.hako` wrappers remain compatibility surfaces, not archive payload.
- `src/runner/modes/mod.rs` remains a compatibility re-export surface.
- current pointers are thin enough again after `80x`.
- worker rerun found a narrower pressure point:
  - several top-level selfhost wrappers now have zero repo-internal callers
  - `lang/src/runner/runner_facade.hako` no longer has meaningful live source callers outside the embedded snapshot contract
  - whether those should move is now a **policy / façade** decision, not a fact-finding gap

## Candidate Ranking

1. `phase-83x selfhost top-level facade/archive decision`
   - target: `tools/selfhost/build_stage1.sh`, `run_stage1_cli.sh`, `run_stageb_compiler_vm.sh`, `stage1_mainline_smoke.sh`, `selfhost_smoke.sh`, `selfhost_vm_smoke.sh`, `bootstrap_selfhost_smoke.sh`, `selfhost_stage3_accept_smoke.sh`
   - question: keep these as public/top-level façades or archive the true caller-zero subset
2. `phase-84x runner wrapper/source contract thinning`
   - target: `lang/src/runner/runner_facade.hako` and remaining top-level wrapper pressure / snapshot coupling
3. `phase-85x phase index / current mirror hygiene`
   - target: stale registry surfaces such as `phases/README.md`

## Acceptance

1. the next lane is selected once
2. the selected lane is ranked against at least two alternatives
3. the closeout hands off cleanly to the chosen successor
