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
typedef void* (*hako_lts_int_type_fn)(void*);
typedef void* (*hako_lts_pointer_type_fn)(void*, unsigned);
typedef void* (*hako_lts_array_type_fn)(void*, uint64_t);
typedef void* (*hako_lts_struct_type_fn)(void*, void**, unsigned, int);
typedef unsigned long long (*hako_lts_abi_size_fn)(void*, void*);
typedef unsigned (*hako_lts_abi_align_fn)(void*, void*);
typedef unsigned long long (*hako_lts_element_offset_fn)(void*, void*, unsigned);

/* One invocation owns every LLVM resource through object publication. */
typedef struct {
  void *lib, *machine, *data;
  char *triple, *data_layout;
  hako_lts_dispose_message_fn dispose_message;
  hako_lts_dispose_machine_fn dispose_machine;
  hako_lts_dispose_data_fn dispose_data;
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
    void* data, const hako_llvmc_lifecycle_target_session_v1* in,
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

/* This is deliberately a text-only session check: it never selects a tool or emits an object. */
static int hako_lts_text_preamble_matches(const char* triple, const char* data_layout) {
  FILE* file;
  char *expected = NULL, *actual = NULL;
  size_t expected_len, read_count;
  int matched = 0;
  if (!triple || !data_layout) return 0;
  expected_len = strlen("target triple = \"\"\n") + strlen(triple) +
      strlen("target datalayout = \"\"\n") + strlen(data_layout);
  expected = (char*)malloc(expected_len + 1u);
  actual = (char*)malloc(expected_len + 1u);
  if (!expected || !actual) goto done;
  if (snprintf(expected, expected_len + 1u,
          "target triple = \"%s\"\ntarget datalayout = \"%s\"\n",
          triple, data_layout) != (int)expected_len) goto done;
  file = tmpfile();
  if (!file) goto done;
  if (fwrite(expected, 1u, expected_len, file) != expected_len || fflush(file) != 0 ||
      fseek(file, 0L, SEEK_SET) != 0) {
    fclose(file);
    goto done;
  }
  read_count = fread(actual, 1u, expected_len, file);
  fclose(file);
  actual[read_count] = '\0';
  matched = read_count == expected_len && strcmp(actual, expected) == 0;
done:
  free(actual);
  free(expected);
  return matched;
}

static int hako_lts_open(
    const hako_llvmc_lifecycle_target_session_v1* in, hako_lts_session* out, char** err_out) {
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
  if (!in || in->revision != 1u || !in->target_triple || !in->target_triple[0])
    return set_err_owned(err_out, "[freeze:contract][lifecycle-session/input]");
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
  lib = dlopen("libLLVM-18.so", RTLD_LAZY | RTLD_LOCAL);
  if (!lib) lib = dlopen("libLLVM.so.18", RTLD_LAZY | RTLD_LOCAL);
  if (!lib) return set_err_owned(err_out, "[freeze:contract][lifecycle-session/llvm-unavailable]");
#define HAKO_LTS_SYM(name, type) ((type)dlsym(lib, name))
  dispose_message = HAKO_LTS_SYM("LLVMDisposeMessage", hako_lts_dispose_message_fn);
  dispose_machine = HAKO_LTS_SYM("LLVMDisposeTargetMachine", hako_lts_dispose_machine_fn);
  dispose_data = HAKO_LTS_SYM("LLVMDisposeTargetData", hako_lts_dispose_data_fn);
  dispose_context = HAKO_LTS_SYM("LLVMContextDispose", hako_lts_dispose_context_fn);
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
  if (!dispose_message || !dispose_machine || !dispose_data || !dispose_context || !get_version ||
      !get_target || !make_machine || !machine_triple || !make_data || !data_layout_text || !pointer_size ||
      !make_context || !int32_type || !int64_type || !pointer_type || !array_type || !struct_type ||
      !abi_size || !abi_align || !element_offset || !init_info || !init_target || !init_mc || !init_printer)
    goto cleanup;
  get_version(&major, &minor, &patch);
  if (major != 18u) goto cleanup;
  init_info(); init_target(); init_mc(); init_printer();
  if (get_target(in->target_triple, &target, &diagnostic) != 0 || !target) goto cleanup;
  if (diagnostic) { dispose_message(diagnostic); diagnostic = NULL; }
  machine = make_machine(target, in->target_triple, "generic", "", 0, 0, 0);
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

static int hako_llvmc_lifecycle_text_tool_session_v1_validate(
    const hako_llvmc_lifecycle_target_session_v1* in, char** err_out) {
  hako_lts_session session = {0};
  int matched;
  if (hako_lts_open(in, &session, err_out) != 0) return -1;
  matched = hako_lts_text_preamble_matches(session.triple, session.data_layout);
  hako_lts_close(&session);
  if (!matched) return set_err_owned(err_out, "[freeze:contract][lifecycle-session/text-preamble]");
  return 0;
}
