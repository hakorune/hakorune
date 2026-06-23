      if (needs.string_const_box ||
          module_has_planned_same_module_function_definition() ||
          object_storage_plan_selected_alignment_result_ready())
        fprintf(f, "declare i64 @\"nyash.box.from_i8_string_const\"(ptr)\n");
      if (needs.string_len || module_has_planned_same_module_function_definition())
        fprintf(f, "declare i64 @\"nyash.string.len_fast_h\"(i64)\n");
      fprintf(f, "declare i64 @\"nyash.string.eq_hh\"(i64, i64)%s\n", HAKO_LLVM_READONLY_ATTRS);
      fprintf(f, "declare i64 @\"nyash.string.lt_hh\"(i64, i64)%s\n", HAKO_LLVM_READONLY_ATTRS);
      if (needs.string_substring || needs.arr_set || needs.arr_get || needs.arr_string_indexof || needs.string_indexof || module_has_planned_same_module_function_definition())
        fprintf(f, "declare i64 @\"nyash.string.substring_hii\"(i64, i64, i64)\n");
      if (needs.string_substring) fprintf(f, "declare i64 @\"nyash.string.substring_concat_hhii\"(i64, i64, i64, i64)\n");
      if (needs.string_substring) fprintf(f, "declare i64 @\"nyash.string.substring_concat3_hhhii\"(i64, i64, i64, i64, i64)\n");
      if (needs.string_substring || needs.arr_set || needs.arr_get || needs.arr_string_indexof || needs.string_indexof)
        fprintf(f, "declare i64 @\"nyash.string.piecewise_subrange_hsiii\"(i64, ptr, i64, i64, i64)\n");
      if (needs.string_substring || needs.arr_set || needs.arr_get || needs.arr_string_indexof || needs.string_indexof)
        fprintf(f, "declare i64 @\"nyash.string.kernel_slot_piecewise_subrange_hsiii\"(ptr, i64, ptr, i64, i64, i64)\n");
      if (needs.string_substring || needs.arr_set || needs.arr_get || needs.arr_string_indexof || needs.string_indexof)
        fprintf(f, "declare i64 @\"nyash.string.kernel_slot_substring_hii_in_place\"(ptr, i64, i64)\n");
      if (needs.string_substring || needs.arr_set || needs.arr_get || needs.arr_string_indexof || needs.string_indexof)
        fprintf(f, "declare i64 @\"nyash.string.kernel_slot_publish_h\"(ptr)\n");
      if (needs.string_indexof || needs.arr_string_indexof || module_has_planned_same_module_function_definition())
        fprintf(f, "declare i64 @\"nyash.string.indexOf_hh\"(i64, i64)\n");
      if (needs.string_indexof || needs.arr_string_indexof || module_has_planned_same_module_function_definition())
        fprintf(f, "declare i64 @\"nyash.string.lastIndexOf_hh\"(i64, i64)\n");
      if (needs.string_contains || module_has_planned_same_module_function_definition())
        fprintf(f, "declare i64 @\"nyash.string.contains_hh\"(i64, i64)\n");
      if (needs.any_length || module_has_planned_same_module_function_definition())
        fprintf(f, "declare i64 @\"nyash.any.length_h\"(i64)%s\n", HAKO_LLVM_READONLY_ATTRS);
      if (needs.any_handle_live || module_has_planned_same_module_function_definition())
        fprintf(f, "declare i64 @\"nyash.any.handle_live_h\"(i64)%s\n", HAKO_LLVM_READONLY_ATTRS);
      if (needs.env_get || module_has_planned_same_module_function_definition())
        fprintf(f, "declare i64 @\"nyash.env.get\"(i64)\n");
      if (needs.env_set || module_has_planned_same_module_function_definition())
        fprintf(f, "declare i64 @\"nyash.env.set\"(i64, i64)\n");
      if (needs.env_now_ms || module_has_planned_same_module_function_definition())
        fprintf(f, "declare i64 @\"nyash.env.now_ms\"()\n");
      if (needs.hako_mem_alloc || module_has_planned_same_module_function_definition())
        fprintf(f, "declare ptr @\"hako_mem_alloc\"(i64)\n");
      if (needs.hako_mem_free || module_has_planned_same_module_function_definition())
        fprintf(f, "declare void @\"hako_mem_free\"(ptr)\n");
      if (needs.hako_atomic_slot_cas_i64 ||
          module_has_planned_same_module_function_definition())
        fprintf(f, "declare i64 @\"hako_atomic_slot_cas_i64\"(i64, i64, i64)\n");
      if (needs.hako_atomic_slot_fetch_add_i64 ||
          module_has_planned_same_module_function_definition())
        fprintf(f, "declare i64 @\"hako_atomic_slot_fetch_add_i64\"(i64, i64)\n");
      if (needs.hako_atomic_slot_load_i64 ||
          module_has_planned_same_module_function_definition())
        fprintf(f, "declare i64 @\"hako_atomic_slot_load_i64\"(i64)\n");
      if (needs.hako_atomic_slot_store_i64 ||
          module_has_planned_same_module_function_definition())
        fprintf(f, "declare i64 @\"hako_atomic_slot_store_i64\"(i64, i64)\n");
      if (needs.hako_atomic_ptr_cas_ordered ||
          module_has_planned_same_module_function_definition())
        fprintf(f, "declare ptr @\"hako_atomic_ptr_cas_ordered\"(ptr, ptr, ptr, i64, i64)\n");
      if (needs.hako_atomic_ptr_load_ordered ||
          module_has_planned_same_module_function_definition())
        fprintf(f, "declare ptr @\"hako_atomic_ptr_load_ordered\"(ptr, i64)\n");
      if (needs.hako_atomic_ptr_store_ordered ||
          module_has_planned_same_module_function_definition())
        fprintf(f, "declare i64 @\"hako_atomic_ptr_store_ordered\"(ptr, ptr, i64)\n");
      if (needs.hako_osvm_reserve_bytes_i64 ||
          module_has_planned_same_module_function_definition())
        fprintf(f, "declare i64 @\"hako_osvm_reserve_bytes_i64\"(i64)\n");
      if (needs.hako_osvm_commit_bytes_i64 ||
          module_has_planned_same_module_function_definition())
        fprintf(f, "declare i64 @\"hako_osvm_commit_bytes_i64\"(i64, i64)\n");
      if (needs.hako_osvm_decommit_bytes_i64 ||
          module_has_planned_same_module_function_definition())
        fprintf(f, "declare i64 @\"hako_osvm_decommit_bytes_i64\"(i64, i64)\n");
      if (needs.hako_osvm_unreserve_bytes_i64 ||
          module_has_planned_same_module_function_definition())
        fprintf(f, "declare i64 @\"hako_osvm_unreserve_bytes_i64\"(i64, i64)\n");
      if (needs.hako_tls_cache_slot_get_i64 ||
          module_has_planned_same_module_function_definition())
        fprintf(f, "declare i64 @\"hako_tls_cache_slot_get_i64\"(i64)\n");
      if (needs.hako_tls_cache_slot_set_i64 ||
          module_has_planned_same_module_function_definition())
        fprintf(f, "declare i64 @\"hako_tls_cache_slot_set_i64\"(i64, i64)\n");
      if (needs.hako_worker_current_id_i64 ||
          module_has_planned_same_module_function_definition())
        fprintf(f, "declare i64 @\"hako_worker_current_id_i64\"(i64)\n");
      if (needs.stage1_emit_program_json || module_has_planned_same_module_function_definition())
        fprintf(f, "declare i64 @\"nyash.stage1.emit_program_json_v0_h\"(i64)\n");
      if (needs.stage1_emit_mir_from_source ||
          module_has_planned_same_module_function_definition())
        fprintf(f, "declare i64 @\"nyash.stage1.emit_mir_from_source_v0_h\"(i64)\n");
      if (needs.stage1_emit_mir_from_program_json ||
          module_has_planned_same_module_function_definition())
        fprintf(f, "declare i64 @\"nyash.stage1.emit_mir_from_program_json_v0_h\"(i64)\n");
      if (needs.string_concat || needs.string_substring || needs.arr_get ||
          needs.arr_string_indexof || module_has_planned_same_module_function_definition()) {
        fprintf(f, "declare i64 @\"nyash.box.from_i64\"(i64)\n");
        fprintf(f, "declare i64 @\"nyash.box.from_bool\"(i64)\n");
        fprintf(f, "declare i64 @\"nyash.any.toString_h\"(i64)\n");
        fprintf(f, "declare i64 @\"nyash.string.concat_hh\"(i64, i64)\n");
        fprintf(f, "declare i64 @\"nyash.string.concat_hs\"(i64, ptr)\n");
        fprintf(f, "declare i64 @\"nyash.string.kernel_slot_concat_hs\"(ptr, i64, ptr)\n");
        fprintf(f, "declare i64 @\"nyash.string.kernel_slot_insert_hsi\"(ptr, i64, ptr, i64)\n");
        fprintf(f, "declare i64 @\"nyash.string.insert_hsi\"(i64, ptr, i64)\n");
        fprintf(f, "declare i64 @\"nyash.string.concat3_hhh\"(i64, i64, i64)\n");
      }
      if (needs.printf || module_has_planned_same_module_function_definition())
        fprintf(f, "@.fmt_i64 = private unnamed_addr constant [5 x i8] c\"%%ld\\0A\\00\", align 1\n");
      for (size_t sci=0; sci<str_consts_n; sci++) {
        size_t slen = strlen(str_consts[sci].val);
        fprintf(f, "@.str_reg_%lld = private unnamed_addr constant [%zu x i8] c\"", str_consts[sci].reg, slen + 1);
        hako_llvmc_emit_llvm_byte_string(f, str_consts[sci].val, (long long)slen);
        fprintf(f, "\\00\", align 1\n");
      }
      fprintf(f, "@.hako_type_FileBox = private unnamed_addr constant [8 x i8] c\"FileBox\\00\", align 1\n");
      if (object_storage_plan_selected_alignment_result_ready()) {
        fprintf(f, "@.objstore_alignment_result_last_requested = private unnamed_addr constant [32 x i8] c\"alignment_result.last_requested\\00\", align 1\n");
        fprintf(f, "@.objstore_alignment_result_last_normalized = private unnamed_addr constant [33 x i8] c\"alignment_result.last_normalized\\00\", align 1\n");
        fprintf(f, "@.objstore_alignment_result_last_reason = private unnamed_addr constant [29 x i8] c\"alignment_result.last_reason\\00\", align 1\n");
        fprintf(f, "@.objstore_alignment_result_last_supported = private unnamed_addr constant [32 x i8] c\"alignment_result.last_supported\\00\", align 1\n");
      }
      fprintf(f, "declare i64 @\"nyash.integer.get_h\"(i64)\n");
      fprintf(f, "declare i64 @\"nyash.env.box.new\"(ptr)\n");
      fprintf(f, "declare i64 @\"nyash.file.open_hhh\"(i64, i64, i64)\n");
      fprintf(f, "declare i64 @\"nyash.file.read_h\"(i64)\n");
      fprintf(f, "declare i64 @\"nyash.file.close_h\"(i64)\n");
      if (needs.hostbridge_extern_invoke_trap)
        fprintf(f, "declare void @llvm.trap()\n");
      if (needs.printf || module_has_planned_same_module_function_definition())
        fprintf(f, "declare i32 @printf(ptr, ...)\n");
      if (needs.printf || module_has_planned_same_module_function_definition())
        fprintf(f, "declare i64 @\"nyash.console.log_handle\"(i64)\n");
      if (needs.string_substring_len) fprintf(f, "declare i64 @\"nyash.string.substring_len_hii\"(i64, i64, i64)\n");
