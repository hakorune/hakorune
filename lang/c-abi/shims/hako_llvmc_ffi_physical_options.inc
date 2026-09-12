/* Invocation-owned physical compile options.  The public contract is borrowed
 * at admission; these strings are copied before any lowering/effect. */
struct HakoLlvmcPhysicalOptions {
  char* compile_recipe;
  char* compat_replay;
  char* opt_level;
  char* opt_tool_path;
  char* llc_tool_path;
  char* llc_flags;
  char* llvmc_path;
  int explicit_contract;
};

static int hako_llvmc_physical_options_is_explicit(
    const struct HakoLlvmcPhysicalOptions* options) {
  return options && options->explicit_contract;
}

static char* hako_llvmc_duplicate_option(const char* value) {
  size_t length;
  char* copy;
  if (!value) return NULL;
  length = strlen(value);
  copy = (char*)malloc(length + 1);
  if (!copy) return NULL;
  memcpy(copy, value, length + 1);
  return copy;
}

static void hako_llvmc_physical_options_destroy(
    struct HakoLlvmcPhysicalOptions* options) {
  if (!options) return;
  free(options->compile_recipe);
  free(options->compat_replay);
  free(options->opt_level);
  free(options->opt_tool_path);
  free(options->llc_tool_path);
  free(options->llc_flags);
  free(options->llvmc_path);
  *options = (struct HakoLlvmcPhysicalOptions){0};
}

static int hako_llvmc_physical_options_copy(
    struct HakoLlvmcPhysicalOptions* destination,
    const hako_llvmc_physical_contract_v1* contract,
    char** error) {
  struct HakoLlvmcPhysicalOptions copy = {0};
  const char* level;
  if (!destination || !contract) {
    return set_err_owned(error, "[freeze:contract][compile-options/null]");
  }
  if (contract->revision != HAKO_LLVMC_PHYSICAL_CONTRACT_REVISION ||
      contract->byte_size != sizeof(*contract)) {
    return set_err_owned(error, "[freeze:contract][compile-options/revision-size]");
  }
  if ((contract->ingress_profile != HAKO_LLVMC_PHYSICAL_PROFILE_GENERIC_COMPAT &&
       contract->ingress_profile != HAKO_LLVMC_PHYSICAL_PROFILE_BOUNDARY_PURE_FIRST &&
       contract->ingress_profile != HAKO_LLVMC_PHYSICAL_PROFILE_STATIC_V2) ||
      contract->flags != 0) {
    return set_err_owned(error, "[freeze:contract][compile-options/profile-flags]");
  }
  if (!contract->compile_recipe || strcmp(contract->compile_recipe, "pure-first") != 0 ||
      !contract->compat_replay || strcmp(contract->compat_replay, "none") != 0) {
    return set_err_owned(error, "[freeze:contract][compile-options/recipe-replay]");
  }
  level = contract->opt_level;
  if (!level || level[0] < '0' || level[0] > '3' || level[1] != '\0') {
    return set_err_owned(error, "[freeze:contract][compile-options/opt-level]");
  }
  if (contract->llvmc_path) {
    return set_err_owned(error, "[freeze:contract][compile-options/llvmc-unsupported]");
  }
  if (contract->opt_tool_path && !hako_llvmc_tool_exists(contract->opt_tool_path)) {
    return set_err_owned(error, "[freeze:contract][compile-options/opt-tool]");
  }
  if (contract->llc_tool_path && !hako_llvmc_tool_exists(contract->llc_tool_path)) {
    return set_err_owned(error, "[freeze:contract][compile-options/llc-tool]");
  }
  copy.compile_recipe = hako_llvmc_duplicate_option(contract->compile_recipe);
  copy.compat_replay = hako_llvmc_duplicate_option(contract->compat_replay);
  copy.opt_level = hako_llvmc_duplicate_option(level);
  copy.opt_tool_path = hako_llvmc_duplicate_option(contract->opt_tool_path);
  copy.llc_tool_path = hako_llvmc_duplicate_option(contract->llc_tool_path);
  copy.llc_flags = hako_llvmc_duplicate_option(contract->llc_flags);
  copy.explicit_contract = 1;
  if (!copy.compile_recipe || !copy.compat_replay || !copy.opt_level ||
      (contract->opt_tool_path && !copy.opt_tool_path) ||
      (contract->llc_tool_path && !copy.llc_tool_path) ||
      (contract->llc_flags && !copy.llc_flags)) {
    hako_llvmc_physical_options_destroy(&copy);
    return set_err_owned(error, "[freeze:contract][compile-options/oom]");
  }
  *destination = copy;
  return 0;
}

static const char* hako_llvmc_compile_recipe_for(
    const struct HakoLlvmcPhysicalOptions* options) {
  return options && options->explicit_contract
      ? options->compile_recipe : hako_llvmc_backend_compile_recipe();
}

static const char* hako_llvmc_compat_replay_for(
    const struct HakoLlvmcPhysicalOptions* options) {
  return options && options->explicit_contract
      ? options->compat_replay : hako_llvmc_backend_compat_replay();
}

static const char* hako_llvmc_opt_level_for(
    const struct HakoLlvmcPhysicalOptions* options) {
  return options && options->explicit_contract && options->opt_level
      ? options->opt_level : hako_llvmc_opt_level();
}

static const char* hako_llvmc_llc_flags_for(
    const struct HakoLlvmcPhysicalOptions* options) {
  if (options && options->explicit_contract) {
    return options->llc_flags ? options->llc_flags : "";
  }
  return hako_llvmc_llc_flags();
}

static const char* hako_llvmc_resolve_explicit_tool(
    const struct HakoLlvmcPhysicalOptions* options,
    int opt_tool) {
  const char* configured = options && options->explicit_contract
      ? (opt_tool ? options->opt_tool_path : options->llc_tool_path) : NULL;
  if (configured) return configured;
  if (opt_tool) {
    if (hako_llvmc_tool_exists("opt")) return "opt";
    if (hako_llvmc_tool_exists("opt-18")) return "opt-18";
  } else {
    if (hako_llvmc_tool_exists("llc")) return "llc";
    if (hako_llvmc_tool_exists("llc-18")) return "llc-18";
  }
  return NULL;
}
