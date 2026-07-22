# CUT0-I0 ROOT0-CANON0 LOWER0 実行タスク

Status: **Closed — LOWER0 draft-only plan consumer and evidence gate passed; RECEIPT0 next**

Related:

- `cut0-i0-root0-canon0-source-binding-execution-task-2026-07-22.md`
- `cut0-i0-root0-canon0-source-binding-consultation-2026-07-22.md`
- `CURRENT_STATE.toml`

## Objective

`SourceBoundCanonicalPackageV1`をby-valueで一度だけconsumeし、exact preflight planを
実lowererへmoveする。SOURCE-BIND0のpackage transportを、plan drop-only scaffoldの
まま残さない。

対象はcanonical four routesのみである。

```text
APlus
BindingSsaTrivial
BindingSsaAcyclic
BindingSsaRecursive
```

Raw、receipt retention、recursive capability receipt、DRAIN0、public ingress、
external commitはこのrowに入れない。

## Current seam inventory

既存のlowering入口は次のとおりで、いずれも現在はpackageをconsumeしていない。

```text
MirBuilder::build_resolved_function_module
MirBuilder::build_resolved_trivial_function_module
MirBuilder::build_acyclic_callable_module_candidate
MirBuilder::build_recursive_callable_module_candidate
```

既存の`canonical_root_completion.rs` scaffoldには、planを`Option`で保持して
`take().expect`後にdropする経路が残っている。この旧scaffoldをproduction consumerへ
昇格させず、LOWER0専用のprivate terminalへ集約する。

## Selected implementation

```text
MirCompiler::bind_canonical_source(exact_plan)
  -> SourceBoundCanonicalPackageV1
  -> private consume_lowering(package)
  -> exact route dispatch
  -> unpublished draft/module product
```

packageは`self` by-valueで受け、内部matchでplanとcontinuationを一度だけ分ける。
第二のsplit API、clone、re-resolve、`current_module`再取得、legacy fallback、retryは
追加しない。

A+/trivialはまずdraft-only lowerer seamを作り、既存のmodule finalizationを直接呼ぶ
経路を避ける。acyclic/recursiveは既存のwhole-batch lowererを再利用できるかを、
source continuationとexact planのownershipを保ったまま確認する。再利用できない場合
は新しい一般transaction箱を増やさず、route-local private adapterへ分ける。

## Acceptance

```text
SourceBoundCanonicalPackageV1 consumer count = 1
plan-consuming lowering terminal count = 1
plan drop-only consumer = 0
Option<Plan>/take().expect in the new path = 0
plan clone/re-resolve/current_module lookup = 0
lowering failure -> typed rejected owner, no retry/fallback
live Builder mutation before successful candidate = 0
receipt retention = 0 (RECEIPT0 owns it)
production canonical capture/drain/finalizer/external commit = 0
all touched source/check files < 800 lines
```

## Required evidence

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
RUSTFLAGS='-Awarnings' cargo check -q --lib
python3 tools/checks/lib/cut0_i0_root0_canon0_source_bind0_guard.py
python3 tools/checks/lib/cut0_i0_root0_canon0_lower0_guard.py
```

LOWER0専用のfocused fixtureは、成功、plan lowering failure、package drop後のlive
Builder不変を別ファイルで固定する。RECEIPT0/CANON-FIXTURE0のreceipt matrixを
先取りしない。

## Stop line

LOWER0完了は「planが実lowererへ一度だけmoveされた」ことだけを意味する。collector
receipt、root completion、recursive marker、drain、finalizer、external commit、
atomic CUT0 activationは後続rowのまま停止する。

## Implementation result

`MirCompiler::lower_canonical_source` now consumes the package by value and
returns an unpublished `CanonicalLoweringCandidateV1`. The package dispatches
all four canonical routes to draft-only seams: A+ uses the resolved function
draft session, trivial uses the existing trivial draft lowerer, and acyclic /
recursive use unpublished callable draft sets. The live Builder is changed
only through the disconnected candidate session; no module preparation,
finalization, receipt, publication, drain, retry, or fallback is connected.

Evidence:

```text
source-bound focused tests: 6 passed
RUSTFLAGS='-Awarnings' cargo check -q --lib: passed
source-bind0 guard: passed
lower0 guard: passed
git diff --check: passed
```

The next executable row is `RECEIPT0`: make collector and exact receipt one
by-value product and retain that receipt in the canonical completion witness.
