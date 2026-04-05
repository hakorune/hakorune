# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-04-05
Scope: repo root から current lane / next lane / restart read order に最短で戻るための薄い anchor。

## Purpose

- root から current lane と current front を最短で読む
- 長い landed history や implementation detail は phase docs を正本にする
- `CURRENT_TASK.md` は pointer に徹し、ledger にはしない

## Quick Restart Pointer

1. `docs/development/current/main/05-Restart-Quick-Resume.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `git status -sb`
4. `tools/checks/dev_gate.sh quick`

## Order At A Glance

1. `phase-79x launcher emit_mir residual blocker follow-up` (landed)
2. `phase-80x root/current pointer thinning` (landed)
3. `phase-81x caller-zero archive rerun` (landed)
4. `phase-82x next source lane selection` (landed)
5. `phase-83x selfhost top-level facade/archive decision` (landed)
6. `phase-84x runner wrapper/source contract thinning` (landed)
7. `phase-85x next source lane selection` (landed)
8. `phase-86x phase index / current mirror hygiene` (landed)
9. `phase-87x embedded snapshot / wrapper repoint rerun` (landed)
10. `phase-88x archive/deletion rerun` (landed)
11. `phase-89x next source lane selection` (landed)
12. `phase-90x current-doc/design stale surface hygiene` (landed)
13. `phase-91x top-level .hako wrapper policy review` (landed)
14. `phase-92x selfhost proof/compat caller rerun` (landed)
15. `phase-93x archive-later engineering helper sweep` (landed)
16. `phase-94 escape route P5b “完全E2E” のための ch 再代入対応` (landed)
17. `phase-95 json_loader escape loop E2E lock` (landed)
18. `phase-96 MiniJsonLoader next_non_ws loop E2E lock` (landed)
19. `phase-97 LLVM EXE parity for MiniJsonLoader fixtures` (landed)
20. `phase-98 Plugin loader fail-fast + LLVM parityの持続化` (landed)
21. `phase-99 Trim/escape 実コード寄り強化（VM+LLVM EXE）` (landed)
22. `phase-100 Pinned Read-Only Captures` (landed)
23. `phase-102 real-app read_quoted loop regression (VM + LLVM EXE)` (landed)
24. `phase-103 if-only regression baseline（VM + LLVM EXE）` (landed)
25. `phase-104 loop(true) + break-only digits（read_digits 系）` (landed)
26. `phase-105 digit OR-chain LLVM parity regression` (landed)
27. `phase-110x selfhost execution vocabulary SSOT` (landed)
28. `phase-111x selfhost runtime route naming cleanup` (landed)
29. `phase-112x vm-family lane naming hardening` (landed)
30. `phase-113x kernel vs vm-reference cluster wording correction` (landed)
31. `phase-114x execution surface wording closeout` (landed)
32. `phase-115x vm route retirement planning` (landed)
33. `phase-116x execution surface alias pruning` (landed)
34. `phase-117x vm compat/proof env hardening` (landed)
35. `phase-118x proof wrapper surface review` (landed)
36. `phase-119x vm debug/observability surface review` (landed)
37. `phase-120x vm route retirement decision refresh` (landed)
38. `phase-121x vm backend retirement gate decision` (landed)
39. `phase-122x vm compat route exit plan` (landed)
40. `phase-123x proof gate shrink follow-up` (landed)
41. `phase-124x vm public docs/manual demotion` (landed)
42. `phase-125x vm bridge/backend gate follow-up` (landed)
43. `phase-126x vm public gate shrink decision` (landed)
44. `phase-127x compat route raw vm cut prep` (landed)
45. `phase-128x stage1 bridge vm gate softening` (landed)
46. `phase-129x vm orchestrator/public gate follow-up` (landed)
47. `phase-130x vm public gate final cleanup` (landed)
48. `phase-131x vm legacy contract migration` (landed)
49. `phase-132x vm default backend decision` (active)

## Current Front

- Active lane: `phase-132x vm default backend decision`
- Active micro: omitted-backend caller inventory を根拠に default `vm` を keep するか決める
- Current blocker: `src/cli/args.rs` default-vm がまだ legacy default として残っている
- Exact focus: default backend decision は phase-132x で最後に決める

## Successor Corridor

1. `phase-132x vm default backend decision`
2. `phase-kx vm-hako small reference interpreter recut`

## Parked After Optimization

- `phase-kx vm-hako small reference interpreter recut`
  - keep `vm-hako` as reference/conformance only
  - do not promote to product/mainline
  - revisit after the optimization corridor, not before

## Rust-VM Stop Line

- mainline retirement: achieved
- full source retirement: deferred
- residual explicit keep: frozen
- `vm-hako`: reference/conformance keep

## Read Next

1. `docs/development/current/main/05-Restart-Quick-Resume.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/phases/phase-132x/README.md`

## Notes

- `phase-105` fixed pure LLVM string compare lowering in `hako_llvmc_ffi_pure_compile.inc` and restored long digit OR-chain parity on VM and LLVM EXE.
- `phase-112x` hardened vm-family lane naming to `rust-vm-keep / vm-hako-reference / vm-compat-fallback` across source and active observability smokes.
- `phase-111x` made live selfhost runtime smokes and current docs route-first (`--runtime-route mainline|compat`) while keeping `--runtime-mode` as a compatibility alias.
- `phase-110x` fixed long-lived execution vocabulary SSOT and corrected `lang/src/vm` / `tools/selfhost` wording to match it.
- `phase-114x` closed out remaining public/help wording where raw `--backend vm` still looked like a normal execution route.
- `phase-115x` fixed the remaining vm-family route buckets as compat/proof/debug only, ahead of alias pruning or explicit env hardening.
- `phase-116x` removes the naked `stage-a` alias and keeps compat on `runtime-route compat` / `runtime-mode stage-a-compat`.
- `phase-117x` makes compat shell preflight require `NYASH_VM_USE_FALLBACK=1` before entering raw `--backend vm`.
- `phase-118x` narrows the proof wrapper story so public proof surfaces stay small and the rest remain internal engineering helpers.
- `phase-119x` reviews the remaining vm-family debug/observability keep so route observability does not look like a general front-door runtime.
- `phase-120x` refreshes the final retirement order so vm-family remains explicit keep only and the next cut can be source-backed.
- `phase-121x` decides whether `--backend vm` can shrink beyond a public explicit gate, or whether the remaining blockers still keep it public.
- `phase-122x` locked the compat-route exit order: shell surface first, bridge/direct route second, backend gate last.
- `phase-123x` narrowed the remaining proof surface so public proof entry stays small and the rest is explicit engineering keep only.
- `phase-124x` demoted public docs/manual wording so raw `--backend vm` and proof gates stop reading like day-to-day runtime guidance.
- `phase-125x` returned to source blockers and fixed the cut order: shell compat first, direct bridge second, backend gate last.
- `phase-126x` decided that compat smoke contract is the hard blocker and that the next source seam after compat is the `stage1_bridge` backend-hint chain.
- `phase-127x` landed after compat boundary smoke was converted to route-first selfhost contract checks.
- compat temp-MIR handoff is green again because the helper now receives the parser-EXE preference env internally.
- `phase-128x` kept the binary-only direct-route vm gate as an explicit legacy contract while removing backend-hint forwarding from the default child path.
- deeper inventory keeps `src/runner/dispatch.rs`, `src/runner/route_orchestrator.rs`, `src/runner/stage1_bridge/direct_route/mod.rs`, and the legacy compat/proof entry points as the next public-gate seam.
- current inventory buckets are:
  - compat route: `tools/selfhost/run.sh --runtime --runtime-route compat`
  - proof gates: `tools/selfhost/proof/run_stageb_compiler_vm.sh` / `tools/selfhost/proof/selfhost_vm_smoke.sh`
  - active debug/observability: phase29x vm-family route smokes
- `phase-98` locked plugin loader strict/best-effort runtime contract and kept `phase-97` LLVM EXE parity green.
- `phase-103` fixed if-only merge / early return parity on VM and LLVM EXE.
- `phase-102` fixed real-app read_quoted loop parity on VM and LLVM EXE under compat replay=harness.
- `phase-100` fixed pinned read-only captures and locked VM/LLVM proof for accumulator cases.
- `phase-99` fixed trailing-backslash trim/escape parity on both VM and LLVM EXE.
- `phase-97` fixed LLVM EXE parity for `phase95/96` fixtures under `HAKO_BACKEND_COMPAT_REPLAY=harness`.
- post-`phase-102`, execution SSOT cleanup will separate `stage / route / backend override / lane / kernel`.
- planned naming corrections:
  - `runtime-mode exe` -> `runtime-route mainline` family
  - internal VM lanes -> `rust-vm-keep / vm-hako-reference / vm-compat-fallback`
  - `kernel` stays reserved for `nyash_kernel`; `lang/src/vm` is treated as VM/reference cluster
- `launcher.hako emit_mir_mainline` is green.
- `stage1_mainline_smoke.sh` is green.
- thin top-level wrappers remain public/front-door keep, not archive-ready by default.
- `vm-hako` stays reference/conformance keep; future interpreter recut is parked until after optimization work.
- `phase-130x` closed the final wording-only public gate cleanup and left the behavior gate isolated.
- `phase-131x` closed the remaining explicit legacy `vm` contract caller/bridge cuts up through the default-child and route-gate changes; the default-vm decision moved to `phase-132x`.
