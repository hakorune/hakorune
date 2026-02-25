# Legacy Loop Pattern Detection

This directory contains the Phase 188 name-based loop pattern detectors.
They are **still production code** (used by pattern lowerers), but are
kept separate from the Phase 194+ structure-based classifiers
(`extract_features` + `classify`).

**Policy**
- Treat these functions as compatibility/legacy logic.
- New pattern detection should go into `features` + `classify`.
- Migrate callsites gradually; keep behavior stable while moving.
