# Phase 109: error_tags hints SSOT

**Purpose**: Unify policy/validator errors with "tag + message + hint" format.
**Changes**: Added `freeze_with_hint()` API, migrated Phase 107/104/100 representative policies.
**Acceptance**: cargo test --lib + smoke regression PASS.
