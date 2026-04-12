# Phase 209x: agg_local scalarization owner seam

Status: Landed

Purpose
- fold the landed sum / thin-entry / storage-class pilots into one generic agg_local owner seam
  - keep the first code slice inspection-only and behavior-preserving

Scope
- derive folded agg_local routes from current MIR semantic metadata
- keep sum placement, thin-entry, and storage-class pilot owners intact
- export the folded view through MIR JSON only after the MIR-side owner exists

Follow-on
- `thin-entry actual consumer switch` (`phase210x`)

Non-goals
- no lowering / codegen widening
- thin-entry actual consumer switch is the follow-on phase
- no DCE / simplification-bundle / string changes

Acceptance
- `agg_local_scalarization` has a MIR-side owner seam
- current pointers move from docs/facts phase into this code phase
- `git diff --check`
