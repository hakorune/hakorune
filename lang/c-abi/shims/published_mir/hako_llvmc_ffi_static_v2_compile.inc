/* Private retained-input execution. Public open/query/compile/close activates
 * only with the complete consumer and host cutover. No Map-through-V1 route. */
static int hako_llvmc_compile_static_v2_retained(
    struct HakoLlvmcInvocation* invocation,
    const hako_llvmc_published_static_frame_v2* frame,
    const char* obj_out, char** error) {
  if (!invocation || !invocation->document || !obj_out || !*obj_out)
    return set_err_owned(error, "[freeze:contract][static-v2/input]");
  if (hako_llvmc_reject_ambient_harness_replay(error) != 0) return -1;
  if (hako_static_v2_bind(&invocation->static_v2, invocation->document, frame, error) != 0) return -1;
  int rc = -1;
  char* staging = NULL;
  if (hako_llvmc_published_call_rows_begin_v2(frame->calls, (size_t)frame->call_count, error) != 0)
    goto finish_frame;
  size_t length = strlen(obj_out);
  if (length > SIZE_MAX - 24) {
    set_err_owned(error, "[freeze:contract][static-v2/output-path]");
    goto finish_calls;
  }
  staging = malloc(length + 24);
  if (!staging) {
    set_err_owned(error, "[freeze:contract][static-v2/output-storage]");
    goto finish_calls;
  }
  snprintf(staging, length + 24, "%s.v2-XXXXXX", obj_out);
  int fd = mkstemp(staging);
  if (fd < 0) {
    set_err_owned(error, "[freeze:contract][static-v2/staging-open]");
    goto finish_calls;
  }
  if (close(fd) != 0 || remove(staging) != 0) {
    set_err_owned(error, "[freeze:contract][static-v2/staging-close]");
    goto finish_calls;
  }
  /* Early optimizer returns may bypass the generic pre-object residual check. */
  rc = compile_doc_compat_pure(invocation, NULL, staging, error);
  if (rc == 0) rc = hako_llvmc_published_static_method_rows_finish(error);
  if (rc == 0) rc = hako_static_v2_finish(&invocation->static_v2, error);
  if (rc == 0 && rename(staging, obj_out) != 0)
    rc = set_err_owned(error, "[freeze:contract][static-v2/publish]");
finish_calls:
  if (staging) { remove(staging); free(staging); }
  hako_llvmc_published_static_method_rows_end();
finish_frame:
  hako_static_v2_clear(&invocation->static_v2);
  return rc;
}
