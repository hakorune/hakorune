/*
 * Contract-bound LLVM C API target session.
 *
 * This is the only C-side realization owner for a module carrying the
 * pinned-Text backend-frame contract.  The Rust projection remains the
 * expectation authority; this session only observes LLVM 18 and retains the
 * exact TargetMachine used for both realization checks and object emission.
 */
#ifndef HAKO_LLVM_C_PTFB_TARGET_MACHINE_SESSION_INC
#define HAKO_LLVM_C_PTFB_TARGET_MACHINE_SESSION_INC

#if !defined(_WIN32)
#include <dlfcn.h>
#include <errno.h>
#include <unistd.h>
#endif

typedef void (*hako_ptfb_init_fn)(void);
typedef void (*hako_ptfb_dispose_message_fn)(char*);
typedef void (*hako_ptfb_dispose_target_machine_fn)(void*);
typedef void (*hako_ptfb_dispose_target_data_fn)(void*);
typedef void (*hako_ptfb_dispose_memory_buffer_fn)(void*);
typedef void (*hako_ptfb_context_dispose_fn)(void*);
typedef void (*hako_ptfb_dispose_module_fn)(void*);
typedef void (*hako_ptfb_get_version_fn)(unsigned*, unsigned*, unsigned*);
typedef int (*hako_ptfb_get_target_from_triple_fn)(const char*, void**, char**);
typedef void* (*hako_ptfb_create_target_machine_fn)(
    void*, const char*, const char*, const char*, int, int, int);
typedef char* (*hako_ptfb_get_target_machine_triple_fn)(void*);
typedef void* (*hako_ptfb_create_target_data_layout_fn)(void*);
typedef char* (*hako_ptfb_copy_target_data_layout_fn)(void*);
typedef void* (*hako_ptfb_context_create_fn)(void);
typedef int (*hako_ptfb_create_memory_buffer_with_contents_of_file_fn)(
    const char*, void**, char**);
typedef void* (*hako_ptfb_create_memory_buffer_with_memory_range_copy_fn)(
    const char*, size_t, const char*);
typedef int (*hako_ptfb_parse_ir_in_context_fn)(void*, void*, void**, char**);
typedef void (*hako_ptfb_set_target_fn)(void*, const char*);
typedef void (*hako_ptfb_set_data_layout_fn)(void*, const char*);
typedef int (*hako_ptfb_emit_to_file_fn)(void*, void*, const char*, int, char**);

struct HakoPtfbTargetMachineSession;
struct HakoPtfSelectedCandidateView;
static int hako_llvmc_ptfc_verify_final_module_v1(
    struct HakoPtfbTargetMachineSession* session,
    void* module,
    const struct HakoPtfSelectedCandidateView* candidate,
    const char* function_symbol,
    char** err_out);

struct HakoPtfbTargetMachineSession {
  void* handle;
  void* target;
  void* machine;
  void* target_data;
  int active;
  hako_ptfb_dispose_message_fn dispose_message;
  hako_ptfb_dispose_target_machine_fn dispose_machine;
  hako_ptfb_dispose_target_data_fn dispose_target_data;
  hako_ptfb_dispose_memory_buffer_fn dispose_memory_buffer;
  hako_ptfb_context_dispose_fn context_dispose;
  hako_ptfb_dispose_module_fn dispose_module;
  hako_ptfb_context_create_fn context_create;
  hako_ptfb_create_memory_buffer_with_contents_of_file_fn create_buffer;
  hako_ptfb_create_memory_buffer_with_memory_range_copy_fn create_buffer_copy;
  hako_ptfb_parse_ir_in_context_fn parse_ir;
  hako_ptfb_set_target_fn set_target;
  hako_ptfb_set_data_layout_fn set_data_layout;
  hako_ptfb_emit_to_file_fn emit_to_file;
  char triple[128];
  char data_layout[512];
};

static void hako_llvmc_ptfb_session_zero(
    struct HakoPtfbTargetMachineSession* session) {
  if (session) memset(session, 0, sizeof(*session));
}

static void hako_llvmc_ptfb_session_close(
    struct HakoPtfbTargetMachineSession* session) {
  if (!session) return;
  if (session->target_data && session->dispose_target_data) {
    session->dispose_target_data(session->target_data);
  }
  if (session->machine && session->dispose_machine) {
    session->dispose_machine(session->machine);
  }
  if (session->handle) dlclose(session->handle);
  hako_llvmc_ptfb_session_zero(session);
}

static void* hako_llvmc_ptfb_session_symbol(void* handle, const char* name) {
  return handle && name ? dlsym(handle, name) : NULL;
}

