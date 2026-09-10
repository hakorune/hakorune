/* Lifecycle-only LLVM target observation. No pinned-text contract enters here. */
#include "../../../../include/nyrt_fault_v1.h"

typedef void (*hako_lts_init_fn)(void);
typedef void (*hako_lts_dispose_message_fn)(char*);
typedef void (*hako_lts_dispose_machine_fn)(void*);
typedef void (*hako_lts_dispose_data_fn)(void*);
typedef void (*hako_lts_dispose_context_fn)(void*);
typedef void (*hako_lts_get_version_fn)(unsigned*, unsigned*, unsigned*);
typedef int (*hako_lts_get_target_fn)(const char*, void**, char**);
typedef void* (*hako_lts_make_machine_fn)(void*, const char*, const char*, const char*, int, int, int);
typedef char* (*hako_lts_machine_triple_fn)(void*);
typedef void* (*hako_lts_make_data_fn)(void*);
typedef char* (*hako_lts_data_layout_text_fn)(void*);
typedef unsigned (*hako_lts_pointer_size_fn)(void*);
typedef void* (*hako_lts_make_context_fn)(void);
typedef void (*hako_lts_dispose_memory_buffer_fn)(void*);
typedef int (*hako_lts_create_memory_buffer_fn)(const char*, void**, char**);
typedef int (*hako_lts_parse_ir_fn)(void*, void*, void**, char**);
typedef void (*hako_lts_dispose_module_fn)(void*);
typedef int (*hako_lts_verify_module_fn)(void*, int, char**);
typedef const char* (*hako_lts_module_text_fn)(void*);
typedef int (*hako_lts_emit_to_file_fn)(void*, void*, char*, int, char**);
typedef void* (*hako_lts_int_type_fn)(void*);
typedef void* (*hako_lts_pointer_type_fn)(void*, unsigned);
typedef void* (*hako_lts_array_type_fn)(void*, uint64_t);
typedef void* (*hako_lts_struct_type_fn)(void*, void**, unsigned, int);
typedef unsigned long long (*hako_lts_abi_size_fn)(void*, void*);
typedef unsigned (*hako_lts_abi_align_fn)(void*, void*);
typedef unsigned long long (*hako_lts_element_offset_fn)(void*, void*, unsigned);

#if defined(HAKO_LLVMC_LIFECYCLE_TEST_SEAM)
static int hako_lts_test_mode(const char* expected) {
  const char* mode = getenv("HAKO_LIFECYCLE_TEST_SEAM");
  return mode && expected && strcmp(mode, expected) == 0;
}
#endif

/* One invocation owns every LLVM resource through object publication. */
typedef struct {
  void *lib, *machine, *data;
  char *triple, *data_layout;
  hako_lts_dispose_message_fn dispose_message;
  hako_lts_dispose_machine_fn dispose_machine;
  hako_lts_dispose_data_fn dispose_data;
  hako_lts_make_context_fn make_context;
  hako_lts_dispose_context_fn dispose_context;
  hako_lts_create_memory_buffer_fn create_buffer;
  hako_lts_dispose_memory_buffer_fn dispose_buffer;
  hako_lts_parse_ir_fn parse_ir;
  hako_lts_dispose_module_fn dispose_module;
  hako_lts_verify_module_fn verify_module;
  hako_lts_module_text_fn module_target;
  hako_lts_module_text_fn module_data_layout;
  hako_lts_emit_to_file_fn emit_to_file;
} hako_lts_session;

static void hako_lts_close(hako_lts_session* session) {
  if (!session) return;
  if (session->data_layout) session->dispose_message(session->data_layout);
  if (session->triple) session->dispose_message(session->triple);
  if (session->data) session->dispose_data(session->data);
  if (session->machine) session->dispose_machine(session->machine);
#if !defined(_WIN32)
  if (session->lib) dlclose(session->lib);
#endif
  memset(session, 0, sizeof(*session));
}

