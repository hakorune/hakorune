/* Direct-harness child process transport.
 *
 * The direct AOT harness owns an effective HAKO/NYASH option pair.  POSIX
 * keeps using the existing shell command issuer; Windows must pass an
 * environment block at CreateProcess time so NAME= remains distinct from an
 * absent variable.
 */

#if defined(_WIN32)
#include <windows.h>
#endif

struct HakoAotDirectHarnessEnv {
  const char* hako_opt_level;
  const char* nyash_opt_level;
};

static void hako_aot_capture_direct_harness_env(
    struct HakoAotDirectHarnessEnv* out) {
  const char* hako;
  const char* nyash;
  if (!out) return;
  hako = getenv("HAKO_LLVM_OPT_LEVEL");
  nyash = getenv("NYASH_LLVM_OPT_LEVEL");
  out->hako_opt_level = hako ? hako : "0";
  out->nyash_opt_level = nyash ? nyash : "0";
}

#if defined(_WIN32)
static char hako_aot_windows_ascii_lower(char value) {
  return (value >= 'A' && value <= 'Z') ? (char)(value - 'A' + 'a') : value;
}

static int hako_aot_windows_env_entry_matches(
    const char* entry, const char* name) {
  size_t i;
  size_t name_len;
  if (!entry || !name) return 0;
  name_len = strlen(name);
  for (i = 0; i < name_len; i++) {
    if (entry[i] == '\0' ||
        hako_aot_windows_ascii_lower(entry[i]) !=
            hako_aot_windows_ascii_lower(name[i])) {
      return 0;
    }
  }
  return entry[name_len] == '=';
}

static size_t hako_aot_windows_env_value_len(
    const char* name, const char* value) {
  return strlen(name) + 1 + strlen(value ? value : "");
}

static int hako_aot_windows_env_name_compare_entry(
    const char* name, const char* entry) {
  size_t i = 0;
  if (!name || !entry) return 0;
  while (name[i] && entry[i] && entry[i] != '=') {
    char lhs = hako_aot_windows_ascii_lower(name[i]);
    char rhs = hako_aot_windows_ascii_lower(entry[i]);
    if (lhs < rhs) return -1;
    if (lhs > rhs) return 1;
    i++;
  }
  if (!name[i] && (!entry[i] || entry[i] == '=')) return 0;
  return !name[i] ? -1 : 1;
}

static int hako_aot_windows_env_block_size(
    const char* inherited,
    const struct HakoAotDirectHarnessEnv* child_env,
    size_t* out_size,
    int found[2]) {
  static const char* const names[] = {
      "HAKO_LLVM_OPT_LEVEL", "NYASH_LLVM_OPT_LEVEL"};
  const char* entry;
  size_t total = 1; /* The second NUL after the final entry. */
  size_t i;
  if (!inherited || !child_env || !out_size || !found) return -1;
  found[0] = 0;
  found[1] = 0;
  for (entry = inherited; *entry; entry += strlen(entry) + 1) {
    int match = -1;
    for (i = 0; i < 2; i++) {
      if (hako_aot_windows_env_entry_matches(entry, names[i])) {
        match = (int)i;
        found[i] = 1;
        break;
      }
    }
    if (match >= 0) {
      size_t item = hako_aot_windows_env_value_len(
          names[match], match == 0 ? child_env->hako_opt_level
                                   : child_env->nyash_opt_level);
      if (item > SIZE_MAX - total - 1) return -1;
      total += item + 1;
    } else {
      size_t item = strlen(entry);
      if (item > SIZE_MAX - total - 1) return -1;
      total += item + 1;
    }
  }
  for (i = 0; i < 2; i++) {
    if (!found[i]) {
      size_t item = hako_aot_windows_env_value_len(
          names[i], i == 0 ? child_env->hako_opt_level
                            : child_env->nyash_opt_level);
      if (item > SIZE_MAX - total - 1) return -1;
      total += item + 1;
    }
  }
  *out_size = total;
  return 0;
}

