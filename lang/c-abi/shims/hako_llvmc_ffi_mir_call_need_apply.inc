      /*
       * MIR call declaration need application.
       *
       * Route classifiers return MirCallNeedKind; this file is the only place
       * that maps those kinds onto declaration flags.
       */
      auto void apply_mir_call_need_kind(
          struct MirCallNeedFlags* needs,
          enum MirCallNeedKind kind) {
        if (!needs) return;
        switch (kind) {
          case HAKO_LLVMC_MIR_CALL_NEED_MAP_BIRTH:
            needs->map_birth = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_MAP_SET:
            needs->map_set = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_MAP_SIZE:
            needs->map_size = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_MAP_GET:
            needs->map_get = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_MAP_HAS:
            needs->map_has = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_MAP_DELETE:
            needs->map_delete = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_MAP_KEYS:
            needs->map_keys = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_RUNTIME_DATA_HAS:
            needs->runtime_data_has = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_ARRAY_BIRTH:
            needs->arr_birth = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_ARRAY_PUSH:
            needs->arr_push = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_ARRAY_LEN:
            needs->arr_len = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_ARRAY_SET:
            needs->arr_set = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_ARRAY_GET:
            needs->arr_get = 1;
            needs->arr_slot_load = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_ARRAY_HAS:
            needs->arr_has = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_ARRAY_STRING_INDEXOF:
            needs->arr_string_indexof = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_STRING_LEN:
            needs->string_len = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING:
            needs->string_substring = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING_LEN:
            needs->string_substring_len = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_STRING_INDEXOF:
            needs->string_indexof = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_STRING_CONTAINS:
            needs->string_contains = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_STRING_CONCAT:
            needs->string_concat = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_ANY_HANDLE_LIVE:
            needs->any_handle_live = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_ENV_GET:
            needs->env_get = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_ENV_SET:
            needs->env_set = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_HAKO_ATOMIC_SLOT_CAS_I64:
            needs->hako_atomic_slot_cas_i64 = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_HAKO_ATOMIC_SLOT_FETCH_ADD_I64:
            needs->hako_atomic_slot_fetch_add_i64 = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_HAKO_ATOMIC_SLOT_LOAD_I64:
            needs->hako_atomic_slot_load_i64 = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_HAKO_ATOMIC_SLOT_STORE_I64:
            needs->hako_atomic_slot_store_i64 = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_HAKO_ATOMIC_PTR_CAS_ORDERED:
            needs->hako_atomic_ptr_cas_ordered = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_HAKO_ATOMIC_PTR_LOAD_ORDERED:
            needs->hako_atomic_ptr_load_ordered = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_HAKO_ATOMIC_PTR_STORE_ORDERED:
            needs->hako_atomic_ptr_store_ordered = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_HAKO_MEM_ALLOC:
            needs->hako_mem_alloc = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_HAKO_MEM_FREE:
            needs->hako_mem_free = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_HAKO_OSVM_RESERVE_BYTES_I64:
            needs->hako_osvm_reserve_bytes_i64 = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_HAKO_OSVM_COMMIT_BYTES_I64:
            needs->hako_osvm_commit_bytes_i64 = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_HAKO_OSVM_DECOMMIT_BYTES_I64:
            needs->hako_osvm_decommit_bytes_i64 = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_HAKO_TLS_CACHE_SLOT_GET_I64:
            needs->hako_tls_cache_slot_get_i64 = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_HAKO_TLS_CACHE_SLOT_SET_I64:
            needs->hako_tls_cache_slot_set_i64 = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_HOSTBRIDGE_EXTERN_INVOKE_TRAP:
            needs->hostbridge_extern_invoke_trap = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_STAGE1_EMIT_PROGRAM_JSON:
            needs->stage1_emit_program_json = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_STAGE1_EMIT_MIR_FROM_SOURCE:
            needs->stage1_emit_mir_from_source = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_STAGE1_EMIT_MIR_FROM_PROGRAM_JSON:
            needs->stage1_emit_mir_from_program_json = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_PRINTF:
            needs->printf = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_NONE:
          default:
            break;
        }
      }