static int hako_llvmc_ptfb_session_open(
    struct HakoPtfbTargetMachineSession* session,
    const struct HakoPtfbModuleCensus* census,
    char** err_out) {
#if defined(_WIN32)
  (void)session;
  (void)census;
  return set_err_owned(err_out, "contract-bound LLVM C API is unavailable on Windows");
#else
  static const char* const candidates[] = {
      "libLLVM-18.so", "libLLVM.so.18", "libLLVM.so", NULL};
  void* target = NULL;
  void* diagnostic = NULL;
  char* actual_triple = NULL;
  char* actual_layout = NULL;
  unsigned major = 0;
  unsigned minor = 0;
  unsigned patch = 0;
  hako_ptfb_get_target_from_triple_fn get_target_from_triple;
  hako_ptfb_create_target_machine_fn create_machine;
  hako_ptfb_get_target_machine_triple_fn get_machine_triple;
  hako_ptfb_create_target_data_layout_fn create_target_data;
  hako_ptfb_copy_target_data_layout_fn copy_target_data;
  hako_ptfb_get_version_fn get_version;
  hako_ptfb_init_fn init_target_info;
  hako_ptfb_init_fn init_target;
  hako_ptfb_init_fn init_target_mc;
  hako_ptfb_init_fn init_asm_printer;
  if (!session || !census || census->contract_count == 0) {
    return set_err_owned(err_out, "missing contract-bound module census");
  }
  hako_llvmc_ptfb_session_zero(session);
  for (size_t i = 0; candidates[i]; i++) {
    session->handle = dlopen(candidates[i], RTLD_LAZY | RTLD_LOCAL);
    if (session->handle) break;
  }
  if (!session->handle) {
    return set_err_owned(err_out, "LLVM 18 C API library unavailable");
  }
  session->dispose_message = (hako_ptfb_dispose_message_fn)
      hako_llvmc_ptfb_session_symbol(session->handle, "LLVMDisposeMessage");
  session->dispose_machine = (hako_ptfb_dispose_target_machine_fn)
      hako_llvmc_ptfb_session_symbol(session->handle, "LLVMDisposeTargetMachine");
  session->dispose_target_data = (hako_ptfb_dispose_target_data_fn)
      hako_llvmc_ptfb_session_symbol(session->handle, "LLVMDisposeTargetData");
  session->dispose_memory_buffer = (hako_ptfb_dispose_memory_buffer_fn)
      hako_llvmc_ptfb_session_symbol(session->handle, "LLVMDisposeMemoryBuffer");
  session->context_dispose = (hako_ptfb_context_dispose_fn)
      hako_llvmc_ptfb_session_symbol(session->handle, "LLVMContextDispose");
  session->dispose_module = (hako_ptfb_dispose_module_fn)
      hako_llvmc_ptfb_session_symbol(session->handle, "LLVMDisposeModule");
  session->context_create = (hako_ptfb_context_create_fn)
      hako_llvmc_ptfb_session_symbol(session->handle, "LLVMContextCreate");
  session->create_buffer = (hako_ptfb_create_memory_buffer_with_contents_of_file_fn)
      hako_llvmc_ptfb_session_symbol(
          session->handle, "LLVMCreateMemoryBufferWithContentsOfFile");
  session->create_buffer_copy =
      (hako_ptfb_create_memory_buffer_with_memory_range_copy_fn)
          hako_llvmc_ptfb_session_symbol(
              session->handle, "LLVMCreateMemoryBufferWithMemoryRangeCopy");
  session->parse_ir = (hako_ptfb_parse_ir_in_context_fn)
      hako_llvmc_ptfb_session_symbol(session->handle, "LLVMParseIRInContext");
  session->set_target = (hako_ptfb_set_target_fn)
      hako_llvmc_ptfb_session_symbol(session->handle, "LLVMSetTarget");
  session->set_data_layout = (hako_ptfb_set_data_layout_fn)
      hako_llvmc_ptfb_session_symbol(session->handle, "LLVMSetDataLayout");
  session->emit_to_file = (hako_ptfb_emit_to_file_fn)
      hako_llvmc_ptfb_session_symbol(session->handle, "LLVMTargetMachineEmitToFile");
  get_version = (hako_ptfb_get_version_fn)
      hako_llvmc_ptfb_session_symbol(session->handle, "LLVMGetVersion");
  get_target_from_triple = (hako_ptfb_get_target_from_triple_fn)
      hako_llvmc_ptfb_session_symbol(session->handle, "LLVMGetTargetFromTriple");
  create_machine = (hako_ptfb_create_target_machine_fn)
      hako_llvmc_ptfb_session_symbol(session->handle, "LLVMCreateTargetMachine");
  get_machine_triple = (hako_ptfb_get_target_machine_triple_fn)
      hako_llvmc_ptfb_session_symbol(session->handle, "LLVMGetTargetMachineTriple");
  create_target_data = (hako_ptfb_create_target_data_layout_fn)
      hako_llvmc_ptfb_session_symbol(session->handle, "LLVMCreateTargetDataLayout");
  copy_target_data = (hako_ptfb_copy_target_data_layout_fn)
      hako_llvmc_ptfb_session_symbol(session->handle, "LLVMCopyStringRepOfTargetData");
  init_target_info = (hako_ptfb_init_fn)
      hako_llvmc_ptfb_session_symbol(session->handle, "LLVMInitializeX86TargetInfo");
  init_target = (hako_ptfb_init_fn)
      hako_llvmc_ptfb_session_symbol(session->handle, "LLVMInitializeX86Target");
  init_target_mc = (hako_ptfb_init_fn)
      hako_llvmc_ptfb_session_symbol(session->handle, "LLVMInitializeX86TargetMC");
  init_asm_printer = (hako_ptfb_init_fn)
      hako_llvmc_ptfb_session_symbol(session->handle, "LLVMInitializeX86AsmPrinter");
  if (!session->dispose_message || !session->dispose_machine ||
      !session->dispose_target_data || !session->dispose_memory_buffer ||
      !session->context_dispose || !session->dispose_module ||
      !session->context_create || !session->create_buffer ||
      !session->create_buffer_copy || !session->parse_ir ||
      !session->set_target || !session->set_data_layout || !session->emit_to_file ||
      !get_version || !get_target_from_triple || !create_machine ||
      !get_machine_triple || !create_target_data || !copy_target_data ||
      !init_target_info || !init_target || !init_target_mc || !init_asm_printer) {
    hako_llvmc_ptfb_session_close(session);
    return set_err_owned(err_out, "LLVM 18 C API symbol set is incomplete");
  }
  get_version(&major, &minor, &patch);
  if (major != 18) {
    hako_llvmc_ptfb_session_close(session);
    return set_err_owned(err_out, "LLVM C API major revision is not 18");
  }
  init_target_info();
  init_target();
  init_target_mc();
  init_asm_printer();
  if (get_target_from_triple(HAKO_PTFB_TARGET_TRIPLE, &target, (char**)&diagnostic) != 0 ||
      !target) {
    if (diagnostic) session->dispose_message((char*)diagnostic);
    hako_llvmc_ptfb_session_close(session);
    return set_err_owned(err_out, "LLVM 18 rejected the contract target triple");
  }
  if (diagnostic) {
    session->dispose_message((char*)diagnostic);
    diagnostic = NULL;
  }
  session->machine = create_machine(
      target,
      HAKO_PTFB_TARGET_TRIPLE,
      HAKO_PTFB_OBJECT_CPU,
      HAKO_PTFB_OBJECT_FEATURES,
      HAKO_PTFB_OBJECT_CODEGEN_OPT_LEVEL,
      HAKO_PTFB_OBJECT_RELOCATION_MODEL,
      HAKO_PTFB_OBJECT_CODE_MODEL);
  if (!session->machine) {
    hako_llvmc_ptfb_session_close(session);
    return set_err_owned(err_out, "contract TargetMachine creation failed");
  }
  session->target_data = create_target_data(session->machine);
  actual_triple = get_machine_triple(session->machine);
  actual_layout = session->target_data ? copy_target_data(session->target_data) : NULL;
  if (!actual_triple || !actual_layout ||
      strcmp(actual_triple, HAKO_PTFB_TARGET_TRIPLE) != 0 ||
      strcmp(actual_layout, HAKO_PTFB_DATA_LAYOUT) != 0) {
    if (actual_triple) session->dispose_message(actual_triple);
    if (actual_layout) session->dispose_message(actual_layout);
    hako_llvmc_ptfb_session_close(session);
    return set_err_owned(err_out, "contract TargetMachine triple/data-layout mismatch");
  }
  snprintf(session->triple, sizeof(session->triple), "%s", actual_triple);
  snprintf(session->data_layout, sizeof(session->data_layout), "%s", actual_layout);
  session->dispose_message(actual_triple);
  session->dispose_message(actual_layout);
  session->target = target;
  session->active = 1;
  return 0;
#endif
}