static int hako_lts_fault_layout_matches(
    void* data, const hako_llvmc_lifecycle_target_session_v2* in,
    hako_lts_make_context_fn make_context, hako_lts_dispose_context_fn dispose_context,
    hako_lts_int_type_fn int32_type, hako_lts_int_type_fn int64_type,
    hako_lts_pointer_type_fn pointer_type, hako_lts_array_type_fn array_type,
    hako_lts_struct_type_fn struct_type, hako_lts_abi_size_fn abi_size,
    hako_lts_abi_align_fn abi_align, hako_lts_element_offset_fn element_offset) {
  void *context = NULL, *i32, *i64, *pointer, *details, *diagnostic, *suppressed, *frame;
  void* diagnostic_fields[6];
  void* frame_fields[6];
  int matched = 0;
  if (!make_context || !dispose_context || !int32_type || !int64_type || !pointer_type ||
      !array_type || !struct_type || !abi_size || !abi_align || !element_offset) return 0;
  context = make_context();
  if (!context) return 0;
  i32 = int32_type(context); i64 = int64_type(context); pointer = pointer_type(context, 0);
  details = i64 ? array_type(i64, 2u) : NULL;
  if (!i32 || !i64 || !pointer || !details) goto done;
  diagnostic_fields[0] = i32; diagnostic_fields[1] = i32; diagnostic_fields[2] = i64;
  diagnostic_fields[3] = details; diagnostic_fields[4] = pointer; diagnostic_fields[5] = pointer;
  diagnostic = struct_type(context, diagnostic_fields, 6u, 0);
  suppressed = diagnostic ? array_type(diagnostic, 8u) : NULL;
  if (!diagnostic || !suppressed) goto done;
  frame_fields[0] = i32; frame_fields[1] = i32; frame_fields[2] = i32;
  frame_fields[3] = i32; frame_fields[4] = diagnostic; frame_fields[5] = suppressed;
  frame = struct_type(context, frame_fields, 6u, 0);
  if (!frame) goto done;
  matched = abi_size(data, diagnostic) == in->diagnostic_size &&
      abi_align(data, diagnostic) == in->diagnostic_align &&
      element_offset(data, diagnostic, 2u) == in->diagnostic_site_offset &&
      element_offset(data, diagnostic, 3u) == in->diagnostic_details_offset &&
      element_offset(data, diagnostic, 4u) == in->diagnostic_message_offset &&
      abi_size(data, frame) == in->frame_size && abi_align(data, frame) == in->frame_align &&
      element_offset(data, frame, 4u) == in->frame_primary_offset &&
      element_offset(data, frame, 5u) == in->frame_suppressed_offset;
done:
  dispose_context(context);
  return matched;
}

