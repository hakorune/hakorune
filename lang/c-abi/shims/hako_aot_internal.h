// Private cross-translation-unit AOT seams.
// These are transport helpers, not public language or MIR ABI.

#pragma once

enum hako_aot_link_invocation_mode {
  HAKO_AOT_LINK_INVOCATION_COMPAT_V1 = 1,
  HAKO_AOT_LINK_INVOCATION_EXPLICIT_V2 = 2,
};

int hako_aot_link_obj_for_llvmc(
    const char* obj_path,
    const char* exe_path,
    const char* runtime_archive_path,
    const char* extra_ldflags,
    enum hako_aot_link_invocation_mode mode,
    char** err_out);
