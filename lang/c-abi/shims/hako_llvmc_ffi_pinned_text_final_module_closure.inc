/* Selected-only final LLVM module closure.  No pass or mutation may follow. */
#ifndef HAKO_LLVM_C_PINNED_TEXT_FINAL_MODULE_CLOSURE_INC
#define HAKO_LLVM_C_PINNED_TEXT_FINAL_MODULE_CLOSURE_INC

typedef int (*hako_ptfc_verify_module_fn)(void*, int, char**);
typedef const char* (*hako_ptfc_get_module_text_fn)(void*);
typedef void* (*hako_ptfc_get_named_function_fn)(void*, const char*);
typedef void* (*hako_ptfc_get_first_function_fn)(void*);
typedef void* (*hako_ptfc_get_next_function_fn)(void*);
typedef int (*hako_ptfc_is_declaration_fn)(void*);
typedef unsigned (*hako_ptfc_count_params_fn)(void*);
typedef unsigned (*hako_ptfc_get_enum_attribute_kind_fn)(const char*, size_t);
typedef void* (*hako_ptfc_get_enum_attribute_fn)(void*, int, unsigned);
typedef void* (*hako_ptfc_get_first_basic_block_fn)(void*);
typedef void* (*hako_ptfc_get_next_basic_block_fn)(void*);
typedef void* (*hako_ptfc_get_first_instruction_fn)(void*);
typedef void* (*hako_ptfc_get_next_instruction_fn)(void*);
typedef int (*hako_ptfc_get_instruction_opcode_fn)(void*);
typedef void* (*hako_ptfc_get_called_value_fn)(void*);
typedef const char* (*hako_ptfc_get_value_name_fn)(void*, size_t*);

struct HakoPtfFinalModuleApiV1 {
  hako_ptfc_verify_module_fn verify_module;
  hako_ptfc_get_module_text_fn get_target;
  hako_ptfc_get_module_text_fn get_data_layout;
  hako_ptfc_get_named_function_fn get_named_function;
  hako_ptfc_get_first_function_fn get_first_function;
  hako_ptfc_get_next_function_fn get_next_function;
  hako_ptfc_is_declaration_fn is_declaration;
  hako_ptfc_count_params_fn count_params;
  hako_ptfc_get_enum_attribute_kind_fn get_attribute_kind;
  hako_ptfc_get_enum_attribute_fn get_attribute;
  hako_ptfc_get_first_basic_block_fn get_first_block;
  hako_ptfc_get_next_basic_block_fn get_next_block;
  hako_ptfc_get_first_instruction_fn get_first_instruction;
  hako_ptfc_get_next_instruction_fn get_next_instruction;
  hako_ptfc_get_instruction_opcode_fn get_opcode;
  hako_ptfc_get_called_value_fn get_called_value;
  hako_ptfc_get_value_name_fn get_value_name;
};

enum {
  HAKO_PTFC_LLVM_RETURN_STATUS_ACTION = 2,
  HAKO_PTFC_LLVM_ATTRIBUTE_FUNCTION_INDEX = -1,
  HAKO_PTFC_LLVM_RET = 1,
  HAKO_PTFC_LLVM_INVOKE = 5,
  HAKO_PTFC_LLVM_UNREACHABLE = 7,
  HAKO_PTFC_LLVM_CALL = 45,
  HAKO_PTFC_LLVM_RESUME = 58,
  HAKO_PTFC_LLVM_LANDING_PAD = 59,
  HAKO_PTFC_LLVM_CLEANUP_RET = 61,
  HAKO_PTFC_LLVM_CATCH_RET = 62,
  HAKO_PTFC_LLVM_CATCH_PAD = 63,
  HAKO_PTFC_LLVM_CLEANUP_PAD = 64,
  HAKO_PTFC_LLVM_CATCH_SWITCH = 65,
  HAKO_PTFC_LLVM_CALL_BR = 67,
};