static int hako_lts_open(
    const hako_llvmc_lifecycle_target_session_v2* in, hako_lts_session* out, char** err_out) {
  if (!out) return set_err_owned(err_out, "[freeze:contract][lifecycle-session/output]");
  memset(out, 0, sizeof(*out));
#if defined(_WIN32)
  (void)in;
  return set_err_owned(err_out, "[freeze:contract][lifecycle-session/windows]");
#else
  void *lib = NULL, *target = NULL, *machine = NULL, *data = NULL;
  char *diagnostic = NULL, *actual_triple = NULL, *data_layout = NULL;
  unsigned major = 0, minor = 0, patch = 0;
  hako_lts_dispose_message_fn dispose_message;
  hako_lts_dispose_machine_fn dispose_machine;
  hako_lts_dispose_data_fn dispose_data;
  hako_lts_dispose_context_fn dispose_context;
  hako_lts_dispose_memory_buffer_fn dispose_buffer;
  hako_lts_create_memory_buffer_fn create_buffer;
  hako_lts_parse_ir_fn parse_ir;
  hako_lts_dispose_module_fn dispose_module;
  hako_lts_verify_module_fn verify_module;
  hako_lts_module_text_fn module_target;
  hako_lts_module_text_fn module_data_layout;
  hako_lts_emit_to_file_fn emit_to_file;
  hako_lts_get_version_fn get_version;
  hako_lts_get_target_fn get_target;
  hako_lts_make_machine_fn make_machine;
  hako_lts_machine_triple_fn machine_triple;
  hako_lts_make_data_fn make_data;
  hako_lts_data_layout_text_fn data_layout_text;
  hako_lts_pointer_size_fn pointer_size;
  hako_lts_make_context_fn make_context;
  hako_lts_int_type_fn int32_type, int64_type;
  hako_lts_pointer_type_fn pointer_type;
  hako_lts_array_type_fn array_type;
  hako_lts_struct_type_fn struct_type;
  hako_lts_abi_size_fn abi_size;
  hako_lts_abi_align_fn abi_align;
  hako_lts_element_offset_fn element_offset;
  hako_lts_init_fn init_info, init_target, init_mc, init_printer;
  int rc = -1;
  if (!in || in->revision != 2u || !in->target_triple || !in->target_triple[0])
    return set_err_owned(err_out, "[freeze:contract][lifecycle-session/input]");
#if defined(HAKO_LLVMC_LIFECYCLE_TEST_SEAM)
  if (hako_lts_test_mode("missing-library"))
    return set_err_owned(err_out, "[freeze:contract][lifecycle-session/test-missing-library]");
#endif
  if (in->endian != 1u || in->fault_abi_version != NYRT_FAULT_ABI_VERSION_V1 ||
      in->status_abi_version != 1u || in->pointer_width != sizeof(void*) ||
      in->diagnostic_size != sizeof(NyrtFaultDiagnosticV1) ||
      in->diagnostic_align != _Alignof(NyrtFaultDiagnosticV1) ||
      in->diagnostic_site_offset != offsetof(NyrtFaultDiagnosticV1, site) ||
      in->diagnostic_details_offset != offsetof(NyrtFaultDiagnosticV1, details) ||
      in->diagnostic_message_offset != offsetof(NyrtFaultDiagnosticV1, runtime_private_message) ||
      in->frame_size != sizeof(NyrtFaultFrameV1) || in->frame_align != _Alignof(NyrtFaultFrameV1) ||
      in->frame_primary_offset != offsetof(NyrtFaultFrameV1, primary) ||
      in->frame_suppressed_offset != offsetof(NyrtFaultFrameV1, suppressed))
    return set_err_owned(err_out, "[freeze:contract][lifecycle-session/runtime-abi]");
  if (in->map_revision != 1u || !in->map_size || !in->map_align ||
      (in->map_align & (in->map_align - 1u)) || in->map_size % in->map_align)
    return set_err_owned(err_out, "[freeze:contract][lifecycle-session/opaque-map]");
  if (in->key_revision != 1u || !in->key_size || !in->key_align ||
      (in->key_align & (in->key_align - 1u)) || in->key_size % in->key_align)
    return set_err_owned(err_out, "[freeze:contract][lifecycle-session/opaque-key]");
  if (in->outcome_revision != 1u || !in->outcome_size || !in->outcome_align ||
      (in->outcome_align & (in->outcome_align - 1u)) || in->outcome_size % in->outcome_align)
    return set_err_owned(err_out, "[freeze:contract][lifecycle-session/opaque-outcome]");
  lib = dlopen("libLLVM-18.so", RTLD_LAZY | RTLD_LOCAL);
  if (!lib) lib = dlopen("libLLVM.so.18", RTLD_LAZY | RTLD_LOCAL);
  if (!lib) return set_err_owned(err_out, "[freeze:contract][lifecycle-session/llvm-unavailable]");
#define HAKO_LTS_SYM(name, type) ((type)dlsym(lib, name))
  dispose_message = HAKO_LTS_SYM("LLVMDisposeMessage", hako_lts_dispose_message_fn);
  dispose_machine = HAKO_LTS_SYM("LLVMDisposeTargetMachine", hako_lts_dispose_machine_fn);
  dispose_data = HAKO_LTS_SYM("LLVMDisposeTargetData", hako_lts_dispose_data_fn);
  dispose_context = HAKO_LTS_SYM("LLVMContextDispose", hako_lts_dispose_context_fn);
  dispose_buffer = HAKO_LTS_SYM("LLVMDisposeMemoryBuffer", hako_lts_dispose_memory_buffer_fn);
  create_buffer = HAKO_LTS_SYM(
      "LLVMCreateMemoryBufferWithContentsOfFile", hako_lts_create_memory_buffer_fn);
  parse_ir = HAKO_LTS_SYM("LLVMParseIRInContext", hako_lts_parse_ir_fn);
  dispose_module = HAKO_LTS_SYM("LLVMDisposeModule", hako_lts_dispose_module_fn);
  verify_module = HAKO_LTS_SYM("LLVMVerifyModule", hako_lts_verify_module_fn);
  module_target = HAKO_LTS_SYM("LLVMGetTarget", hako_lts_module_text_fn);
  module_data_layout = HAKO_LTS_SYM("LLVMGetDataLayoutStr", hako_lts_module_text_fn);
  emit_to_file = HAKO_LTS_SYM("LLVMTargetMachineEmitToFile", hako_lts_emit_to_file_fn);
#if defined(HAKO_LLVMC_LIFECYCLE_TEST_SEAM)
  if (hako_lts_test_mode("missing-symbol")) emit_to_file = NULL;
#endif
  get_version = HAKO_LTS_SYM("LLVMGetVersion", hako_lts_get_version_fn);
  get_target = HAKO_LTS_SYM("LLVMGetTargetFromTriple", hako_lts_get_target_fn);
  make_machine = HAKO_LTS_SYM("LLVMCreateTargetMachine", hako_lts_make_machine_fn);
  machine_triple = HAKO_LTS_SYM("LLVMGetTargetMachineTriple", hako_lts_machine_triple_fn);
  make_data = HAKO_LTS_SYM("LLVMCreateTargetDataLayout", hako_lts_make_data_fn);
  data_layout_text = HAKO_LTS_SYM("LLVMCopyStringRepOfTargetData", hako_lts_data_layout_text_fn);
  pointer_size = HAKO_LTS_SYM("LLVMPointerSize", hako_lts_pointer_size_fn);
  make_context = HAKO_LTS_SYM("LLVMContextCreate", hako_lts_make_context_fn);
  int32_type = HAKO_LTS_SYM("LLVMInt32TypeInContext", hako_lts_int_type_fn);
  int64_type = HAKO_LTS_SYM("LLVMInt64TypeInContext", hako_lts_int_type_fn);
  pointer_type = HAKO_LTS_SYM("LLVMPointerTypeInContext", hako_lts_pointer_type_fn);
  array_type = HAKO_LTS_SYM("LLVMArrayType2", hako_lts_array_type_fn);
  struct_type = HAKO_LTS_SYM("LLVMStructTypeInContext", hako_lts_struct_type_fn);
  abi_size = HAKO_LTS_SYM("LLVMABISizeOfType", hako_lts_abi_size_fn);
  abi_align = HAKO_LTS_SYM("LLVMABIAlignmentOfType", hako_lts_abi_align_fn);
  element_offset = HAKO_LTS_SYM("LLVMOffsetOfElement", hako_lts_element_offset_fn);
  init_info = HAKO_LTS_SYM("LLVMInitializeX86TargetInfo", hako_lts_init_fn);
  init_target = HAKO_LTS_SYM("LLVMInitializeX86Target", hako_lts_init_fn);
  init_mc = HAKO_LTS_SYM("LLVMInitializeX86TargetMC", hako_lts_init_fn);
  init_printer = HAKO_LTS_SYM("LLVMInitializeX86AsmPrinter", hako_lts_init_fn);
#undef HAKO_LTS_SYM
  if (!dispose_message || !dispose_machine || !dispose_data || !dispose_context ||
      !dispose_buffer || !create_buffer || !parse_ir || !dispose_module || !verify_module ||
      !module_target || !module_data_layout || !emit_to_file || !get_version ||
      !get_target || !make_machine || !machine_triple || !make_data || !data_layout_text || !pointer_size ||
      !make_context || !int32_type || !int64_type || !pointer_type || !array_type || !struct_type ||
      !abi_size || !abi_align || !element_offset || !init_info || !init_target || !init_mc || !init_printer)
    goto cleanup;
  get_version(&major, &minor, &patch);
  if (major != 18u) goto cleanup;
  init_info(); init_target(); init_mc(); init_printer();
  if (get_target(in->target_triple, &target, &diagnostic) != 0 || !target) goto cleanup;
  if (diagnostic) { dispose_message(diagnostic); diagnostic = NULL; }
  machine = make_machine(target, in->target_triple, "generic", "", 0, 2, 0);
  data = machine ? make_data(machine) : NULL;
  actual_triple = machine ? machine_triple(machine) : NULL;
  data_layout = data ? data_layout_text(data) : NULL;
  if (!data || !actual_triple || strcmp(actual_triple, in->target_triple) != 0 ||
      !data_layout || !data_layout[0] ||
      pointer_size(data) != in->pointer_width ||
      !hako_lts_fault_layout_matches(data, in, make_context, dispose_context, int32_type,
          int64_type, pointer_type, array_type, struct_type, abi_size, abi_align, element_offset))
    goto cleanup;
  rc = 0;
cleanup:
  if (rc == 0) {
    out->lib = lib; out->machine = machine; out->data = data;
    out->triple = actual_triple; out->data_layout = data_layout;
    out->dispose_message = dispose_message;
    out->dispose_machine = dispose_machine;
    out->dispose_data = dispose_data;
    out->make_context = make_context;
    out->dispose_context = dispose_context;
    out->create_buffer = create_buffer;
    out->dispose_buffer = dispose_buffer;
    out->parse_ir = parse_ir;
    out->dispose_module = dispose_module;
    out->verify_module = verify_module;
    out->module_target = module_target;
    out->module_data_layout = module_data_layout;
    out->emit_to_file = emit_to_file;
    lib = machine = data = NULL;
    actual_triple = data_layout = NULL;
  }
  if (diagnostic && dispose_message) dispose_message(diagnostic);
  if (data_layout && dispose_message) dispose_message(data_layout);
  if (actual_triple && dispose_message) dispose_message(actual_triple);
  if (data && dispose_data) dispose_data(data);
  if (machine && dispose_machine) dispose_machine(machine);
  if (lib) dlclose(lib);
  if (rc != 0) return set_err_owned(err_out, "[freeze:contract][lifecycle-session/llvm-layout]");
  return 0;
#endif
}