static char* hako_aot_windows_build_env_block(
    const struct HakoAotDirectHarnessEnv* child_env) {
  static const char* const names[] = {
      "HAKO_LLVM_OPT_LEVEL", "NYASH_LLVM_OPT_LEVEL"};
  LPCH inherited = GetEnvironmentStringsA();
  char* block = NULL;
  char* cursor;
  const char* entry;
  size_t size;
  int found[2];
  int inserted[2] = {0, 0};
  size_t i;
  if (!inherited) return NULL;
  if (hako_aot_windows_env_block_size(
          inherited, child_env, &size, found) != 0) {
    FreeEnvironmentStringsA(inherited);
    return NULL;
  }
  block = (char*)malloc(size);
  if (!block) {
    FreeEnvironmentStringsA(inherited);
    return NULL;
  }
  cursor = block;
  for (entry = inherited; *entry; entry += strlen(entry) + 1) {
    int match = -1;
    for (i = 0; i < 2; i++) {
      if (!found[i] && !inserted[i] &&
          hako_aot_windows_env_name_compare_entry(names[i], entry) < 0) {
        const char* value = i == 0 ? child_env->hako_opt_level
                                   : child_env->nyash_opt_level;
        size_t name_len = strlen(names[i]);
        size_t value_len = strlen(value ? value : "");
        memcpy(cursor, names[i], name_len);
        cursor += name_len;
        *cursor++ = '=';
        memcpy(cursor, value ? value : "", value_len);
        cursor += value_len;
        *cursor++ = '\0';
        inserted[i] = 1;
      }
    }
    for (i = 0; i < 2; i++) {
      if (hako_aot_windows_env_entry_matches(entry, names[i])) {
        match = (int)i;
        break;
      }
    }
    if (match >= 0) {
      const char* value =
          match == 0 ? child_env->hako_opt_level : child_env->nyash_opt_level;
      size_t name_len = strlen(names[match]);
      size_t value_len = strlen(value ? value : "");
      memcpy(cursor, names[match], name_len);
      cursor += name_len;
      *cursor++ = '=';
      memcpy(cursor, value ? value : "", value_len);
      cursor += value_len;
      *cursor++ = '\0';
    } else {
      size_t item = strlen(entry) + 1;
      memcpy(cursor, entry, item);
      cursor += item;
    }
  }
  for (i = 0; i < 2; i++) {
    if (!found[i] && !inserted[i]) {
      const char* value = i == 0 ? child_env->hako_opt_level
                                 : child_env->nyash_opt_level;
      size_t name_len = strlen(names[i]);
      size_t value_len = strlen(value ? value : "");
      memcpy(cursor, names[i], name_len);
      cursor += name_len;
      *cursor++ = '=';
      memcpy(cursor, value ? value : "", value_len);
      cursor += value_len;
      *cursor++ = '\0';
    }
  }
  *cursor = '\0';
  FreeEnvironmentStringsA(inherited);
  return block;
}
#endif

static int hako_aot_run_direct_harness_process(
    const char* cmd,
    const struct HakoAotDirectHarnessEnv* child_env) {
#if defined(_WIN32)
  size_t cmd_len;
  char* cmdline;
  char* env_block;
  int written;
  STARTUPINFOA startup;
  PROCESS_INFORMATION process;
  DWORD wait_result;
  DWORD exit_code = 1;
  if (!cmd || !child_env) return -1;
  env_block = hako_aot_windows_build_env_block(child_env);
  if (!env_block) return -1;
  cmd_len = strlen(cmd);
  if (cmd_len > SIZE_MAX - 32) {
    free(env_block);
    return -1;
  }
  cmdline = (char*)malloc(cmd_len + 32);
  if (!cmdline) {
    free(env_block);
    return -1;
  }
  written = snprintf(cmdline, cmd_len + 32, "cmd.exe /D /S /C \"%s\"", cmd);
  if (written <= 0 || (size_t)written >= cmd_len + 32) {
    free(cmdline);
    free(env_block);
    return -1;
  }
  ZeroMemory(&startup, sizeof(startup));
  ZeroMemory(&process, sizeof(process));
  startup.cb = sizeof(startup);
  if (!CreateProcessA(
          NULL, cmdline, NULL, NULL, TRUE, 0, env_block, NULL, &startup,
          &process)) {
    free(cmdline);
    free(env_block);
    return -1;
  }
  free(cmdline);
  free(env_block);
  wait_result = WaitForSingleObject(process.hProcess, INFINITE);
  if (wait_result == WAIT_OBJECT_0) {
    (void)GetExitCodeProcess(process.hProcess, &exit_code);
  }
  CloseHandle(process.hThread);
  CloseHandle(process.hProcess);
  return (wait_result == WAIT_OBJECT_0 && exit_code == 0) ? 0 : -1;
#else
  (void)child_env;
  return cmd ? system(cmd) : -1;
#endif
}
