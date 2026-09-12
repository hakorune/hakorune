/* AOT Generic FFI compile transport.  The caller owns admission; this file
 * only packages the existing physical-options contract and invokes it. */

static int hako_aot_use_ffi(void) {
  return hako_aot_env_truthy(getenv("HAKO_AOT_USE_FFI"));
}

static void hako_aot_ensure_default_opt_env(void) {
  if (!getenv("HAKO_LLVM_OPT_LEVEL")) {
    setenv("HAKO_LLVM_OPT_LEVEL", "0", 1);
  }
  if (!getenv("NYASH_LLVM_OPT_LEVEL")) {
    setenv("NYASH_LLVM_OPT_LEVEL", "0", 1);
  }
}

#if !defined(_WIN32)
static const char* hako_aot_effective_opt_level(void) {
  const char* value = getenv("NYASH_LLVM_OPT_LEVEL");
  if (value && value[0]) return value;
  value = getenv("HAKO_LLVM_OPT_LEVEL");
  return (value && value[0]) ? value : "0";
}

static const char* hako_aot_effective_llc_flags(void) {
  const char* value = getenv("NYASH_NY_LLVM_LLC_FLAGS");
  return (value && value[0]) ? value : "-O3 -mcpu=native";
}

static const char* hako_aot_configured_tool(const char* env_key) {
  const char* value = getenv(env_key);
  return (value && value[0]) ? value : NULL;
}

static int hako_aot_try_ffi_compile(
    const char* mir_json_path,
    const char* obj_path,
    char** err_out) {
  void* h = hako_aot_open_ffi_lib();
  if (!h) {
    hako_set_last_error("UNSUPPORTED");
    return hako_aot_set_err(err_out, "FFI library not found");
  }
  typedef int (*ffi_compile_with_options_fn)(
      const char*, const char*, const hako_llvmc_physical_contract_v1*, char**);
  ffi_compile_with_options_fn fn = (ffi_compile_with_options_fn)dlsym(
      h, "hako_llvmc_compile_json_with_options_v1");
  if (!fn) {
    dlclose(h);
    hako_set_last_error("UNSUPPORTED");
    return hako_aot_set_err(
        err_out,
        "FFI symbol missing: hako_llvmc_compile_json_with_options_v1");
  }
  hako_llvmc_physical_contract_v1 contract = {
      HAKO_LLVMC_PHYSICAL_CONTRACT_REVISION,
      sizeof(hako_llvmc_physical_contract_v1),
      HAKO_LLVMC_PHYSICAL_PROFILE_GENERIC_COMPAT,
      0,
      "pure-first",
      "none",
      hako_aot_effective_opt_level(),
      hako_aot_configured_tool("NYASH_NY_LLVM_OPT_TOOL"),
      hako_aot_configured_tool("NYASH_NY_LLVM_LLC_TOOL"),
      hako_aot_effective_llc_flags(),
      NULL,
  };
  int rc = fn(mir_json_path, obj_path, &contract, err_out);
  dlclose(h);
  return rc;
}
#endif

static int hako_aot_reject_legacy_capi_pure_alias(char** err_out) {
  const char* value = getenv("HAKO_CAPI_PURE");
  if (!value || value[0] != '1') {
    return 0;
  }
  hako_set_last_error("UNSUPPORTED");
  return hako_aot_set_err(
      err_out,
      "[freeze:contract][env/hako_capi_pure_retired] HAKO_CAPI_PURE is retired; use HAKO_BACKEND_COMPILE_RECIPE=pure-first");
}

static int hako_aot_require_pure_first_recipe(char** err_out) {
  const char* recipe = getenv("HAKO_BACKEND_COMPILE_RECIPE");
  if (recipe && strcmp(recipe, "pure-first") == 0) {
    return 0;
  }
  hako_set_last_error("UNSUPPORTED");
  return hako_aot_set_err(
      err_out,
      "[freeze:route][generic-aot-recipe-required] set HAKO_BACKEND_COMPILE_RECIPE=pure-first");
}

static int hako_aot_reject_ambient_harness_replay(char** err_out) {
  const char* replay = getenv("HAKO_BACKEND_COMPAT_REPLAY");
  if (!replay || !*replay || strcmp(replay, "none") == 0) {
    return 0;
  }
  hako_set_last_error("UNSUPPORTED");
  return hako_aot_set_err(
      err_out,
      "[freeze:route][aot-compat-admission-required] use hako_aot_compile_json_compat_harness");
}