static void hako_lts_take_diagnostic(
    hako_lts_session* session, char** err_out, char** diagnostic, const char* fallback) {
  const char* message = (diagnostic && *diagnostic && **diagnostic) ? *diagnostic : fallback;
  set_err_owned(err_out, message);
  if (diagnostic && *diagnostic && session && session->dispose_message) {
    session->dispose_message(*diagnostic);
    *diagnostic = NULL;
  }
}

/* Parse, verify, and emit one file-backed module through the invocation-owned
 * target session.  LLVMParseIRInContext consumes its memory buffer on both
 * success and failure; ownership is therefore cleared immediately after the
 * call and the cleanup path never disposes it twice. */
static int hako_lts_emit_object_from_ir(
    hako_lts_session* session, const char* ll_path, const char* obj_path, char** err_out) {
#if defined(_WIN32)
  (void)session;
  (void)ll_path;
  (void)obj_path;
  return set_err_owned(err_out, "[freeze:contract][lifecycle-session/windows]");
#else
  void* context = NULL;
  void* buffer = NULL;
  void* module = NULL;
  char* diagnostic = NULL;
  int rc = -1;
  if (!session || !session->make_context || !session->create_buffer || !session->parse_ir ||
      !session->verify_module || !session->module_target || !session->module_data_layout ||
      !session->emit_to_file || !ll_path || !ll_path[0] || !obj_path || !obj_path[0]) {
    return set_err_owned(err_out, "[freeze:contract][lifecycle-session/emit-input]");
  }
  context = session->make_context();
  if (!context) {
    return set_err_owned(err_out, "[freeze:contract][lifecycle-session/context]");
  }
  if (session->create_buffer(ll_path, &buffer, &diagnostic) != 0 || !buffer) {
    hako_lts_take_diagnostic(
        session, err_out, &diagnostic, "[freeze:contract][lifecycle-session/buffer]");
    goto done;
  }
  if (diagnostic && session->dispose_message) {
    session->dispose_message(diagnostic);
    diagnostic = NULL;
  }
  {
    int parse_failed = session->parse_ir(context, buffer, &module, &diagnostic);
    buffer = NULL;
    if (parse_failed != 0 || !module) {
      hako_lts_take_diagnostic(
          session, err_out, &diagnostic, "[freeze:contract][lifecycle-session/parse]");
      goto done;
    }
  }
  if (session->verify_module(module, 2, &diagnostic) != 0) {
    hako_lts_take_diagnostic(
        session, err_out, &diagnostic, "[freeze:contract][lifecycle-session/verify]");
    goto done;
  }
  if (diagnostic && session->dispose_message) {
    session->dispose_message(diagnostic);
    diagnostic = NULL;
  }
  {
    const char* module_target = session->module_target(module);
    const char* module_layout = session->module_data_layout(module);
    if (!module_target || !module_layout || strcmp(module_target, session->triple) != 0 ||
        strcmp(module_layout, session->data_layout) != 0) {
      hako_lts_take_diagnostic(
          session, err_out, &diagnostic, "[freeze:contract][lifecycle-session/module-layout]");
      goto done;
    }
  }
  if (session->emit_to_file(session->machine, module, (char*)obj_path, 1, &diagnostic) != 0) {
    hako_lts_take_diagnostic(
        session, err_out, &diagnostic, "[freeze:contract][lifecycle-session/emit]");
    goto done;
  }
  if (diagnostic && session->dispose_message) {
    session->dispose_message(diagnostic);
    diagnostic = NULL;
  }
#if defined(HAKO_LLVMC_LIFECYCLE_TEST_SEAM)
  if (hako_lts_test_mode("emit-failure")) {
    set_err_owned(err_out, "[freeze:contract][lifecycle-session/test-emit-failure]");
    goto done;
  }
  if (hako_lts_test_mode("empty-output")) {
    FILE* empty = fopen(obj_path, "wb");
    if (empty) fclose(empty);
  }
#endif
  rc = 0;
done:
  if (diagnostic && session->dispose_message) session->dispose_message(diagnostic);
  if (buffer && session->dispose_buffer) session->dispose_buffer(buffer);
  if (module && session->dispose_module) session->dispose_module(module);
  if (context && session->dispose_context) session->dispose_context(context);
  return rc;
#endif
}