static int hako_llvmc_ptfc_load_final_module_api_v1(
    struct HakoPtfbTargetMachineSession* session,
    struct HakoPtfFinalModuleApiV1* api,
    char** err_out) {
  if (!(session && session->handle && api)) {
    return set_err_owned(err_out, "missing selected final-module API owner");
  }
  memset(api, 0, sizeof(*api));
#define HAKO_PTFC_LOAD(field, type, symbol) \
  api->field = (type)hako_llvmc_ptfb_session_symbol(session->handle, symbol)
  HAKO_PTFC_LOAD(verify_module, hako_ptfc_verify_module_fn, "LLVMVerifyModule");
  HAKO_PTFC_LOAD(get_target, hako_ptfc_get_module_text_fn, "LLVMGetTarget");
  HAKO_PTFC_LOAD(
      get_data_layout, hako_ptfc_get_module_text_fn, "LLVMGetDataLayoutStr");
  HAKO_PTFC_LOAD(
      get_named_function, hako_ptfc_get_named_function_fn, "LLVMGetNamedFunction");
  HAKO_PTFC_LOAD(
      get_first_function, hako_ptfc_get_first_function_fn, "LLVMGetFirstFunction");
  HAKO_PTFC_LOAD(
      get_next_function, hako_ptfc_get_next_function_fn, "LLVMGetNextFunction");
  HAKO_PTFC_LOAD(is_declaration, hako_ptfc_is_declaration_fn, "LLVMIsDeclaration");
  HAKO_PTFC_LOAD(count_params, hako_ptfc_count_params_fn, "LLVMCountParams");
  HAKO_PTFC_LOAD(
      get_attribute_kind,
      hako_ptfc_get_enum_attribute_kind_fn,
      "LLVMGetEnumAttributeKindForName");
  HAKO_PTFC_LOAD(
      get_attribute,
      hako_ptfc_get_enum_attribute_fn,
      "LLVMGetEnumAttributeAtIndex");
  HAKO_PTFC_LOAD(
      get_first_block, hako_ptfc_get_first_basic_block_fn, "LLVMGetFirstBasicBlock");
  HAKO_PTFC_LOAD(
      get_next_block, hako_ptfc_get_next_basic_block_fn, "LLVMGetNextBasicBlock");
  HAKO_PTFC_LOAD(
      get_first_instruction,
      hako_ptfc_get_first_instruction_fn,
      "LLVMGetFirstInstruction");
  HAKO_PTFC_LOAD(
      get_next_instruction,
      hako_ptfc_get_next_instruction_fn,
      "LLVMGetNextInstruction");
  HAKO_PTFC_LOAD(
      get_opcode, hako_ptfc_get_instruction_opcode_fn, "LLVMGetInstructionOpcode");
  HAKO_PTFC_LOAD(
      get_called_value, hako_ptfc_get_called_value_fn, "LLVMGetCalledValue");
  HAKO_PTFC_LOAD(get_value_name, hako_ptfc_get_value_name_fn, "LLVMGetValueName2");
#undef HAKO_PTFC_LOAD
  if (!api->verify_module || !api->get_target || !api->get_data_layout ||
      !api->get_named_function || !api->get_first_function ||
      !api->get_next_function || !api->is_declaration || !api->count_params ||
      !api->get_attribute_kind || !api->get_attribute || !api->get_first_block ||
      !api->get_next_block || !api->get_first_instruction ||
      !api->get_next_instruction || !api->get_opcode || !api->get_called_value ||
      !api->get_value_name) {
    return set_err_owned(err_out, "LLVM18 final-module API set is incomplete");
  }
  return 0;
}

static int hako_llvmc_ptfc_value_name_is_v1(
    const struct HakoPtfFinalModuleApiV1* api,
    void* value,
    const char* expected) {
  size_t length = 0;
  const char* name;
  size_t expected_length;
  if (!(api && value && expected)) return 0;
  name = api->get_value_name(value, &length);
  expected_length = strlen(expected);
  return name && length == expected_length &&
      memcmp(name, expected, expected_length) == 0;
}

static int hako_llvmc_ptfc_has_function_attribute_v1(
    const struct HakoPtfFinalModuleApiV1* api,
    void* function,
    const char* name) {
  unsigned kind;
  if (!(api && function && name)) return 0;
  kind = api->get_attribute_kind(name, strlen(name));
  return kind && api->get_attribute(
      function, HAKO_PTFC_LLVM_ATTRIBUTE_FUNCTION_INDEX, kind) != NULL;
}

static int hako_llvmc_ptfc_is_eh_opcode_v1(int opcode) {
  return opcode == HAKO_PTFC_LLVM_INVOKE || opcode == HAKO_PTFC_LLVM_CALL_BR ||
      opcode == HAKO_PTFC_LLVM_RESUME || opcode == HAKO_PTFC_LLVM_LANDING_PAD ||
      opcode == HAKO_PTFC_LLVM_CLEANUP_RET || opcode == HAKO_PTFC_LLVM_CATCH_RET ||
      opcode == HAKO_PTFC_LLVM_CATCH_PAD || opcode == HAKO_PTFC_LLVM_CLEANUP_PAD ||
      opcode == HAKO_PTFC_LLVM_CATCH_SWITCH;
}

static int hako_llvmc_ptfc_runtime_declaration_matches_v1(
    const struct HakoPtfFinalModuleApiV1* api,
    void* function,
    unsigned parameter_count) {
  static const char* const forbidden[] = {
      "noreturn", "readonly", "readnone", "nofree", "speculatable"};
  if (!(function && api->is_declaration(function) &&
        api->count_params(function) == parameter_count &&
        hako_llvmc_ptfc_has_function_attribute_v1(api, function, "nounwind"))) {
    return 0;
  }
  for (size_t i = 0; i < sizeof(forbidden) / sizeof(forbidden[0]); i++) {
    if (hako_llvmc_ptfc_has_function_attribute_v1(api, function, forbidden[i])) {
      return 0;
    }
  }
  return 1;
}

