/*
 * Strict consumer for the Rust-owned pinned Text backend-frame contract.
 * This file validates a projection; it never reconstructs lane/root meaning
 * and never issues a second contract.  The realization constants below are
 * the currently selected pure-first target row, not a producer authority.
 */
#include <limits.h>
#if !defined(_WIN32)
#include <dlfcn.h>
#endif

#ifndef HAKO_LLVM_C_PTFB_CONTRACT_INC
#define HAKO_LLVM_C_PTFB_CONTRACT_INC

#define HAKO_PTFB_CONTRACT_ID "hako.pinned_text_backend_frame@1"
#define HAKO_PTFB_SCHEMA_REVISION 1
#define HAKO_PTFB_PROFILE_ID "nyrt-text-residence-ptr64-as0-v1"
#define HAKO_PTFB_TARGET_TRIPLE "x86_64-pc-linux-gnu"
#define HAKO_PTFB_DATA_LAYOUT \
  "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
#define HAKO_PTFB_RESIDENCE_ABI "text-formal-residence-v1"
#define HAKO_PTFB_CONSUMER_ABI "hako-llvmc-pure-first-v1"
#define HAKO_PTFB_HEADER_SIZE 32
#define HAKO_PTFB_ROOT_ROW_SIZE 16
#define HAKO_PTFB_ALIGNMENT 8
#define HAKO_PTFB_MAX_ROOTS 1024
#define HAKO_PTFB_MAX_FRAME_BYTES 65536

static int hako_llvmc_ptfb_keys_only(
    yyjson_val* object,
    const char* const* keys,
    size_t key_count) {
  yyjson_obj_iter iter;
  yyjson_val* key;
  if (!(object && yyjson_is_obj(object))) return 0;
  iter = yyjson_obj_iter_with(object);
  while ((key = yyjson_obj_iter_next(&iter))) {
    const char* name = yyjson_is_str(key) ? yyjson_get_str(key) : NULL;
    int allowed = 0;
    if (!name) return 0;
    for (size_t i = 0; i < key_count; i++) {
      if (strcmp(name, keys[i]) == 0) {
        allowed = 1;
        break;
      }
    }
    if (!allowed) return 0;
  }
  return 1;
}

static int hako_llvmc_ptfb_nonnegative(
    yyjson_val* object,
    const char* key,
    long long* out) {
  yyjson_val* value = object ? yyjson_obj_get(object, key) : NULL;
  if (!(value && yyjson_is_int(value) && yyjson_get_sint(value) >= 0)) return 0;
  if (out) *out = (long long)yyjson_get_sint(value);
  return 1;
}

static int hako_llvmc_ptfb_text(
    yyjson_val* object,
    const char* key,
    const char* expected) {
  yyjson_val* value = object ? yyjson_obj_get(object, key) : NULL;
  const char* text = value && yyjson_is_str(value) ? yyjson_get_str(value) : NULL;
  return text && expected && strcmp(text, expected) == 0;
}

static int hako_llvmc_ptfb_bool(
    yyjson_val* object,
    const char* key,
    int expected) {
  yyjson_val* value = object ? yyjson_obj_get(object, key) : NULL;
  return value && yyjson_is_bool(value) && (yyjson_get_bool(value) ? 1 : 0) == expected;
}

/*
 * The Rust contract owns the expected profile.  This helper only observes
 * the selected LLVM TargetMachine and returns a private realization check;
 * it never issues or rewrites the contract.  It runs before the pure-first
 * lowering walk, so a missing library, unsupported triple, or realized layout
 * drift cannot reach IR effect.
 */
