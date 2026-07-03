# 2116 - MIRBUILDER-STRING-CORRIDOR-NAME-VOCABULARY-HAKO-NATIVE-IMPLEMENTATION-001

## Token

```text
MIRBUILDER-STRING-CORRIDOR-NAME-VOCABULARY-HAKO-NATIVE-IMPLEMENTATION-001
```

## Purpose

Add the hand-authored `.hako` implementation for the fourth narrow
Rust-oracle parity pilot: `string_corridor_name_vocabulary_classifier`.

The implementation mirrors only the Rust oracle vocabulary predicates from
`src/mir/string_corridor_names.rs`.

## Implementation

```text
hako_source:
  lang/src/compiler/lib/string_corridor_name_vocabulary.hako

entry_box:
  StringCorridorNameVocabularyBox
```

## Included Surface

```text
is_stringish_box_name
is_len_method_name
is_slice_method_name
is_lowered_len_global
is_runtime_len_export
is_runtime_len_handle_export
is_runtime_slice_export
is_runtime_substring_export
is_runtime_substring_len_export
is_runtime_substring_concat3_export
is_runtime_concat3_export
```

## Excluded Surface

```text
string corridor fact inference
string corridor recognizer shape matching
compat recovery policy
MIR instruction traversal
runtime export lowering
```

## Acceptance

```text
bash tools/bin/hako --backend mir --verify \
  lang/src/compiler/lib/string_corridor_name_vocabulary.hako
```

## Decision

```text
decision:
  SelectParityGate

reason_token:
  StringCorridorNameVocabularyHakoNativeImplementationCreated

selected_next_card:
  MIRBUILDER-STRING-CORRIDOR-NAME-VOCABULARY-PARITY-GATE-001
```

## Non-Claims

```text
no Source Selfhost claim
no Hako adoption decision
no generated artifact edit authority
no string corridor fact inference migration
no MIR instruction traversal migration
no runtime fallback
no new backend route
no new ABI
```
