      auto int same_module_function_definition_is_eligible(
          yyjson_val* fn,
          const char* name) {
        if (!name ||
            !llvm_quoted_symbol_name_is_safe(name) ||
            !same_module_function_symbol_will_be_defined(name) ||
            same_module_function_symbol_was_emitted(name)) {
          return 0;
        }
        if (module_function_is_numeric_i64_leaf(fn) &&
            module_leaf_symbol_will_be_defined(name)) {
          return 0;
        }
        return 1;
      }

      auto int same_module_function_read_view(
          yyjson_val* fn,
          size_t fn_index,
          hako_llvmc_generic_pure_function_view* out) {
        return hako_llvmc_read_generic_pure_function_view(
                   fn,
                   fn_index,
                   0,
                   out) &&
               out->metadata;
      }

      struct SameModuleFunctionDefinitionTarget {
        yyjson_val* fn;
        size_t fn_index;
      };

      auto int same_module_function_find_planned_definition(
          const char* name,
          struct SameModuleFunctionDefinitionTarget* out) {
        yyjson_val* fn = NULL;
        size_t fn_index = 0;
        if (!out) return 0;
        out->fn = NULL;
        out->fn_index = 0;
        fn = same_module_function_find_with_index(name, &fn_index);
        if (!fn || fn == program.entry.fn) return 0;
        out->fn = fn;
        out->fn_index = fn_index;
        return 1;
      }

      auto int same_module_function_prepare_definition(
          yyjson_val* fn,
          size_t fn_index,
          const char** out_name,
          hako_llvmc_generic_pure_function_view* out_function) {
        const char* name = fn ? read_str(fn, "name") : NULL;
        if (!same_module_function_definition_is_eligible(fn, name)) {
          return 0;
        }
        if (!same_module_function_read_view(fn, fn_index, out_function)) {
          return -1;
        }
        if (out_name) *out_name = name;
        return 1;
      }

      auto int emit_same_module_function_definition(
          FILE* f,
          yyjson_val* fn,
          size_t fn_index) {
        struct SameModuleFunctionContextSnapshot* saved_context;
        hako_llvmc_generic_pure_function_view local_function;
        const char* name = NULL;
        int rc = -1;
        rc = same_module_function_prepare_definition(
            fn,
            fn_index,
            &name,
            &local_function);
        if (rc <= 0) {
          return rc;
        }

        saved_context = calloc(1, sizeof(*saved_context));
        if (!saved_context) {
          note_pure_unsupported_shape(
              -1,
              0,
              "same_module_function_definition",
              "context_snapshot_allocation_failed",
              NULL);
          return -1;
        }

        same_module_function_save_function_context(saved_context);
        same_module_function_activate_function_context(&local_function);

        rc = same_module_function_emit_function_pipeline(
            f,
            fn,
            fn_index,
            name);

        same_module_function_restore_function_context(saved_context);
        free(saved_context);
        return rc;
      }

      auto int emit_planned_same_module_function_definition(
          FILE* f,
          const char* name,
          int* emitted) {
        struct SameModuleFunctionDefinitionTarget target;
        int rc = 0;
        if (!same_module_function_find_planned_definition(name, &target)) return 1;
        rc = emit_same_module_function_definition(
            f,
            target.fn,
            target.fn_index);
        if (rc < 0) return -1;
        if (rc > 0 && emitted) (*emitted)++;
        return 1;
      }

      auto int emit_same_module_function_definitions(FILE* f) {
        int emitted = 0;
        for (size_t i = 0; i < planned_same_module_function_symbols_n; i++) {
          const char* name = planned_same_module_function_symbols[i];
          int rc = emit_planned_same_module_function_definition(
              f,
              name,
              &emitted);
          if (rc < 0) return -1;
        }
        reset_string_concat_tracking_state();
        return emitted;
      }
