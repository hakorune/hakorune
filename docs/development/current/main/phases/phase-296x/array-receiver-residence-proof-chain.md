# Array Receiver Residence Proof Chain

Status: Navigation note / current developer entry
Date: 2026-06-15

## Read This First

Use `ArrayReceiverResidenceProofChain` as the developer-facing entry.

```text
ArrayReceiverResidenceProofChain
  .construct_input_source_from_representation_source(source)
```

This facade keeps the older staged proof gates but prevents future work from
patching the nearest internal noun.

The thinning detour is closed by 296x-805. New work should start from this note
and then move to the next `ArrayReceiverResidenceInput` consumer row.

## Stages

```text
ArrayReceiverRepresentationSource
  -> ArrayReceiverResidenceProofChain
  -> ArrayReceiverResidenceInputSource
  -> ArrayReceiverResidenceInput
  -> ArrayReceiverResidenceFact
  -> backend consumer later
```

`ArrayReceiverConstructorHandoff` still exists as compatibility vocabulary for
296x-796..800 reports. It is not the primary mental model for new work.

## Current Guarantees

```text
fallback_source_is_not_direct_proof=1
backend_direct_handle_bypass_enabled=0
mir_json_export_enabled=0
backend_consumption_enabled=0
mirbuilder_object_management_enabled=0
```

## Next Work

Return to `ArrayReceiverResidenceInput` only after the facade closeout. The next
implementation must still keep direct proof and backend bypass closed unless a
separate proof row opens them.

## Compatibility Vocabulary

`ArrayReceiverConstructorHandoff` remains in code and reports because 296x-796
through 296x-800 used it as a proof-gate vocabulary. Treat it as an internal
compatibility noun, not as the public entry for new design or implementation
work.
