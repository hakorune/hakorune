---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: X50 の NewClosure 契約を「runtime fail-fast 維持」で固定し、runtime semantics 拡張をこのレーンから除外する。
Related:
  - docs/development/current/main/phases/phase-29x/29x-76-vm-hako-strict-dev-replay-gate-ssot.md
  - docs/development/current/main/design/de-rust-compiler-thin-rust-roadmap-ssot.md
  - tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_newclosure_contract_vm.sh
  - tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s5_newclosure_probe_vm.sh
---

# 29x-77: NewClosure Contract Lock (SSOT)

## 0. Decision

- Decision: `accepted`（fail-fast 維持）
- このレーン（X50）では NewClosure runtime semantics を追加しない。
- `new_closure` は compiler 側 shape 契約を維持しつつ、runtime は fail-fast 契約を固定する。
- Decision owner: NewClosure runtime 境界の canonical owner はこの SSOT（29x-77）。
- X57 以降の refresh 文書は本 Decision を参照し、矛盾時は 29x-77 を優先する。

## 1. Contract

1. compiler-side:
   - MIR allowlist は `new_closure` shape を受理し続ける。
2. runtime-side:
   - rust-vm route は loader で unsupported op(new_closure) を返す。
   - hako-runner route は `[vm-hako/unimplemented op=new_closure]` で fail-fast する。
3. silent fallback は禁止。

## 2. Evidence command

- `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_newclosure_contract_vm.sh`

## 3. Non-goals

- NewClosure の runtime execution 実装
- closure capture semantics の追加
- GC/finalizer 実装