static int hako_llvmc_ptfb_validate_target_machine_realization(
    const char* expected_triple,
    const char* expected_layout,
    char** err_out) {
#if defined(_WIN32)
  (void)expected_triple;
  (void)expected_layout;
  return set_err_owned(
      err_out,
      "pinned Text backend-frame TargetMachine query unavailable");
#else
  typedef void (*ptfb_init_fn)(void);
  typedef void (*ptfb_dispose_message_fn)(char*);
  typedef int (*ptfb_get_target_from_triple_fn)(const char*, void**, char**);
  typedef void* (*ptfb_create_target_machine_fn)(
      void*, const char*, const char*, const char*, int, int, int);
  typedef char* (*ptfb_get_target_machine_triple_fn)(void*);
  typedef void* (*ptfb_create_target_data_layout_fn)(void*);
  typedef char* (*ptfb_copy_target_data_layout_fn)(void*);
  typedef void (*ptfb_dispose_target_data_fn)(void*);
  typedef void (*ptfb_dispose_target_machine_fn)(void*);
  static const char* const candidates[] = {
      "libLLVM-20.so",
      "libLLVM-18.so",
      "libLLVM.so.18",
      "libLLVM-17.so",
      "libLLVM.so",
      NULL};
  void* handle = NULL;
  void* target = NULL;
  void* machine = NULL;
  void* target_data = NULL;
  char* diagnostic = NULL;
  char* actual_triple = NULL;
  char* actual_layout = NULL;
  int result = -1;
  ptfb_dispose_message_fn dispose_message;
  ptfb_get_target_from_triple_fn get_target_from_triple;
  ptfb_create_target_machine_fn create_target_machine;
  ptfb_get_target_machine_triple_fn get_target_machine_triple;
  ptfb_create_target_data_layout_fn create_target_data_layout;
  ptfb_copy_target_data_layout_fn copy_target_data_layout;
  ptfb_dispose_target_data_fn dispose_target_data;
  ptfb_dispose_target_machine_fn dispose_target_machine;
  ptfb_init_fn initialize_target_info;
  ptfb_init_fn initialize_target;
  ptfb_init_fn initialize_target_mc;
  ptfb_init_fn initialize_asm_printer;

  if (!expected_triple || !expected_layout) {
    return set_err_owned(
        err_out,
        "pinned Text backend-frame TargetMachine expectation missing");
  }
  for (size_t i = 0; candidates[i]; i++) {
    handle = dlopen(candidates[i], RTLD_LAZY | RTLD_LOCAL);
    if (handle) break;
  }
  if (!handle) {
    return set_err_owned(
        err_out,
        "pinned Text backend-frame TargetMachine query unavailable");
  }

#define HAKO_PTFB_DLSYM(name, type) ((type)dlsym(handle, name))
  dispose_message = HAKO_PTFB_DLSYM("LLVMDisposeMessage", ptfb_dispose_message_fn);
  get_target_from_triple = HAKO_PTFB_DLSYM(
      "LLVMGetTargetFromTriple", ptfb_get_target_from_triple_fn);
  create_target_machine = HAKO_PTFB_DLSYM(
      "LLVMCreateTargetMachine", ptfb_create_target_machine_fn);
  get_target_machine_triple = HAKO_PTFB_DLSYM(
      "LLVMGetTargetMachineTriple", ptfb_get_target_machine_triple_fn);
  create_target_data_layout = HAKO_PTFB_DLSYM(
      "LLVMCreateTargetDataLayout", ptfb_create_target_data_layout_fn);
  copy_target_data_layout = HAKO_PTFB_DLSYM(
      "LLVMCopyStringRepOfTargetData", ptfb_copy_target_data_layout_fn);
  dispose_target_data = HAKO_PTFB_DLSYM(
      "LLVMDisposeTargetData", ptfb_dispose_target_data_fn);
  dispose_target_machine = HAKO_PTFB_DLSYM(
      "LLVMDisposeTargetMachine", ptfb_dispose_target_machine_fn);
  initialize_target_info = HAKO_PTFB_DLSYM(
      "LLVMInitializeX86TargetInfo", ptfb_init_fn);
  initialize_target = HAKO_PTFB_DLSYM("LLVMInitializeX86Target", ptfb_init_fn);
  initialize_target_mc = HAKO_PTFB_DLSYM("LLVMInitializeX86TargetMC", ptfb_init_fn);
  initialize_asm_printer = HAKO_PTFB_DLSYM(
      "LLVMInitializeX86AsmPrinter", ptfb_init_fn);
#undef HAKO_PTFB_DLSYM

  if (!dispose_message || !get_target_from_triple || !create_target_machine ||
      !get_target_machine_triple || !create_target_data_layout ||
      !copy_target_data_layout || !dispose_target_data ||
      !dispose_target_machine || !initialize_target_info || !initialize_target ||
      !initialize_target_mc || !initialize_asm_printer) {
    set_err_owned(
        err_out,
        "pinned Text backend-frame TargetMachine query unavailable");
    goto cleanup;
  }
  initialize_target_info();
  initialize_target();
  initialize_target_mc();
  initialize_asm_printer();

  if (get_target_from_triple(expected_triple, &target, &diagnostic) != 0 || !target) {
    if (diagnostic) dispose_message(diagnostic);
    diagnostic = NULL;
    set_err_owned(
        err_out,
        "pinned Text backend-frame TargetMachine rejected triple");
    goto cleanup;
  }
  if (diagnostic) {
    dispose_message(diagnostic);
    diagnostic = NULL;
  }
  machine = create_target_machine(
      target, expected_triple, "", "", 0, /*Reloc*/ 0, /*CodeModel*/ 0);
  if (!machine) {
    set_err_owned(
        err_out,
        "pinned Text backend-frame TargetMachine creation failed");
    goto cleanup;
  }
  target_data = create_target_data_layout(machine);
  actual_triple = get_target_machine_triple(machine);
  actual_layout = target_data ? copy_target_data_layout(target_data) : NULL;
  if (!actual_triple || !actual_layout ||
      strcmp(actual_triple, expected_triple) != 0 ||
      strcmp(actual_layout, expected_layout) != 0) {
    set_err_owned(
        err_out,
        "pinned Text backend-frame TargetMachine layout mismatch");
    goto cleanup;
  }
  result = 0;

cleanup:
  if (diagnostic && dispose_message) dispose_message(diagnostic);
  if (actual_triple && dispose_message) dispose_message(actual_triple);
  if (actual_layout && dispose_message) dispose_message(actual_layout);
  if (target_data && dispose_target_data) dispose_target_data(target_data);
  if (machine && dispose_target_machine) dispose_target_machine(machine);
  dlclose(handle);
  return result;
#endif
}

