# vm_front — VM Front EXE (Phase 20.13; optional)

Purpose
- Provide light dispatch hints/normalization prior to VM/LLVM execution.
- Keep semantics in VM/LLVM; vm_front only shapes inputs and rejects invalid cases early.

Responsibilities
- Normalize callee/method hints; produce OK/NOOP/FAIL (short) with exit code.

Notes (AOT / Single‑EXE)
- 将来的に単一exeに内蔵（AOT）され、同一プロセスで前段整流を行います。
- 開発時は tools/front_exe/vm_front.sh をゲートONで差し替え可能です。

Inputs/Outputs
- Input: small JSON describing callsite/callee
- Output: one-line verdict (OK/NOOP/FAIL) + exit code

ENV (planned; default OFF)
- HAKO_VM_USE_SCRIPT_EXE=1 (alias NYASH_*) — enable vm front EXE
- HAKO_QUIET=1

Non‑Goals
- No execution, no state, no ABI interaction.

TTL
- Runner-side formatting will migrate here after stabilization.
