      /*
       * MIR call declaration need vocabulary.
       *
       * Shared flags and need kinds only. Site inspection and route
       * classification stay in hako_llvmc_ffi_mir_call_need_policy.inc.
       */
      struct MirCallNeedFlags {
        int map_birth;
        int map_set;
        int map_size;
        int map_get;
        int map_has;
        int map_delete;
        int map_keys;
        int runtime_data_has;
        int arr_birth;
        int arr_push;
        int arr_len;
        int arr_set;
        int arr_get;
        int arr_has;
        int arr_slot_load;
        int arr_kernel_slot_concat_by_index;
        int arr_string_indexof;
        int arr_string_indexof_const_utf8;
        int string_const_box;
        int string_len;
        int string_substring;
        int string_substring_len;
        int string_indexof;
        int string_contains;
        int string_concat;
        int any_handle_live;
        int env_get;
        int env_set;
        int hako_mem_alloc;
        int hako_mem_free;
        int hako_osvm_reserve_bytes_i64;
        int hako_osvm_commit_bytes_i64;
        int hako_osvm_decommit_bytes_i64;
        int hostbridge_extern_invoke_trap;
        int stage1_emit_program_json;
        int stage1_emit_mir_from_source;
        int stage1_emit_mir_from_program_json;
        int printf;
        int typed_object;
      };

      enum MirCallNeedKind {
        HAKO_LLVMC_MIR_CALL_NEED_NONE = 0,
        HAKO_LLVMC_MIR_CALL_NEED_MAP_BIRTH,
        HAKO_LLVMC_MIR_CALL_NEED_MAP_SET,
        HAKO_LLVMC_MIR_CALL_NEED_MAP_SIZE,
        HAKO_LLVMC_MIR_CALL_NEED_MAP_GET,
        HAKO_LLVMC_MIR_CALL_NEED_MAP_HAS,
        HAKO_LLVMC_MIR_CALL_NEED_MAP_DELETE,
        HAKO_LLVMC_MIR_CALL_NEED_MAP_KEYS,
        HAKO_LLVMC_MIR_CALL_NEED_RUNTIME_DATA_HAS,
        HAKO_LLVMC_MIR_CALL_NEED_ARRAY_BIRTH,
        HAKO_LLVMC_MIR_CALL_NEED_ARRAY_PUSH,
        HAKO_LLVMC_MIR_CALL_NEED_ARRAY_LEN,
        HAKO_LLVMC_MIR_CALL_NEED_ARRAY_SET,
        HAKO_LLVMC_MIR_CALL_NEED_ARRAY_GET,
        HAKO_LLVMC_MIR_CALL_NEED_ARRAY_HAS,
        HAKO_LLVMC_MIR_CALL_NEED_ARRAY_STRING_INDEXOF,
        HAKO_LLVMC_MIR_CALL_NEED_STRING_LEN,
        HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING,
        HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING_LEN,
        HAKO_LLVMC_MIR_CALL_NEED_STRING_INDEXOF,
        HAKO_LLVMC_MIR_CALL_NEED_STRING_CONTAINS,
        HAKO_LLVMC_MIR_CALL_NEED_STRING_CONCAT,
        HAKO_LLVMC_MIR_CALL_NEED_ANY_HANDLE_LIVE,
        HAKO_LLVMC_MIR_CALL_NEED_ENV_GET,
        HAKO_LLVMC_MIR_CALL_NEED_ENV_SET,
        HAKO_LLVMC_MIR_CALL_NEED_HAKO_MEM_ALLOC,
        HAKO_LLVMC_MIR_CALL_NEED_HAKO_MEM_FREE,
        HAKO_LLVMC_MIR_CALL_NEED_HAKO_OSVM_RESERVE_BYTES_I64,
        HAKO_LLVMC_MIR_CALL_NEED_HAKO_OSVM_COMMIT_BYTES_I64,
        HAKO_LLVMC_MIR_CALL_NEED_HAKO_OSVM_DECOMMIT_BYTES_I64,
        HAKO_LLVMC_MIR_CALL_NEED_HOSTBRIDGE_EXTERN_INVOKE_TRAP,
        HAKO_LLVMC_MIR_CALL_NEED_STAGE1_EMIT_PROGRAM_JSON,
        HAKO_LLVMC_MIR_CALL_NEED_STAGE1_EMIT_MIR_FROM_SOURCE,
        HAKO_LLVMC_MIR_CALL_NEED_STAGE1_EMIT_MIR_FROM_PROGRAM_JSON,
        HAKO_LLVMC_MIR_CALL_NEED_PRINTF,
      };
