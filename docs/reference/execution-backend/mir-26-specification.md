# MIR 26-Instruction Specification

Status: historical target profile (not the executable SSOT)

この文書は「Core-26 をどう考えていたか」という historical target profile の入口だけを残す。
現在の実装語彙や JSON 契約を判断するときは、ここを正本にしないこと。

## Current canonical references

1. `docs/reference/mir/INSTRUCTION_SET.md`
   - current kept / removed ledger
   - executable instruction inventory
2. `docs/reference/mir/metadata-facts-ssot.md`
   - `functions[].metadata` JSON contract
3. `docs/reference/architecture/mir-26-instruction-diet.md`
   - Core-26 target / diet rationale as a target profile

## Why this file is thin now

- current MIR has canonical sum ops (`SumMake` / `SumTag` / `SumProject`)
- current MIR metadata includes thin-entry and sum-placement chains
- retired callsite docs (`BoxCall` / `ExternCall`) must not be mistaken for the live contract

If you need executable truth, read the canonical references above and ignore older
26-instruction examples in archived notes.
