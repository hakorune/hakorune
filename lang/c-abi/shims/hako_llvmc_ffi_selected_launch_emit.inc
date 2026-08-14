/*
 * Selected Dynamic launch projection.
 *
 * The identity census already selected the zero-argument launch function.
 * Reuse the existing generic function pipeline for that borrowed body and
 * emit the runtime's canonical `ny_main` symbol.  No selected metadata is
 * copied, no stub is invented, and no launch arguments are synthesized.
 */
if (program.selected_dynamic) {
  struct SameModuleFunctionContextSnapshot launch_snapshot;
  hako_llvmc_generic_pure_function_view launch_function = program.launch;
  int launch_rc;

  if (!(launch_function.fn && launch_function.blocks)) {
    note_pure_unsupported_shape(
        -1,
        0,
        "selected_dynamic_launch",
        "launch_function_view_missing",
        NULL);
    fclose(f);
    yyjson_doc_free(d);
    goto GEN_ABORT;
  }
  if (yyjson_arr_size(yyjson_obj_get(launch_function.fn, "params")) != 0) {
    note_pure_unsupported_shape(
        -1,
        0,
        "selected_dynamic_launch",
        "launch_function_requires_zero_parameters",
        NULL);
    fclose(f);
    yyjson_doc_free(d);
    goto GEN_ABORT;
  }

  same_module_function_save_function_context(&launch_snapshot);
  same_module_function_activate_function_context(&launch_function);
  launch_rc = same_module_function_emit_function_pipeline(
      f,
      launch_function.fn,
      launch_function.index,
      "ny_main");
  same_module_function_restore_function_context(&launch_snapshot);
  if (launch_rc != 1) {
    note_pure_unsupported_shape(
        -1,
        0,
        "selected_dynamic_launch",
        "launch_function_emission_failed",
        NULL);
    fclose(f);
    yyjson_doc_free(d);
    goto GEN_ABORT;
  }
}
