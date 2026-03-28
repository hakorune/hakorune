# integration archive smokes

Archive-only integration smoke scripts live under `archive/<family>/...`.

Rules:
- These scripts are excluded from active discovery by default.
- Explicit suite manifests may point at archive paths when replaying retired evidence.
- Do not add new daily owner coverage here; only retired compare / boundary evidence belongs in this subtree.
