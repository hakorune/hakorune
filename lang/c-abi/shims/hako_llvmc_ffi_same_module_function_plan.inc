      /* Same-module emitted-symbol progress and function lookup.
       * Definition intent is borrowed from PhysicalDefinitionPlan; these
       * registries track emission only and never select a new role. */
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

      auto yyjson_val* same_module_function_find_with_index(
          const char* name,
          size_t* out_index) {
        if (!name) return NULL;
        for (size_t fi = 0; fi < program->function_count; fi++) {
          yyjson_val* fn = yyjson_arr_get(program->functions, fi);
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
