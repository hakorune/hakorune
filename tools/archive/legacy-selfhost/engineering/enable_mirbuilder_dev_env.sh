#!/usr/bin/env bash
# Archived shell dev profile for Hakorune MirBuilder (historical evidence only).
# The active smoke helper is enable_mirbuilder_dev_env() in tools/smokes/v2/lib/test_runner.sh.
#
# Exports a recommended set of env toggles to develop via Hakorune scripts
# without rebuilding Rust frequently. Defaults keep logs quiet.

export HAKO_SELFHOST_BUILDER_FIRST=${HAKO_SELFHOST_BUILDER_FIRST:-1}
# Set to 1 to hard-disable Rust delegate builder (fail fast on selfhost errors)
export HAKO_SELFHOST_NO_DELEGATE=${HAKO_SELFHOST_NO_DELEGATE:-0}
# Try minimal builder fallback to keep selfhost-first green on mini cases
export HAKO_SELFHOST_TRY_MIN=${HAKO_SELFHOST_TRY_MIN:-1}

# LoopJsonFrag: force minimal MIR for loops + normalize/purify
export HAKO_MIR_BUILDER_LOOP_JSONFRAG=${HAKO_MIR_BUILDER_LOOP_JSONFRAG:-1}
export HAKO_MIR_BUILDER_LOOP_FORCE_JSONFRAG=${HAKO_MIR_BUILDER_LOOP_FORCE_JSONFRAG:-1}
export HAKO_MIR_BUILDER_JSONFRAG_NORMALIZE=${HAKO_MIR_BUILDER_JSONFRAG_NORMALIZE:-1}
export HAKO_MIR_BUILDER_JSONFRAG_PURIFY=${HAKO_MIR_BUILDER_JSONFRAG_PURIFY:-1}

# Keep normalization tag silent by default
export HAKO_MIR_BUILDER_NORMALIZE_TAG=${HAKO_MIR_BUILDER_NORMALIZE_TAG:-0}

# Functions/Call resolution（Phase 21.7 dev 一軍）
export HAKO_STAGEB_FUNC_SCAN=${HAKO_STAGEB_FUNC_SCAN:-1}
export HAKO_MIR_BUILDER_FUNCS=${HAKO_MIR_BUILDER_FUNCS:-1}
export HAKO_MIR_BUILDER_CALL_RESOLVE=${HAKO_MIR_BUILDER_CALL_RESOLVE:-1}
# Emit v1 JSON schema + unified mir_call（委譲時の安定化）
export NYASH_JSON_SCHEMA_V1=${NYASH_JSON_SCHEMA_V1:-1}
export NYASH_MIR_UNIFIED_CALL=${NYASH_MIR_UNIFIED_CALL:-1}

# Parser: Stage-3 ON, allow semicolons
export NYASH_FEATURES=${NYASH_PARSER_STAGE3:-1}
export NYASH_FEATURES=${HAKO_PARSER_STAGE3:-1}
export NYASH_PARSER_ALLOW_SEMICOLON=${NYASH_PARSER_ALLOW_SEMICOLON:-1}

if [[ "${1:-}" != "quiet" ]]; then
  echo "[mirbuilder/dev] HAKO_SELFHOST_BUILDER_FIRST=$HAKO_SELFHOST_BUILDER_FIRST"
  echo "[mirbuilder/dev] HAKO_SELFHOST_NO_DELEGATE=$HAKO_SELFHOST_NO_DELEGATE"
  echo "[mirbuilder/dev] LOOP_FORCE_JSONFRAG=$HAKO_MIR_BUILDER_LOOP_FORCE_JSONFRAG (normalize=$HAKO_MIR_BUILDER_JSONFRAG_NORMALIZE purify=$HAKO_MIR_BUILDER_JSONFRAG_PURIFY)"
  echo "[mirbuilder/dev] NORMALIZE_TAG=$HAKO_MIR_BUILDER_NORMALIZE_TAG (0=silent)"
  echo "[mirbuilder/dev] FUNCS=$HAKO_MIR_BUILDER_FUNCS CALL_RESOLVE=$HAKO_MIR_BUILDER_CALL_RESOLVE (func_scan=$HAKO_STAGEB_FUNC_SCAN)"
  echo "[mirbuilder/dev] JSON_SCHEMA_V1=$NYASH_JSON_SCHEMA_V1 UNIFIED_CALL=$NYASH_MIR_UNIFIED_CALL"
fi
