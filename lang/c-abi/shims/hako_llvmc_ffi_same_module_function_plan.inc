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

      auto int plan_function_global_call_definition_edges(yyjson_val* fn) {
        int planned = 0;
        yyjson_val* metadata = fn ? yyjson_obj_get(fn, "metadata") : NULL;
        yyjson_val* entries = metadata ? yyjson_obj_get(metadata, "lowering_plan") : NULL;
        if (!(entries && yyjson_is_arr(entries))) return 0;
        for (size_t pi = 0; pi < yyjson_arr_size(entries); pi++) {
          yyjson_val* entry = yyjson_arr_get(entries, pi);
          struct LoweringPlanGlobalCallView view;
          if (!read_lowering_plan_global_call_view(entry, &view)) continue;
          if (lowering_plan_global_call_view_uses_leaf_i64_definition(&view)) {
            if (!remember_planned_module_leaf_symbol(view.target_symbol)) return -1;
            planned++;
            continue;
          }
          if (lowering_plan_global_call_view_uses_generic_i64_or_leaf_definition(&view)) {
            yyjson_val* target_fn = same_module_function_find(view.target_symbol);
            if (module_function_is_numeric_i64_leaf(target_fn)) {
              if (!remember_planned_module_leaf_symbol(view.target_symbol)) return -1;
              planned++;
              continue;
            }
          }
          if (!lowering_plan_global_call_view_requires_same_module_function_definition(&view)) {
            continue;
          }
          if (!remember_planned_same_module_function_symbol(view.target_symbol)) return -1;
          planned++;
        }
        return planned;
      }

      auto int plan_same_module_function_definitions(void) {
        int planned = plan_function_global_call_definition_edges(program.entry.fn);
        size_t cursor = 0;
        if (planned < 0) return -1;
        while (cursor < planned_same_module_function_symbols_n) {
          const char* symbol = planned_same_module_function_symbols[cursor++];
          yyjson_val* fn = same_module_function_find(symbol);
          int rc = plan_function_global_call_definition_edges(fn);
          if (rc < 0) return -1;
          planned += rc;
        }
        return planned;
      }