static int hako_llvmc_ptfc_verify_final_module_v1(
    struct HakoPtfbTargetMachineSession* session,
    void* module,
    const struct HakoPtfSelectedCandidateView* candidate,
    const char* function_symbol,
    char** err_out) {
  struct HakoPtfFinalModuleApiV1 api;
  void* function;
  void* enter;
  void* finish;
  void* first_block;
  char* diagnostic = NULL;
  unsigned definition_count = 0;
  unsigned enter_count = 0;
  unsigned finish_count = 0;
  unsigned trap_count = 0;
  unsigned return_count = 0;
  unsigned unreachable_count = 0;
  if (!(session && module && candidate && function_symbol && function_symbol[0])) {
    return set_err_owned(err_out, "missing selected final-module expectation");
  }
  if (hako_llvmc_ptfc_load_final_module_api_v1(session, &api, err_out) != 0) {
    return -1;
  }
  if (api.verify_module(
          module, HAKO_PTFC_LLVM_RETURN_STATUS_ACTION, &diagnostic) != 0) {
    set_err_owned(
        err_out, diagnostic ? diagnostic : "selected LLVM module verification failed");
    if (diagnostic) session->dispose_message(diagnostic);
    return -1;
  }
  if (diagnostic) session->dispose_message(diagnostic);
  if (!api.get_target(module) || !api.get_data_layout(module) ||
      strcmp(api.get_target(module), session->triple) != 0 ||
      strcmp(api.get_data_layout(module), session->data_layout) != 0) {
    return set_err_owned(err_out, "selected final-module target/layout mismatch");
  }
  function = api.get_named_function(module, function_symbol);
  enter = api.get_named_function(module, "hako_text_formal_residence_enter_v1");
  finish = api.get_named_function(
      module, "hako_text_formal_residence_finish_or_abort_v1");
  if (!(function && !api.is_declaration(function) &&
        hako_llvmc_ptfc_runtime_declaration_matches_v1(&api, enter, 4) &&
        hako_llvmc_ptfc_runtime_declaration_matches_v1(&api, finish, 1))) {
    return set_err_owned(err_out, "selected final-module function/ABI mismatch");
  }
  for (void* current = api.get_first_function(module); current;
       current = api.get_next_function(current)) {
    if (!api.is_declaration(current)) definition_count++;
  }
  first_block = api.get_first_block(function);
  for (void* block = first_block; block; block = api.get_next_block(block)) {
    void* previous = NULL;
    for (void* instruction = api.get_first_instruction(block); instruction;
         instruction = api.get_next_instruction(instruction)) {
      int opcode = api.get_opcode(instruction);
      void* next = api.get_next_instruction(instruction);
      if (hako_llvmc_ptfc_is_eh_opcode_v1(opcode)) {
        return set_err_owned(err_out, "selected final-module EH construct is forbidden");
      }
      if (opcode == HAKO_PTFC_LLVM_CALL) {
        void* callee = api.get_called_value(instruction);
        if (hako_llvmc_ptfc_value_name_is_v1(&api, callee,
                "hako_text_formal_residence_enter_v1")) {
          if (block != first_block) {
            return set_err_owned(err_out, "selected Enter escaped the entry block");
          }
          enter_count++;
        } else if (hako_llvmc_ptfc_value_name_is_v1(&api, callee,
                       "hako_text_formal_residence_finish_or_abort_v1")) {
          if (!next || api.get_opcode(next) != HAKO_PTFC_LLVM_RET) {
            return set_err_owned(err_out, "selected Finish is not followed by Return");
          }
          finish_count++;
        } else if (hako_llvmc_ptfc_value_name_is_v1(&api, callee, "llvm.trap")) {
          if (!next || api.get_opcode(next) != HAKO_PTFC_LLVM_UNREACHABLE) {
            return set_err_owned(err_out, "selected Trap is not terminal");
          }
          trap_count++;
        } else {
          return set_err_owned(err_out, "selected final-module call set drifted");
        }
      } else if (opcode == HAKO_PTFC_LLVM_RET) {
        if (!previous || api.get_opcode(previous) != HAKO_PTFC_LLVM_CALL ||
            !hako_llvmc_ptfc_value_name_is_v1(
                &api,
                api.get_called_value(previous),
                "hako_text_formal_residence_finish_or_abort_v1")) {
          return set_err_owned(err_out, "selected Return bypassed Finish");
        }
        return_count++;
      } else if (opcode == HAKO_PTFC_LLVM_UNREACHABLE) {
        unreachable_count++;
      }
      previous = instruction;
    }
  }
  if (definition_count != 1 || enter_count != 1 || trap_count != 1 ||
      unreachable_count != 1 ||
      finish_count != (unsigned)candidate->carrier.normal_exit_count ||
      return_count != (unsigned)candidate->carrier.normal_exit_count) {
    return set_err_owned(err_out, "selected final-module lifecycle census mismatch");
  }
  return 0;
}

#endif
