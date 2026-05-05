      /*
       * Module generic pure string function plan registry.
       *
       * MIR/LoweringPlan owns the permission (`generic_pure_string_body`).
       * This registry only records same-module functions that must be emitted
       * as LLVM definitions so direct calls cannot silently externalize them.
       */
      enum ModuleGenericStringPlannedKind {
        MODULE_GENERIC_STRING_PLANNED_GENERIC = 0,
        MODULE_GENERIC_STRING_PLANNED_PARSER_PROGRAM_JSON = 1,
        MODULE_GENERIC_STRING_PLANNED_STATIC_STRING_ARRAY = 2,
      };

      const char* planned_module_generic_string_symbols[1024];
      int planned_module_generic_string_kinds[1024];
      size_t planned_module_generic_string_symbols_n = 0;
      const char* emitted_module_generic_string_symbols[1024];
      size_t emitted_module_generic_string_symbols_n = 0;

      auto int module_generic_planned_kind_is_special(int kind) {
        return kind == MODULE_GENERIC_STRING_PLANNED_PARSER_PROGRAM_JSON ||
               kind == MODULE_GENERIC_STRING_PLANNED_STATIC_STRING_ARRAY;
      }

      auto int remember_planned_module_generic_string_symbol_kind(
          const char* name,
          int kind) {
        if (!name) return 0;
        for (size_t i = 0; i < planned_module_generic_string_symbols_n; i++) {
          if (!strcmp(planned_module_generic_string_symbols[i], name)) {
            int existing = planned_module_generic_string_kinds[i];
            if (existing == kind || kind == MODULE_GENERIC_STRING_PLANNED_GENERIC) return 1;
            if (existing == MODULE_GENERIC_STRING_PLANNED_GENERIC &&
                module_generic_planned_kind_is_special(kind)) {
              planned_module_generic_string_kinds[i] = kind;
              return 1;
            }
            return 0;
          }
        }
        if (planned_module_generic_string_symbols_n >=
            ARR_LEN(planned_module_generic_string_symbols)) {
          note_pure_unsupported_shape_detail(
              -1,
              0,
              "module_function",
              "module_generic_registry_full",
              NULL,
              NULL,
              name,
              NULL);
          return 0;
        }
        planned_module_generic_string_symbols[planned_module_generic_string_symbols_n] = name;
        planned_module_generic_string_kinds[planned_module_generic_string_symbols_n] = kind;
        planned_module_generic_string_symbols_n++;
        return 1;
      }

      auto int remember_planned_module_generic_string_symbol(const char* name) {
        return remember_planned_module_generic_string_symbol_kind(
            name,
            MODULE_GENERIC_STRING_PLANNED_GENERIC);
      }

      auto int module_generic_symbol_will_be_defined(const char* name) {
        if (!name) return 0;
        for (size_t i = 0; i < planned_module_generic_string_symbols_n; i++) {
          if (!strcmp(planned_module_generic_string_symbols[i], name)) return 1;
        }
        return 0;
      }

      auto int module_generic_symbol_planned_kind(const char* name) {
        if (!name) return 0;
        for (size_t i = 0; i < planned_module_generic_string_symbols_n; i++) {
          if (!strcmp(planned_module_generic_string_symbols[i], name)) {
            return planned_module_generic_string_kinds[i];
          }
        }
        return 0;
      }

      auto int module_generic_symbol_is_parser_program_json(const char* name) {
        return module_generic_symbol_planned_kind(name) ==
               MODULE_GENERIC_STRING_PLANNED_PARSER_PROGRAM_JSON;
      }

      auto int module_generic_symbol_is_static_string_array(const char* name) {
        return module_generic_symbol_planned_kind(name) ==
               MODULE_GENERIC_STRING_PLANNED_STATIC_STRING_ARRAY;
      }

      auto int remember_emitted_module_generic_string_symbol(const char* name) {
        if (!name) return 0;
        for (size_t i = 0; i < emitted_module_generic_string_symbols_n; i++) {
          if (!strcmp(emitted_module_generic_string_symbols[i], name)) return 1;
        }
        if (emitted_module_generic_string_symbols_n >=
            ARR_LEN(emitted_module_generic_string_symbols)) {
          note_pure_unsupported_shape_detail(
              -1,
              0,
              "module_function",
              "module_generic_registry_full",
              NULL,
              NULL,
              name,
              NULL);
          return 0;
        }
        emitted_module_generic_string_symbols[emitted_module_generic_string_symbols_n++] = name;
        return 1;
      }

      auto int module_generic_symbol_was_emitted(const char* name) {
        if (!name) return 0;
        for (size_t i = 0; i < emitted_module_generic_string_symbols_n; i++) {
          if (!strcmp(emitted_module_generic_string_symbols[i], name)) return 1;
        }
        return 0;
      }

      auto int module_has_planned_generic_string_definition(void) {
        return planned_module_generic_string_symbols_n > 0;
      }

      auto int module_has_planned_parser_program_json_definition(void) {
        for (size_t i = 0; i < planned_module_generic_string_symbols_n; i++) {
          if (planned_module_generic_string_kinds[i] ==
              MODULE_GENERIC_STRING_PLANNED_PARSER_PROGRAM_JSON) {
            return 1;
          }
        }
        return 0;
      }

      auto yyjson_val* module_generic_string_find_function(const char* name) {
        if (!name) return NULL;
        for (size_t fi = 0; fi < program.function_count; fi++) {
          yyjson_val* fn = yyjson_arr_get(program.functions, fi);
          const char* fn_name = fn ? read_str(fn, "name") : NULL;
          if (fn_name && !strcmp(fn_name, name)) return fn;
        }
        return NULL;
      }

      auto int module_generic_string_find_function_index(const char* name, size_t* out_index) {
        if (!name) return 0;
        for (size_t fi = 0; fi < program.function_count; fi++) {
          yyjson_val* fn = yyjson_arr_get(program.functions, fi);
          const char* fn_name = fn ? read_str(fn, "name") : NULL;
          if (fn_name && !strcmp(fn_name, name)) {
            if (out_index) *out_index = fi;
            return 1;
          }
        }
        return 0;
      }

      auto int plan_module_generic_string_definition_edges(yyjson_val* fn) {
        int planned = 0;
        yyjson_val* metadata = fn ? yyjson_obj_get(fn, "metadata") : NULL;
        yyjson_val* entries = metadata ? yyjson_obj_get(metadata, "lowering_plan") : NULL;
        if (!(entries && yyjson_is_arr(entries))) return 0;
        for (size_t pi = 0; pi < yyjson_arr_size(entries); pi++) {
          yyjson_val* entry = yyjson_arr_get(entries, pi);
          struct LoweringPlanGlobalCallView view;
          int is_parser_program_json = 0;
          int is_static_string_array = 0;
          int is_mir_schema_map_constructor = 0;
          int is_box_type_inspector_describe = 0;
          int is_pattern_util_local_value_probe = 0;
          if (!read_lowering_plan_global_call_view(entry, &view)) continue;
          is_parser_program_json =
              lowering_plan_global_call_view_is_direct_parser_program_json(&view);
          is_static_string_array =
              lowering_plan_global_call_view_is_direct_static_string_array(&view);
          is_mir_schema_map_constructor =
              lowering_plan_global_call_view_is_direct_mir_schema_map_constructor(&view);
          is_box_type_inspector_describe =
              lowering_plan_global_call_view_is_direct_box_type_inspector_describe(&view);
          is_pattern_util_local_value_probe =
              lowering_plan_global_call_view_is_direct_pattern_util_local_value_probe(&view);
          if (lowering_plan_global_call_view_is_direct_leaf_i64(&view)) {
            if (!remember_planned_module_leaf_symbol(view.target_symbol)) return -1;
            planned++;
            continue;
          }
          int planned_kind = MODULE_GENERIC_STRING_PLANNED_GENERIC;
          if (is_parser_program_json) {
            planned_kind = MODULE_GENERIC_STRING_PLANNED_PARSER_PROGRAM_JSON;
          } else if (is_static_string_array) {
            planned_kind = MODULE_GENERIC_STRING_PLANNED_STATIC_STRING_ARRAY;
          }
          if (!(lowering_plan_global_call_view_is_direct_generic_string(&view) ||
                 lowering_plan_global_call_view_is_direct_void_sentinel_i64_zero(&view) ||
                 lowering_plan_global_call_view_is_direct_generic_i64(&view) ||
                 is_parser_program_json ||
                 is_static_string_array ||
                 is_mir_schema_map_constructor ||
                 is_box_type_inspector_describe ||
                 is_pattern_util_local_value_probe)) continue;
          if (!remember_planned_module_generic_string_symbol_kind(
                  view.target_symbol,
                  planned_kind)) return -1;
          planned++;
        }
        return planned;
      }

      auto int plan_module_generic_string_definitions(void) {
        int planned = plan_module_generic_string_definition_edges(program.entry.fn);
        size_t cursor = 0;
        if (planned < 0) return -1;
        while (cursor < planned_module_generic_string_symbols_n) {
          const char* symbol = planned_module_generic_string_symbols[cursor++];
          yyjson_val* fn = module_generic_string_find_function(symbol);
          int rc = plan_module_generic_string_definition_edges(fn);
          if (rc < 0) return -1;
          planned += rc;
        }
        return planned;
      }
