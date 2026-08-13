// Open the unpublished IR file and emit artifact-bound module facts before
// function declarations.  Included inside the generic pure compile scope.
char llpath[1024];
snprintf(llpath, sizeof(llpath), "%s/hako_pure_gen_%d.ll", "/tmp", (int)getpid());
FILE* f = fopen(llpath, "wb");
if (!f) {
  note_pure_unsupported_shape(
      -1,
      0,
      "generic_pure_ir",
      "ir_temp_file_open_failed",
      NULL);
  yyjson_doc_free(d);
  break;
}
fprintf(f, "; nyash pure IR (generic)\n");
fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");
if (!hako_llvmc_c1_emit_artifact_descriptor(f, fn_metadata)) {
  note_pure_unsupported_shape(
      -1,
      0,
      "selected_dynamic_artifact_descriptor",
      "artifact_descriptor_projection_failed",
      NULL);
  fclose(f);
  yyjson_doc_free(d);
  goto GEN_ABORT;
}