static int hako_llvmc_ptfb_validate_contract(
    yyjson_val* contract,
    char** err_out) {
  static const char* const contract_keys[] = {
      "contract_id", "schema_revision", "owner", "invocation_ordinal",
      "source_logical_arity", "receiver_lane_count",
      "physical_formal_lane_count", "physical_callable_lane_count",
      "exact_text_root_count", "plan_stamp", "plan_count", "residence",
      "target"};
  static const char* const owner_keys[] = {"compilation_brand", "slot"};
  static const char* const residence_keys[] = {
      "abi_revision", "frame_revision", "header_size", "root_row_size",
      "header_alignment", "root_row_alignment", "max_root_count",
      "max_frame_bytes", "derived_frame_size"};
  static const char* const target_keys[] = {
      "profile_id", "triple", "data_layout", "little_endian",
      "address_space_zero_pointer_width", "address_space_zero_abi_alignment",
      "max_root_count", "max_private_frame_bytes", "residence_abi_revision",
      "consumer_abi_revision"};
  yyjson_val* owner;
  yyjson_val* residence;
  yyjson_val* target;
  const char* target_triple = NULL;
  const char* target_layout = NULL;
  long long compilation_brand = 0;
  long long invocation = 0;
  long long receiver = 0;
  long long formal_lanes = 0;
  long long callable_lanes = 0;
  long long roots = 0;
  long long schema_revision = 0;
  long long frame_revision = 0;
  long long header_size = 0;
  long long root_row_size = 0;
  long long header_alignment = 0;
  long long root_row_alignment = 0;
  long long residence_max_roots = 0;
  long long residence_max_bytes = 0;
  long long frame_size = 0;
  long long target_width = 0;
  long long target_alignment = 0;
  long long target_max_roots = 0;
  long long target_max_bytes = 0;
  long long source_arity = 0;
  long long plan_count = 0;

  if (!(contract && yyjson_is_obj(contract)) ||
      !hako_llvmc_ptfb_keys_only(
          contract, contract_keys, sizeof(contract_keys) / sizeof(contract_keys[0])) ||
      !hako_llvmc_ptfb_text(contract, "contract_id", HAKO_PTFB_CONTRACT_ID) ||
      !hako_llvmc_ptfb_nonnegative(contract, "schema_revision", &schema_revision) ||
      schema_revision != HAKO_PTFB_SCHEMA_REVISION ||
      !hako_llvmc_ptfb_nonnegative(contract, "invocation_ordinal", &invocation) ||
      invocation == 0 ||
      !hako_llvmc_ptfb_nonnegative(contract, "source_logical_arity", &source_arity) ||
      !hako_llvmc_ptfb_nonnegative(contract, "receiver_lane_count", &receiver) ||
      !hako_llvmc_ptfb_nonnegative(contract, "physical_formal_lane_count", &formal_lanes) ||
      !hako_llvmc_ptfb_nonnegative(contract, "physical_callable_lane_count", &callable_lanes) ||
      !hako_llvmc_ptfb_nonnegative(contract, "exact_text_root_count", &roots) ||
      !hako_llvmc_ptfb_nonnegative(contract, "plan_count", &plan_count)) {
    return set_err_owned(err_out, "invalid pinned Text backend-frame contract header");
  }
  (void)source_arity;
  (void)plan_count;
  if (receiver > 1 || formal_lanes < 0 ||
      receiver > callable_lanes ||
      callable_lanes - receiver != formal_lanes || roots <= 0 ||
      roots > HAKO_PTFB_MAX_ROOTS) {
    return set_err_owned(err_out, "pinned Text backend-frame lane census mismatch");
  }

  owner = yyjson_obj_get(contract, "owner");
  if (!hako_llvmc_ptfb_keys_only(owner, owner_keys, 2) ||
      !hako_llvmc_ptfb_nonnegative(owner, "compilation_brand", &compilation_brand) ||
      compilation_brand == 0 ||
      !hako_llvmc_ptfb_nonnegative(owner, "slot", NULL)) {
    return set_err_owned(err_out, "invalid pinned Text backend-frame owner stamp");
  }

  residence = yyjson_obj_get(contract, "residence");
  if (!hako_llvmc_ptfb_keys_only(
          residence, residence_keys, sizeof(residence_keys) / sizeof(residence_keys[0])) ||
      !hako_llvmc_ptfb_text(residence, "abi_revision", HAKO_PTFB_RESIDENCE_ABI) ||
      !hako_llvmc_ptfb_nonnegative(residence, "frame_revision", &frame_revision) ||
      !hako_llvmc_ptfb_nonnegative(residence, "header_size", &header_size) ||
      !hako_llvmc_ptfb_nonnegative(residence, "root_row_size", &root_row_size) ||
      !hako_llvmc_ptfb_nonnegative(residence, "header_alignment", &header_alignment) ||
      !hako_llvmc_ptfb_nonnegative(residence, "root_row_alignment", &root_row_alignment) ||
      !hako_llvmc_ptfb_nonnegative(residence, "max_root_count", &residence_max_roots) ||
      !hako_llvmc_ptfb_nonnegative(residence, "max_frame_bytes", &residence_max_bytes) ||
      !hako_llvmc_ptfb_nonnegative(residence, "derived_frame_size", &frame_size) ||
      frame_revision != 1 || header_size != HAKO_PTFB_HEADER_SIZE ||
      root_row_size != HAKO_PTFB_ROOT_ROW_SIZE ||
      header_alignment != HAKO_PTFB_ALIGNMENT ||
      root_row_alignment != HAKO_PTFB_ALIGNMENT ||
      residence_max_roots != HAKO_PTFB_MAX_ROOTS ||
      residence_max_bytes != HAKO_PTFB_MAX_FRAME_BYTES ||
      roots > 0 && root_row_size > (LLONG_MAX - header_size) / roots ||
      frame_size != header_size + root_row_size * roots ||
      frame_size > residence_max_bytes) {
    return set_err_owned(err_out, "pinned Text backend-frame Residence layout mismatch");
  }

  target = yyjson_obj_get(contract, "target");
  if (!hako_llvmc_ptfb_keys_only(
          target, target_keys, sizeof(target_keys) / sizeof(target_keys[0])) ||
      !hako_llvmc_ptfb_text(target, "profile_id", HAKO_PTFB_PROFILE_ID) ||
      !hako_llvmc_ptfb_text(target, "triple", HAKO_PTFB_TARGET_TRIPLE) ||
      !hako_llvmc_ptfb_text(target, "data_layout", HAKO_PTFB_DATA_LAYOUT) ||
      !hako_llvmc_ptfb_bool(target, "little_endian", 1) ||
      !hako_llvmc_ptfb_nonnegative(target, "address_space_zero_pointer_width", &target_width) ||
      !hako_llvmc_ptfb_nonnegative(target, "address_space_zero_abi_alignment", &target_alignment) ||
      !hako_llvmc_ptfb_nonnegative(target, "max_root_count", &target_max_roots) ||
      !hako_llvmc_ptfb_nonnegative(target, "max_private_frame_bytes", &target_max_bytes) ||
      !hako_llvmc_ptfb_text(target, "residence_abi_revision", HAKO_PTFB_RESIDENCE_ABI) ||
      !hako_llvmc_ptfb_text(target, "consumer_abi_revision", HAKO_PTFB_CONSUMER_ABI) ||
      target_width != 64 || target_alignment != HAKO_PTFB_ALIGNMENT ||
      target_max_roots != HAKO_PTFB_MAX_ROOTS ||
      target_max_bytes != HAKO_PTFB_MAX_FRAME_BYTES) {
    return set_err_owned(err_out, "pinned Text backend-frame target layout mismatch");
  }
  target_triple = yyjson_get_str(yyjson_obj_get(target, "triple"));
  target_layout = yyjson_get_str(yyjson_obj_get(target, "data_layout"));
  if (hako_llvmc_ptfb_validate_target_machine_realization(
          target_triple, target_layout, err_out) != 0) {
    return -1;
  }
  return 0;
}