static const char* hako_llvmc_ptfb_session_triple(
    const struct HakoPtfbTargetMachineSession* session) {
  return session && session->triple[0] ? session->triple : HAKO_PTFB_TARGET_TRIPLE;
}

static const char* hako_llvmc_ptfb_session_data_layout(
    const struct HakoPtfbTargetMachineSession* session) {
  return session && session->data_layout[0] ? session->data_layout : HAKO_PTFB_DATA_LAYOUT;
}

static int hako_llvmc_ptfb_session_emit_owned_buffer(
    struct HakoPtfbTargetMachineSession* session,
    void* buffer,
    const struct HakoPtfSelectedCandidateView* selected_candidate,
    const char* selected_function_symbol,
    const char* obj_out,
    char** err_out) {
#if defined(_WIN32)
  (void)session;
  (void)buffer;
  (void)obj_out;
  return set_err_owned(err_out, "contract-bound LLVM C API is unavailable on Windows");
#else
  char tmp_path[1024];
  int n;
  void* context = NULL;
  void* module = NULL;
  char* diagnostic = NULL;
  int result = -1;
  if (!(session && session->active && buffer && obj_out && obj_out[0])) {
    if (buffer && session && session->dispose_memory_buffer) {
      session->dispose_memory_buffer(buffer);
    }
    return set_err_owned(err_out, "inactive contract TargetMachine session");
  }
  n = snprintf(tmp_path, sizeof(tmp_path), "%s.ptfb-tm-%ld.tmp", obj_out, (long)getpid());
  if (n <= 0 || (size_t)n >= sizeof(tmp_path)) {
    session->dispose_memory_buffer(buffer);
    return set_err_owned(err_out, "contract object temporary path is too long");
  }
  remove(tmp_path);
  context = session->context_create();
  if (!context) goto fail;
  {
    int parse_failed = session->parse_ir(context, buffer, &module, &diagnostic);
    /* LLVMParseIRInContext consumes the memory buffer on both outcomes. */
    buffer = NULL;
    if (parse_failed != 0 || !module) goto fail;
  }
  session->set_target(module, hako_llvmc_ptfb_session_triple(session));
  session->set_data_layout(module, hako_llvmc_ptfb_session_data_layout(session));
  if (selected_candidate &&
      hako_llvmc_ptfc_verify_final_module_v1(
          session,
          module,
          selected_candidate,
          selected_function_symbol,
          err_out) != 0) {
    remove(tmp_path);
    goto cleanup;
  }
  if (session->emit_to_file(
          session->machine, module, tmp_path, /*LLVMObjectFile*/ 1, &diagnostic) != 0) {
    goto fail;
  }
  if (!hako_llvmc_file_exists(tmp_path) || rename(tmp_path, obj_out) != 0) goto fail;
  result = 0;
  goto cleanup;
fail:
  set_err_owned(
      err_out,
      diagnostic ? diagnostic : "contract TargetMachine object emission failed");
  if (diagnostic && session->dispose_message) {
    session->dispose_message(diagnostic);
    diagnostic = NULL;
  }
  remove(tmp_path);
cleanup:
  if (diagnostic && session->dispose_message) session->dispose_message(diagnostic);
  if (buffer && session->dispose_memory_buffer) session->dispose_memory_buffer(buffer);
  if (module && session->dispose_module) session->dispose_module(module);
  if (context && session->context_dispose) session->context_dispose(context);
  return result;
#endif
}

