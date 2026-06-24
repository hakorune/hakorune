# 296x-835 ARRAY-RECEIVER-RESIDENCE-PROOF-CHAIN-RETIRE-INVENTORY-001

Status: Landed
Date: 2026-06-16

## Purpose

Inventory the legacy array receiver residence proof-chain module before any
retirement.

This row does not delete code. It classifies the remaining residue so a later
retire gate can remove or quarantine it without breaking historical guards or
live proof modules.

## Finding

`src/array_receiver_representation_source.rs` remains a large legacy proof-chain
module:

```text
array_receiver_representation_source_line_count=719
```

The module is still exported from `src/lib.rs`, but current production evidence
does not show an execution consumer beyond historical guard/report surfaces:

```text
array_receiver_representation_source_src_reference_file_count=2
array_receiver_representation_source_tools_reference_file_count=14
array_receiver_representation_source_docs_reference_file_count=18
```

The broader legacy residence vocabulary is still widespread:

```text
legacy_residence_vocabulary_total_reference_count=565
legacy_residence_vocabulary_tools_file_count=31
legacy_residence_vocabulary_docs_file_count=36
legacy_residence_vocabulary_src_file_count=2
```

`src/mir/array_receiver_proof.rs` is separate and live:

```text
live_array_receiver_proof_module=src/mir/array_receiver_proof.rs
live_array_receiver_proof_line_count=148
```

That module must not be retired as part of this residue cleanup.

## Result

```text
output_contract=hako-array-receiver-proof-chain-retire-inventory-v0
source_evidence=296x-833,296x-834

array_receiver_representation_source_consumers_classified=1
array_receiver_representation_source_line_count=719
array_receiver_representation_source_src_reference_file_count=2
array_receiver_representation_source_tools_reference_file_count=14
array_receiver_representation_source_docs_reference_file_count=18

legacy_residence_vocabulary_total_reference_count=565
legacy_residence_vocabulary_tools_file_count=31
legacy_residence_vocabulary_docs_file_count=36
legacy_residence_vocabulary_src_file_count=2

live_array_receiver_proof_module=src/mir/array_receiver_proof.rs
live_array_receiver_proof_line_count=148
live_array_receiver_proof_must_keep=1

retire_gate_required=1
implementation_started=0
code_deleted=0
product_default_changed=0

selected_next=MIMALLOC-FRESH-FRONT-SELECTION-001
summary=ok
```

## Retire Project Shape

The retire project must be split. A safe order is:

```text
ARRAY-RECEIVER-PROOF-CHAIN-RETIRE-GATE-001
  define deletion/quarantine acceptance and replacement report vocabulary

ARRAY-RECEIVER-PROOF-CHAIN-GUARD-SWEEP-001
  update historical guards that still require src/array_receiver_representation_source.rs

ARRAY-RECEIVER-PROOF-CHAIN-DOC-SWEEP-001
  quarantine historical phase references and remove current-entry references

ARRAY-RECEIVER-PROOF-CHAIN-CODE-RETIRE-001
  remove src/lib.rs export and the legacy module only after guards/docs are green
```

## Stop Line

```text
do not delete src/array_receiver_representation_source.rs in this inventory row
do not touch live src/mir/array_receiver_proof.rs
do not remove src/lib.rs export before retire gate exists
do not edit historical guards piecemeal without a guard sweep row
do not rename legacy Residence/ProofChain vocabulary into ObjectPlan without a retire contract
```
