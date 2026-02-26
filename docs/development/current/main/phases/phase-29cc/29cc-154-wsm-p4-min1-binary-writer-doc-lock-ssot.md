---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: WSM-P4-min1（.hako-only roadmap P4）として wasm binary writer の最小契約を docs-first で lock する。
Related:
  - docs/development/current/main/design/wasm-hako-only-output-roadmap-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-153-wsm-p3-min1-import-object-lock-ssot.md
  - docs/guides/wasm-guide/planning/unsupported_features.md
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p4_min1_docs_lock_vm.sh
---

# 29cc-154 WSM-P4-min1 Binary Writer Docs Lock

## Purpose
`.hako` 側 wasm binary writer 実装（P4）に入る前に、section/LEB128 の最小契約を docs-first で固定し、実装時の判断分岐を減らす。

## Decision
1. P4 の最小 writer 対象は以下に固定する（順序も固定）。
   - magic/version
   - type section
   - function section
   - export section
   - code section
2. `LEB128` は unsigned/signed の最小実装を対象とし、overlong encoding は reject（fail-fast）する。
3. P4-min2 実装中は「WAT fallback」「silent recovery」を禁止し、未対応 section は fail-fast で明示する。
4. 受け入れ観点は次段（P4-min2）で fixture/gate 化するが、P4-min1 は docs lock smoke で入口固定する。

## P4-min2 Entry Contract
1. 入力は `.hako` 側で確定した最小 MIR subset（const-return 相当）に限定する。
2. 出力 `.wasm` は `\0asm` magic を必須にし、type/function/export/code の最小構成で main export を持つ。
3. section 長・関数 body 長・即値は `LEB128` で符号化し、decode 可能であること。

## Acceptance
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p4_min1_docs_lock_vm.sh`
- `tools/checks/dev_gate.sh wasm-boundary-lite`

## Next
- `WSM-P4-min2`: binary writer skeleton（magic/version + section/LEB128 最小）を実装し、fixture/gate で lock する。
