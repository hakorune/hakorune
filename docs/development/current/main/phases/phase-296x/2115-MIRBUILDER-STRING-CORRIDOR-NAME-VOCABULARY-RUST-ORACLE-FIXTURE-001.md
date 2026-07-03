# 2115 - MIRBUILDER-STRING-CORRIDOR-NAME-VOCABULARY-RUST-ORACLE-FIXTURE-001

## Token

```text
MIRBUILDER-STRING-CORRIDOR-NAME-VOCABULARY-RUST-ORACLE-FIXTURE-001
```

## Purpose

Create the Rust-oracle fixture for the fourth narrow `.hako` native owner
parity pilot: `string_corridor_name_vocabulary_classifier`.

The fixture captures only the pure vocabulary quarantine surface from
`src/mir/string_corridor_names.rs`.

## Fixture

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-string-corridor-name-vocabulary-rust-oracle-v0.json

rows:
  18
```

## Included Surface

```text
name -> stringish box?
name -> length method?
name -> slice method?
name -> lowered length global?
name -> runtime length export?
name -> runtime length handle export?
name -> runtime slice export?
name -> runtime substring export?
name -> runtime substring length export?
name -> runtime substring concat3 export?
name -> runtime concat3 export?
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
python3 -m json.tool \
  docs/development/current/main/design/fixtures/rust-lifecycle/\
mirbuilder-string-corridor-name-vocabulary-rust-oracle-v0.json >/dev/null
```

## Decision

```text
decision:
  SelectHakoNativeImplementation

reason_token:
  StringCorridorNameVocabularyRustOracleFixtureCreated

selected_next_card:
  MIRBUILDER-STRING-CORRIDOR-NAME-VOCABULARY-HAKO-NATIVE-IMPLEMENTATION-001
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