static int hako_llvmc_ptfb_session_emit_object(
    struct HakoPtfbTargetMachineSession* session,
    const char* llpath,
    const char* obj_out,
    char** err_out) {
#if defined(_WIN32)
  (void)session;
  (void)llpath;
  (void)obj_out;
  return set_err_owned(err_out, "contract-bound LLVM C API is unavailable on Windows");
#else
  void* buffer = NULL;
  char* diagnostic = NULL;
  if (!(session && session->active && llpath && obj_out && obj_out[0])) {
    return set_err_owned(err_out, "inactive contract TargetMachine session");
  }
  if (session->create_buffer(llpath, &buffer, &diagnostic) != 0 || !buffer) {
    if (buffer && session->dispose_memory_buffer) session->dispose_memory_buffer(buffer);
    if (diagnostic && session->dispose_message) session->dispose_message(diagnostic);
    return set_err_owned(err_out, "contract LLVM input file could not be read");
  }
  if (diagnostic && session->dispose_message) session->dispose_message(diagnostic);
  return hako_llvmc_ptfb_session_emit_owned_buffer(
      session, buffer, NULL, NULL, obj_out, err_out);
#endif
}

static int hako_llvmc_ptfb_session_emit_object_from_bytes(
    struct HakoPtfbTargetMachineSession* session,
    const char* bytes,
    size_t byte_count,
    const struct HakoPtfSelectedCandidateView* selected_candidate,
    const char* selected_function_symbol,
    const char* obj_out,
    char** err_out) {
  void* buffer = NULL;
  if (!(session && session->active && bytes && byte_count &&
        selected_candidate && selected_function_symbol && selected_function_symbol[0] &&
        obj_out && obj_out[0])) {
    return set_err_owned(err_out, "inactive contract TargetMachine byte ingress");
  }
  buffer = session->create_buffer_copy(
      bytes, byte_count, "hako-pinned-text-selected.ll");
  if (!buffer) {
    return set_err_owned(err_out, "contract LLVM byte buffer creation failed");
  }
  return hako_llvmc_ptfb_session_emit_owned_buffer(
      session,
      buffer,
      selected_candidate,
      selected_function_symbol,
      obj_out,
      err_out);
}

#endif
