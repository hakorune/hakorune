/* Lifecycle-only LLVM target observation. No pinned-text contract enters here. */
#include "../../../../include/nyrt_fault_v1.h"

typedef void (*hako_lts_init_fn)(void);
typedef void (*hako_lts_dispose_message_fn)(char*);
typedef void (*hako_lts_dispose_machine_fn)(void*);
typedef void (*hako_lts_dispose_data_fn)(void*);
typedef void (*hako_lts_get_version_fn)(unsigned*, unsigned*, unsigned*);
typedef int (*hako_lts_get_target_fn)(const char*, void**, char**);
typedef void* (*hako_lts_make_machine_fn)(void*, const char*, const char*, const char*, int, int, int);
typedef char* (*hako_lts_machine_triple_fn)(void*);
typedef void* (*hako_lts_make_data_fn)(void*);
typedef unsigned (*hako_lts_pointer_size_fn)(void*);

static int hako_llvmc_lifecycle_target_session_v1_validate(
    const hako_llvmc_lifecycle_target_session_v1* in, char** err_out) {
#if defined(_WIN32)
  (void)in;
  return set_err_owned(err_out, "[freeze:contract][lifecycle-session/windows]");
#else
  void *lib = NULL, *target = NULL, *machine = NULL, *data = NULL;
  char *diagnostic = NULL, *actual_triple = NULL;
  unsigned major = 0, minor = 0, patch = 0;
  hako_lts_dispose_message_fn dispose_message;
  hako_lts_dispose_machine_fn dispose_machine;
  hako_lts_dispose_data_fn dispose_data;
  hako_lts_get_version_fn get_version;
  hako_lts_get_target_fn get_target;
  hako_lts_make_machine_fn make_machine;
  hako_lts_machine_triple_fn machine_triple;
  hako_lts_make_data_fn make_data;
  hako_lts_pointer_size_fn pointer_size;
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
  get_version = HAKO_LTS_SYM("LLVMGetVersion", hako_lts_get_version_fn);
  get_target = HAKO_LTS_SYM("LLVMGetTargetFromTriple", hako_lts_get_target_fn);
  make_machine = HAKO_LTS_SYM("LLVMCreateTargetMachine", hako_lts_make_machine_fn);
  machine_triple = HAKO_LTS_SYM("LLVMGetTargetMachineTriple", hako_lts_machine_triple_fn);
  make_data = HAKO_LTS_SYM("LLVMCreateTargetDataLayout", hako_lts_make_data_fn);
  pointer_size = HAKO_LTS_SYM("LLVMPointerSize", hako_lts_pointer_size_fn);
  init_info = HAKO_LTS_SYM("LLVMInitializeX86TargetInfo", hako_lts_init_fn);
  init_target = HAKO_LTS_SYM("LLVMInitializeX86Target", hako_lts_init_fn);
  init_mc = HAKO_LTS_SYM("LLVMInitializeX86TargetMC", hako_lts_init_fn);
  init_printer = HAKO_LTS_SYM("LLVMInitializeX86AsmPrinter", hako_lts_init_fn);
#undef HAKO_LTS_SYM
  if (!dispose_message || !dispose_machine || !dispose_data || !get_version || !get_target ||
      !make_machine || !machine_triple || !make_data || !pointer_size || !init_info || !init_target || !init_mc || !init_printer)
    goto cleanup;
  get_version(&major, &minor, &patch);
  if (major != 18u) goto cleanup;
  init_info(); init_target(); init_mc(); init_printer();
  if (get_target(in->target_triple, &target, &diagnostic) != 0 || !target) goto cleanup;
  if (diagnostic) { dispose_message(diagnostic); diagnostic = NULL; }
  machine = make_machine(target, in->target_triple, "generic", "", 0, 0, 0);
  data = machine ? make_data(machine) : NULL;
  actual_triple = machine ? machine_triple(machine) : NULL;
  if (!data || !actual_triple || strcmp(actual_triple, in->target_triple) != 0 || pointer_size(data) != in->pointer_width)
    goto cleanup;
  rc = 0;
cleanup:
  if (diagnostic && dispose_message) dispose_message(diagnostic);
  if (actual_triple && dispose_message) dispose_message(actual_triple);
  if (data && dispose_data) dispose_data(data);
  if (machine && dispose_machine) dispose_machine(machine);
  if (lib) dlclose(lib);
  if (rc != 0) return set_err_owned(err_out, "[freeze:contract][lifecycle-session/llvm-layout]");
  return 0;
#endif
}
