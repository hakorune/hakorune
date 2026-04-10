# MIR Dumper Output Guide

SSOT:

- `docs/reference/mir/INSTRUCTION_SET.md`
- `docs/reference/mir/metadata-facts-ssot.md`
- `docs/reference/mir/call-instructions-current.md`

このガイドは、現在の MIR テキストダンプ / verbose ダンプ / MIR JSON をどう読むかの
入口だけを扱うよ。命令総覧や metadata の正本は上の SSOT を参照してね。

## 1. Call の読み方

Canonical MIR では callsite は `MirInstruction::Call` に統一されている。
古い `BoxCall` / `ExternCall` は **canonical MIR の正本ではない**。

### 典型的な読み方

```mir
%17 = call @main(%1, %2)
```

- 実際の呼び先種別 (`Global` / `Method` / `Extern` / `Value`) は `callee` 側の契約で決まる
- 詳細は `docs/reference/mir/call-instructions-current.md`

## 2. Variant 命令の読み方

phase-163x 以降、canonical variant op lane は専用命令で観測する。

```mir
%7 = variant.make Option::Some(tag=1, payload=%3)
%8 = variant.tag %7
%9 = variant.project %7 as Option::Some(tag=1)
```

- `variant.make` = canonical variant construction
- `variant.tag` = tag read
- `variant.project` = matched variant payload projection
- JSON ではそれぞれ `variant_make` / `variant_tag` / `variant_project`

## 3. Verbose ダンプで見える metadata

Verbose MIR (`--mir-verbose`) では命令本体に加えて inspection metadata が出る。

主な見出し:

- `String Corridor Facts`
- `Thin Entry Candidates`
- `Thin Entry Selections`
- `Sum Placement Facts`
- `Sum Placement Selections`
- `Sum Placement Layouts`

意味や JSON shape の正本は `docs/reference/mir/metadata-facts-ssot.md`。

## 4. MIR JSON で見る場所

MIR JSON では次の 2 箇所を読む:

1. `functions[].blocks[].instructions[]` — canonical instructions
2. `functions[].metadata` — inspection-only metadata facts

例:

```json
{
  "op": "variant_make",
  "dst": 7,
  "enum": "Option",
  "variant": "Some",
  "tag": 1,
  "payload": 3,
  "payload_type": "Integer"
}
```

## 5. Historical note

このガイドの旧版は「BoxCall vs 通常の Call」の見分け方が主題だったけど、
それは retired になった古い読み方だよ。現在の call 読みは `Call + Callee`
前提で統一すること。
