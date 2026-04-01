# compat

Purpose
- Compat/proof and legacy bridge surfaces live here.
- These paths should read as temporary or archive-facing, not as daily owner homes.

Layout
- `codegen/` - legacy emit/link bridge payloads and proof-only compat boxes.

Rules
- New daily callers should stop at the owner boundary in `lang/src/shared/backend/`.
- If a surface still exists here, treat it as compat/proof or archive-facing only.
