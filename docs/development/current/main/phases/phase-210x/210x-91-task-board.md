# 210x-91 Task Board

Status: Active

## Tasks

- [ ] A0: add a shared thin-entry decision helper for lowering consumers
- [ ] A1: wire `method_call`, `field_access`, and `user_box_local` to the shared helper
- [ ] A2: add focused tests or update existing tests for the shared consumer seam
- [ ] A3: sync current pointers to the new code phase

## Exit

- thin-entry actual-consumer decisions are centralized in lowering
- known-receiver user-box method routes and inline-scalar field routes still behave the same
- the phase stays behavior-preserving