static int hako_llvmc_validate_pinned_text_backend_frame_contract_file(
    const char* json_in,
    char** err_out) {
  yyjson_read_err read_error;
  yyjson_doc* document = yyjson_read_file(json_in, 0, NULL, &read_error);
  yyjson_val* root;
  yyjson_val* functions;
  if (!document) return set_err_owned(err_out, "cannot read pinned Text backend-frame JSON");
  root = yyjson_doc_get_root(document);
  functions = root ? yyjson_obj_get(root, "functions") : NULL;
  if (!(functions && yyjson_is_arr(functions))) {
    yyjson_doc_free(document);
    return set_err_owned(err_out, "pinned Text backend-frame JSON has no functions");
  }
  for (size_t i = 0; i < yyjson_arr_size(functions); i++) {
    yyjson_val* function = yyjson_arr_get(functions, i);
    yyjson_val* metadata = function ? yyjson_obj_get(function, "metadata") : NULL;
    yyjson_val* contract = metadata ? yyjson_obj_get(metadata, "pinned_text_backend_frame_v1") : NULL;
    if (!contract) continue;
    if (hako_llvmc_ptfb_validate_contract(contract, err_out) != 0) {
      yyjson_doc_free(document);
      return -1;
    }
  }
  yyjson_doc_free(document);
  return 0;
}

#endif
