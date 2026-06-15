---
Status: Landed
Date: 2026-06-16
Task: SELFHOST-MIR-OBJECT-METADATA-001
Scope: Define the minimal object metadata selfhost `.hako` MIRBuilder may emit.
Related:
  - docs/development/current/main/design/selfhost-mir-object-metadata-ssot.md
  - docs/development/current/main/design/compiler-object-final-shape-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-826-MIRBUILDER-OBJECT-BOUNDARY-GUARD-001.md
---

# SELFHOST-MIR-OBJECT-METADATA-001

## Purpose

`MIRBUILDER-OBJECT-BOUNDARY-GUARD-001` prevents Rust MIRBuilder from owning
object representation truth.  This row fixes the corresponding selfhost
contract: `.hako` MIRBuilder may emit only object meaning metadata.

## Result

```text
output_contract=hako-selfhost-mir-object-metadata-v0
selfhost_mir_object_metadata_contract=hako-selfhost-mir-object-metadata-v0
source_evidence=296x-826,296x-825
selfhost_mirbuilder_metadata_only=1
selfhost_mirbuilder_allowed_metadata=source_span,receiver_origin,known_type_hint,field_key,call_site_id,newbox_origin
selfhost_mirbuilder_representation_truth_enabled=0
selfhost_mirbuilder_publication_truth_enabled=0
selfhost_mirbuilder_backend_route_truth_enabled=0
selfhost_mirbuilder_hosthandle_bypass_proof_enabled=0
selfhost_mirbuilder_arc_retirement_proof_enabled=0
selfhost_mirbuilder_fail_fast_prefix=[freeze:contract][hako_mirbuilder]
implementation_started=0
product_default_changed=0
selected_next=OBJECTPLAN-PASSIVE-UNIFY-001
summary=ok
```

## Stop Line

```text
do not add representation truth to selfhost MIRBuilder
do not add publication truth to selfhost MIRBuilder
do not add backend direct route truth to selfhost MIRBuilder
do not use metadata as a HostHandle bypass proof
do not use metadata as an Arc retirement proof
```

## Proof

```bash
bash tools/checks/k2_wide_phase296x_selfhost_mir_object_metadata_guard.sh
```
