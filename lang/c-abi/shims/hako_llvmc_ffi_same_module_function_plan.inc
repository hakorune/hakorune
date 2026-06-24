      /*
       * Same-module function definition plan registry.
       *
       * MIR/LoweringPlan owns the definition owner and direct-call contract.
       * This registry only records same-module functions that must be emitted
       * as LLVM definitions so direct calls cannot silently externalize them.
       */
      const char* planned_same_module_function_symbols[1024];
      size_t planned_same_module_function_symbols_n = 0;
      const char* emitted_same_module_function_symbols[1024];
      size_t emitted_same_module_function_symbols_n = 0;

      auto int same_module_function_symbol_registry_contains(
          const char* const* symbols,
          size_t symbol_count,
          const char* name) {
        if (!name) return 0;
        for (size_t i = 0; i < symbol_count; i++) {
          if (!strcmp(symbols[i], name)) return 1;
        }
        return 0;
      }

      auto int remember_same_module_function_symbol_in_registry(
          const char** symbols,
          size_t* symbol_count,
          size_t symbol_capacity,
          const char* name) {
        if (same_module_function_symbol_registry_contains(
                symbols,
                symbol_count ? *symbol_count : 0,
                name)) {
          return 1;
        }
        if (!symbol_count || *symbol_count >= symbol_capacity) {
          note_pure_unsupported_shape_detail(
              -1,
              0,
              "module_function",
              "same_module_function_registry_full",
              NULL,
              NULL,
              name,
              NULL);
          return 0;
        }
        symbols[*symbol_count] = name;
        (*symbol_count)++;
        return 1;
      }

      auto int remember_planned_same_module_function_symbol(const char* name) {
        if (!name) return 0;
        if (!remember_same_module_function_symbol_in_registry(
                planned_same_module_function_symbols,
                &planned_same_module_function_symbols_n,
                ARR_LEN(planned_same_module_function_symbols),
                name)) {
          return 0;
        }
        return 1;
      }

      auto int same_module_function_symbol_will_be_defined(const char* name) {
        return same_module_function_symbol_registry_contains(
            planned_same_module_function_symbols,
            planned_same_module_function_symbols_n,
            name);
      }

      auto int remember_emitted_same_module_function_symbol(const char* name) {
        if (!name) return 0;
        if (!remember_same_module_function_symbol_in_registry(
                emitted_same_module_function_symbols,
                &emitted_same_module_function_symbols_n,
                ARR_LEN(emitted_same_module_function_symbols),
                name)) {
          return 0;
        }
        return 1;
      }

      auto int same_module_function_symbol_was_emitted(const char* name) {
        return same_module_function_symbol_registry_contains(
            emitted_same_module_function_symbols,
            emitted_same_module_function_symbols_n,
            name);
      }

      auto int module_has_planned_same_module_function_definition(void) {
        return planned_same_module_function_symbols_n > 0;
      }

      auto yyjson_val* same_module_function_find_with_index(
          const char* name,
          size_t* out_index) {
        if (!name) return NULL;
        for (size_t fi = 0; fi < program.function_count; fi++) {
          yyjson_val* fn = yyjson_arr_get(program.functions, fi);
          const char* fn_name = fn ? read_str(fn, "name") : NULL;
          if (fn_name && !strcmp(fn_name, name)) {
            if (out_index) *out_index = fi;
            return fn;
          }
        }
        return NULL;
      }

      auto yyjson_val* same_module_function_find(const char* name) {
        return same_module_function_find_with_index(name, NULL);
      }

      auto int same_module_function_find_index(
          const char* name,
          size_t* out_index) {
        return same_module_function_find_with_index(name, out_index) != NULL;
      }

      auto int plan_explicit_same_module_function_definition_rows(yyjson_val* fn) {
        yyjson_val* metadata = fn ? yyjson_obj_get(fn, "metadata") : NULL;
        yyjson_val* rows =
            metadata ? yyjson_obj_get(metadata, "same_module_function_definitions") : NULL;
        int planned = 0;
        if (!rows || yyjson_is_null(rows)) return 0;
        if (!yyjson_is_arr(rows)) return -1;
        for (size_t i = 0; i < yyjson_arr_size(rows); i++) {
          yyjson_val* row = yyjson_arr_get(rows, i);
          const char* target_symbol = row ? read_str(row, "target_symbol") : NULL;
          const char* definition_kind = row ? read_str(row, "definition_kind") : NULL;
          if (!(target_symbol && definition_kind)) return -1;
          if (!llvm_quoted_symbol_name_is_safe(target_symbol)) return -1;
          if (!strcmp(definition_kind, "leaf_i64_function")) {
            if (!remember_planned_module_leaf_symbol(target_symbol)) return -1;
            planned++;
            continue;
          }
          if (!strcmp(definition_kind, "same_module_function")) {
            if (!remember_planned_same_module_function_symbol(target_symbol)) return -1;
            planned++;
            continue;
          }
          return -1;
        }
        return planned;
      }

      auto int plan_same_module_function_definitions(void) {
        return plan_explicit_same_module_function_definition_rows(program.entry.fn);
      }
