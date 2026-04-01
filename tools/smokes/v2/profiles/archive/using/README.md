# Archived Using Smokes

This folder holds historical `using` smoke pins that were phased out of the `quick` lane.

Contents
- `using_profiles_ast.sh`: archived AST-prelude pin for dev/prod file and alias policy.
- `using_multi_prelude_dep_ast.sh`: archived AST pin for multi-prelude package resolution.

Run them explicitly with the archive profile when you need to replay the historical pins:

```bash
./tools/smokes/v2/run.sh --profile archive --filter "using_profiles_ast.sh$"
./tools/smokes/v2/run.sh --profile archive --filter "using_multi_prelude_dep_ast.sh$"
```

These scripts are not part of the daily `quick` surface anymore.
