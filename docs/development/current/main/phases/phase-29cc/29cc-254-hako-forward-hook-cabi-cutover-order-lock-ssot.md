---
Status: Active
Decision: accepted
Date: 2026-03-01
Scope: `nyrt_hako_register_*`/`nyrt_hako_try_*` を Core C ABI 正本に固定し、`nyrt.hako.register_*` は互換aliasとして維持しつつ source-zero の最終撤去順を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-29cc/29cc-220-runtime-source-zero-cutover-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-253-source-zero-static-link-boundary-lock-ssot.md
  - docs/reference/abi/nyrt_host_surface_v0.md
  - include/nyrt.h
  - lang/c-abi/shims/hako_kernel.c
---

# 29cc-254 Hako Forward Hook C ABI Cutover Order Lock

## Purpose

`crates/nyash_kernel/src/hako_forward.rs` は `.hako` への移行橋として有効だが、  
source-zero の最終形では「関数ポインタ保持/登録面」も C ABI 正本へ移す。

この lock は、撤去を急がず `no-delete-first` のまま順序だけを固定する。

## Fixed Order (HFK-min*)

1. HFK-min1: ABI surface docs lock
   - `docs/reference/abi/nyrt_host_surface_v0.md` に `nyrt_hako_register_*` を planned extension として明記
   - contract を先に固定（登録/未登録時挙動、fail-fast、ownership）

2. HFK-min2: Header + C shim contract lock
   - `include/nyrt.h` に `nyrt_hako_register_*` 宣言を追加
   - `lang/c-abi/shims/hako_kernel.c` にレジストリ保持面を追加（Rustと同形）
   - Rust `hako_forward` はこの段階では維持（dual-route）

3. HFK-min3: Kernel entry wiring cutover
   - `plugin/invoke/by_name.rs` / `plugin/future.rs` / `exports/string.rs` は C shim registry を正本に切替
   - Rust `hako_forward` はトランポリン層へ縮退（登録実体を持たない）

4. HFK-min4: Gate + portability confirmation
   - `tools/checks/dev_gate.sh runtime-exec-zero` green
   - `tools/checks/dev_gate.sh portability` green
   - GitHub Actions（linux/windows/mac）で hook 経路が崩れていないことを確認

5. HFK-min5: Rust hook retirement readiness
   - `hako_forward.rs` のレジストリ状態保持を削除可能化
   - 削除は別 lock（Deletion Gate）で実施

## Contracts

1. symbol surface は互換維持（既存 `nyash.*` は維持）
2. 未登録時は現行Rust経路へフォールスルー（互換維持）
3. mainline 既定は fail-fast、silent fallback 追加は禁止
4. Rust source の物理削除は本 lock の対象外

## Acceptance

1. docs / header / shim / runtime route が同じ順序で同期されている
2. `runtime-exec-zero` が緑
3. `portability` が緑
4. `CURRENT_TASK.md` / `10-Now.md` / `phase-29cc/README.md` に本 lock が参照される

## Execution Update (2026-03-01)

1. HFK-min1 done:
   - `docs/reference/abi/nyrt_host_surface_v0.md` に `nyrt_hako_register_*` / `nyrt_hako_try_*` を active symbol として固定。
2. HFK-min2 done:
   - `include/nyrt.h` に register/try contract を追加。
   - `lang/c-abi/shims/hako_kernel.c` に C registry 実装を追加。
3. HFK-min3 done:
   - `crates/nyash_kernel/src/hako_forward.rs` は C registry 呼び出しトランポリンへ縮退。
   - 互換のため `nyrt.hako.register_*` dot-name export は維持（実体保持なし）。
4. HFK-min4 active:
   - `tools/checks/dev_gate.sh runtime-exec-zero`
   - `tools/checks/dev_gate.sh portability`
   - local result (2026-03-01): both green
   - remaining: GitHub Actions linux/windows/mac run log sync
