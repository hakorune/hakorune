      /*
       * Same-module function context switching.
       *
       * This seam owns save/restore/activation of the currently emitted
       * function. It must not inspect MIR instructions or choose routes.
       */

      struct SameModuleFunctionContextSnapshot {
        hako_llvmc_generic_pure_function_view* active_function;
        yyjson_val* fn0;
        yyjson_val* fn_metadata;
        yyjson_val* blocks;
        size_t generic_pure_block_count;
        hako_llvmc_entry_rune_selection selection;
        struct GenericPureFunctionLoweringState lowering_state;
      };

      auto void same_module_function_save_function_context(
          struct SameModuleFunctionContextSnapshot* snapshot) {
        snapshot->active_function = active_function;
        snapshot->fn0 = fn0;
        snapshot->fn_metadata = fn_metadata;
        snapshot->blocks = blocks;
        snapshot->generic_pure_block_count = generic_pure_block_count;
        snapshot->selection = selection;
        snapshot->lowering_state = lowering_state;
      }

      auto void same_module_function_restore_function_context(
          const struct SameModuleFunctionContextSnapshot* snapshot) {
        free_owned_str_consts();
        reset_string_concat_tracking_state();
        lowering_state = snapshot->lowering_state;
        active_function = snapshot->active_function;
        fn0 = snapshot->fn0;
        fn_metadata = snapshot->fn_metadata;
        blocks = snapshot->blocks;
        generic_pure_block_count = snapshot->generic_pure_block_count;
        selection = snapshot->selection;
      }

      auto void same_module_function_activate_function_context(
          hako_llvmc_generic_pure_function_view* local_function) {
        active_function = local_function;
        fn0 = local_function->fn;
        fn_metadata = local_function->metadata;
        blocks = local_function->blocks;
        generic_pure_block_count = local_function->block_count;
        reset_generic_pure_function_lowering_state(&lowering_state);
        reset_string_concat_tracking_state();
      }
