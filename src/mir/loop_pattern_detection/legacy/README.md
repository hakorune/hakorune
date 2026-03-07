# Legacy Loop Route Detection

This directory contains the Phase 188 name-based loop route detectors.
They are **still production code** (used by route lowerers), but are
kept separate from the Phase 194+ structure-based classifiers
(`extract_features` + `classify`).

**Policy**
- Treat these functions as compatibility/legacy logic.
- New route detection should go into `features` + `classify`.
- Migrate callsites gradually; keep behavior stable while moving.
